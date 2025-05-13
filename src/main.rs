use std::collections::HashMap;
use std::sync::Arc;
use poise::{serenity_prelude as serenity, Framework};
use tokio::sync::Mutex;

mod commands;

pub struct Data {
    tokens: Arc<Mutex<HashMap<u64, String>>>
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();
    let poise = init_poise();
    
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
                commands::verify::verify()
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { tokens: Arc::new(Default::default()) })
            })
        })
        .build()
}
