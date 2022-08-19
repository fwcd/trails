use std::sync::Arc;

use piet::{FontWeight, Color, kurbo::{Point, Rect, Size}, FontFamily, RenderContext, Text, TextLayoutBuilder, TextLayout};
use piet_common::Piet;
use trails_base::log::{trace, debug};
use trails_model::dom::{Node, Element, Document};

use super::{INLINE_TAGS, RENDERED_TAGS};

/// Styling info used during a DOM rendering pass.
#[derive(Clone)] // TODO: Derive `Copy` once https://github.com/linebender/piet/pull/524 is merged
pub struct Styling {
    /// The current font size.
    font_size: f64,
    /// The current font weight.
    font_weight: FontWeight,
    /// The color to render text with.
    color: Color,
    /// The default spacing to render between inline elements.
    spacing: f64,
}

/// Parameters to pass to the (top-level) renderer.
pub struct RenderParams<'a, 'b> {
    /// The paint context, if painting.
    pub paint: Option<&'a mut Piet<'b>>,
    /// The size of the viewport.
    pub base_size: Size,
}

/// A clickable area on the page.
#[derive(Debug, Clone, PartialEq)]
pub struct LinkArea {
    /// The clickable area.
    pub area: Rect,
    /// The link target.
    pub href: String,
}

/// Link areas on the page.
// TODO: Move to a separate file?
// TODO: Explore using a more efficient data structure, e.g. a quadtree
#[derive(Clone)]
pub struct LinkAreas {
    /// A list of (clickable) link areas.
    pub areas: Vec<LinkArea>,
}

/// Results from the rendering pass.
pub struct RenderResult {
    /// The rendered size of the document.
    pub size: Size,
    /// The clickable link areas.
    pub link_areas: LinkAreas,
    /// The title of the document.
    pub title: Option<String>,
}

/// Internal paint state during a rendering pass that may change for a child
/// (and thus is cloneable).
#[derive(Clone)]
pub struct RenderCursor {
    /// The size of the current layout container.
    base_size: Size,
    /// The current (top-left) point of the current layout container
    /// (e.g. where the next line is started).
    base_point: Point,
    /// The current (top-left) point at which to paint (e.g. text).
    point: Point,
    /// Styling info.
    styling: Styling,
}

/// Internal state used during a DOM rendering pass.
pub struct RenderCtx<'a, 'b> {
    /// The paint context if painting (None if just layouting).
    paint: Option<&'a mut Piet<'b>>,
    /// Whether we are currently in a rendered part of the tree.
    in_rendered_tree: bool,
    /// The clickable link areas.
    link_areas: LinkAreas,
    /// The title of the document, if found.
    title: Option<String>,
    /// The paint state.
    cursor: RenderCursor,
}

