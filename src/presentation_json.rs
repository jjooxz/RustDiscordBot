use poise::serenity_prelude as serenity;
use serde::{Serialize, Deserialize};
use tokio::fs::{OpenOptions, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Presentation {
    pub id: u32,
    pub member: serenity::Member,
    pub resposta1: String,
    pub resposta2: String,
    pub resposta3: String,
    pub resposta4: String,
}

const FILE_PATH: &str = "presentations.json";

pub async fn load_presentations() -> Vec<Presentation> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(FILE_PATH)
        .await
        .unwrap();

    let mut content = String::new();
    file.read_to_string(&mut content).await.unwrap();

    if content.is_empty() {
        vec![]
    } else {
        serde_json::from_str(&content).unwrap()
    }
}

pub async fn save_presentations(presentations: &Vec<Presentation>) {
    let mut file = File::create(FILE_PATH).await.unwrap();
    file.write_all(serde_json::to_string_pretty(presentations).unwrap().as_bytes())
        .await
        .unwrap();
}

pub async fn push_presentation(presentation: Presentation) {
    let mut presentations = load_presentations().await;
    presentations.push(presentation);
    save_presentations(&presentations).await;
}

pub async fn pull_presentation(id: u32) -> Option<Presentation> {
    let presentations = load_presentations().await;
    presentations.into_iter().find(|p| p.id == id)
}

pub async fn remove_presentation(id: u32) {
    let mut presentations = load_presentations().await;
    presentations.retain(|p| p.id != id);
    save_presentations(&presentations).await;
}