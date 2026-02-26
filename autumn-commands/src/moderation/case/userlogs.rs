use poise::serenity_prelude as serenity;

use crate::CommandMeta;
use crate::moderation::embeds::{guild_only_message, usage_message};
use autumn_core::{Context, Error};
use autumn_database::impls::user_logs::{UserLogFilters, list_recent_user_logs};
use autumn_utils::pagination::paginate_embed_pages;
use autumn_utils::permissions::has_user_permission;

pub const META: CommandMeta = CommandMeta {
    name: "userlogs",
    desc: "View recent user message edit/delete activity.",
    category: "moderation",
    usage: "!userlogs [target_user] [event]",
};

const LOGS_PER_PAGE: usize = 5;

#[poise::command(prefix_command, slash_command, category = "Moderation")]
pub async fn userlogs(
    ctx: Context<'_>,
    #[description = "Filter by target user"] target_user: Option<serenity::User>,
    #[description = "Filter by event (message_edit, message_delete, attachment_delete)"]
    event: Option<String>,
) -> Result<(), Error> {
    let Some(guild_id) = ctx.guild_id() else {
        ctx.say(guild_only_message()).await?;
        return Ok(());
    };

    if !has_user_permission(
        ctx.http(),
        guild_id,
        ctx.author().id,
        serenity::Permissions::MANAGE_MESSAGES,
    )
    .await?
    {
        return Ok(());
    }

    if event
        .as_deref()
        .is_some_and(|value| value.trim().is_empty())
    {
        ctx.say(usage_message(META.usage)).await?;
        return Ok(());
    }

    let rows = list_recent_user_logs(
        &ctx.data().db,
        guild_id.get(),
        UserLogFilters {
            author_user_id: target_user.as_ref().map(|user| user.id.get()),
            event_type: event
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty()),
            limit: 200,
        },
    )
    .await?;

    if rows.is_empty() {
        ctx.say("No matching user log entries found.").await?;
        return Ok(());
    }

    let total = rows.len();
    let total_pages = total.div_ceil(LOGS_PER_PAGE);
    let mut pages = Vec::with_capacity(total_pages);

    for page in 0..total_pages {
        let start = page * LOGS_PER_PAGE;
        let end = (start + LOGS_PER_PAGE).min(total);

        let mut body = String::new();
        body.push_str(&format!("Total entries: **{}**\n\n", total));

        for entry in &rows[start..end] {
            let event_name = match entry.event_type.as_str() {
                "message_edit" => "Message Edit",
                "message_delete" => "Message Delete",
                "attachment_delete" => "Attachment Delete",
                _ => "Event",
            };

            let mut fields = Vec::new();
            fields.push(format!("**Event :** {}", event_name));

            if entry.event_type == "message_delete" {
                if let Some(before) = entry
                    .before_content
                    .as_deref()
                    .filter(|value| !value.is_empty())
                {
                    fields.push(format!("**Message :** {}", format_content_display(before)));
                }
            } else {
                if let Some(before) = entry
                    .before_content
                    .as_deref()
                    .filter(|value| !value.is_empty())
                {
                    fields.push(format!(
                        "**Before :** {}",
                        truncate_text(&sanitize(before), 300)
                    ));
                }

                if let Some(after) = entry
                    .after_content
                    .as_deref()
                    .filter(|value| !value.is_empty())
                {
                    fields.push(format!(
                        "**After :** {}",
                        truncate_text(&sanitize(after), 300)
                    ));
                }
            }

            if let Some(attachments) = entry
                .attachment_summary
                .as_deref()
                .filter(|value| !value.trim().is_empty())
            {
                let filenames = extract_attachment_filenames(attachments);
                fields.push(format!("**Attachments :** {}", filenames));
            }

            if let Some(author_id) = entry.author_user_id {
                fields.push(format!("**Author :** <@{}>", author_id));
            }

            fields.push(format!("**Channel :** <#{}>", entry.channel_id));

            if let Some(message_id) = entry.message_id {
                let jump_url = format!(
                    "https://discord.com/channels/{}/{}/{}",
                    guild_id.get(),
                    entry.channel_id,
                    message_id
                );
                fields.push(format!("**[Jump to context]({})**", jump_url));
            }

            fields.push(format!("**When :** <t:{}:R>", entry.created_at));

            body.push_str(&format!("{}\n\n", fields.join("\n")));
        }

        pages.push(body.trim_end().to_owned());
    }

    paginate_embed_pages(ctx, "User Logs", &pages, 1).await?;
    Ok(())
}

fn sanitize(value: &str) -> String {
    value.replace('@', "@\u{200B}").replace('\n', " ")
}

fn truncate_text(value: &str, max_len: usize) -> String {
    if value.chars().count() <= max_len {
        return value.to_owned();
    }

    let mut out = String::new();
    for character in value.chars().take(max_len) {
        out.push(character);
    }
    out.push('…');
    out
}

fn format_content_display(content: &str) -> String {
    let trimmed = content.trim();
    let lower = trimmed.to_ascii_lowercase();

    if lower.contains("tenor.com") || lower.contains("giphy.com") {
        return format!(
            "[GIF]({})",
            trimmed.split_whitespace().next().unwrap_or(trimmed)
        );
    }

    let media_extensions = [".png", ".jpg", ".jpeg", ".gif", ".webp"];
    let video_extensions = [".mp4", ".webm", ".mov", ".mkv"];

    if let Some(url) = trimmed.split_whitespace().next() {
        let url_lower = url.to_ascii_lowercase();
        if (url_lower.starts_with("http://") || url_lower.starts_with("https://"))
            && media_extensions.iter().any(|ext| url_lower.ends_with(ext))
        {
            return format!("[Image]({})", url);
        }
        if (url_lower.starts_with("http://") || url_lower.starts_with("https://"))
            && video_extensions.iter().any(|ext| url_lower.ends_with(ext))
        {
            return format!("[Video]({})", url);
        }
    }

    truncate_text(&sanitize(trimmed), 300)
}

fn extract_attachment_filenames(raw: &str) -> String {
    raw.lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                return None;
            }
            if let Some(start) = trimmed.rfind(" (")
                && trimmed.ends_with(')')
            {
                let filename = &trimmed[..start];
                let url = &trimmed[start + 2..trimmed.len() - 1];
                return Some(format!("[{}]({})", filename, url));
            }
            Some(trimmed.to_owned())
        })
        .collect::<Vec<_>>()
        .join(", ")
}
