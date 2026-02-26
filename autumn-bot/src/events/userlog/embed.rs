use poise::serenity_prelude as serenity;
use tracing::error;

use autumn_core::Data;
use autumn_database::impls::userlog_config::get_userlog_channel_id;

use super::media::{
    download_media_bytes, extract_first_media_url, extract_first_unfurl_link, infer_media_filename,
    is_direct_image_url, parse_attachment_summary, sanitize_attachment_filename,
};
use super::util::{sanitize_mentions, truncate_for_embed};

/// Data needed to build and publish a user log embed.
pub struct PublishUserLogEntry<'a> {
    pub guild_id: serenity::GuildId,
    pub event_type: &'a str,
    pub channel_id: serenity::ChannelId,
    pub message_id: Option<serenity::MessageId>,
    pub author_user_id: Option<serenity::UserId>,
    pub deleted_by_user_id: Option<serenity::UserId>,
    pub before_content: Option<&'a str>,
    pub after_content: Option<&'a str>,
    pub attachment_summary: Option<&'a str>,
    pub created_at: u64,
}

/// Builds and sends the user log embed (plus media uploads / unfurl links) to
/// the configured user log channel.
pub async fn publish_userlog_embed(
    ctx: &serenity::Context,
    data: &Data,
    entry: PublishUserLogEntry<'_>,
) {
    let userlog_channel_id = match get_userlog_channel_id(&data.db, entry.guild_id.get()).await {
        Ok(channel_id) => channel_id,
        Err(source) => {
            error!(?source, "failed to read user log channel config");
            None
        }
    };

    let Some(target_channel_id) = userlog_channel_id else {
        return;
    };

    let event_label = match entry.event_type {
        "message_edit" => "Message Edited",
        "message_delete" => "Message Deleted",
        "attachment_delete" => "Attachment Deleted",
        _ => "User Log Event",
    };

    let (author_display, author_avatar_url) = match entry.author_user_id {
        Some(user_id) => match ctx.http.get_user(user_id).await {
            Ok(user) => (
                user.global_name
                    .clone()
                    .unwrap_or_else(|| user.name.clone()),
                Some(user.face()),
            ),
            Err(_) => (format!("User {}", user_id.get()), None),
        },
        None => ("Unknown user".to_owned(), None),
    };

    let attachment_items = entry
        .attachment_summary
        .map(parse_attachment_summary)
        .unwrap_or_default();
    let attachment_filenames = attachment_items
        .iter()
        .map(|item| item.filename.as_str())
        .collect::<Vec<_>>();
    let first_media_url = attachment_items
        .iter()
        .find(|item| item.is_media)
        .map(|item| item.url.clone())
        .or_else(|| extract_first_media_url(entry.after_content))
        .or_else(|| extract_first_media_url(entry.before_content));

    let mut description_lines = Vec::new();
    if entry.event_type == "attachment_delete" {
        if !attachment_filenames.is_empty() {
            description_lines.push(attachment_filenames.join("\n"));
        }
    } else {
        if let Some(before_content) = entry.before_content.filter(|value| !value.is_empty()) {
            description_lines.push(truncate_for_embed(&sanitize_mentions(before_content), 600));
        }

        if let Some(after_content) = entry.after_content.filter(|value| !value.is_empty()) {
            if !description_lines.is_empty() {
                description_lines.push("\u{200B}".to_owned());
            }
            description_lines.push("**After edit**".to_owned());
            description_lines.push(truncate_for_embed(&sanitize_mentions(after_content), 600));
        } else if !attachment_filenames.is_empty() {
            if !description_lines.is_empty() {
                description_lines.push("\u{200B}".to_owned());
            }
            description_lines.push(attachment_filenames.join("\n"));
        }
    }

    let jump_link = entry.message_id.map(|message_id| {
        format!(
            "https://discord.com/channels/{}/{}/{}",
            entry.guild_id.get(),
            entry.channel_id.get(),
            message_id.get()
        )
    });

    let mut metadata_lines = vec![
        format!(
            "**Message author :** {}",
            entry
                .author_user_id
                .map(|user_id| format!("<@{}>", user_id.get()))
                .unwrap_or_else(|| author_display.clone())
        ),
        format!("**Channel :** <#{}>", entry.channel_id.get()),
    ];

    metadata_lines.push(
        jump_link
            .as_deref()
            .map(|url| {
                if entry.event_type == "message_delete" {
                    format!("**[Jump to context]({})**", url)
                } else {
                    format!("**[Jump to message]({})**", url)
                }
            })
            .unwrap_or_else(|| "**Jump unavailable**".to_owned()),
    );
    metadata_lines.push(String::new());

    if entry.event_type == "message_delete" {
        metadata_lines.push(format!(
            "**Deleted by :** {}",
            entry
                .deleted_by_user_id
                .map(|user_id| format!("<@{}>", user_id.get()))
                .unwrap_or_else(|| "Unknown".to_owned())
        ));
    }

    metadata_lines.push(format!("**When :** <t:{}:R>", entry.created_at));

    if !metadata_lines.is_empty() {
        if !description_lines.is_empty() {
            description_lines.push("\u{200B}".to_owned());
        }
        description_lines.extend(metadata_lines);
    }

    let mut embed = serenity::CreateEmbed::new()
        .color(autumn_utils::embed::DEFAULT_EMBED_COLOR)
        .title(event_label)
        .description(if description_lines.is_empty() {
            "\u{200B}".to_owned()
        } else {
            description_lines.join("\n")
        });

    if let Some(avatar_url) = author_avatar_url {
        embed = embed.author(serenity::CreateEmbedAuthor::new(author_display).icon_url(avatar_url));
    }

    let mut files = Vec::new();
    for attachment in &attachment_items {
        if !attachment.is_media {
            continue;
        }

        let safe_filename = sanitize_attachment_filename(&attachment.filename);
        match download_media_bytes(&attachment.url).await {
            Some(bytes) => {
                files.push(serenity::CreateAttachment::bytes(bytes, safe_filename));
            }
            None => {
                error!(url = %attachment.url, "failed to download attachment media for user log reupload");
            }
        }
    }

    if files.is_empty()
        && let Some(media_url) = first_media_url.as_deref()
        && is_direct_image_url(media_url)
    {
        let fallback_name =
            sanitize_attachment_filename(&infer_media_filename(media_url, "preview.gif"));
        match download_media_bytes(media_url).await {
            Some(bytes) => {
                files.push(serenity::CreateAttachment::bytes(bytes, fallback_name));
            }
            None => {
                error!(url = %media_url, "failed to download fallback media url for embed image");
            }
        }
    }

    let mut unfurl_links = Vec::new();
    if files.is_empty()
        && let Some(media_url) = first_media_url
        && !is_direct_image_url(&media_url)
    {
        unfurl_links.push(media_url);
    }

    if let Some(link_preview) = extract_first_unfurl_link(entry.after_content)
        .or_else(|| extract_first_unfurl_link(entry.before_content))
        .filter(|link| !unfurl_links.iter().any(|existing| existing == link))
    {
        unfurl_links.push(link_preview);
    }

    if !files.is_empty()
        && let Err(source) = serenity::ChannelId::new(target_channel_id)
            .send_message(&ctx.http, serenity::CreateMessage::new().add_files(files))
            .await
    {
        error!(
            ?source,
            "failed to publish user log media preview attachments"
        );
    }

    if !unfurl_links.is_empty()
        && let Err(source) = serenity::ChannelId::new(target_channel_id)
            .send_message(
                &ctx.http,
                serenity::CreateMessage::new().content(unfurl_links.join("\n")),
            )
            .await
    {
        error!(?source, "failed to publish user log unfurl link preview");
    }

    if let Err(source) = serenity::ChannelId::new(target_channel_id)
        .send_message(&ctx.http, serenity::CreateMessage::new().embed(embed))
        .await
    {
        error!(?source, "failed to publish user log embed");
    }
}
