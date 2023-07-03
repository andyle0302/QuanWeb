use libpassgen::{generate_password, Pool};
use miette::{miette, Report};

use config::{Config, ConfigError, File};

pub const KEY_SECRET: &'static str = "secret_key";
pub const KEY_PORT: &str = "port";
pub const DEFAULT_PORT: u16 = 3721;
pub const ALPHANUMERIC: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

pub fn gen_fallback_secret() -> String {
    let pool: Pool = ALPHANUMERIC.parse().unwrap_or_default();
    // 64 is the secret bytes count required by axum-sessions
    generate_password(&pool, 64).into()
}

pub fn get_config() -> Result<Config, ConfigError> {
    let fallback_secret = gen_fallback_secret();
    Config::builder()
        .set_default(KEY_SECRET, fallback_secret)?
        .set_default(KEY_PORT, DEFAULT_PORT)?
        .add_source(File::with_name("custom_settings.toml").required(false))
        .add_source(File::with_name(".secrets.toml").required(false))
        .build()
}

pub fn get_secret_bytes(config: &Config) -> Result<Vec<u8>, Report> {
    let secret_str = config
        .get_string(KEY_SECRET)
        .map_err(|e| miette!("Failed to get secret key: {e}"))?;
    tracing::info!("Secret key: {}", secret_str);
    Ok(secret_str.as_bytes().into())
}

pub fn get_listening_port(config: &Config) -> u16 {
    let port = config
        .get_int(KEY_PORT)
        .map_err(|e| miette!("Failed to get port: {e}"))
        .map(|p| p as u16);
    port.unwrap_or(DEFAULT_PORT)
}