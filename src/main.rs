mod constants;
mod error;
mod network;

use std::sync::Arc;

use druid::{AppLauncher, WindowDesc, Widget, widget::{Flex, TextBox}, WidgetExt, Data, Lens};

#[derive(Clone, Data, Lens)]
struct AppState {
    url: Arc<String>,
}

fn main() {
    let window = WindowDesc::new(build_ui)
        .title("Trails")
        .window_size((800.0, 600.0));

    let initial_state = AppState {
        url: Arc::new("".to_owned()),
    };

    AppLauncher::with_window(window)
        .launch(initial_state)
        .expect("Failed to launch app");
}

fn build_ui() -> impl Widget<AppState> {
    Flex::column()
        // Address bar
        .with_child(
            TextBox::new()
                .with_placeholder("Enter URL...")
                .lens(AppState::url)
                .expand_width()
                .padding(10.0)
        )
}
