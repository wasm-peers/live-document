use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub signaling_server_url: String,
    pub stun_server_urls: String,
    pub turn_server_urls: String,
    pub turn_server_username: String,
    pub turn_server_credential: String,
}

/// Access to parsed environment variables.
pub static CONFIG: Lazy<Config> = Lazy::new(|| envy::from_env().expect("some env vars missing"));
