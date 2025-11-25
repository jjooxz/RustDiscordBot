use super::super::{Context, Error};
// Comando que faz o bot repetir a mensagem fornecida pelo usuário
#[poise::command(slash_command, aliases("falar"))]
pub async fn say(
    ctx: Context<'_>,
    #[description = "A Mensagem à ser dita"] message: String,
) -> Result<(), Error> {
    ctx.say(message).await?;
    Ok(())
}