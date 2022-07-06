use druid::{Widget, widget::Label, Env};

use crate::state::AppState;

pub fn content_widget() -> impl Widget<AppState> {
    Label::new(|data: &AppState, _env: &Env| {
        // TODO: Actually render the doc in a meaningful way
        let rendered = format!("{:#?}", data.document);
        rendered
    })
}
