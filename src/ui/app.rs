use std::sync::Arc;

use crate::{state::AppState, services::AppServices};
use druid::{Widget, widget::{Flex, Scroll}};

use super::{bar_widget, content_widget, Tighten};

pub fn app_widget(services: &Arc<AppServices>) -> impl Widget<AppState> {
    Flex::column()
        // Address bar
        .with_child(bar_widget(services.clone()))
        // Content
        .with_flex_child(
            Tighten::new(
                Scroll::new(content_widget(services))
                    .content_must_fill(true)
            ),
            1.0
        )
}
