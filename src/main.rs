#[macro_use]
extern crate log;

use std::env;

#[derive(thiserror::Error, Debug)]
enum AppError {
    #[error("{0}")]
    Serenity(#[from] poise::serenity::Error),
}

type Context<'a> = poise::Context<'a, (), AppError>;

#[poise::command(prefix_command, hide_in_help)]
async fn register(ctx: Context<'_>, #[flag] global: bool) -> Result<(), AppError> {
    poise::builtins::register_application_commands(ctx, global).await?;
    Ok(())
}

/// Add two number.
#[poise::command(prefix_command, slash_command)]
async fn add(
    ctx: Context<'_>,
    #[description = "The first number."] a: i32,
    #[description = "The second number."] b: i32,
) -> Result<(), AppError> {
    poise::say_reply(ctx, format!("{}", a + b)).await?;
    Ok(())
}

async fn on_error(error: poise::FrameworkError<'_, (), AppError>) {
    error!("{:?}", error);
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");

    let options = poise::FrameworkOptions {
        commands: vec![register(), add()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("::".to_string()),
            ..Default::default()
        },
        on_error: |err| Box::pin(on_error(err)),
        ..Default::default()
    };

    poise::Framework::build()
        .token(token)
        .options(options)
        .user_data_setup(|_, _, _| Box::pin(async { Ok(()) }))
        .run()
        .await
        .unwrap();
}