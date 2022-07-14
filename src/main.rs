mod constants;
mod model;
mod error;
mod parse;
mod network;
mod services;
mod state;
mod ui;

use std::sync::Arc;

use druid::{WindowDesc, AppLauncher};
use log::LevelFilter;
use model::dom::Document;
use services::AppServices;
use simple_logger::SimpleLogger;
use state::AppState;
use ui::app_widget;

fn main() {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    let services = Arc::new(AppServices::new());
    let window = WindowDesc::new(app_widget(&services))
        .title("Trails")
        .window_size((800.0, 600.0));

    let initial_state = AppState {
        bar_query: "https://en.wikipedia.org".to_owned(),
        document: Document::new(),
    };

    AppLauncher::with_window(window)
        .launch(initial_state)
        .expect("Failed to launch app");
}
