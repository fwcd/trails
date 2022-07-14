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
    // Bootstrap logging
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    // Bootstrap services and state
    let services = Arc::new(AppServices::new());
    let mut initial_state = AppState {
        bar_query: "about:blank".to_owned(),
        document: Document::new(),
    };
    initial_state.perform(|data| data.reload(&services));

    // Create window
    let window = WindowDesc::new(app_widget(&services))
        .title("Trails")
        .window_size((800.0, 600.0));

    // Launch app
    AppLauncher::with_window(window)
        .launch(initial_state)
        .expect("Failed to launch app");
}
