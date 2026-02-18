use tokio::time::{Duration, sleep};
use tracing::error;

use poise::serenity_prelude as serenity;

use crate::CommandMeta;
use crate::moderation::logging::create_case_and_publish;
use autumn_core::{Context, Error};
use autumn_database::impls::cases::NewCase;
use autumn_utils::confirmation::{ConfirmationResult, prompt_confirm_decline};
use autumn_utils::permissions::has_user_permission;

pub const META: CommandMeta = CommandMeta {
    name: "purge",
    desc: "Delete the latest messages in this channel.",
    category: "moderation",
    usage: "!purge <amount>",
};

const MAX_PURGE: u16 = 100;
const PURGE_CONFIRM_TIMEOUT_SECS: u64 = 30;

#[poise::command(prefix_command, slash_command, category = "Moderation")]
pub async fn purge(
    ctx: Context<'_>,
    #[description = "Amount of messages to purge"] amount: Option<u16>,
) -> Result<(), Error> {
    let Some(guild_id) = ctx.guild_id() else {
        ctx.say("This command only works in servers.").await?;
        return Ok(());
    };

    let Some(requested) = amount else {
        ctx.say(format!("Usage: `{}`", META.usage)).await?;
        return Ok(());
    };

    if requested == 0 {
        ctx.say("Amount must be at least 1.").await?;
        return Ok(());
    }

    if !has_user_permission(
        ctx.http(),
        guild_id,
        ctx.author().id,
        serenity::Permissions::MANAGE_MESSAGES,
    )
    .await?
    {
        ctx.say("You are not permitted to use this command.").await?;
        return Ok(());
    }

    let amount = requested.min(MAX_PURGE);
    let delete_count = amount.saturating_add(1).min(MAX_PURGE);

    let confirm_embed = serenity::CreateEmbed::new().description(format!(
        "This will delete up to {} recent message(s) in this channel.",
        amount
    ));
    let confirmation = prompt_confirm_decline(
        ctx,
        "Confirm purge",
        confirm_embed,
        Duration::from_secs(PURGE_CONFIRM_TIMEOUT_SECS),
    )
    .await?;

    let interaction = match confirmation {
        ConfirmationResult::TimedOut(message) => {
            message
                .channel_id
                .edit_message(
                    ctx.http(),
                    message.id,
                    serenity::EditMessage::new()
                        .content("Timed out")
                        .embeds(vec![])
                        .components(vec![]),
                )
                .await?;
            return Ok(());
        }
        ConfirmationResult::Declined(interaction) => {
            interaction
                .create_response(
                    ctx.http(),
                    serenity::CreateInteractionResponse::UpdateMessage(
                        serenity::CreateInteractionResponseMessage::new()
                            .content("Purge cancelled.")
                            .embeds(vec![])
                            .components(vec![]),
                    ),
                )
                .await?;
            return Ok(());
        }
        ConfirmationResult::Confirmed(interaction) => {
            interaction
                .create_response(
                    ctx.http(),
                    serenity::CreateInteractionResponse::UpdateMessage(
                        serenity::CreateInteractionResponseMessage::new()
                            .content("Purging...")
                            .embeds(vec![])
                            .components(vec![]),
                    ),
                )
                .await?;
                interaction
        }
            };

    let channel_id = ctx.channel_id();
    let interaction_message_id = interaction.message.id;
    let messages = channel_id
        .messages(ctx.http(), serenity::GetMessages::new().limit(delete_count as u8))
        .await?;

    let ids: Vec<serenity::MessageId> = messages
        .into_iter()
        .map(|message| message.id)
        .filter(|id| *id != interaction_message_id)
        .collect();

    if ids.is_empty() {
        interaction
            .edit_response(
                ctx.http(),
                serenity::EditInteractionResponse::new()
                    .content("No messages found to delete.")
                    .embeds(vec![]),
            )
            .await?;
        return Ok(());
    }

    let delete_result = if ids.len() == 1 {
        channel_id.delete_message(ctx.http(), ids[0]).await
    } else {
        channel_id.delete_messages(ctx.http(), ids).await
    };

    if let Err(source) = delete_result {
        error!(?source, "purge delete request failed");
        interaction
            .edit_response(
                ctx.http(),
                serenity::EditInteractionResponse::new()
                    .content("I couldn't delete messages. I likely need the 'Manage Messages' permission.")
                    .embeds(vec![]),
            )
            .await?;
        return Ok(());
    }

    let case_reason = "No reason provided".to_owned();
    let _case_label = create_case_and_publish(
        &ctx,
        guild_id,
        NewCase {
            guild_id: guild_id.get(),
            target_user_id: None,
            moderator_user_id: ctx.author().id.get(),
            action: "purge",
            reason: &case_reason,
            status: "active",
            duration_seconds: None,
        },
    )
    .await;

    let confirmation_text = format!("Purged {} message(s).", amount);

    interaction
        .edit_response(
            ctx.http(),
            serenity::EditInteractionResponse::new()
                .content(confirmation_text)
                .embeds(vec![]),
        )
        .await?;

    sleep(Duration::from_secs(3)).await;
    let _ = interaction.delete_response(ctx.http()).await;

    Ok(())
}
