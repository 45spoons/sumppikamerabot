use std::env;
use dotenvy::dotenv;
use tracing::{event, Level};
use tracing_subscriber::{EnvFilter};
use teloxide::{prelude::*, utils::command::BotCommands};

use crate::kattila_api::SeurantaUsers;


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

#[derive(Debug, BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Olen Linkki Jyväskylä ry:n kahvikamera, voit käyttä seuraavia komentojani:")]
enum Command {
    #[command(description = "Selittää botin tarkoituksen")]
    Start,
    #[command(description = "Listaa botin komennot")]
    Help,
    #[command(description = "Antaa kuvan Kattilan kahvinkeittimistä")]
    KahviKamera,
    #[command(description = "Lähettää kahvin halukkuusilmoituksen")]
    HaluanKahvia,
    #[command(description = "Listaa kattilan paikallaolijat")]
    Kattilassa,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    tracing::info!(?cmd, ?msg.id, "begin answering command");
    let resulting_message = match cmd {
        Command::Start => {
            bot.send_message(msg.chat.id, "Hei! Minä olen Linkki Jyväskylä ry:n kahvikamera 📸🐝").await?
        },
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?
        },
        Command::KahviKamera => {
            match reqwest::get("https://kattila-api.linkkijkl.fi/coffee/image").await {
                Ok(response) => {
                    match response.bytes().await {
                        Ok(bytes) => {
                            tracing::info!(bytes = ?bytes.len(), "success fetching coffee camera image");
                            let photo = teloxide::types::InputFile::memory(bytes);
                            bot.send_photo(msg.chat.id, photo).await?
                        },
                        Err(error) => {
                            tracing::error!(?error, "couldn't form message with coffee image");
                            bot.send_message(msg.chat.id, format!("Jotain meni pieleen kahvikameran kuvan käsittelyssä ... :-(")).await?
                        },
                    }
                },
                Err(error) => {
                    tracing::error!(?error, "couldn't fetch coffee image");
                    bot.send_message(msg.chat.id, format!("En saanut haettua kahvikameran kuvaa ... :-(")).await?
                },
            }
        }
        Command::HaluanKahvia => {
            let client = reqwest::Client::new();
            match client.post("https://kattila-api.linkkijkl.fi/interested").send().await {
                Ok(response) => {
                    match response.text().await {
                        Ok(interested_amount) => {
                            bot.send_message(msg.chat.id, format!("Halukkaat: {}", interested_amount)).await?
                        },
                        Err(error) => {
                            tracing::error!(?error, "couldn't process interest response");
                            bot.send_message(msg.chat.id, format!("Jotain meni pieleen halukkuustiedon käsittelyssä ... :-(")).await?
                        },
                    }
                },
                Err(error) => {
                    tracing::error!(?error, "couldn't send coffee interest");
                    bot.send_message(msg.chat.id, format!("Jotain meni pieleen halukkuuden lähetyksessä ... :-(")).await?
                },
            }
        }
        Command::Kattilassa => {
            match reqwest::get("https://kattila-api.linkkijkl.fi/seuranta/users").await {
                Ok(response) => {
                    match response.json::<SeurantaUsers>().await {
                        Ok(users) => {
                            bot.send_message(msg.chat.id, format!("Kattilassa paikalla: {}", users.users.iter().map(|user| user.username.as_str()).collect::<Vec<&str>>().join(", "))).await?
                        },
                        Err(error) => {
                            tracing::error!(?error, "couldn't parse seuranta data");
                            bot.send_message(msg.chat.id, format!("Jotain meni pieleen seurantatietojen käsittelyssä ... :-(")).await?
                        }
                    }
                },
                Err(error) => {
                    tracing::error!(?error, "couldn't fetch seuranta data");
                    bot.send_message(msg.chat.id, format!("Jotain meni pieleen seurantatietojen haussa ... :-(")).await?
                },
            }
        }
    };
    tracing::info!(?cmd, ?msg.id, response_msg_id = ?resulting_message.id, "finish answering command");
    Ok(())
}

mod kattila_api {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct SeurantaUser {
        pub username: String,
    }

    #[derive(Deserialize)]
    pub struct SeurantaUsers {
        pub users: Vec<SeurantaUser>,
    }
}