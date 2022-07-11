use druid::{widget::{Flex, TextBox, Button}, Widget, WidgetExt};

use crate::state::AppState;

use super::Submit;

pub fn bar_widget() -> impl Widget<AppState> {
    Flex::row()
        .with_child(
            Submit::new(
                TextBox::new()
                    .with_placeholder("Enter URL or search query...")
                    .lens(AppState::bar_query)
                    .fix_width(500.0)
            )
            .on_enter(|data: &mut AppState| data.perform(|data| {
                data.reload()
            }))
        )
        .with_child(
            Button::new("Visit")
                .on_click(|_ctx, data: &mut AppState, _env| data.perform(|data| {
                    data.reload()
                }))
        )
        .padding(10.0)
}
