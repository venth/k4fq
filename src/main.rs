mod app;
pub mod ports;
mod adapterclap;
mod predicates;
mod adapterfigment;
mod adapterindicatif;

use std::sync::Arc;
use crate::app::App;

use tokio;

#[tokio::main]
async fn main() {
    let command_parser = Arc::new(adapterclap::new());
    let configuration_source = Arc::new(adapterfigment::new());
    let reporter_factory = Arc::new(adapterindicatif::new());
    let app = app::new(command_parser.clone(), configuration_source.clone(), reporter_factory.clone());
    app.run().await.unwrap();
}

#[cfg(test)] #[macro_use]
extern crate assert_matches;
