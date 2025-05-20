mod anilist;
mod commands;
mod database;
mod listener;
mod utils;
mod verification;

use crate::utils::cooldown::Cooldown;
use poise::serenity_prelude::{GatewayIntents, UserId};
use poise::{serenity_prelude as serenity, Framework};
use std::time::Duration;
use tokio::sync::Mutex;

pub struct Data {
    verified_role_id: u64,
    client_id: u32,
    client_secret: String,
    minimum_account_age: Duration,
    cooldown: Mutex<Cooldown<UserId>>
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");
    let intents = GatewayIntents::non_privileged();
    let poise = init_poise();

    database::init_database().expect("Failed to initialize database");

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(poise)
        .await;

    client.unwrap().start().await.unwrap();
}

/// Initializes the command framework Poise
fn init_poise() -> Framework<Data, Error> {
    Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::account::account(),
                commands::link::link(),
                commands::setup::setup(),
                commands::unlink::unlink()
            ],
            event_handler: |ctx, event, framework, data| {
                Box::pin(listener::button_listener::event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            let verified_role_id = std::env::var("DISCORD_ROLE").expect("Missing DISCORD_ROLE").parse().expect("Invalid DISCORD_ROLE");
            let client_id: u32 = std::env::var("ANILIST_CLIENT_ID").expect("Missing ANILIST_CLIENT_ID").parse().expect("Invalid ANILIST_CLIENT_ID");
            let client_secret: String = std::env::var("ANILIST_CLIENT_SECRET").expect("Missing ANILIST_CLIENT_SECRET");
            
            let account_age_hours: u64 = std::env::var("ANILIST_ACCOUNT_AGE_HOURS").expect("Missing ANILIST_ACCOUNT_AGE_HOURS").parse().expect("Invalid ANILIST_ACCOUNT_AGE_HOURS");
            let minimum_account_age: Duration = Duration::from_secs(account_age_hours * 60 * 60);

            let cooldown_minutes: u64 = std::env::var("VERIFICATION_COOLDOWN_MINUTES").expect("Missing VERIFICATION_COOLDOWN_MINUTES").parse().expect("Invalid VERIFICATION_COOLDOWN_MINUTES");
            let verification_cooldown: Duration = Duration::from_secs(cooldown_minutes * 60);

            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    verified_role_id,
                    client_id,
                    client_secret,
                    minimum_account_age,
                    cooldown: Mutex::new(Cooldown::new(verification_cooldown))
                })
            })
        })
        .build()
}
