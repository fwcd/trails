mod constants;
mod model;
mod error;
mod parse;
mod network;
mod state;
mod ui;

use std::sync::{Arc, Mutex};

use druid::{WindowDesc, AppLauncher};
use log::LevelFilter;
use network::Session;
use parse::html;
use simple_logger::SimpleLogger;
use state::AppState;
use ui::app_widget;

fn main() {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    let window = WindowDesc::new(app_widget)
        .title("Trails")
        .window_size((800.0, 600.0));

    let initial_state = AppState {
        url: Arc::new("https://en.wikipedia.org".to_owned()),
        parser: Arc::new(html::Parser::default()),
        session: Arc::new(Mutex::new(Session::default())),
        document: None,
    };

    AppLauncher::with_window(window)
        .launch(initial_state)
        .expect("Failed to launch app");
}
