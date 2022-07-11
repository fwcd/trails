use std::collections::HashSet;

use druid::{Widget, Size, Env, BoxConstraints, LifeCycle, Event, PaintCtx, LayoutCtx, UpdateCtx, LifeCycleCtx, EventCtx, RenderContext, Rect, Color, Point, piet::{Text, TextLayoutBuilder, TextLayout}, FontFamily, FontWeight};
use log::debug;
use once_cell::sync::Lazy;

use crate::{state::AppState, model::dom::{Node, Element, Document}};

pub struct WebRenderer;

/// Styling info used during a DOM rendering pass.
#[derive(Clone)] // TODO: Derive `Copy` once https://github.com/linebender/piet/pull/524 is merged
struct Styling {
    /// The current font size.
    font_size: f64,
    /// The current font weight.
    font_weight: FontWeight,
    /// The color to render text with.
    color: Color,
}

/// State used during a DOM rendering pass.
struct RenderCtx<'a, 'b, 'c, 'd> {
    /// The paint context, if painting.
    paint: Option<&'a mut PaintCtx<'b, 'c, 'd>>,
    /// The current (top-left) point at which to paint.
    point: Point,
    /// Styling info.
    styling: Styling,
}

static RENDERED_TAGS: Lazy<HashSet<&str>> = Lazy::new(|| {
    let mut set = HashSet::new();
    set.insert("$root");
    set.insert("html");
    set.insert("body");
    set.insert("div");
    set.insert("a");
    set.insert("ul");
    set.insert("li");
    set.insert("p");
    set.insert("span");
    set.insert("b");
    set.insert("i");
    set.insert("u");
    set.insert("strong");
    set.insert("em");
    set.insert("h1");
    set.insert("h2");
    set.insert("h3");
    set.insert("h4");
    set.insert("h5");
    set.insert("h6");
    set.insert("table");
    set.insert("th");
    set.insert("tr");
    set.insert("td");
    set.insert("nav");
    set.insert("section");
    set.insert("article");
    set.insert("footer");
    set.insert("aside");
    set.insert("main");
    set.insert("label");
    set.insert("noscript");
    set.insert("abbr");
    set.insert("nobr");
    set.insert("wbr");
    set.insert("center");
    set
});

static INLINE_TAGS: Lazy<HashSet<&str>> = Lazy::new(|| {
    let mut set = HashSet::new();
    set.insert("div");
    set.insert("a");
    set.insert("span");
    set.insert("b");
    set.insert("i");
    set.insert("u");
    set.insert("strong");
    set.insert("em");
    set
});

impl WebRenderer {
    pub fn new() -> Self {
        Self
    }

    /// Creates a new render context with the default settings.
    fn make_render_ctx<'a, 'b, 'c, 'd>(&'a self, paint: Option<&'a mut PaintCtx<'b, 'c, 'd>>) -> RenderCtx<'a, 'b, 'c, 'd> {
        RenderCtx {
            paint,
            point: Point::ZERO,
            styling: Styling {
                font_size: 12.0,
                font_weight: FontWeight::REGULAR,
                color: Color::BLACK,
            },
        }
    }

    /// Renders a DOM document.
    fn render_document(&self, ctx: &mut RenderCtx, document: &Document) {
        // Draw background
        if let Some(paint) = &mut ctx.paint {
            let size = paint.size();
            paint.fill(Rect::from_origin_size(Point::ZERO, size), &Color::WHITE);
        }

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
        if RENDERED_TAGS.contains(node.tag_name()) {
            let old_styling = ctx.styling.clone();

            // Change styling info as needed
            match node.tag_name() {
                "b" | "strong" => ctx.styling.font_weight = FontWeight::BOLD,
                "h1" => ctx.styling.font_size = 32.0,
                "h2" => ctx.styling.font_size = 26.0,
                "h3" => ctx.styling.font_size = 22.0,
                "h4" => ctx.styling.font_size = 20.0,
                "a" => ctx.styling.color = Color::BLUE,
                _ => {},
            }
            if node.is_heading() {
                ctx.styling.font_weight = FontWeight::BOLD;
            }

            // Render children
            for child in node.children() {
                self.render_node(ctx, child)
            }

            ctx.styling = old_styling;
        } else {
            debug!("Not rendering <{}>", node.tag_name());
        }
    }

    /// Renders some text from the DOM.
    fn render_text(&self, ctx: &mut RenderCtx, text: &str) {
        if let Some(paint) = &mut ctx.paint {
            // We are painting
            let layout = paint.text()
                .new_text_layout(text.to_owned())
                .font(FontFamily::SERIF, ctx.styling.font_size)
                .default_attribute(ctx.styling.font_weight)
                .text_color(ctx.styling.color.clone())
                .build()
                .expect("Could not construct text layout"); // TODO: Better error handling
            paint.draw_text(&layout, ctx.point);
            ctx.point.y += layout.size().height;
        } else {
            // We are just layouting
            ctx.point.y += ctx.styling.font_size;
        }
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
        let min_size = bc.min();
        if let Some(document) = &data.document {
            // Perform a render pass without a paint context to determine the document's size
            let mut render_ctx = self.make_render_ctx(None);
            self.render_document(&mut render_ctx, &*document);
            // Compute size 
            // TODO: Using the final point is not a super-accurate heuristic for determining the rendered,
            //       we should keep track of a 'maximum' point during the render pass (which e.g. also should
            //       consider text width etc.)
            Size::new(
                min_size.width.max(render_ctx.point.x),
                min_size.height.max(render_ctx.point.y),
            )
        } else {
            min_size
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        if let Some(document) = &data.document {
            // Perform a render pass over the document
            let mut render_ctx = self.make_render_ctx(Some(ctx));
            self.render_document(&mut render_ctx, &*document);
        }
    }
}
