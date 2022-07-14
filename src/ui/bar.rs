use std::sync::Arc;

use druid::{widget::{Flex, TextBox, Button}, Widget, WidgetExt, Insets, EventCtx, Env};

use crate::{state::AppState, services::AppServices};

use super::{Submit, icon_button};

pub fn bar_widget(services: &Arc<AppServices>) -> impl Widget<AppState> {
    let size = 28.0;
    let padding = 5.0;

    // Reloading closures for different handlers

    let services_reload = services.clone();
    let reload = move |data: &mut AppState| {
        data.perform(|data| {
            data.reload(&services_reload)
        })
    };

    let services_reload_by_button = services.clone();
    let reload_by_button = move |_ctx: &mut EventCtx, data: &mut AppState, _env: &Env| {
        data.perform(|data| {
            data.reload(&services_reload_by_button)
        })
    };

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
                .on_click(reload_by_button.clone())
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
            .on_enter(reload)
        )
        .with_child(
            Button::new("Visit")
                .on_click(reload_by_button)
                .fix_height(size)
        )
        .padding(10.0)
}
