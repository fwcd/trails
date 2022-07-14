use std::sync::Arc;

use druid::{Widget, WidgetExt};

use crate::{state::AppState, services::AppServices};

use super::WebRenderer;

pub fn content_widget(services: &Arc<AppServices>) -> impl Widget<AppState> {
    WebRenderer::new()
        .lens(AppState::document)
}
