use std::collections::HashMap;
use std::path::Path;

use config::{Config, ConfigError, Source, Value};
use serde::{Deserialize, Deserializer};
use serde::ser::Error;

use crate::domain::model::{ApplicationProperties, CollectableProperties, K4QError};
use crate::domain::ports;

#[derive(shaku::Component)]
#[shaku(interface = ports::PropertiesSource)]
pub struct ConfigurationLoader {}

impl ports::PropertiesSource for ConfigurationLoader {
    fn load(&self, config_location: &Path) -> Result<Box<dyn ApplicationProperties>, K4QError> {
        Config::default()
            .with_merged(config::File::with_name(config_location.to_str().unwrap()))
            .map(ApplicationConfig::<PartialConfig>::new)
            .map(Box::new)
            .map(|c| c as Box<dyn ApplicationProperties>)
            .map_err(ConfigurationLoader::description_of)
            .map_err(K4QError::ConfigError)
        // .and_then(|config| config.try_into())
        // .map_err(ConfigurationLoader::description_of)
        // .map_err(K4QError::ConfigError)
    }
}

impl<T> CollectableProperties for ApplicationConfig<T> where
    T: CollectableProperties + ApplicationProperties {
    fn try_collect<'de, V>(self) -> Result<V, K4QError> where V: Sized + Deserialize<'de> {
        self.config
            .try_collect()
    }
}


impl<T: ApplicationProperties + CollectableProperties> ApplicationProperties for ApplicationConfig<T> {
    fn properties_by(&self, prefix: &str) -> Result<Box<dyn ApplicationProperties>, K4QError> {
        self.config.properties_by(prefix)
    }
}


pub struct ApplicationConfig<T>
    where
        T: CollectableProperties + ApplicationProperties,
{
    config: T,
}

impl ApplicationConfig<PartialConfig> {
    pub fn new(config: Config) -> Self {
        ApplicationConfig { config: PartialConfig { config } }
    }
}


struct PartialConfig {
    config: Config,
}

impl ApplicationProperties for PartialConfig {
    fn properties_by(&self, prefix: &str) -> Result<Box<dyn ApplicationProperties>, K4QError> {
        self.config
            .get_table(prefix)
            .map(PartialConfigSource::new)
            .and_then(|c| Config::new().with_merged(c))
            .map(|c| Box::new(c) as Box<dyn ApplicationProperties>)
            .map_err(ConfigurationLoader::description_of)
            .map_err(K4QError::ConfigError)
    }
}


impl CollectableProperties for &dyn ApplicationProperties {
    fn try_collect<'de, T>(self) -> Result<T, K4QError> where Self: CollectableProperties, T: Sized + Deserialize<'de> {
        self.try_collect()
    }
}

impl CollectableProperties for PartialConfig {

    fn try_collect<'de, T>(self) -> Result<T, K4QError> where T: Sized + Deserialize<'de> {
        self.config
            .try_into()
            .map_err(ConfigurationLoader::description_of)
            .map_err(K4QError::ConfigError)
    }
}

impl ApplicationProperties for Config {
    fn properties_by(&self, prefix: &str) -> Result<Box<dyn ApplicationProperties>, K4QError> {
        self
            .get_table(prefix)
            .map(PartialConfigSource::new)
            .and_then(|c| Config::new().with_merged(c))
            .map(|c| Box::new(c) as Box<dyn ApplicationProperties>)
            .map_err(ConfigurationLoader::description_of)
            .map_err(K4QError::ConfigError)
        // .and_then(|c| c.try_into())
        // .map_err(ConfigurationLoader::description_of)
        // .map_err(K4QError::ConfigError)
    }
}

#[derive(Debug, Clone)]
struct PartialConfigSource {
    props: HashMap<String, Value>,
}

impl PartialConfigSource {
    fn new(props: HashMap<String, Value>) -> PartialConfigSource {
        PartialConfigSource { props }
    }
}

impl Source for PartialConfigSource {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new(self.clone())
    }

    fn collect(&self) -> Result<HashMap<String, Value>, ConfigError> {
        Result::Ok(self.props.clone())
    }
}

impl ConfigurationLoader {
    fn description_of(err: ConfigError) -> String {
        match err {
            ConfigError::Frozen => String::from("Internal error - configuration is frozen and cannot be changed"),
            ConfigError::NotFound(msg) => format!("Check configuration file location. {}", msg),
            ConfigError::PathParse(err_kind) => {
                format!("Check supplied configuration file location path. The error: {}",
                        String::from(err_kind.description()))
            }
            ConfigError::FileParse { uri, cause } => match uri {
                None => format!("Correct configuration file. An error occurred during parsing: {}", cause),
                Some(file) => format!("Correct configuration file: {}. An error occurred during parsing: {}", file, cause)
            },
            ConfigError::Type { key, origin, unexpected, expected } => {
                let msg_key = key
                    .map(|k| format!(" Parsed key: {}", k))
                    .unwrap_or_else(|| String::from(""));
                let msg_config_location = origin
                    .map(|loc| format!(": {}", loc))
                    .unwrap_or_else(|| String::from(""));
                format!("Correct configuration file{}.{} Expected: {}, but encountered: {}",
                        msg_config_location,
                        msg_key,
                        expected,
                        unexpected)
            }
            ConfigError::Message(m) => format!("An issue with the configuration file: {}", m),
            ConfigError::Foreign(e) => {
                format!("An issue with the configuration file: {}", String::from(e.to_string()))
            }
        }
    }
}