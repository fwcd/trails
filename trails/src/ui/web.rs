use std::sync::Arc;

use druid::{Widget, Size, Env, BoxConstraints, LifeCycle, Event, PaintCtx, LayoutCtx, UpdateCtx, LifeCycleCtx, EventCtx, piet::NullRenderContext};
use trails_base::log::{debug, info};
use trails_model::dom::Document;
use trails_render::web::{LinkAreas, RenderParams, Renderer};

pub struct WebRenderer {
    /// The clickable link areas from the last render.
    link_areas: Option<LinkAreas>,
    /// Tracks a visit request after an event. The parent may or may not choose to honor this.
    active_link: Option<String>,
}

impl WebRenderer {
    pub fn new() -> Self {
        Self {
            link_areas: None,
            active_link: None,
        }
    }

    /// The clicked link after an event.
    pub fn active_link(&self) -> Option<&str> {
        self.active_link.as_ref().map(|s| s.as_str())
    }
}

impl Widget<Arc<Document>> for WebRenderer {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _document: &mut Arc<Document>, _env: &Env) {
        self.active_link = None;

        match event {
            Event::MouseUp(e) => {
                let point = e.pos;

                // Find the clicked link area
                if let Some(area) = self.link_areas.as_ref().and_then(|l| l.areas.iter().find(|a| a.area.contains(point)).cloned()) {
                    info!("Clicked {:?}", area);
                    self.active_link = Some(area.href);
                    ctx.set_handled();
                }
            },
            _ => {},
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _document: &Arc<Document>, _env: &Env) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_document: &Arc<Document>, document: &Arc<Document>, _env: &Env) {
        if old_document != document {
            ctx.request_layout();
            ctx.request_paint();
        }
    }

    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, document: &Arc<Document>, _env: &Env) -> Size {
        let min_size = bc.min();

        // Perform a render pass without a paint context to determine the document's size
        let params = RenderParams::<NullRenderContext> {
            paint: None,
            base_size: min_size
        };
        let result = Renderer::new(params).render_document(document);

        debug!("Document size: {}", result.size);
        Size::new(
            min_size.width.max(result.size.width),
            min_size.height.max(result.size.height),
        )
    }

    fn paint(&mut self, ctx: &mut PaintCtx, document: &Arc<Document>, _env: &Env) {
        let size = ctx.size();

        // Perform a render pass over the document
        let params = RenderParams {
            paint: Some(&mut **ctx),
            base_size: size,
        };
        let result = Renderer::new(params).render_document(document);

        // Update found link areas
        self.link_areas = Some(result.link_areas);

        // Update window title if needed
        if let Some(title) = result.title {
            info!("Setting title to '{}'", &title);
            ctx.window().set_title(&title);
        }
    }
}
