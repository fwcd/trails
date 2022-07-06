use druid::{Widget, Size, Env, BoxConstraints, LifeCycle, Event, PaintCtx, LayoutCtx, UpdateCtx, LifeCycleCtx, EventCtx, RenderContext, Rect, Color, Point, piet::{Text, TextLayoutBuilder, TextLayout}, FontFamily};
use log::info;

use crate::{state::AppState, model::dom::{Node, Element, Document}};

pub struct WebRenderer;

/// State used during a DOM rendering pass.
struct RenderCtx<'a, 'b, 'c, 'd> {
    /// The paint context.
    paint: &'a mut PaintCtx<'b, 'c, 'd>,
    /// An environment holding theme data etc.
    env: &'a Env,
    /// The current (top-left) point at which to paint.
    point: Point,
}

impl WebRenderer {
    pub fn new() -> Self {
        Self
    }

    /// Renders a DOM document.
    fn render_document(&self, ctx: &mut RenderCtx, document: &Document) {
        // Draw background
        let size = ctx.paint.size();
        ctx.paint.fill(Rect::from_origin_size(Point::ZERO, size), &Color::WHITE);

        // Render the tree
        self.render_element(ctx, document.root());
    }

    /// Renders a single DOM node.
    fn render_node(&self, ctx: &mut RenderCtx, node: &Node) {
        match node {
            Node::Element(element) => self.render_element(ctx, element),
            Node::Text(text) => self.render_text(ctx, text),
        }
    }

    /// Renders a single DOM element.
    fn render_element(&self, ctx: &mut RenderCtx, node: &Element) {
        for child in node.children() {
            self.render_node(ctx, child)
        }
    }

    /// Renders some text from the DOM.
    fn render_text(&self, ctx: &mut RenderCtx, text: &str) {
        let layout = ctx.paint.text()
            .new_text_layout(text.to_owned())
            .font(FontFamily::SERIF, 12.0)
            .build()
            .expect("Could not construct text layout"); // TODO: Better error handling
        ctx.paint.draw_text(&layout, ctx.point);
        ctx.point.y += layout.size().height;
    }
}

impl Widget<AppState> for WebRenderer {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {
        
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &AppState, env: &Env) -> Size {
        // TODO: Proper minimum height
        Size::new(bc.max().width, 500.0)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        if let Some(document) = &data.document {
            let mut render_ctx = RenderCtx {
                paint: ctx,
                env,
                point: Point::ZERO,
            };
            self.render_document(&mut render_ctx, &*document);
        }
    }
}
