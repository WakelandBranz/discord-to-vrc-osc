// TODO: Process all sensitive information with a .env file instead of through the toml
// TODO: Process all config information with a yaml file to eventually make this project work with docker

#![warn(clippy::str_to_string)]

// Functionality imports
mod commands;
mod config;
mod utils;
mod vrc_client;

use tokio;
use tokio::sync::mpsc;

// Poise/Serenity imports
use poise::serenity_prelude as serenity;
use std::sync::{Arc, Mutex};
use crate::vrc_client::Action;
use crate::vrc_client::traits::Input;

// Types used by all command functions
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passed to all command functions
pub struct Data {
    config: Arc<Mutex<config::Config>>,
    vrc_client: Arc<vrc_client::client::Client>,
    vrc_transmitter: tokio::sync::mpsc::Sender<Action>,
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    // Parse the config
    let config = config::Config::new();
    config.print();

    let mention_as_prefix = config.options.mention_as_prefix.clone();
    let token = config.auth.token.clone();

    // Wrap the config in an Arc<Mutex<>>
    let config = Arc::new(Mutex::new(config));
    let config_clone = Arc::clone(&config); // For other tasks

    // Use config to construct VRChat client
    let receiver_port = config.lock().unwrap().vrc_client.receiver_port;
    let transmitter_port = config.lock().unwrap().vrc_client.transmitter_port;

    let vrc_client = Arc::new(vrc_client::client::Client::new(receiver_port, transmitter_port));
    let vrc_client_ref1 = Arc::clone(&vrc_client);
    let vrc_client_ref2 = Arc::clone(&vrc_client);

    let (vrc_transmitter, mut vrc_receiver) = mpsc::channel::<Action>(64);

    // FrameworkOptions contains all of poise's configuration option in one struct
    // Every option can be omitted to use its default value
    let options = poise::FrameworkOptions {
        commands: vec![
            commands::register(),
            commands::shutdown(),
            commands::vrc::move_horizontal(),
            commands::vrc::look(),
            commands::vrc::run(),
            commands::vrc::jump(),
            commands::vrc::action_combined(),
            commands::vrc::chatbox(),
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            //edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
            //    Duration::from_secs(3600),
            //))),
            mention_as_prefix,
            //additional_prefixes: prefixes,
            ..Default::default()
        },
        // The global error handler for all error cases that may occur
        on_error: |error| Box::pin(on_error(error)),
        // This code is run before every command
        pre_command: |ctx| {
            Box::pin(async move {
                let author = ctx.author();
                println!("{} ({}) -> Executing command {}...", author.name, author.id, ctx.command().qualified_name);
            })
        },
        // This code is run after a command if it was successful (returned Ok)
        post_command: |ctx| {
            Box::pin(async move {
                let author = ctx.author();
                println!("{} ({}) -> Successfully executed command {}!", author.name, author.id, ctx.command().qualified_name);
            })
        },
        // Every command invocation must pass this check to continue execution
        //command_check: Some(|ctx| {
        //    Box::pin(async move {
        //        if ctx.author().id == 123456789 {
        //            return Ok(false);
        //        }
        //        Ok(true)
        //    })
        //}),
        // Enforce command checks even for owners (enforced by default)
        // Set to true to bypass checks, which is useful for testing
        skip_checks_for_owners: true,
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                println!(
                    "Got an event in event handler: {:?}",
                    event.snake_case_name()
                );
                Ok(())
            })
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    config,
                    vrc_client: Arc::clone(&vrc_client),
                    vrc_transmitter,
                })
            })
        })
        .options(options)
        .build();

    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;


    // TODO: Migrate task system to spawn_blocking since this is just a stupid solution quite frankly
    // First tokio::spawn (movement handler)
    // All spawned tasks allow for asynchronous movement queuing
    tokio::spawn(async move {
        let vrc_client = Arc::clone(&vrc_client_ref1);
        while let Some(action) = vrc_receiver.recv().await {

            // Horizontal character movement
            if let Some(movement) = action.movement {
                vrc_client.input_move(&movement, true);
                let vrc_client_clone = Arc::clone(&vrc_client);
                // Timer for asynchronous actions
                tokio::spawn(async move {
                    tokio::time::sleep(std::time::Duration::from_secs(action.duration)).await;
                    vrc_client_clone.input_move(&movement, false);
                });
            }

            // Horizontal view angle movement
            if let Some(look) = action.look {
                vrc_client.input_look(&look, true);
                let vrc_client_clone = Arc::clone(&vrc_client);
                // Timer for asynchronous actions
                tokio::spawn(async move {
                    tokio::time::sleep(std::time::Duration::from_secs(action.duration)).await;
                    vrc_client_clone.input_look(&look, false);
                });
            }

            // Duration to run
            if let Some(_) = action.run {
                vrc_client.input_run(1);
                let vrc_client_clone = Arc::clone(&vrc_client);
                // Timer for asynchronous actions
                tokio::spawn(async move {
                    tokio::time::sleep(std::time::Duration::from_secs(action.duration)).await;
                    vrc_client_clone.input_run(0);
                });
            }

            // Makes character jump
            if action.jump.unwrap_or(false) {
                let vrc_client_clone = Arc::clone(&vrc_client);
                tokio::spawn(async move {
                    vrc_client_clone.input_jump();
                });
            }
        }
    });

    // Second tokio::spawn (message spammer) BAD CODE REMOVE LATER
    //tokio::spawn(async move {
    //    let vrc_client = Arc::clone(&vrc_client_ref2);
    //    let config = Arc::new(&config_clone);
    //    loop {
    //        let message = config.lock().unwrap().options.message.clone();
    //        vrc_client.chatbox_message(&message);
    //        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    //        config.lock().unwrap().update();
    //    }
    //});

    client.unwrap().start().await.unwrap()
}