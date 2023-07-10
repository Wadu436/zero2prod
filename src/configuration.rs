use secrecy::{ExposeSecret, Secret, SecretBox};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub host: Box<str>,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: Box<str>,
    pub password: SecretBox<str>,
    pub port: u16,
    pub host: Box<str>,
    pub database_name: Box<str>,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }

    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_dir = std::env::current_dir().expect("Failed to determine current directory.");
    let configuration_dir = base_dir.join("configuration");

    // Detect running environment
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .as_str()
        .try_into()
        .expect("failed to parse APP_ENVIRONMENT");
    let environment_filename = format!("{}.yaml", environment.as_str());

    let settings = config::Config::builder()
        .add_source(config::File::from(configuration_dir.join("base.yaml")))
        .add_source(config::File::from(
            configuration_dir.join(environment_filename),
        ))
        .build()?;

    settings.try_deserialize::<Settings>()
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

impl TryFrom<&str> for Environment {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "local" => Ok(Environment::Local),
            "production" => Ok(Environment::Production),
            other => Err(format!(
                "`{}` is not a supported environment. Use either `local` or `production`",
                other
            )),
        }
    }
}
