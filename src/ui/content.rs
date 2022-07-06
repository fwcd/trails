use druid::Widget;

use crate::state::AppState;

use super::WebRenderer;

pub fn content_widget() -> impl Widget<AppState> {
    WebRenderer::new()
}
