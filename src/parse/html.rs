use crate::dom::Document;

/// A parser for HTML.
pub struct HtmlParser {}

impl Default for HtmlParser {
    fn default() -> Self {
        HtmlParser {}
    }
}

impl HtmlParser {
    /// Parses an HTML document.
    pub fn parse(&self, raw: &str) -> Document {
        // TODO
        Document::new()
    }
}
