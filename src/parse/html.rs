use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::{dom::{Document, Node, Element}, error::{Result, Error}, util::none_if_empty};

use super::Tokens;

/// A parser for HTML.
pub struct HtmlParser {}

impl Default for HtmlParser {
    fn default() -> Self {
        HtmlParser {}
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum HtmlToken {
    Doctype(String), // <!DOCTYPE ...>
    Comment(String), // <!-- ... -->
    Left, // <
    Closing, // </
    SelfClosing, // />
    Right, // >
    Eq, // =
    Quoted(String), // "..."
    Text(String), // e.g. tag names, attribute names, values, ...
}

static HTML_LEXER: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&[
        r#"(?:<!DOCTYPE\s+(?<doctype>[^>]+)>)"#,
        r#"(?:<!--(?<comment>-?[^-]+|--[^>])*-->)"#,
        r#"(?<left><)"#,
        r#"(?<closing><\/)"#,
        r#"(?<selfclosing>\/\s*>)"#,
        r#"(?<right>>)"#,
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
        } else if !raw_token["closing"].is_empty() {
            tokens.push(HtmlToken::Closing);
        } else if !raw_token["selfclosing"].is_empty() {
            tokens.push(HtmlToken::SelfClosing);
        } else if !raw_token["right"].is_empty() {
            tokens.push(HtmlToken::Right);
        } else if !raw_token["eq"].is_empty() {
            tokens.push(HtmlToken::Eq);
        } else if let Some(quoted) = none_if_empty(&raw_token["doublequoted"]).or_else(|| none_if_empty(&raw_token["singlequoted"])) {
            tokens.push(HtmlToken::Quoted(quoted.to_owned()));
        } else if !raw_token["white"].is_empty() {
            // We skip whitespace
        } else if let Some(text) = none_if_empty(&raw_token["text"]) {
            tokens.push(HtmlToken::Text(text.to_owned()));
        }
    }
    tokens
}

struct Opening {
    tag_name: String,
    attributes: HashMap<String, String>,
    self_closing: bool,
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

    /// Parse an element or simply text (between tags).
    fn parse_node(&self, tokens: &mut Tokens<HtmlToken>) -> Result<Document> {
        // TODO
    }

    /// Parse `<tag> ... </tag>` (or only the opening tag for some tags)
    fn parse_element(&self, tokens: &mut Tokens<HtmlToken>) -> Result<Element> {
        // TODO
    }

    /// Parse `<tag attr="value" ...>`
    fn parse_opening(&self, tokens: &mut Tokens<HtmlToken>) -> Result<Opening> {
        tokens.expect(&HtmlToken::Left)?;

        let tag_name = match tokens.next()? {
            HtmlToken::Text(name) => Ok(name),
            token => Err(Error::UnexpectedToken(format!("Expected tag name but got {:?}", token))),
        }?;

        let mut attributes = HashMap::new();
        while let Some((key, value)) = self.parse_attribute(tokens)? {
            attributes.insert(key, value);
        }

        let self_closing = tokens.expect_optionally(&HtmlToken::SelfClosing)?;
        if !self_closing {
            tokens.expect(&HtmlToken::Right)?;
        }

        Ok(Opening {
            tag_name,
            attributes,
            self_closing
        })
    }

    /// Parse `attr="value"`
    fn parse_attribute(&self, tokens: &mut Tokens<HtmlToken>) -> Result<Option<(String, String)>> {
        let token = tokens.peek()?;
        if let HtmlToken::Text(key) = token {
            tokens.next()?;
            tokens.expect(&HtmlToken::Eq)?;
            let value = match tokens.next()? {
                HtmlToken::Text(txt) | HtmlToken::Quoted(txt) => Ok(txt),
                token => Err(Error::UnexpectedToken(format!("Expected attribute value but got {:?}", token))),
            }?;
            Ok(Some((key.clone(), value)))
        } else {
            Ok(None)
        }
    }
}
