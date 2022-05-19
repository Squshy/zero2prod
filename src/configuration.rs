use serde::Deserialize;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Init config reader
    let mut settings = config::Config::default();

    // Look for a file named 'configuration' in any top level file
    // that `config` knows how to parse
    settings.merge(config::File::with_name("configuration"))?;

    // Try to convert into our config type
    settings.try_into()
}
