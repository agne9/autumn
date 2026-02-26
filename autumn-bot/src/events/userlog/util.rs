use std::time::{SystemTime, UNIX_EPOCH};

use poise::serenity_prelude as serenity;

use autumn_core::Data;

/// Builds the `filename (url)\nfilename2 (url2)` summary from message attachments.
pub fn attachment_summary_from_message(message: &serenity::Message) -> Option<String> {
    if message.attachments.is_empty() {
        return None;
    }

    let entries = message
        .attachments
        .iter()
        .map(|attachment| format!("{} ({})", attachment.filename, attachment.url))
        .collect::<Vec<_>>();

    Some(entries.join("\n"))
}

pub fn sanitize_mentions(value: &str) -> String {
    value.replace('@', "@\u{200B}")
}

pub fn truncate_for_embed(value: &str, max_len: usize) -> String {
    if value.chars().count() <= max_len {
        return value.to_owned();
    }

    let mut output = String::new();
    for character in value.chars().take(max_len) {
        output.push(character);
    }
    output.push('…');
    output
}

pub fn now_unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs())
}

/// Looks up the audit log to determine who deleted a message (if it wasn't the author).
pub async fn resolve_deleted_by_user_id(
    ctx: &serenity::Context,
    _data: &Data,
    guild_id: serenity::GuildId,
    channel_id: serenity::ChannelId,
    message_id: serenity::MessageId,
    target_author_user_id: serenity::UserId,
) -> Option<serenity::UserId> {
    let audit_logs = guild_id
        .audit_logs(
            &ctx.http,
            Some(serenity::all::audit_log::Action::Message(
                serenity::all::audit_log::MessageAction::Delete,
            )),
            None,
            None,
            Some(25),
        )
        .await
        .ok()?;

    let message_ts = message_id.created_at().unix_timestamp();
    audit_logs.entries.into_iter().find_map(|entry| {
        let same_channel = entry
            .options
            .as_ref()
            .and_then(|options| options.channel_id)
            .is_some_and(|id| id == channel_id);

        let same_target = entry
            .target_id
            .is_some_and(|target_id| target_id.get() == target_author_user_id.get());

        if !same_channel || !same_target {
            return None;
        }

        let audit_ts = entry.id.created_at().unix_timestamp();
        let close_in_time = (audit_ts - message_ts).abs() <= 20;
        if !close_in_time {
            return None;
        }

        Some(entry.user_id)
    })
}
