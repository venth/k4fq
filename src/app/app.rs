use crate::app::errors;
use crate::ports;
use std::env;
use std::sync::Arc;

pub trait App: Send + Sync {
    async fn run(&self) -> Result<(), errors::Errors>;
}

pub fn new(
    command_parser: Arc<dyn ports::CommandParser>,
    configuration_source: Arc<dyn ports::ConfigurationSource>,
    reporter_factory: Arc<dyn ports::ReporterFactory>,
) -> impl App {
    AppImpl {
        command_parser,
        configuration_source,
        reporter_factory
    }
}
struct AppImpl {
    command_parser: Arc<dyn ports::CommandParser>,
    configuration_source: Arc<dyn ports::ConfigurationSource>,
    reporter_factory: Arc<dyn ports::ReporterFactory>,
}

impl App for AppImpl {
    async fn run(&self) -> Result<(), errors::Errors> {
        let reporter = self.reporter_factory.create_unchained_reporter("k4fq");
        let parsed_command = self.command_parser.parse(&(env::args().collect()));
        reporter.inc(1);
        match parsed_command {
            ports::Command::ShowConfig { config } => {
                let config = self.configuration_source.load(&config)?;
                reporter.info(&format!("{}", config));
                reporter.inc(1);
            }
            ports::Command::Query { .. } => todo!(),
            ports::Command::Skip { cause } => reporter.info(format!("Skipped. The cause: {}", cause).as_str()),
        }
        reporter.inc(1);
        reporter.finish();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::app::app;
    use crate::app::app::App;
    use crate::app::errors::Errors;
    use crate::ports;
    use crate::ports::{Command, Reporter};
    use mockall::mock;
    use std::path::PathBuf;
    use std::sync::Arc;

    #[tokio::test]
    async fn runs_app() {
        let expected_error = "some error";
        let mut command_parser = MockSomeCommandParser::new();
        command_parser
            .expect_parse()
            .return_once(move |_| Command::skip_because_of(expected_error.to_string()));
        let mut config_source = MockSomeConfigurationSource::new();
        config_source.expect_load().return_once(move |_| {
            Ok(ports::DynamicConfig::from(
                serde_json::json!( { "foo": "bar" } ),
            ))
        });

        let mut reporter_factory = MockSomeReporterFactory::new();
        let mut some_reporter = MockSomeReporter::new();
        some_reporter.expect_inc().return_const(());
        some_reporter.expect_info().return_const(());
        some_reporter.expect_finish().return_const(());

        reporter_factory.expect_create_unchained_reporter()
            .return_once(move |_| Box::new(some_reporter));

        let app = app::new(Arc::new(command_parser), Arc::new(config_source), Arc::new(reporter_factory));
        let result = app.run().await;
        assert!(result.is_ok());
    }

    mock! {
        SomeReporterFactory {}
        impl ports::ReporterFactory for SomeReporterFactory {
            fn create_unchained_reporter(&self, name: &str) -> Box<dyn Reporter> { todo!() }
            fn create_chained_reporter(&self, name: &str, len: u64) -> Box<dyn Reporter> { todo!() }
        }
    }

    mock! {
        SomeReporter {}
        impl ports::Reporter for SomeReporter {
            fn stage(&self, stage_name: &str) { todo!() }
            fn info(&self, message: &str) { todo!() }
            fn inc(&self, delta: u64) { todo!() }
            fn finish(self: Box<Self>) { todo!() }
        }
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
