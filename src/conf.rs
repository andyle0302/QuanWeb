
use hex;
use rand::Rng;
use miette::{miette, Report};

use config::{Config, ConfigError, File};

pub const KEY_SECRET: &'static str = "secret_hex";
pub const KEY_PORT: &str = "port";
pub const DEFAULT_PORT: u16 = 3721;

pub fn get_config() -> Result<Config, ConfigError> {
    let fallback_secret = rand::thread_rng().gen::<[u8; 64]>();
    let fallback_secret = hex::encode(fallback_secret);
    Config::builder()
        .set_default(KEY_SECRET, fallback_secret)?
        .set_default(KEY_PORT, DEFAULT_PORT)?
        .add_source(File::with_name("custom_settings.toml").required(false))
        .add_source(File::with_name(".secrets.toml").required(false))
        .build()
}

pub fn get_secret_bytes(config: &Config) -> Result<Vec<u8>, Report> {
    let secret_hex = config.get_string(KEY_SECRET).map_err(|e| miette!("Failed to get secret hex: {e}"))?;
    tracing::info!("Secret hex: {}", secret_hex);
    let secret_bytes = hex::decode(secret_hex).map_err(|e| miette!("Invalid secret hex string {e}"))?;
    Ok(secret_bytes)
}

pub fn get_listening_port(config: &Config) -> u16 {
    let port = config.get_int(KEY_PORT).map_err(|e| miette!("Failed to get port: {e}")).map(|p| p as u16);
    port.unwrap_or(DEFAULT_PORT)
}
