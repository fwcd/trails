use crate::state::AppState;
use druid::{Widget, widget::{Flex, Scroll}};

use super::{bar_widget, content_widget, Tighten};

pub fn app_widget() -> impl Widget<AppState> {
    Flex::column()
        // Address bar
        .with_child(bar_widget())
        // Content
        .with_flex_child(
            Tighten::new(
                Scroll::new(content_widget())
                    .content_must_fill(true)
            ),
            1.0
        )
}
