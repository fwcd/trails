use std::sync::Arc;

use druid::{widget::{Flex, TextBox, Button}, Widget, WidgetExt, Insets};

use crate::{state::AppState, services::AppServices};

use super::{Submit, icon_button};

pub fn bar_widget(services: &Arc<AppServices>) -> impl Widget<AppState> {
    let size = 28.0;
    let padding = 5.0;

    // TODO: Replace this boilerplate with a macro or hope that clone-into-closure
    //       finally gets merged (https://github.com/rust-lang/rfcs/issues/2407)
    let services1 = services.clone();
    let services2 = services.clone();
    let services3 = services.clone();
    let services4 = services.clone();
    let services5 = services.clone();

    Flex::row()
        .with_child(
            icon_button("\u{27e8}", 20.0) // Back (<)
                .fix_size(size, size)
                .disabled_if(|data: &AppState, _| data.history.is_empty())
                .on_click(move |_, data: &mut AppState, _| data.perform(|data| {
                    data.go_back(&services1)
                }))
        )
        .with_child(
            icon_button("\u{27E9}", 20.0) // Forward (>)
                .fix_size(size, size)
                .disabled_if(|data: &AppState, _| data.forward_history.is_empty())
                .on_click(move |_, data: &mut AppState, _| data.perform(|data| {
                    data.go_forward(&services2)
                }))
        )
        .with_child(
            icon_button("\u{27F3}", 24.0) // Reload
                .fix_size(size, size)
                .on_click(move |_, data: &mut AppState, _| data.perform(|data| {
                    data.reload(&services3)
                }))
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
            .on_enter(move |data: &mut AppState| data.perform(|data| {
                data.visit(&services4)
            }))
        )
        .with_child(
            Button::new("Visit")
                .on_click(move |_, data: &mut AppState, _| data.perform(|data| {
                    data.visit(&services5)
                }))
                .fix_height(size)
        )
        .padding(10.0)
}
