mod constants;
mod dom;
mod error;
mod parse;
mod network;
mod util;

use std::sync::{Arc, Mutex};

use dom::Document;
use druid::{AppLauncher, WindowDesc, Widget, widget::{Flex, TextBox, Button, Label}, WidgetExt, Data, Lens, Env};
use network::NetworkSession;
use parse::HtmlParser;

#[derive(Clone, Data, Lens)]
struct AppState {
    url: Arc<String>,
    parser: Arc<HtmlParser>,
    session: Arc<Mutex<NetworkSession>>,
    document: Arc<Mutex<Option<Document>>>,
}

fn main() {
    let window = WindowDesc::new(build_ui)
        .title("Trails")
        .window_size((800.0, 600.0));

    let initial_state = AppState {
        url: Arc::new("".to_owned()),
        parser: Arc::new(HtmlParser::default()),
        session: Arc::new(Mutex::new(NetworkSession::default())),
        document: Arc::new(Mutex::new(None)),
    };

    AppLauncher::with_window(window)
        .launch(initial_state)
        .expect("Failed to launch app");
}

fn build_ui() -> impl Widget<AppState> {
    Flex::column()
        // Address bar
        .with_child(
            Flex::row()
                .with_child(
                    TextBox::new()
                        .with_placeholder("Enter URL...")
                        .lens(AppState::url)
                        .fix_width(500.0)
                )
                .with_child(
                    Button::new("Visit")
                        .on_click(|_ctx, data: &mut AppState, _env| {
                            let raw = data.session.lock().unwrap().get_text(data.url.as_str())
                                .expect("Could not perform request"); // TODO: Handle this error
                            let doc = data.parser.parse(raw.as_str())
                                .expect("Could not parse HTML"); // TODO: Handle this error
                            *data.document.lock().unwrap() = Some(doc);
                        })
                )
                .padding(10.0)
        )
        // Content
        .with_child(
            Label::new(|data: &AppState, _env: &Env| {
                // TODO: Actually render the doc in a meaningful way
                let rendered = format!("{:#?}", data.document.lock().unwrap());
                rendered
            })
        )
}
