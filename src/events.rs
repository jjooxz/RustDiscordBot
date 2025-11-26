use poise::serenity_prelude::{
    self as serenity,
    CreateInteractionResponse,
    CreateInteractionResponseMessage
};
use crate::{Data, Error};
use crate::presentation_system;
use crate::presentation_json; // <- importante!

/// Lida com eventos do Discord
pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        // ---------------------------------------
        // 🔘 CLICK NO BOTÃO
        // ---------------------------------------
        serenity::FullEvent::InteractionCreate { interaction } => {
            if let serenity::Interaction::Component(component_interaction) = interaction {
                match component_interaction.data.custom_id.as_str() {
                    "start_presentation" => {
                        println!("Botão START clicado por {}", component_interaction.user.name);

                        let guild_id = serenity::GuildId::new(1437905258591289496);
                        let role_member = serenity::RoleId::new(1442271923521454341);
                        let role_in_presentation = serenity::RoleId::new(1442273830117834974);

                        let user_id = component_interaction.user.id.get();

                        // ---------------------------------------
                        // 🔍 1. CHECAR SE JÁ EXISTE APRESENTAÇÃO PENDENTE
                        // ---------------------------------------
                        let presentations = presentation_json::load_presentations().await;

                        let already_exists = presentations
                            .iter()
                            .any(|p| p.member.user.id.get() == user_id);

                        if already_exists {
                            println!("Usuário já tem apresentação pendente.");
                            component_interaction.create_response(
                                &ctx.http,
                                CreateInteractionResponse::Message(
                                    CreateInteractionResponseMessage::default()
                                        .content("Você já iniciou uma apresentação e ela ainda não foi finalizada!")
                                        .ephemeral(true)
                                )
                            ).await?;
                            return Ok(());
                        }

                        // ---------------------------------------
                        // 🔒 2. VERIFICAR SE JÁ POSSUI O CARGO DE "EM APRESENTAÇÃO"
                        // ---------------------------------------
                        if component_interaction.user.has_role(&ctx.http, guild_id, role_in_presentation).await? {
                            println!("User already in presentation (role match).");
                            component_interaction.create_response(
                                &ctx.http,
                                CreateInteractionResponse::Message(
                                    CreateInteractionResponseMessage::default()
                                        .content("O registro de apresentação já foi iniciado!")
                                        .ephemeral(true)
                                )
                            ).await?;
                            return Ok(());
                        }

                        // ---------------------------------------
                        // 🔒 3. VERIFICAR SE O USUÁRIO JÁ ESTÁ REGISTRADO NO SERVIDOR
                        // ---------------------------------------
                        if !(component_interaction.user.has_role(&ctx.http, guild_id, role_member).await?) {
                            println!("User already registered.");
                            component_interaction.create_response(
                                &ctx.http,
                                CreateInteractionResponse::Message(
                                    CreateInteractionResponseMessage::default()
                                        .content("Você já faz parte do servidor! Não é necessário registrar novamente.")
                                        .ephemeral(true)
                                )
                            ).await?;
                            return Ok(());
                        }

                        // ---------------------------------------
                        // 🚀 4. INICIAR APRESENTAÇÃO
                        // ---------------------------------------
                        println!("Iniciando apresentação para {}", component_interaction.user.name);

                        // Chama o sistema de apresentação
                        presentation_system::start_presentation(ctx, component_interaction).await?;
                    }

                    _ => {}
                }
            }
        }

        // ---------------------------------------
        // 👤 NOVO MEMBRO ENTROU
        // ---------------------------------------
        serenity::FullEvent::GuildMemberAddition { new_member } => {
            println!("Novo membro entrou: {}", new_member.user.name);
            new_member.add_role(&ctx.http, 1442271923521454341).await?;
        }

        _ => {}
    }

    Ok(())
}