/// The main web rendering engine.
pub struct Renderer;

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer {
    /// Creates a new web rendering widget.
    pub fn new() -> Self {
        Self
    }

    /// Creates a new render context with the default settings.
    pub fn make_render_ctx<'a, 'b>(&'a self, params: RenderParams<'a, 'b>) -> RenderCtx<'a, 'b> {
        let font_size = 12.0;
        RenderCtx {
            paint: params.paint,
            in_rendered_tree: true,
            link_areas: LinkAreas {
                areas: Vec::new()
            },
            title: None,
            cursor: RenderCursor {
                base_point: Point::ZERO,
                base_size: params.base_size,
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
    pub fn render_document(&self, params: RenderParams, document: &Arc<Document>) -> RenderResult {
        // Create a rendering context.
        let size = params.base_size;
        let mut ctx = self.make_render_ctx(params);

        // Draw background
        if let Some(paint) = &mut ctx.paint {
            paint.fill(Rect::from_origin_size(Point::ZERO, size), &Color::WHITE);
        }

        // Render the tree
        let size = self.render_element(&mut ctx, document.root());

        // Aggregate results from the rendering pass
        RenderResult {
            size,
            link_areas: ctx.link_areas,
            title: ctx.title,
        }
    }

    /// Renders a single DOM node.
    fn render_node(&self, ctx: &mut RenderCtx, node: &Node) -> Size {
        match node {
            Node::Element(element) => self.render_element(ctx, element),
            Node::Text(text) => self.render_text(ctx, text),
        }
    }

    /// Renders a single DOM element.
    fn render_element(&self, ctx: &mut RenderCtx, element: &Element) -> Size {
        match element.tag_name() {
            "title" => {
                // Update window title if we have a paint context.
                let title = element.text();
                ctx.title = Some(title);
                Size::ZERO
            },
            tag_name if ctx.in_rendered_tree && RENDERED_TAGS.contains(tag_name) => {
                // Render the tag. To begin, we save the initial cursor state (which we
                // will revert to later, the reason for not simply passing a cloned state
                // to the child is that this plays better with the borrow checker).
                let start_cursor = ctx.cursor.clone();
                let mut size = Size::ZERO;

                // Change styling info as needed
                {
                    let mut styling = &mut ctx.cursor.styling;
                    match element.tag_name() {
                        "b" | "strong" => styling.font_weight = FontWeight::BOLD,
                        "h1" => styling.font_size = 32.0,
                        "h2" => styling.font_size = 26.0,
                        "h3" => styling.font_size = 22.0,
                        "h4" => styling.font_size = 20.0,
                        "a" => styling.color = Color::BLUE,
                        _ => {},
                    }
                    if element.is_heading() {
                        styling.font_weight = FontWeight::BOLD;
                    }
                }

                // Render children
                let mut line_size = Size::ZERO;
                for child in element.children() {
                    // Check whether this is an inline tag
                    let is_inline = child.tag_name().map_or(true, |t| INLINE_TAGS.contains(t));
                    // Render spacing if we have adjacent inline elements
                    if is_inline && line_size != Size::ZERO {
                        ctx.cursor.point.x += ctx.cursor.styling.spacing;
                    }
                    // Render the child element, which computes its size
                    let child_point = ctx.cursor.point;
                    let child_size = self.render_node(ctx, child);
                    // Insert links into the link target map.
                    match child {
                        Node::Element(child_elem) if child_elem.tag_name() == "a" => {
                            if let Some(href) = child_elem.attribute("href") {
                                let child_rect = Rect::from_origin_size(child_point, child_size);
                                ctx.link_areas.areas.push(LinkArea {
                                    area: child_rect,
                                    href: href.to_owned(),
                                });
                            }
                        },
                        _ => {},
                    }
                    // Depending on whether this is an inline tag and whether we are past the
                    // container size we decide whether we should break the line.
                    let relative_pos = ctx.cursor.point - ctx.cursor.base_point;
                    let break_line = !is_inline || relative_pos.x + child_size.width >= ctx.cursor.base_size.width;
                    if break_line {
                        line_size.width = line_size.width.max(child_size.width);
                        line_size.height = line_size.height.max(child_size.height);
                        size.width = size.width.max(line_size.width);
                        size.height += line_size.height;
                        // Jump to next line
                        ctx.cursor.point.x = ctx.cursor.base_point.x;
                        ctx.cursor.point.y += line_size.height;
                        line_size = Size::ZERO;
                    } else {
                        let width_delta = child_size.width;
                        line_size.width += width_delta;
                        line_size.height = line_size.height.max(child_size.height);
                        // Move 'cursor' forward on this line
                        ctx.cursor.point.x += width_delta;
                    }
                }
                if line_size != Size::ZERO {
                    size.width = size.width.max(line_size.width);
                    size.height += line_size.height;
                }

                ctx.cursor = start_cursor;
                trace!("<{}> has size {}", element.tag_name(), size);
                size
            },
            tag_name => {
                if ctx.in_rendered_tree {
                    debug!("Not rendering <{}> (and its subtree)", tag_name);
                }

                let was_in_rendered_tree = ctx.in_rendered_tree;
                ctx.in_rendered_tree = false;

                // Traverse non-rendered childs (since they main contain relevant metadata, e.g. the window title).
                for child in element.children() {
                    self.render_node(ctx, child);
                }

                ctx.in_rendered_tree = was_in_rendered_tree;
                Size::ZERO
            }
        }
    }

    /// Renders some text from the DOM.
    fn render_text(&self, ctx: &mut RenderCtx, text: &str) -> Size {
        if ctx.in_rendered_tree {
            let state = &ctx.cursor;
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
        } else {
            // We skip this text node since we aren't in a rendered part of the tree
            // This includes things like <script> or <style> content, etc.
            Size::ZERO
        }
    }
}

