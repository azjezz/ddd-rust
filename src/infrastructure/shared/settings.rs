use actix_settings::BasicSettings;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct ApplicationSettings {
    pub secret: String,
    pub templates: String,
    pub database_url: String,
}

pub type Settings = BasicSettings<ApplicationSettings>;

pub fn load<P>(filepath: P) -> Settings
where
    P: AsRef<Path>,
{
    Settings::parse_toml(filepath).expect("Failed to parse `Settings` from actix.toml")
}
