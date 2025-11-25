use std::time::Duration;
use crate::presentation_json::Presentation;
use crate::proximoid;
use crate::presentation_json;

use poise::serenity_prelude::ChannelId;
use poise::serenity_prelude::CreateChannel;
use poise::serenity_prelude::CreateEmbed;
use poise::serenity_prelude::CreateEmbedFooter;
use poise::serenity_prelude::CreateMessage;
use poise::serenity_prelude::MessageCollector;
use poise::serenity_prelude::PermissionOverwrite;
use poise::serenity_prelude::ReactionType;
use poise::serenity_prelude::RoleId;
use poise::serenity_prelude::{self as serenity};
use poise::futures_util::StreamExt;

pub async fn start_presentation(
    ctx: &serenity::Context,
    component_interaction: &serenity::ComponentInteraction,
) -> Result<(), serenity::Error> {
    println!("start presentation");
    
    let bot_user = ctx.http.get_current_user().await?;
    let avatar_url = bot_user.avatar_url().unwrap_or_default();
    let guild = poise::serenity_prelude::GuildId::new(1437905258591289496);

    let cargo_fazendo = guild.role(&ctx.http, RoleId::new(1442273830117834974)).await.unwrap();

    let user_id = component_interaction.user.id;
    let member = component_interaction.member.as_ref().unwrap();
    let next_id = proximoid::pegar_proximo_id().await? as u32;

    member.add_role(&ctx.http, cargo_fazendo.id).await?;

    println!("added role");
    
    let channel = guild.create_channel(
        &ctx.http,
        CreateChannel::new(format!("apresentação-#{}", next_id.clone().to_string()))
            .permissions(vec![
                PermissionOverwrite {
                    allow: serenity::Permissions::VIEW_CHANNEL | serenity::Permissions::SEND_MESSAGES,
                    deny: serenity::Permissions::empty(),
                    kind: serenity::PermissionOverwriteType::Member(user_id),
                },
                PermissionOverwrite {
                    allow: serenity::Permissions::empty(),
                    deny: serenity::Permissions::VIEW_CHANNEL,
                    kind: serenity::PermissionOverwriteType::Role(guild.everyone_role()),
                },
            ]).position(guild.channels(&ctx.http).await?.len() as u16 + 1)
            .category(1442343428481486848)
    ).await?;

    // manda a mensagem no canal recém criado
    channel.send_message(
        &ctx.http,
        serenity::CreateMessage::default().add_embed(
            serenity::CreateEmbed::default()
                .title("The Bonfire | Apresentação")
                .description("Por favor, responda às perguntas a seguir para completar sua apresentação.")
                .color(0xFFbbff)
                .thumbnail(avatar_url)
        )
    ).await?;

    /* = = = = = = = = = = = = = = = = = = = =
         QUESTION 1
    = = = = = = = = = = = = = = = = = = = =*/

    channel.send_message(
        &ctx.http,
        serenity::CreateMessage::default().add_embed(
            serenity::CreateEmbed::default()
                .title("The Bonfire | Pergunta 1")
                .field("1. Qual a sua data de nascimento?", "Digite no formato DD/MM/AAAA (exemplo: 25/12/2004). Essa informação nos ajuda a manter o espaço seguro e apropriado para todos.", false)
                .color(0xFFbbff)
                .footer(CreateEmbedFooter::new("Mínimo: 6 caracteres | Máximo: 10 caracteres | Timeout em 60 segundos..."))
        )
    ).await?;

    // await user message here
    let resposta1: String;
    loop {
        // espera mensagem do usuário
        let mut collector = MessageCollector::new(&ctx.shard)
            .channel_id(channel.id)
            .author_id(user_id)
            .timeout(Duration::from_secs(60))
            .stream();

        let Some(message) = collector.next().await else {
            println!("timeout");

            channel.delete(&ctx.http).await?;
            member.remove_role(&ctx.http, cargo_fazendo.id).await?;
            return Ok(());
        };

        let conteudo = message.clone().content.trim().to_string();

        // validação de tamanho
        if conteudo.len() >= 6 && conteudo.len() <= 10 {
            let _ = message.react(&ctx.http, ReactionType::Unicode("✅".to_string())).await;
            resposta1 = conteudo;
            break; // <- saiu do loop, finalmente OK
        }

        let _ = message.react(&ctx.http, ReactionType::Unicode("❌".to_string())).await;

        // se inválido → manda msg e repete o loop
        channel.send_message(
            &ctx.http,
            CreateMessage::default().content(
                "⚠️ **Formato inválido!**  
    A data deve ter entre **6 e 10 caracteres** (ex: 25/12/2004).  
    Tente novamente:"
            )
        ).await?;
    }

    /* = = = = = = = = = = = = = = = = = = = =
         QUESTION 2
    = = = = = = = = = = = = = = = = = = = =*/

    channel.send_message(
        &ctx.http,
        serenity::CreateMessage::default().add_embed(
            serenity::CreateEmbed::default()
                .title("The Bonfire | Pergunta 2")
                .field("2. Fale um pouco sobre você!", "O que você gosta de fazer em seu tempo livre? quais são seus interesses ou paixões? Pode ser hobbies, estudos, trabalho, sonhos... Queremos te conhecer melhor!", false)
                .color(0xFFbbff)
                .footer(CreateEmbedFooter::new("Mínimo: 30 caracteres | Máximo: 500 caracteres | Timeout em 120 segundos..."))
        )
    ).await?;

    // await user message here
    let resposta2: String;
    loop {
        // espera mensagem do usuário
        let mut collector = MessageCollector::new(&ctx.shard)
            .channel_id(channel.id)
            .author_id(user_id)
            .timeout(Duration::from_secs(60))
            .stream();

        let Some(message) = collector.next().await else {
            println!("timeout");

            channel.delete(&ctx.http).await?;
            member.remove_role(&ctx.http, cargo_fazendo.id).await?;
            return Ok(());
        };

        let conteudo = message.clone().content.trim().to_string();

        // validação de tamanho
        if conteudo.len() >= 30 && conteudo.len() <= 500 {
            let _ = message.react(&ctx.http, ReactionType::Unicode("✅".to_string())).await;
            resposta2 = conteudo;
            break; // <- saiu do loop, finalmente OK
        }

        let _ = message.react(&ctx.http, ReactionType::Unicode("❌".to_string())).await;

        // se inválido → manda msg e repete o loop
        channel.send_message(
            &ctx.http,
            CreateMessage::default().content(
                "⚠️ **Formato inválido!**  
    A data deve ter entre **6 e 10 caracteres** (ex: 25/12/2004).  
    Tente novamente:"
            )
        ).await?;
    }

    /* = = = = = = = = = = = = = = = = = = = =
         QUESTION 3
    = = = = = = = = = = = = = = = = = = = =*/

    channel.send_message(
        &ctx.http,
        serenity::CreateMessage::default().add_embed(
            serenity::CreateEmbed::default()
                .title("The Bonfire | Pergunta 3")
                .field("3. Com o que você se identifica?", "Pode ser femboy, genderfluid, não-binário, crossdresser, etc. Sinta-se à vontade para compartilhar o que achar relevante. Essa informação nos ajuda a criar um ambiente acolhedor e respeitoso para todos os membros.", false)
                .color(0xFFbbff)
                .footer(CreateEmbedFooter::new("Mínimo: 3 caracteres | Máximo: 100 caracteres | Timeout em 120 segundos..."))
        )
    ).await?;

    // await user message here
    let resposta3: String;
    loop {
        // espera mensagem do usuário
        let mut collector = MessageCollector::new(&ctx.shard)
            .channel_id(channel.id)
            .author_id(user_id)
            .timeout(Duration::from_secs(60))
            .stream();

        let Some(message) = collector.next().await else {
            println!("timeout");

            channel.delete(&ctx.http).await?;
            member.remove_role(&ctx.http, cargo_fazendo.id).await?;
            return Ok(());
        };

        let conteudo = message.clone().content.trim().to_string();

        // validação de tamanho
        if conteudo.len() >= 3 && conteudo.len() <= 100 {
            let _ = message.react(&ctx.http, ReactionType::Unicode("✅".to_string())).await;
            resposta3 = conteudo;
            break; // <- saiu do loop, finalmente OK
        }

        let _ = message.react(&ctx.http, ReactionType::Unicode("❌".to_string())).await;

        // se inválido → manda msg e repete o loop
        channel.send_message(
            &ctx.http,
            CreateMessage::default().content(
                "⚠️ **Formato inválido!**  
    A data deve ter entre **6 e 10 caracteres** (ex: 25/12/2004).  
    Tente novamente:"
            )
        ).await?;
    }

    /* = = = = = = = = = = = = = = = = = = = =
         QUESTION 4
    = = = = = = = = = = = = = = = = = = = =*/

    channel.send_message(
        &ctx.http,
        serenity::CreateMessage::default().add_embed(
            serenity::CreateEmbed::default()
                .title("The Bonfire | Pergunta 4")
                .field("4. Qual o seu intuíto no servidor?", "O que você espera encontrar ou alcançar aqui? Pode ser fazer amizades, participar de eventos, aprender algo novo, etc. Queremos entender melhor suas expectativas para tornar sua experiência a melhor possível.", false)
                .color(0xFFbbff)
                .footer(CreateEmbedFooter::new("Mínimo: 15 caracteres | Máximo: 400 caracteres | Timeout em 120 segundos..."))
        )
    ).await?;

    // await user message here
    let resposta4: String;
    loop {
        // espera mensagem do usuário
        let mut collector = MessageCollector::new(&ctx.shard)
            .channel_id(channel.id)
            .author_id(user_id)
            .timeout(Duration::from_secs(60))
            .stream();

        let Some(message) = collector.next().await else {
            println!("timeout");

            channel.delete(&ctx.http).await?;
            member.remove_role(&ctx.http, cargo_fazendo.id).await?;
            return Ok(());
        };

        let conteudo = message.clone().content.trim().to_string();

        // validação de tamanho
        if conteudo.len() >= 15 && conteudo.len() <= 400 {
            let _ = message.react(&ctx.http, ReactionType::Unicode("✅".to_string())).await;
            resposta4 = conteudo;
            break; // <- saiu do loop, finalmente OK
        }

        let _ = message.react(&ctx.http, ReactionType::Unicode("❌".to_string())).await;

        // se inválido → manda msg e repete o loop
        channel.send_message(
            &ctx.http,
            CreateMessage::default().content(
                "⚠️ **Formato inválido!**  
    A data deve ter entre **6 e 10 caracteres** (ex: 25/12/2004).  
    Tente novamente:"
            )
        ).await?;
    }

    let presentation = Presentation {
        id: next_id.clone() as u32,
        member: member.clone(),
        resposta1: resposta1.clone(),
        resposta2: resposta2.clone(),
        resposta3: resposta3.clone(),
        resposta4: resposta4.clone()
    };

    let _ = ChannelId::new(1442641827642478652).send_message(&ctx.http, CreateMessage::new().add_embed(CreateEmbed::new()
        .title(format!("Nova apresentação de: @<{:?}>", member.user.id))
        .field("ID", format!("´#{}´", next_id), false)
        .field("Data de nascimento", format!("´{}´", resposta1.to_string()), false)
        .field("Descrição da pessoa", format!("´{}´", resposta2.to_string()), false)
        .field("Como a pessoa de identifica", format!("´{}´", resposta3.to_string()), false)
        .field("Intuito no servidor", format!("´{}´", resposta4.to_string()), false)
    )).await;

    presentation_json::push_presentation(presentation).await;

    channel.send_message(
        &ctx.http,
        serenity::CreateMessage::default().add_embed(
            serenity::CreateEmbed::default()
                .title("The Bonfire | Obrigado Pelo Registro!")
                .field("Obrigado por completar o registro!", format!("Sua apresentação foi enviada para a equipe de moderação com código {:?}. Assim que for aprovada, você receberá acesso completo ao servidor. Bem-vindo ao Bonfire!", next_id.clone()), false)
                .color(0xFFbbff)
                .footer(CreateEmbedFooter::new("Em breve você terá acesso ao servidor!"))
        )
    ).await?;

    // espera 5 segundos antes de deletar o canal
    tokio::time::sleep(Duration::from_secs(15)).await;
    channel.delete(&ctx.http).await?;
    member.remove_role(&ctx.http, cargo_fazendo.id).await?;

    Ok(())
}