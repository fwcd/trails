use crate::state::AppState;
use druid::{Widget, widget::{Flex, Scroll}};

use super::{bar_widget, content_widget};

pub fn app_widget() -> impl Widget<AppState> {
    Flex::column()
        // Address bar
        .with_child(bar_widget())
        // Content
        .with_flex_child(Scroll::new(content_widget()), 1.0)
}
