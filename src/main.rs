use std::env;
use dotenvy::dotenv;
use tracing::{event, Level};
use tracing_subscriber::{EnvFilter};
use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_env("LOG_LEVEL")).init();
    event!(Level::INFO, "Starting kahvikamera bot...");
    let bot_token = env::var("BOT_TOKEN").expect("Bot API key should be set in env as BOT_TOKEN");
    let bot = Bot::new(bot_token);

    let _ = bot.set_my_commands(Command::bot_commands()).await;
    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Username(username) => {
            bot.send_message(msg.chat.id, format!("Your username is @{username}.")).await?
        }
        Command::UsernameAndAge { username, age } => {
            bot.send_message(msg.chat.id, format!("Your username is @{username} and age is {age}."))
                .await?
        }
    };

    Ok(())
}