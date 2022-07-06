use std::sync::Arc;

use crate::state::AppState;
use druid::{Widget, widget::{Flex, TextBox, Button, Label}, WidgetExt, Env};
use log::error;

pub fn build() -> impl Widget<AppState> {
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
                            let doc = data.session.lock().unwrap().get_text(data.url.as_str())
                                .and_then(|raw| data.parser.parse(raw.as_str()));
                            match doc {
                                Ok(doc) => data.document = Some(Arc::new(doc)),
                                Err(e) => error!("Error while fetching/parsing HTML: {:?}", e),
                            };
                        })
                )
                .padding(10.0)
        )
        // Content
        .with_child(
            Label::new(|data: &AppState, _env: &Env| {
                // TODO: Actually render the doc in a meaningful way
                let rendered = format!("{:#?}", data.document);
                rendered
            })
        )
}
