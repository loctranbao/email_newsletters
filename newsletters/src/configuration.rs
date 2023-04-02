use secrecy::{Secret, ExposeSecret};


#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String
}


pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // initialize our configuration reader
    let mut settings = config::Config::default();

    // add configuration values fronm a file named 'configuration'.
    // it will look for any top-level file with an extension
    // that 'config' knows how to parse: yaml, json, etc.
    settings.merge(config::File::with_name("configuration"))?;

    // try to convert the configuration values it read into
    settings.try_into()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String>{
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}", 
            self.username, self.password.expose_secret(), self.host, self.port, self.database_name
        ))
    }

    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}", 
            self.username, self.password.expose_secret(), self.host, self.port
        ))        
    }
}