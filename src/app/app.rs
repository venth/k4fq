use crate::app::errors;
use crate::ports;
use std::env;
use std::sync::Arc;

pub trait App: Send + Sync {
    async fn run(&self) -> Result<(), errors::Errors>;
}

pub fn new(command_parser: Arc<dyn ports::CommandParser>) -> impl App {
    AppImpl { command_parser: command_parser.clone() }
}
struct AppImpl {
    command_parser: Arc<dyn ports::CommandParser>,
}

impl App for AppImpl {
    async fn run(&self) -> Result<(), errors::Errors> {
        self.command_parser.parse(&(env::args().collect()));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::app::app;
    use crate::app::app::App;
    use crate::app::errors::Errors;
    use crate::ports;
    use crate::ports::Command;
    use mockall::mock;
    use std::sync::Arc;

    #[tokio::test]
    async fn runs_app() {
        let some_error = Errors::UnknownError { msg: "ups".to_string() };
        let expected_error = some_error.clone();
        let mut command_parser = MockSomeCommandParser::new();
        command_parser.expect_parse()
            .return_once(move |_| { Command::skip_because_of("just for testing") });
        let app = app::new(Arc::new(command_parser));
        let result = app.run().await;
        assert!(result.is_err());
        assert_eq!(expected_error, result.err().unwrap());
    }


    mock! {
        SomeCommandParser {}
        impl ports::CommandParser for SomeCommandParser {
            fn parse(&self, args: &Vec<String>) -> ports::Command { todo!() }
        }
    }
}