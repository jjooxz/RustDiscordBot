mod commands;
mod events;
mod presentation_system;
mod proximoid;
mod presentation_json;

use poise::serenity_prelude::{self as serenity};

use crate::commands::get_commands;

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: get_commands(),
            event_handler: |ctx, event, framework, data| {
                Box::pin(events::event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                println!("Bot online!");

                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                let activity = serenity::ActivityData::playing("Moderando o Bonfire!");
                ctx.shard
                    .set_presence(Some(activity), serenity::OnlineStatus::DoNotDisturb);

                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}