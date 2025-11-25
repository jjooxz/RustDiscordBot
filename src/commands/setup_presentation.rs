use super::super::{Context, Error};
use poise::serenity_prelude::{CreateActionRow};
use poise::serenity_prelude as serenity;

#[poise::command(slash_command, guild_only)]
pub async fn setup_presentation(
    ctx: Context<'_>,
    #[description = "Presentation Channel"] channel: poise::serenity_prelude::ChannelId,
) -> Result<(), Error> {
    println!("Setting up presentation in channel: {:?}", channel.to_channel(&ctx.http()).await.unwrap().guild().unwrap().name);

    let embed = poise::serenity_prelude::CreateEmbed::default()
        .title("Bem vindo ao nosso servidor!")
        .description("Antes de entrar no servidor, iremos fazer perguntas para conhecê-lo melhor e saber se você está apto a entrar em nosso servidor.")
        .color(poise::serenity_prelude::Colour::RED);

    let button = serenity::CreateButton::new("start_presentation") // Custom ID for an interactive button
        .style(serenity::ButtonStyle::Primary) // Blue button
        .label("Iniciar Apresentação")
        .into();
    
    let action_row = CreateActionRow::Buttons(vec![button]);

    let message = serenity::CreateMessage::default()
        .embed(embed)
        .components(vec![
            action_row
        ]);

    channel.send_message(&ctx.http(), message).await?;


    ctx.say("Presentation setup complete!").await?;
    Ok(())
}