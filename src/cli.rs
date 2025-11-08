use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "aihub")]
#[command(version = "0.1.0")]
#[command(about = "")]
#[command(long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Ask {
        #[arg(short, long)]
        prompt: String,

        #[arg(short, long, default_value = None)]
        instructions: Option<String>,

        #[arg(short, long, default_value = None)]
        effort: Option<String>,
    },

    Config {
        #[command(subcommand)]
        actions: ConfigAction,
    },
}

#[derive(Subcommand)]
pub enum ConfigAction {
    SetKey {
        #[arg(short, long)]
        key: String,
    },
}
