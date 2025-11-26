use super::super::{Context, Error};
use crate::presentation_json;
use poise::serenity_prelude::{self as serenity};

#[poise::command(slash_command, guild_only)]
pub async fn presentation_approve(
    ctx: Context<'_>,
    #[description = "ID da Apresentação"] presentation_id: String,
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
        _ => {
            ctx.say(format!("Apresentação com ID {} não encontrada!", id)).await?;
            return Ok(());
        }
    };

    // Pega o guild_id do contexto
    let guild_id = match ctx.guild_id() {
        Some(g) => g,
        _ => {
            ctx.say("Erro: comando só pode ser usado em servidor.").await?;
            return Ok(());
        }
    };

    // Pega o member do usuário no guild
    let member: serenity::Member = match guild_id.member(&ctx.http(), presentation.member).await {
        Ok(m) => m,
        Err(_) => {
            ctx.say("Erro ao acessar o membro.").await?;
            return Ok(());
        }
    };

    // Remove role "Registrando" e adiciona "Registrado"
    member.remove_role(&ctx.http(), serenity::RoleId::new(1442271923521454341)).await?;

    let dm = member.user.create_dm_channel(&ctx.http()).await?;
    dm.send_message(&ctx.http(), serenity::CreateMessage::default()
        .add_embed(
            serenity::CreateEmbed::default()
                .title("Apresentação Aprovada!")
                .description(format!("Parabéns <@{:?}>! Sua apresentação foi aprovada e você agora tem acesso completo ao servidor. Seja bem-vindo!", member.user.id.get()))
                .color(0x00FF00)
        )
    ).await?;

    let fichas_channel = serenity::ChannelId::new(1443374881247985846);
    fichas_channel.send_message(&ctx.http(), serenity::CreateMessage::default().content(format!("<@{:?}>", member.user.id.get()))
        .add_embed(
            serenity::CreateEmbed::default()
                .title("Nova Apresentação Aprovada!")
                .description(format!("A apresentação de <@{:?}> foi aprovada! Dê as boas-vindas ao novo membro!", member.user.id.get()))
                .field("ID da Apresentação", format!("```#{}```", id.to_string()), false)
                .field("Nome do Membro", format!("```{}```", member.user.name), false)
                .field("Apresentação", format!("```{}```", presentation.resposta2), false)
                .field("Identificação", format!("```{}```", presentation.resposta3), false)
                .field("Intuíto no servidor", format!("```{}```", presentation.resposta4), false)
                .color(0x00FF00)
        )
    ).await?;

    // Remove do JSON
    presentation_json::remove_presentation(id).await;

    ctx.say(format!("Apresentação com ID {} aprovada!", id)).await?;
    Ok(())
}


