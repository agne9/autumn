use poise::serenity_prelude as serenity;

use crate::CommandMeta;
use crate::moderation::embeds::guild_only_message;
use autumn_core::{Context, Error};
use autumn_database::impls::word_filter::{
    add_filter_word, clear_preset_words, get_word_filter_config, list_filter_words,
    load_preset_words, remove_filter_word, set_word_filter_action, set_word_filter_enabled,
};
use autumn_utils::embed::DEFAULT_EMBED_COLOR;
use autumn_utils::pagination::paginate_embed_pages;
use autumn_utils::permissions::has_user_permission;

pub const META: CommandMeta = CommandMeta {
    name: "wordfilter",
    desc: "Manage the word filter for this server.",
    category: "moderation",
    usage: "!wordfilter <enable|disable|action|preset|add|remove|list>",
};

/// Manage the word filter for this server.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderation",
    subcommands("enable", "disable", "action", "preset", "add", "remove", "list")
)]
pub async fn wordfilter(ctx: Context<'_>) -> Result<(), Error> {
    let Some(guild_id) = ctx.guild_id() else {
        ctx.say(guild_only_message()).await?;
        return Ok(());
    };

    if !has_user_permission(
        ctx.http(),
        guild_id,
        ctx.author().id,
        serenity::Permissions::MANAGE_GUILD,
    )
    .await?
    {
        return Ok(());
    }

    let config = get_word_filter_config(&ctx.data().db, guild_id.get()).await?;

    let (enabled, action_label) = match &config {
        Some(cfg) => (cfg.enabled, action_display(&cfg.action)),
        None => (false, action_display("log_only")),
    };

    let status = if enabled { "Enabled" } else { "Disabled" };

    let embed = serenity::CreateEmbed::new()
        .title("Word Filter Status")
        .description(format!(
            "**Status :** {}\n**Action :** {}",
            status, action_label
        ))
        .color(DEFAULT_EMBED_COLOR)
        .footer(serenity::CreateEmbedFooter::new(
            "Subcommands: enable, disable, action, preset, add, remove, list",
        ));

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}

/// Enable the word filter.
#[poise::command(prefix_command, slash_command, category = "Moderation")]
pub async fn enable(ctx: Context<'_>) -> Result<(), Error> {
    let Some(guild_id) = ctx.guild_id() else {
        ctx.say(guild_only_message()).await?;
        return Ok(());
    };

    if !has_user_permission(
        ctx.http(),
        guild_id,
        ctx.author().id,
        serenity::Permissions::MANAGE_GUILD,
    )
    .await?
    {
        return Ok(());
    }

    set_word_filter_enabled(&ctx.data().db, guild_id.get(), true).await?;
    ctx.say("Word filter has been **enabled**.").await?;

    Ok(())
}

/// Disable the word filter.
#[poise::command(prefix_command, slash_command, category = "Moderation")]
pub async fn disable(ctx: Context<'_>) -> Result<(), Error> {
    let Some(guild_id) = ctx.guild_id() else {
        ctx.say(guild_only_message()).await?;
        return Ok(());
    };

    if !has_user_permission(
        ctx.http(),
        guild_id,
        ctx.author().id,
        serenity::Permissions::MANAGE_GUILD,
    )
    .await?
    {
        return Ok(());
    }

    set_word_filter_enabled(&ctx.data().db, guild_id.get(), false).await?;
    ctx.say("Word filter has been **disabled**.").await?;

    Ok(())
}

/// Set the action taken when a filtered word is detected.
#[poise::command(prefix_command, slash_command, category = "Moderation")]
pub async fn action(
    ctx: Context<'_>,
    #[description = "Action: log, delete, or timeout"]
    #[rest]
    input: Option<String>,
) -> Result<(), Error> {
    let Some(guild_id) = ctx.guild_id() else {
        ctx.say(guild_only_message()).await?;
        return Ok(());
    };

    if !has_user_permission(
        ctx.http(),
        guild_id,
        ctx.author().id,
        serenity::Permissions::MANAGE_GUILD,
    )
    .await?
    {
        return Ok(());
    }

    let Some(raw) = input.as_deref().map(str::trim).filter(|s| !s.is_empty()) else {
        ctx.say(
            "Usage: `!wordfilter action <log|delete|warn|timeout>`\n\
             • `log` — Only log the violation\n\
             • `delete` — Delete message and log\n\
             • `warn` — Warn user, delete message, and log\n\
             • `timeout` — Timeout user, delete message, and log",
        )
        .await?;
        return Ok(());
    };

    let action_str = match raw.to_lowercase().as_str() {
        "log" => "log_only",
        "delete" => "delete_and_log",
        "warn" => "warn_and_log",
        "timeout" => "timeout_delete_and_log",
        _ => {
            ctx.say(
                "Invalid action. Use one of: `log`, `delete`, `warn`, `timeout`.\n\n\
                 • `log` — Only log the violation\n\
                 • `delete` — Delete message and log\n\
                 • `warn` — Warn user, delete message, and log\n\
                 • `timeout` — Timeout user, delete message, and log",
            )
            .await?;
            return Ok(());
        }
    };

    set_word_filter_action(&ctx.data().db, guild_id.get(), action_str).await?;

    ctx.say(format!(
        "Word filter action set to **{}**.",
        action_display(action_str)
    ))
    .await?;

    Ok(())
}

