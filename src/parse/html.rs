use once_cell::sync::Lazy;
use regex::Regex;

use crate::{dom::Document, error::Result, util::none_if_empty};

use super::Tokens;

/// A parser for HTML.
pub struct HtmlParser {}

impl Default for HtmlParser {
    fn default() -> Self {
        HtmlParser {}
    }
}

enum HtmlToken {
    Doctype(String), // <!DOCTYPE ...>
    Comment(String), // <!-- ... -->
    Left, // <
    Right, // >
    Slash, // /
    Eq, // =
    Quoted(String), // "..."
    Whitespace(String),
    Text(String), // e.g. tag names, attribute names, values, ...
}

static HTML_LEXER: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&[
        r#"(?:<!DOCTYPE\s+(?<doctype>[^>]+)>)"#,
        r#"(?:<!--(?<comment>-?[^-]+|--[^>])*-->)"#,
        r#"(?<left><)"#,
        r#"(?<right>>)"#,
        r#"(?<slash>\/)"#,
        r#"(?<eq>=)"#,
        r#"(?<doublequoted>"[^"]+")"#,
        r#"(?<singlequoted>'[^']+')"#,
        r#"(?<white>\s+)"#,
        r#"(?<text>[^<>\/=]+)"#,
    ].join("|")).unwrap()
});

/// Tokenizes a raw HTML dcument.
fn lex_document(raw: &str) -> Vec<HtmlToken> {
    let mut tokens = Vec::new();
    for raw_token in HTML_LEXER.captures_iter(raw) {
        if let Some(doctype) = none_if_empty(&raw_token["doctype"]) {
            tokens.push(HtmlToken::Doctype(doctype.to_owned()));
        } else if let Some(comment) = none_if_empty(&raw_token["comment"]) {
            tokens.push(HtmlToken::Comment(comment.to_owned()));
        } else if !raw_token["left"].is_empty() {
            tokens.push(HtmlToken::Left);
        } else if !raw_token["right"].is_empty() {
            tokens.push(HtmlToken::Right);
        } else if !raw_token["eq"].is_empty() {
            tokens.push(HtmlToken::Eq);
        } else if let Some(quoted) = none_if_empty(&raw_token["doublequoted"]).or_else(|| none_if_empty(&raw_token["singlequoted"])) {
            tokens.push(HtmlToken::Quoted(quoted.to_owned()));
        } else if let Some(white) = none_if_empty(&raw_token["white"]) {
            tokens.push(HtmlToken::Whitespace(white.to_owned()));
        } else if let Some(text) = none_if_empty(&raw_token["text"]) {
            tokens.push(HtmlToken::Text(text.to_owned()));
        }
    }
    tokens
}

impl HtmlParser {
    /// Parses an HTML document.
    pub fn parse(&self, raw: &str) -> Result<Document> {
        let mut tokens = Tokens::new(lex_document(raw));
        self.parse_document(&mut tokens)
    }

    fn parse_document(&self, tokens: &mut Tokens<HtmlToken>) -> Result<Document> {
        // TODO
        Ok(Document::new())
    }
}
