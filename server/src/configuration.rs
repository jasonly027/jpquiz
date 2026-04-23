use std::path::PathBuf;

use secrecy::{ExposeSecret, SecretString};
use sqlx::postgres::{PgConnectOptions, PgSslMode};

pub const APP_NAME: &'static str = "jpquiz";

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub require_ssl: bool,
    pub user: String,
    pub password: SecretString,
    pub database: String,
}

impl DatabaseSettings {
    pub fn connection_options(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .ssl_mode(ssl_mode)
            .username(&self.user)
            .password(self.password.expose_secret())
            .database(&self.database)
    }
}

pub fn get() -> Result<Settings, config::ConfigError> {
    let base_path = project_path();
    let config_dir = base_path.join("configuration");

    let environment: Environment = std::env::var("APP_ENV")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENV");
    let environment_filename = format!("{}.yaml", environment.as_str());

    let settings = config::Config::builder()
        .add_source(config::File::from(config_dir.join("base.yaml")))
        .add_source(config::File::from(config_dir.join(environment_filename)))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize()
}

fn project_path() -> PathBuf {
    let mut dir = std::env::current_dir().expect("Failed to get current directory");

    if dir.join("Cargo.lock").exists() {
        return dir;
    }
    // When running cargo test, the workspace crate's directory becomes the CWD,
    // so we have to go up one level.
    dir.pop();

    dir
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{other} is not a support environment. Expecting `local` or `production`"
            )),
        }
    }
}
