use std::sync::Arc;

use shaku::{HasComponent, module};

pub(crate) mod ports;
pub(crate) mod action;
pub(crate) mod service;
pub(crate) mod query;
pub(crate) mod model;
mod app;
mod prepared_command;


pub trait CliModule: HasComponent<dyn ports::CommandRecognizer> + HasComponent<dyn ports::PropertiesLocationProvider> {}

pub trait KafkaModule: HasComponent<dyn ports::KafkaSessionFactory> {}

pub trait ConsoleModule: HasComponent<dyn ports::ProgressNotifier> {}

pub trait PropertiesModule: HasComponent<dyn ports::PropertiesLoader> {}

pub trait Module: HasComponent<dyn service::App> {}

module! {
    DomainModule: Module {
        components = [app::AppImpl],
        providers = [],

        use dyn CliModule {
            components = [dyn ports::CommandRecognizer, dyn ports::PropertiesLocationProvider],
            providers = [],
        },

        use dyn KafkaModule {
            components = [dyn ports::KafkaSessionFactory],
            providers = [],
        },

        use dyn ConsoleModule {
            components = [dyn ports::ProgressNotifier],
            providers = []
        },

        use dyn PropertiesModule {
            components = [dyn ports::PropertiesLoader],
            providers = []
        }
    }
}

pub fn module(cli_module: Arc<dyn CliModule>,
              kafka_module: Arc<dyn KafkaModule>,
              console_module: Arc<dyn ConsoleModule>,
              properties_module: Arc<dyn PropertiesModule>) -> Arc<dyn Module> {
    Arc::new(DomainModule::builder(cli_module, kafka_module, console_module, properties_module)
        .build())
}