/// Load or clear the preset offensive word list.
#[poise::command(prefix_command, slash_command, category = "Moderation")]
pub async fn preset(
    ctx: Context<'_>,
    #[description = "Operation: load or clear"]
    #[rest]
    input: Option<String>,
) -> Result<(), Error> {
    let Some(guild_id) = ctx.guild_id() else {
        ctx.say(guild_only_message()).await?;
        return Ok(());
    };

    if !has_user_permission(
        ctx.http(),
        guild_id,
        ctx.author().id,
        serenity::Permissions::MANAGE_GUILD,
    )
    .await?
    {
        return Ok(());
    }

    let Some(raw) = input.as_deref().map(str::trim).filter(|s| !s.is_empty()) else {
        ctx.say("Usage: `!wordfilter preset <load|clear>`").await?;
        return Ok(());
    };

    match raw.to_lowercase().as_str() {
        "load" => {
            let count = load_preset_words(&ctx.data().db, guild_id.get()).await?;
            ctx.say(format!(
                "Loaded **{}** preset word(s) into the filter list.",
                count
            ))
            .await?;
        }
        "clear" => {
            let count = clear_preset_words(&ctx.data().db, guild_id.get()).await?;
            ctx.say(format!(
                "Removed **{}** preset word(s) from the filter list.",
                count
            ))
            .await?;
        }
        _ => {
            ctx.say("Invalid option. Use `load` or `clear`.\n\nUsage: `!wordfilter preset <load|clear>`")
                .await?;
        }
    }

    Ok(())
}

/// Add a custom word to the filter list.
#[poise::command(prefix_command, slash_command, category = "Moderation")]
pub async fn add(
    ctx: Context<'_>,
    #[description = "Word to add to the filter"]
    #[rest]
    word: Option<String>,
) -> Result<(), Error> {
    let Some(guild_id) = ctx.guild_id() else {
        ctx.say(guild_only_message()).await?;
        return Ok(());
    };

    if !has_user_permission(
        ctx.http(),
        guild_id,
        ctx.author().id,
        serenity::Permissions::MANAGE_GUILD,
    )
    .await?
    {
        return Ok(());
    }

    let Some(raw) = word.as_deref().map(str::trim).filter(|s| !s.is_empty()) else {
        ctx.say("Usage: `!wordfilter add <word>`").await?;
        return Ok(());
    };

    let word = raw.to_lowercase();
    let inserted = add_filter_word(&ctx.data().db, guild_id.get(), &word, false).await?;

    if inserted {
        ctx.say(format!("Added `{}` to the word filter list.", word))
            .await?;
    } else {
        ctx.say(format!("`{}` is already in the word filter list.", word))
            .await?;
    }

    Ok(())
}

/// Remove a word from the filter list.
#[poise::command(prefix_command, slash_command, category = "Moderation")]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "Word to remove from the filter"]
    #[rest]
    word: Option<String>,
) -> Result<(), Error> {
    let Some(guild_id) = ctx.guild_id() else {
        ctx.say(guild_only_message()).await?;
        return Ok(());
    };

    if !has_user_permission(
        ctx.http(),
        guild_id,
        ctx.author().id,
        serenity::Permissions::MANAGE_GUILD,
    )
    .await?
    {
        return Ok(());
    }

    let Some(raw) = word.as_deref().map(str::trim).filter(|s| !s.is_empty()) else {
        ctx.say("Usage: `!wordfilter remove <word>`").await?;
        return Ok(());
    };

    let word = raw.to_lowercase();
    let removed = remove_filter_word(&ctx.data().db, guild_id.get(), &word).await?;

    if removed {
        ctx.say(format!("Removed `{}` from the word filter list.", word))
            .await?;
    } else {
        ctx.say(format!("`{}` was not found in the word filter list.", word))
            .await?;
    }

    Ok(())
}

/// List all filtered words for this server.
#[poise::command(prefix_command, slash_command, category = "Moderation")]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let Some(guild_id) = ctx.guild_id() else {
        ctx.say(guild_only_message()).await?;
        return Ok(());
    };

    if !has_user_permission(
        ctx.http(),
        guild_id,
        ctx.author().id,
        serenity::Permissions::MANAGE_GUILD,
    )
    .await?
    {
        return Ok(());
    }

    let words = list_filter_words(&ctx.data().db, guild_id.get()).await?;

    if words.is_empty() {
        ctx.say("The word filter list is empty.").await?;
        return Ok(());
    }

    let lines: Vec<String> = words
        .iter()
        .enumerate()
        .map(|(i, w)| {
            let tag = if w.is_preset { " (preset)" } else { "" };
            format!("{}. `{}`{}", i + 1, w.word, tag)
        })
        .collect();

    let items_per_page = 20;
    let pages: Vec<String> = lines
        .chunks(items_per_page)
        .map(|chunk| chunk.join("\n"))
        .collect();

    paginate_embed_pages(ctx, "Word Filter List", &pages, 1).await?;

    Ok(())
}

fn action_display(action: &str) -> &str {
    match action {
        "log_only" => "Only Log",
        "delete_and_log" => "Delete and Log",
        "warn_and_log" => "Warn, Delete and Log",
        "timeout_delete_and_log" => "Timeout, Delete and Log",
        _ => "Unknown",
    }
}
