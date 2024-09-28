use std::arch::x86_64::_mm_extract_si64;
use poise::serenity_prelude::{CreateEmbed, Color, Timestamp};
use poise::CreateReply;
use crate::{Context, Error};
use crate::vrc_client;
use crate::vrc_client::traits::Input;

/// Helper function for all actions
async fn send_action(ctx: Context<'_>, action: vrc_client::Action) -> Result<(), Error> {
    let mut reply_embed = CreateEmbed::default();

    match ctx.data().vrc_transmitter.send(action.clone()).await {
        Ok(_) => {
            // Format type of action performed
            // TODO: Make this not buns
            let mut action_type: String = String::from("");
            if action.clone().movement != None {
                action_type += action.clone().movement.unwrap().to_lowercase().as_str();
            }
            if action.clone().look != None {
                action_type += action.clone().look.unwrap().to_lowercase().as_str();
            }
            if action.clone().jump != None {
                action_type += "jump";
            }
            if action.clone().run != None {
                action_type += "run"
            }
            reply_embed = reply_embed
                .title("Successfully sent action")
                .field("**Caller**", format!("{} ({})\nAction type: {}\nAction duration: {}", ctx.author().name, ctx.author().id, action_type, action.duration), false)
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

/// Sends a move input to the bot VRChat client
/// Directions: Forward, Backward, Left, Right
#[poise::command(prefix_command, slash_command)]
pub async fn move_horizontal(
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

/// Sends a look input to the bot VRChat client
/// Directions: Left, Right
#[poise::command(prefix_command, slash_command)]
pub async fn look(
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

/// Sends a run input to the bot VRChat client
#[poise::command(prefix_command, slash_command)]
pub async fn run(
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

/// Sends a jump input to the bot VRChat client
#[poise::command(prefix_command, slash_command)]
pub async fn jump(ctx: Context<'_>) -> Result<(), Error> {
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

// TODO: Rewrite action handling system so that I can avoid a big code block in my main function, maybe write a handler function?
/// Sends a message to the VRChat chatbox
#[poise::command(prefix_command, slash_command, owners_only)]
pub async fn chatbox(
    ctx: Context<'_>,
    #[description = "Sends a message to the client's chatbox (144 char limit)"] message: String,
) -> Result<(), Error> {
    ctx.data().vrc_client.chatbox_message(&message.as_str());

    let reply_embed = CreateEmbed::default()
        .title("Sent chatbox message".to_string())
        .field("**Caller**".to_string(), format!("{} ({})\nMessage: {}", ctx.author().name, ctx.author().id, message), false)
        .color(Color::DARK_GREEN)
        .thumbnail(ctx.author().face())
        .timestamp(Timestamp::now());

    ctx.send(CreateReply::default().embed(reply_embed)).await?;

    Ok(())
}