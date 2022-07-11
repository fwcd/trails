use std::collections::HashSet;

use druid::{Widget, Size, Env, BoxConstraints, LifeCycle, Event, PaintCtx, LayoutCtx, UpdateCtx, LifeCycleCtx, EventCtx, RenderContext, Rect, Color, Point, piet::{Text, TextLayoutBuilder, TextLayout}, FontFamily, FontWeight};
use log::{debug, info, trace};
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
    fn render_document(&self, ctx: &mut RenderCtx, document: &Document) -> Size {
        // Draw background
        if let Some(paint) = &mut ctx.paint {
            let size = paint.size();
            paint.fill(Rect::from_origin_size(Point::ZERO, size), &Color::WHITE);
        }

        // Render the tree
        self.render_element(ctx, document.root())
    }

    /// Renders a single DOM node.
    fn render_node(&self, ctx: &mut RenderCtx, node: &Node) -> Size {
        match node {
            Node::Element(element) => self.render_element(ctx, element),
            Node::Text(text) => self.render_text(ctx, text),
        }
    }

    /// Renders a single DOM element.
    fn render_element(&self, ctx: &mut RenderCtx, node: &Element) -> Size {
        if RENDERED_TAGS.contains(node.tag_name()) {
            // TODO: It would probably be better to just pass a cloned context
            //       to the child, the borrow-checker however complains about
            //       the mutable reference to the paint context in that case.
            let old_point = ctx.point;
            let old_styling = ctx.styling.clone();
            let mut size = Size::ZERO;

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
                let child_size = self.render_node(ctx, child);
                size.width = size.width.max(child_size.width);
                size.height += child_size.height;
                ctx.point.y += child_size.height;
            }

            ctx.point = old_point;
            ctx.styling = old_styling;
            trace!("<{}> has size {}", node.tag_name(), size);
            size
        } else {
            debug!("Not rendering <{}>", node.tag_name());
            Size::ZERO
        }
    }

    /// Renders some text from the DOM.
    fn render_text(&self, ctx: &mut RenderCtx, text: &str) -> Size {
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
            layout.size()
        } else {
            // We are just layouting
            // TODO: Use a more accurate heuristic for determining the text size
            let font_size = ctx.styling.font_size;
            Size::new(text.len() as f64 * font_size * 0.45, font_size)
        }
    }
}

impl Widget<AppState> for WebRenderer {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut AppState, _env: &Env) {
        
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &AppState, _env: &Env) {
        
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {
        
    }

    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &AppState, _env: &Env) -> Size {
        let min_size = bc.min();
        if let Some(document) = &data.document {
            // Perform a render pass without a paint context to determine the document's size
            let mut render_ctx = self.make_render_ctx(None);
            let doc_size = self.render_document(&mut render_ctx, &*document);
            info!("Document size: {}", doc_size);
            Size::new(
                min_size.width.max(doc_size.width),
                min_size.height.max(doc_size.height),
            )
        } else {
            min_size
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, _env: &Env) {
        if let Some(document) = &data.document {
            // Perform a render pass over the document
            let mut render_ctx = self.make_render_ctx(Some(ctx));
            self.render_document(&mut render_ctx, &*document);
        }
    }
}
