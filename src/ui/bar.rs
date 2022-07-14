use druid::{widget::{Flex, TextBox, Button}, Widget, WidgetExt, Insets};

use crate::state::AppState;

use super::{Submit, icon_button};

pub fn bar_widget() -> impl Widget<AppState> {
    let size = 28.0;
    let padding = 5.0;
    Flex::row()
        .with_child(
            icon_button("\u{27e8}", 20.0) // Back (<)
                .fix_size(size, size)
        )
        .with_child(
            icon_button("\u{27E9}", 20.0) // Forward (>)
                .fix_size(size, size)
        )
        .with_child(
            icon_button("\u{27F3}", 24.0) // Reload
                .fix_size(size, size)
        )
        .with_child(
            Submit::new(
                TextBox::new()
                    .with_placeholder("Enter URL or search query...")
                    .lens(AppState::bar_query)
                    .fix_width(500.0)
                    .fix_height(size)
                    .padding(Insets::uniform_xy(padding, 0.0))
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
                .fix_height(size)
        )
        .padding(10.0)
}
