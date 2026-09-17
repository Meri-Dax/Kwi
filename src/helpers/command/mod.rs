use clap::Parser;

use crate::{CONFIG, helpers::AppState};

pub mod csv_extractor;
pub mod import_cook_ingredients;

pub trait Command {
    fn execute(&self, app_state: &AppState) -> impl std::future::Future<Output = ()> + Send;
}

#[derive(clap::Subcommand)]
pub enum Commands {
    ImportCookIngredients(import_cook_ingredients::ImportCookIngredientsCommand),
}

#[derive(Parser)]
#[command(name = "kwi")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

pub async fn run(cmd: Option<Commands>, app_state: &AppState) -> Option<()> {
    let command = cmd?;

    let config = CONFIG.get().unwrap_or_else(|| panic!("Unable to get global config"));

    if config.app_disable_commands {
        panic!("Attempted to run command, but commands are disabled.");
    }

    match command {
        Commands::ImportCookIngredients(c) => c.execute(app_state).await,
    };

    Some(())
}
