use std::sync::Arc;

use druid::{EventCtx, Event, LifeCycleCtx, LayoutCtx, PaintCtx, Widget, Size, UpdateCtx, Env, LifeCycle, BoxConstraints};

use crate::{state::AppState, services::AppServices};

use super::WebRenderer;

/// The content widget that wires up a WebRenderer with the services and the state.
pub struct Content {
    renderer: WebRenderer,
    services: Arc<AppServices>,
}

impl Content {
    pub fn new(services: Arc<AppServices>) -> Self {
        Self {
            renderer: WebRenderer::new(),
            services,
        }
    }
}

impl Widget<AppState> for Content {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        let services = &self.services;

        self.renderer.event(ctx, event, &mut data.document, env);

        // Visit link if clicked
        if let Some(href) = self.renderer.active_link() {
            data.perform(|data| {
                let base_url = data.url(services)?;
                let url = base_url.join(href)?;
                data.visit_url(url, services)
            })
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        self.renderer.lifecycle(ctx, event, &data.document, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {
        self.renderer.update(ctx, &old_data.document, &data.document, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &AppState, env: &Env) -> Size {
        self.renderer.layout(ctx, bc, &data.document, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        self.renderer.paint(ctx, &data.document, env);
    }
}

pub fn content_widget(services: &Arc<AppServices>) -> impl Widget<AppState> {
    Content::new(services.clone())
}
