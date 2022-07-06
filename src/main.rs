mod constants;
mod dom;
mod error;
mod parse;
mod network;
mod ui;
mod state;
mod util;

use std::sync::{Arc, Mutex};

use druid::{WindowDesc, AppLauncher};
use log::LevelFilter;
use network::NetworkSession;
use parse::HtmlParser;
use simple_logger::SimpleLogger;
use state::AppState;

fn main() {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    let window = WindowDesc::new(ui::build)
        .title("Trails")
        .window_size((800.0, 600.0));

    let initial_state = AppState {
        url: Arc::new("https://en.wikipedia.org".to_owned()),
        parser: Arc::new(HtmlParser::default()),
        session: Arc::new(Mutex::new(NetworkSession::default())),
        document: None,
    };

    AppLauncher::with_window(window)
        .launch(initial_state)
        .expect("Failed to launch app");
}
