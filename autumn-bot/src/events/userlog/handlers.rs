use poise::serenity_prelude as serenity;
use tracing::error;

use autumn_core::Data;
use autumn_database::impls::user_logs::{
    NewMessageSnapshot, NewUserLog, delete_message_snapshot, get_message_snapshot, insert_user_log,
    upsert_message_snapshot,
};

use super::embed::{PublishUserLogEntry, publish_userlog_embed};
use super::util::{attachment_summary_from_message, now_unix_secs, resolve_deleted_by_user_id};

pub async fn handle_message_create_userlog(data: &Data, message: &serenity::Message) {
    if message.author.bot || message.webhook_id.is_some() {
        return;
    }

    let Some(guild_id) = message.guild_id else {
        return;
    };

    let attachment_summary = attachment_summary_from_message(message);
    let now = now_unix_secs();

    if let Err(source) = upsert_message_snapshot(
        &data.db,
        NewMessageSnapshot {
            guild_id: guild_id.get(),
            channel_id: message.channel_id.get(),
            message_id: message.id.get(),
            author_user_id: message.author.id.get(),
            content: &message.content,
            attachment_summary: attachment_summary.as_deref(),
            updated_at: now,
        },
    )
    .await
    {
        error!(?source, "failed to upsert message snapshot on create");
    }
}

pub async fn handle_message_update_userlog(
    ctx: &serenity::Context,
    data: &Data,
    update_event: &serenity::MessageUpdateEvent,
) {
    let Some(guild_id) = update_event.guild_id else {
        return;
    };

    let current_message = match update_event
        .channel_id
        .message(&ctx.http, update_event.id)
        .await
    {
        Ok(message) => message,
        Err(_) => return,
    };

    if current_message.author.bot || current_message.webhook_id.is_some() {
        return;
    }

    let attachment_summary = attachment_summary_from_message(&current_message);
    let now = now_unix_secs();

    let snapshot = match get_message_snapshot(
        &data.db,
        guild_id.get(),
        update_event.channel_id.get(),
        update_event.id.get(),
    )
    .await
    {
        Ok(snapshot) => snapshot,
        Err(source) => {
            error!(?source, "failed to get message snapshot on update");
            None
        }
    };

    if let Some(previous) = snapshot {
        let content_changed = previous.content != current_message.content;
        let attachments_changed = previous.attachment_summary != attachment_summary;

        if content_changed || attachments_changed {
            let event_type = if !content_changed && attachments_changed {
                "attachment_delete"
            } else {
                "message_edit"
            };

            let log_entry = NewUserLog {
                guild_id: guild_id.get(),
                channel_id: current_message.channel_id.get(),
                message_id: Some(current_message.id.get()),
                author_user_id: Some(current_message.author.id.get()),
                event_type,
                before_content: Some(previous.content.as_str()),
                after_content: Some(current_message.content.as_str()),
                attachment_summary: attachment_summary.as_deref(),
                created_at: now,
            };

            if let Err(source) = insert_user_log(&data.db, log_entry).await {
                error!(?source, "failed to insert user log on message update");
            } else {
                publish_userlog_embed(
                    ctx,
                    data,
                    PublishUserLogEntry {
                        guild_id,
                        event_type,
                        channel_id: current_message.channel_id,
                        message_id: Some(current_message.id),
                        author_user_id: Some(current_message.author.id),
                        deleted_by_user_id: None,
                        before_content: Some(previous.content.as_str()),
                        after_content: Some(current_message.content.as_str()),
                        attachment_summary: attachment_summary.as_deref(),
                        created_at: now,
                    },
                )
                .await;
            }
        }
    }

    if let Err(source) = upsert_message_snapshot(
        &data.db,
        NewMessageSnapshot {
            guild_id: guild_id.get(),
            channel_id: current_message.channel_id.get(),
            message_id: current_message.id.get(),
            author_user_id: current_message.author.id.get(),
            content: &current_message.content,
            attachment_summary: attachment_summary.as_deref(),
            updated_at: now,
        },
    )
    .await
    {
        error!(?source, "failed to upsert message snapshot on update");
    }
}

pub async fn handle_message_delete_userlog(
    ctx: &serenity::Context,
    data: &Data,
    guild_id: serenity::GuildId,
    channel_id: serenity::ChannelId,
    message_id: serenity::MessageId,
) {
    let snapshot =
        match get_message_snapshot(&data.db, guild_id.get(), channel_id.get(), message_id.get())
            .await
        {
            Ok(snapshot) => snapshot,
            Err(source) => {
                error!(?source, "failed to get message snapshot on delete");
                None
            }
        };

    let now = now_unix_secs();

    if let Some(previous) = snapshot {
        let log_entry = NewUserLog {
            guild_id: guild_id.get(),
            channel_id: previous.channel_id,
            message_id: Some(previous.message_id),
            author_user_id: Some(previous.author_user_id),
            event_type: "message_delete",
            before_content: Some(previous.content.as_str()),
            after_content: None,
            attachment_summary: previous.attachment_summary.as_deref(),
            created_at: now,
        };

        if let Err(source) = insert_user_log(&data.db, log_entry).await {
            error!(?source, "failed to insert user log on message delete");
        } else {
            let deleted_by_user_id = resolve_deleted_by_user_id(
                ctx,
                data,
                guild_id,
                channel_id,
                message_id,
                serenity::UserId::new(previous.author_user_id),
            )
            .await
            .or(Some(serenity::UserId::new(previous.author_user_id)));

            publish_userlog_embed(
                ctx,
                data,
                PublishUserLogEntry {
                    guild_id,
                    event_type: "message_delete",
                    channel_id,
                    message_id: Some(message_id),
                    author_user_id: Some(serenity::UserId::new(previous.author_user_id)),
                    deleted_by_user_id,
                    before_content: Some(previous.content.as_str()),
                    after_content: None,
                    attachment_summary: previous.attachment_summary.as_deref(),
                    created_at: now,
                },
            )
            .await;
        }
    }

    if let Err(source) =
        delete_message_snapshot(&data.db, guild_id.get(), channel_id.get(), message_id.get()).await
    {
        error!(?source, "failed to delete message snapshot");
    }
}
