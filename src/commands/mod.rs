pub mod ping;
pub mod say;
pub mod setup_presentation;
pub mod presentation_approve;
pub mod presentation_decline;

// Função que retorna todos os comandos como um vetor
pub fn get_commands() -> Vec<poise::Command<super::Data, super::Error>> {
    vec![
        ping::ping(),
        say::say(),
        setup_presentation::setup_presentation(),
        presentation_approve::presentation_approve(),
        presentation_decline::presentation_decline()
    ]
}