use poise::serenity_prelude as serenity;
use tracing::error;

use autumn_core::{Data, Error};
use autumn_database::impls::ai_config::get_llm_enabled;
use autumn_database::impls::llm_chat::insert_llm_chat_message;

pub async fn handle_message_mention_llm(
    ctx: &serenity::Context,
    data: &Data,
    new_message: &serenity::Message,
) -> Result<(), Error> {
    if new_message.author.bot || new_message.webhook_id.is_some() {
        return Ok(());
    }

    let Some(guild_id) = new_message.guild_id else {
        return Ok(());
    };

    let Some(llm) = data.llm.as_ref() else {
        return Ok(());
    };

    let llm_enabled = match get_llm_enabled(&data.db, guild_id.get()).await {
        Ok(enabled) => enabled,
        Err(source) => {
            error!(?source, "failed to read guild AI config");
            return Ok(());
        }
    };

    if !llm_enabled {
        return Ok(());
    }

    let mentions_bot = match new_message.mentions_me(ctx).await {
        Ok(value) => value,
        Err(source) => {
            error!(?source, "failed to evaluate bot mention");
            false
        }
    };

    if !mentions_bot {
        return Ok(());
    }

    let bot_user_id = ctx.cache.current_user().id;
    let author_display_name = message_display_name(new_message);
    let bot_display_name = ctx.cache.current_user().name.clone();
    let prompt = strip_bot_mention(&new_message.content, bot_user_id)
        .trim()
        .to_owned();

    if prompt.is_empty() {
        new_message.reply(&ctx.http, "a?").await?;
        return Ok(());
    }

    let _ = new_message.channel_id.broadcast_typing(&ctx.http).await;

    let llm_reply = match llm
        .generate_channel_reply(
            &data.db,
            guild_id.get(),
            new_message.channel_id.get(),
            &prompt,
            &author_display_name,
        )
        .await
    {
        Ok(content) if !content.trim().is_empty() => content,
        Ok(_) => "I couldn't generate a useful response for that. Try rephrasing?".to_owned(),
        Err(source) => {
            error!(?source, "llm reply generation failed");
            new_message
                .reply(&ctx.http, "I ran into an LLM error. Try again in a moment.")
                .await?;
            return Ok(());
        }
    };

    if let Err(source) = insert_llm_chat_message(
        &data.db,
        guild_id.get(),
        new_message.channel_id.get(),
        new_message.author.id.get(),
        Some(author_display_name.as_str()),
        "user",
        &prompt,
    )
    .await
    {
        error!(?source, "failed to persist user llm chat message");
    }

    new_message.reply(&ctx.http, &llm_reply).await?;

    if let Err(source) = insert_llm_chat_message(
        &data.db,
        guild_id.get(),
        new_message.channel_id.get(),
        bot_user_id.get(),
        Some(bot_display_name.as_str()),
        "assistant",
        &llm_reply,
    )
    .await
    {
        error!(?source, "failed to persist assistant llm chat message");
    }

    Ok(())
}

fn strip_bot_mention(content: &str, bot_user_id: serenity::UserId) -> String {
    content
        .replace(&format!("<@{}>", bot_user_id.get()), "")
        .replace(&format!("<@!{}>", bot_user_id.get()), "")
        .trim()
        .to_owned()
}

fn message_display_name(message: &serenity::Message) -> String {
    if let Some(member) = &message.member
        && let Some(nick) = &member.nick
        && !nick.trim().is_empty()
    {
        return nick.clone();
    }

    if let Some(global_name) = &message.author.global_name
        && !global_name.trim().is_empty()
    {
        return global_name.clone();
    }

    message.author.name.clone()
}
