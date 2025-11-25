use super::super::{Context, Error};

// Comando que responde com "Pong!"
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("🏓 Pong!").await?;
    Ok(())
}