mod app;
pub mod ports;
mod adapterclap;
mod predicates;
mod adapterfigment;

use std::sync::Arc;
use crate::app::App;

use tokio;

#[tokio::main]
async fn main() {
    let command_parser = Arc::new(adapterclap::new());
    let configuration_source = Arc::new(adapterfigment::new());
    let app = app::new(command_parser.clone(), configuration_source.clone());
    app.run().await.unwrap();
}

#[cfg(test)] #[macro_use]
extern crate assert_matches;
