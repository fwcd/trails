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
    /// The default spacing to render between inline elements.
    spacing: f64,
}

/// The rendering state.
#[derive(Clone)]
struct RenderState {
    /// The current (top-left) point of the current layout container
    /// (e.g. where the next line is started).
    base_point: Point,
    /// The current (top-left) point at which to paint (e.g. text).
    point: Point,
    /// Styling info.
    styling: Styling,
}

/// State used during a DOM rendering pass.
struct RenderCtx<'a, 'b, 'c, 'd> {
    /// The paint context, if painting.
    paint: Option<&'a mut PaintCtx<'b, 'c, 'd>>,
    /// The rendering state.
    state: RenderState,
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
        let font_size = 12.0;
        RenderCtx {
            paint,
            state: RenderState {
                base_point: Point::ZERO,
                point: Point::ZERO,
                styling: Styling {
                    font_size,
                    font_weight: FontWeight::REGULAR,
                    color: Color::BLACK,
                    spacing: font_size * 0.45,
                },
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
            let start_state = ctx.state.clone();
            let mut size = Size::ZERO;

            // Change styling info as needed
            {
                let mut styling = &mut ctx.state.styling;
                match node.tag_name() {
                    "b" | "strong" => styling.font_weight = FontWeight::BOLD,
                    "h1" => styling.font_size = 32.0,
                    "h2" => styling.font_size = 26.0,
                    "h3" => styling.font_size = 22.0,
                    "h4" => styling.font_size = 20.0,
                    "a" => styling.color = Color::BLUE,
                    _ => {},
                }
                if node.is_heading() {
                    styling.font_weight = FontWeight::BOLD;
                }
            }

            // Render children
            let mut line_size = Size::ZERO;
            for child in node.children() {
                // Check whether this is an inline tag
                let is_inline = child.tag_name().map_or(true, |t| INLINE_TAGS.contains(t));
                // Render spacing if we have adjacent inline elements
                if is_inline && line_size != Size::ZERO {
                    ctx.state.point.x += ctx.state.styling.spacing;
                }
                // Render the child element, which computes its size
                let child_size = self.render_node(ctx, child);
                // Depending on whether this is an inline tag we should either break or not
                if is_inline {
                    let width_delta = child_size.width;
                    line_size.width += width_delta;
                    line_size.height = line_size.height.max(child_size.height);
                    // Move 'cursor' forward on this line
                    ctx.state.point.x += width_delta;
                } else {
                    line_size.width = line_size.width.max(child_size.width);
                    line_size.height = line_size.height.max(child_size.height);
                    size.width = size.width.max(line_size.width);
                    size.height += line_size.height;
                    // Jump to next line
                    ctx.state.point.x = ctx.state.base_point.x;
                    ctx.state.point.y += line_size.height;
                    line_size = Size::ZERO;
                }
            }
            if line_size != Size::ZERO {
                size.width = size.width.max(line_size.width);
                size.height += line_size.height;
            }

            ctx.state = start_state;
            trace!("<{}> has size {}", node.tag_name(), size);
            size
        } else {
            debug!("Not rendering <{}>", node.tag_name());
            Size::ZERO
        }
    }

    /// Renders some text from the DOM.
    fn render_text(&self, ctx: &mut RenderCtx, text: &str) -> Size {
        let state = &ctx.state;
        if let Some(paint) = &mut ctx.paint {
            // We are painting
            let layout = paint.text()
                .new_text_layout(text.to_owned())
                .font(FontFamily::SERIF, state.styling.font_size)
                .default_attribute(state.styling.font_weight)
                .text_color(state.styling.color.clone())
                .build()
                .expect("Could not construct text layout"); // TODO: Better error handling
            paint.draw_text(&layout, state.point);
            layout.size()
        } else {
            // We are just layouting
            // TODO: Use a more accurate heuristic for determining the text size
            let font_size = state.styling.font_size;
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
