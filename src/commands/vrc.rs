use poise::serenity_prelude::{CreateEmbed, Color, Timestamp};
use poise::CreateReply;
use crate::{Context, Error};
use crate::vrc_client;

/// Helper function for all actions
async fn send_action(ctx: Context<'_>, action: vrc_client::Action) -> Result<(), Error> {
    let mut reply_embed = CreateEmbed::default();

    match ctx.data().vrc_transmitter.send(action.clone()).await {
        Ok(_) => {
            reply_embed = reply_embed
                .title("Successfully sent action")
                .field("**Caller**", format!("{} ({})\nAction: {:?}", ctx.author().name, ctx.author().id, action), false)
                .color(Color::DARK_GREEN)
                .thumbnail(ctx.author().face())
                .timestamp(Timestamp::now());
        }
        Err(e) => {
            reply_embed = reply_embed
                .title("Unsuccessfully sent action")
                .field("**Caller**", format!("{} ({})\nError: {}", ctx.author().name, ctx.author().id, e), false)
                .color(Color::RED)
                .thumbnail(ctx.author().face())
                .timestamp(Timestamp::now());
        }
    };

    ctx.send(CreateReply::default().embed(reply_embed)).await?;
    Ok(())
}

/// Sends a move action to the bot VRChat client
/// Directions: Forward, Backward, Left, Right
#[poise::command(prefix_command, slash_command)]
pub async fn action_move(
    ctx: Context<'_>,
    #[description = "Direction to move"] direction: String,
    #[description = "Duration to move in direction"] duration: u64,
) -> Result<(), Error> {
    let action = vrc_client::Action {
        duration,
        movement: Some(direction),
        look: None,
        run: None,
        jump: None,
    };
    send_action(ctx, action).await
}

/// Sends a look action to the bot VRChat client
/// Directions: Left, Right
#[poise::command(prefix_command, slash_command)]
pub async fn action_look(
    ctx: Context<'_>,
    #[description = "Direction to move view angle"] direction: String,
    #[description = "Duration to move view angle in specified direction"] duration: u64,
) -> Result<(), Error> {
    let action = vrc_client::Action {
        duration,
        movement: None,
        look: Some(direction),
        run: None,
        jump: None,
    };
    send_action(ctx, action).await
}

/// Sends a run action to the bot VRChat client
#[poise::command(prefix_command, slash_command)]
pub async fn action_run(
    ctx: Context<'_>,
    #[description = "Duration to run"] duration: u64,
) -> Result<(), Error> {
    let action = vrc_client::Action {
        duration,
        movement: None,
        look: None,
        run: Some(true),
        jump: None,
    };
    send_action(ctx, action).await
}

/// Sends a jump action to the bot VRChat client
#[poise::command(prefix_command, slash_command)]
pub async fn action_jump(ctx: Context<'_>) -> Result<(), Error> {
    let action = vrc_client::Action {
        duration: 0,
        movement: None,
        look: None,
        run: None,
        jump: Some(true),
    };
    send_action(ctx, action).await
}

/// Sends a combined action to the bot VRChat client
#[poise::command(prefix_command, slash_command)]
pub async fn action_combined(
    ctx: Context<'_>,
    #[description = "Direction to move (optional)"] movement: Option<String>,
    #[description = "Direction to look (optional)"] look: Option<String>,
    #[description = "Whether to run (optional)"] run: Option<bool>,
    #[description = "Whether to jump (optional)"] jump: Option<bool>,
    #[description = "Duration of the action"] duration: u64,
) -> Result<(), Error> {
    let action = vrc_client::Action {
        duration,
        movement,
        look,
        run,
        jump,
    };
    send_action(ctx, action).await
}