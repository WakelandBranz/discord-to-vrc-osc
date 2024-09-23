pub mod vrc;

// Functionality imports
use crate::{Context, Error};
// TODO: CLEAN IMPORTS
use poise::{serenity_prelude as serenity};
use poise::CreateReply;
use serenity::{CreateEmbed, Color, Timestamp};

/// Used for registering and unregistering commands
#[poise::command(prefix_command, owners_only)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

/// Shuts the application down gracefully
#[poise::command(prefix_command, owners_only)]
pub async fn shutdown(ctx: Context<'_>) -> Result<(), Error> {

    let reply_embed = CreateEmbed::default()
        .title("Shutting down".to_string())
        .field("**Caller**".to_string(), format!("{} ({})", ctx.author().name, ctx.author().id), false)
        .color(Color::ROSEWATER)
        .thumbnail(ctx.author().face())
        .timestamp(Timestamp::now());

    ctx.send(CreateReply::default().embed(reply_embed)).await?;

    ctx.framework().shard_manager.shutdown_all().await;
    Ok(())
}

/// Refreshes the bot's config
#[poise::command(
    prefix_command,
    owners_only,
)]
pub async fn update_config(
    ctx: Context<'_>,
) -> Result<(), Error> {

    ctx.data().config.lock().unwrap().update();
    let data = ctx.data().config.lock().unwrap().to_string();

    let is_ephemeral = ctx.data().config.lock().unwrap().system.ephemeral_admin_commands;
    println!("is_ephemeral: {}", &is_ephemeral);

    let reply_embed = CreateEmbed::default()
        .title("Refreshed!".to_string())
        .field("**Config Data**".to_string(), data, false)
        .color(Color::DARK_GREEN)
        .thumbnail(ctx.author().face())
        .timestamp(Timestamp::now());

    ctx.send(CreateReply::default().ephemeral(is_ephemeral).embed(reply_embed)).await?;

    Ok(())
}

