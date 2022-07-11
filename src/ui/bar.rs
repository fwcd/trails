use std::sync::Arc;

use druid::{widget::{Flex, TextBox, Button}, Widget, WidgetExt};
use log::error;

use crate::state::AppState;

use super::Submit;

pub fn bar_widget() -> impl Widget<AppState> {
    Flex::row()
        .with_child(
            Submit::new(
                TextBox::new()
                    .with_placeholder("Enter URL...")
                    .lens(AppState::url)
                    .fix_width(500.0)
            )
            .on_enter(|data: &mut AppState| {
                println!("Pressed enter");
            })
        )
        .with_child(
            Button::new("Visit")
                .on_click(|_ctx, data: &mut AppState, _env| {
                    let doc = data.session.lock().unwrap().get_text(data.url.as_str())
                        .and_then(|raw| data.parser.parse(raw.as_str()));
                    match doc {
                        Ok(doc) => data.document = Some(Arc::new(doc)),
                        Err(e) => error!("Error while fetching/parsing HTML: {:?}", e),
                    };
                })
        )
        .padding(10.0)
}
