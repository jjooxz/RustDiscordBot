use super::super::{Context, Error};

// Comando que responde com "Pong!"
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let latency = ctx.ping().await.as_millis();
    ctx.say(format!("🏓 Pong! Latência {:?}ms", latency)).await?;
    Ok(())
}