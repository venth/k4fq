use crate::app::Errors;
use crate::ports;
use crate::ports::DynamicConfig;
use figment::providers::Format;
use figment::value::Dict;
use figment::{Error, Metadata, Profile, Provider};
use std::collections::BTreeMap;
use std::path::PathBuf;

struct FigmentAdapter {}

impl ports::ConfigurationSource for FigmentAdapter {
    fn load(&self, config_path: &PathBuf) -> Result<DynamicConfig, Errors> {
        let provider = Self::get_provider(config_path);
        provider
            .and_then(|provider| Ok(BoxedProvider::new(provider)))
            .and_then(move |config_data| {
                figment::Figment::new()
                    .merge(config_data)
                    .merge(figment::providers::Env::prefixed("K4FQ_"))
                    .extract()
                    .map_err(move |e| Errors::InvalidConfiguration {
                        msg: format!("Failed to load configuration: {}", e),
                    })
            })
    }
}

impl FigmentAdapter {
    fn get_provider(config_path: &PathBuf) -> Result<Box<dyn figment::Provider>, Errors> {
        let extension = config_path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or("".to_string());

        match extension.as_str() {
            "toml" => Ok(Box::new(figment::providers::Toml::file(&config_path))),
            "yaml" => Ok(Box::new(figment::providers::Yaml::file(&config_path))),
            "json" => Ok(Box::new(figment::providers::Json::file(&config_path))),
            _ => Err(Errors::InvalidConfiguration { msg: format!("Unsupported configuration file format: {}. Supported formats: yaml, toml, json", extension) })
        }
    }
}

pub struct BoxedProvider {
    inner: Box<dyn Provider>,
}

impl BoxedProvider {
    pub fn new(provider: Box<dyn Provider>) -> Self {
        Self { inner: provider }
    }
}

impl Provider for BoxedProvider {
    fn metadata(&self) -> Metadata {
        self.inner.metadata()
    }

    fn data(&self) -> Result<BTreeMap<Profile, Dict>, Error> {
        self.inner.data()
    }
}

pub(crate) fn new() -> impl ports::ConfigurationSource {
    FigmentAdapter {}
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::ports::ConfigurationSource;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn parses_supported_formats() {
        let supported_formats = vec![
            ("yaml", "server_port: 9000"),
            ("toml", "server_port=9000"),
            ("json", "{\"server_port\": 9000}"),
        ];
        supported_formats.iter().for_each(|(ext, content)| {
            let mut file = NamedTempFile::new().unwrap();
            write!(file, "{}", content).unwrap();

            let path = file.path().with_extension(ext);
            file.persist(&path).unwrap();

            let configuration_source = super::new();
            let result = configuration_source.load(&path);

            assert!(
                result.is_ok(),
                "expected {} file to be parsed successfully, got: {:?}",
                ext,
                result
            );

            let _ = std::fs::remove_file(path);
        });
    }

    #[test]
    fn throws_invalid_configuration_error_on_unsupported_format() {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", "content").unwrap();

        let unsupported_extension = "unsupported";
        let path = file.path().with_extension(unsupported_extension);
        file.persist(&path).unwrap();

        let configuration_source = super::new();
        let result = configuration_source.load(&path);

        assert_matches!(result, Err(super::Errors::InvalidConfiguration { .. }));

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn complies_to_taking_env_variable_precedence_over_config_file() {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", "server_port = \"file\"").unwrap();

        let path = file.path().with_extension("toml");
        file.persist(&path).unwrap();

        std::env::set_var("K4FQ_SERVER_PORT", "env");

        let configuration_source = super::new();
        let raw_result = configuration_source.load(&path);

        let result: HashMap<String, serde_json::Value> = raw_result.unwrap().try_into_struct().unwrap();

        assert_eq!(result["server_port"], serde_json::json!("env"));

        let _ = std::fs::remove_file(path);
    }
}
