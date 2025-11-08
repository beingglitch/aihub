mod cli;
mod client;
mod config;

use crate::client::openai::OpenAIClient;
use crate::config::Config;
use anyhow::{Result, anyhow};
use clap::Parser;
use cli::{Cli, Commands, ConfigAction};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    let mut config = Config::default();

    match cli.command {
        Commands::Ask {
            prompt,
            instructions,
            effort,
        } => {
            config = Config::load()?;

            let _ = handle_ask(&config, prompt, instructions, effort).await;
        }

        Commands::Config { actions } => {
            let _ = handle_config(&mut config, actions).await;
        }
    }

    Ok(())
}

async fn handle_ask(
    config: &Config,
    prompt: String,
    instructions: Option<String>,
    effort: Option<String>,
) -> Result<()> {
    let api_key = std::env::var("OPENAI_API_KEY")
        .ok()
        .or_else(|| config.openai_api_key.clone())
        .ok_or_else(|| {
            anyhow!("No API key found. Set OPENAI_API_KEY or use `aihub config set-key`")
        })?;

    let client = OpenAIClient::new(api_key);

    println!("Thinking...");

    let response = client.chat(&prompt, instructions, effort).await?;
    println!("\n{}", response);

    Ok(())
}

async fn handle_config(config: &mut Config, action: ConfigAction) -> Result<()> {
    match action {
        ConfigAction::SetKey { key } => {
            config.set_api_key(key)?;

            println!("API key save to {}", Config::config_file()?.display());
        }
    }

    Ok(())
}
