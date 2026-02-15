use crate::app::errors;
use crate::ports;
use std::env;
use std::sync::Arc;

pub trait App: Send + Sync {
    async fn run(&self) -> Result<(), errors::Errors>;
}

pub fn new(command_parser: Arc<dyn ports::CommandParser>, configuration_source: Arc<dyn ports::ConfigurationSource>) -> impl App {
    AppImpl { command_parser: command_parser.clone(), configuration_source: configuration_source.clone() }
}
struct AppImpl {
    command_parser: Arc<dyn ports::CommandParser>,
    configuration_source: Arc<dyn ports::ConfigurationSource>,
}

impl App for AppImpl {
    async fn run(&self) -> Result<(), errors::Errors> {
        let parsed_command = self.command_parser.parse(&(env::args().collect()));
        match parsed_command {
            ports::Command::ShowConfig { config } => {
                let config = self.configuration_source.load(&config)?;
                todo!()
            },
            ports::Command::Query { .. } => todo!(),
            ports::Command::Skip { .. } => todo!(),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::app::app;
    use crate::app::app::App;
    use crate::app::errors::Errors;
    use crate::ports;
    use crate::ports::{Command};
    use mockall::mock;
    use std::path::PathBuf;
    use std::sync::Arc;

    #[tokio::test]
    async fn runs_app() {
        let some_error = Errors::UnknownError { msg: "ups".to_string() };
        let expected_error = some_error.clone();
        let mut command_parser = MockSomeCommandParser::new();
        command_parser.expect_parse()
            .return_once(move |_| { Command::skip_because_of("just for testing") });
        let mut config_source = MockSomeConfigurationSource::new();
        config_source.expect_load()
            .return_once(move |_| { Ok(ports::DynamicConfig::from(serde_json::json!( { "foo": "bar" } ))) });
        let app = app::new(Arc::new(command_parser), Arc::new(config_source));
        let result = app.run().await;
        assert!(result.is_err());
        assert_eq!(expected_error, result.err().unwrap());
    }

    mock! {
        SomeConfigurationSource {}
        impl ports::ConfigurationSource for SomeConfigurationSource {
            fn load(&self, config_path: &PathBuf) -> Result<ports::DynamicConfig, Errors> { todo!() }
        }
    }

    mock! {
        SomeCommandParser {}
        impl ports::CommandParser for SomeCommandParser {
            fn parse(&self, args: &Vec<String>) -> ports::Command { todo!() }
        }
    }
}