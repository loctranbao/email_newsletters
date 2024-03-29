use secrecy::{Secret, ExposeSecret};
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::{postgres::{PgConnectOptions, PgSslMode}, ConnectOptions};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // initialize our configuration reader
    let mut settings = config::Config::default();

    let base_path = std::env::current_dir().expect("fail to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    // read the default configuration
    settings.merge(config::File::from(configuration_directory.join("base")).required(true))?;

    // detect running environment
    // default to 'local' if unspecified
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("failed to parse APP_ENVIRONMENT");

    // layer on the environment-specific value.
    settings.merge(
        config::File::from(configuration_directory.join(environment.as_str())).required(true)
    )?;

    // add in settings from environment variables (with prefix of APP and '__' as separator
    // E.g. 'APP_APPLICATION__PORT=5001' would set Settings.application.port
    settings.merge(config::Environment::with_prefix("app").separator("__"))?;

    // try to convert the configuration values it read into
    settings.try_into()
}

pub enum Environment {
    Local,
    Production
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
                "{} is not a supported environment. Use either local or production",
                other
            )),
        }
    }
}

impl DatabaseSettings {
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(&self.password.expose_secret())
            .ssl_mode(ssl_mode)
    }
    pub fn with_db(&self) -> PgConnectOptions {
        let mut options = self.without_db().database(&self.database_name);
        options.log_statements(tracing::log::LevelFilter::Trace);
        options
        
    }
}