use poise::serenity_prelude::{self as serenity};
use super::super::{Context, Error};
use crate::presentation_json;

#[poise::command(slash_command, guild_only)]
pub async fn presentation_decline(
    ctx: Context<'_>,
    #[description = "ID da Apresentação"] presentation_id: String,
    #[description = "Motivo da Recusa"] decline_reason: String,
) -> Result<(), Error> {

    let id: u32 = match presentation_id.parse() {
        Ok(id) => id,
        Err(_) => {
            ctx.say("ID inválido!").await?;
            return Ok(());
        }
    };

    let presentation = match presentation_json::pull_presentation(id).await {
        Some(p) => p,
        None => {
            ctx.say(format!("Apresentação com ID {} não encontrada!", id)).await?;
            return Ok(());
        }
    };

    // Pega o guild_id do contexto
    let guild_id = match ctx.guild_id() {
        Some(g) => g,
        None => {
            ctx.say("Erro: comando só pode ser usado em servidor.").await?;
            return Ok(());
        }
    };

    // Pega o member do usuário no guild
    let member: serenity::Member = match guild_id.member(&ctx.http(), presentation.clone().member).await {
        Ok(m) => m,
        Err(_) => {
            ctx.say("Erro ao acessar o membro.").await?;
            return Ok(());
        }
    };

    let fichas_channel = serenity::ChannelId::new(1443374881247985846);
    fichas_channel.send_message(&ctx.http(), serenity::CreateMessage::default().content(format!("<@{:?}>", member.user.id.get()))
        .add_embed(
            serenity::CreateEmbed::default()
                .title(format!("Apresentação de <@{:?}> <Recusada", member.user.id.get()))
                .description(format!("Infelizmente, sua apresentação foi recusada. Sinta-se à vontade para tentar novamente no futuro."))
                .field("Nota do moderador:", format!("```{}```", decline_reason), false)
                .color(0xFF0000)
        )
    ).await?;

    member.remove_role(&ctx.http(), 1439488945787371682).await?;

    // Remove do JSON
    presentation_json::remove_presentation(id).await;

    ctx.say(format!("Apresentação com ID {} de user <@{:?}> recusada por {}", id, presentation.member.user.id.get() ,ctx.author())).await?;

    Ok(())
}