# RustDiscordBot

A fast and modular Discord bot built in Rust, focused on clean architecture, reliability, and speed.

<p align="center">
  <img src="https://img.shields.io/badge/Rust-stable-orange?logo=rust" />
  <img src="https://img.shields.io/badge/Discord%20API-Bot%20Ready-5865F2?logo=discord&logoColor=white" />
  <img src="https://img.shields.io/github/last-commit/jjooxz/RustDiscordBot" />
  <img src="https://img.shields.io/badge/License-MIT-blue" />
</p>

---

## 📌 Overview

<details>
<summary>Commands</summary>

### Available Commands

| Command       | Description                                   |
|---------------|-----------------------------------------------|
| `/ping`       | Returns a basic latency check.                |
| `/setup_presentation `       | Sets up the presentation and registry system           |
| `/say`       | Makes the bot say something               |
| `/presentation_approve`  | Approves a presentation from the system |
| `/presentation_decline`  | Declines a presentation from the system |

Commands can be easily made from files on the `/src/commands`!

</details>

<br>

RustDiscordBot is a Discord bot written in [**Rust**](https://rust-lang.org/), designed for performance, modularity, and stability.  
It uses modern libraries such as [**serenity**](https://docs.rs/serenity/latest/serenity/) or [**poise**](https://docs.rs/poise/latest/poise/) (depending on your setup), supports slash commands, and is structured to be easily extended.

---

## 📦 Features

- Slash command support  
- Clear and modular project structure  
- High-performance async runtime  
- Easy configuration through `.env`  
- Designed for scalability and future extensions
- System for user registry
- Language: PT-br 🇧🇷

---

## 🚀 Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/jjooxz/RustDiscordBot.git
cd RustDiscordBot
```

### 2. Configure the environment

Rename the `.env.example` file to `.env` and replace the value of `DISCORD_TOKEN` with your Discord bot token.

### 3. Change the code

Change the channel ids, guild ids, user ids, role ids... to your own IDS from your discord server you are going to run your bot in.

### 4. Build the project

```bash
cargo build --release
```

### 5. Run the bot

```bash
cargo run --release
```

## 📄 License

This project is licensed under the **Creative Commons BY-NC 4.0** license.  
Commercial use is not permitted.  
See the [LICENSE](./LICENSE) file for full terms.