use super::super::{Context, Error};

// Comando que responde com "Pong!"
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let latency = ctx.ping().await.as_millis();

    if latency == 0 {
        ctx.say("🏓 Pong! O bot acabou de ligar, então não foi possivel calcular a latência ainda, espere um pouco e rode esse comando mais tarde ;3").await?;
        return Ok(())
    }

    ctx.say(format!("🏓 Pong! Latência {}ms ", latency)).await?;
    Ok(())
}