use poise::serenity_prelude::{self as serenity, CreateInteractionResponse, CreateInteractionResponseMessage};
use crate::{Data, Error};
use crate::presentation_system;

/// Função para lidar com interações de botão
pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::InteractionCreate { interaction } => {

            // Desreferencia para pegar o variant
            if let serenity::Interaction::Component(component_interaction) = interaction {
                match component_interaction.data.custom_id.as_str() {
                    "start_presentation" => {
                        println!("Botão de apresentação clicado: {}", component_interaction.user.name);

                        if component_interaction.user.has_role(&ctx.http, serenity::GuildId::new(1437905258591289496), serenity::RoleId::new(1442273830117834974)).await? {
                            println!("User already in presentation");
                            component_interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::default()
                                    .content("O registro já foi iniciado!")
                                    .ephemeral(true)
                            )).await?;
                            return Ok(());
                        } 
                        if !(component_interaction.user.has_role(&ctx.http, serenity::GuildId::new(1437905258591289496), serenity::RoleId::new(1442271923521454341)).await?) {
                            println!("User already registrated");
                            component_interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::default()
                                    .content("Você já faz parte do servidor!")
                                    .ephemeral(true)
                            )).await?;
                            return Ok(());         
                        };

                        println!("Starting presentation for user: {}", component_interaction.user.name);


                        let response = CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::default()
                                .content("Iniciando sua apresentação! Por favor, responda às perguntas que serão feitas a seguir.")
                                .ephemeral(true)
                        );



                        component_interaction.create_response(&ctx.http, response).await?;

                        println!("Response sent");

                                                // Inicia o sistema de apresentação
                        presentation_system::start_presentation(ctx, component_interaction).await?;

                    },

                    _ => {}
                }
            }
        }
        serenity::FullEvent::GuildMemberAddition { new_member } => {
            println!("Novo membro entrou: {:?}", new_member.user.name);
            new_member.add_role(&ctx.http, 1442271923521454341).await?;
        }
        _ => {}
    }

    Ok(())
}
