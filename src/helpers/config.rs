use serde::Deserialize;
use tracing::info;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Config load error {0}")]
    Config(#[from] envy::Error),
}

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub app_host: String,
    pub app_port: u16,

    pub database_url: String,
    pub app_disable_commands: bool,
}

impl Config {
    pub fn from_env() -> Result<Self, Error> {
        info!("Initializing config");
        let env = envy::from_env::<Config>()?;

        Ok(env)
    }
}
