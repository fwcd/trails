use std::collections::{HashMap, HashSet};

use once_cell::sync::Lazy;
use regex::Regex;

use crate::{dom::{Document, Node, Element}, error::{Result, Error}};

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
    Script(String), // <script> ... </script>
    Style(String), // <style> ... </style>
    Left(String), // <tag
    AttributeKey(String),
    AttributeValue(String),
    Closing, // </
    SelfClosing, // />
    Right, // >
    Text(String), // e.g. tag names, attribute names, values, ...
}

static HTML_LEXER: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&[
        r#"(?:<!DOCTYPE\s+(?P<doctype>[^>]+)>)"#,
        r#"(?:<!--(?P<comment>-?[^-]+|--[^>])*-->)"#,
        r#"(?:<\s*[sS][cC][rR][iI][pP][tT][^>]*>(?P<script>[\s\S]*?)</\s*[sS][cC][rR][iI][pP][tT]\s*>)"#,
        r#"(?:<\s*[sS][tT][yY][lL][eE][^>]*>(?P<style>[\s\S]*?)</\s*[sS][tT][yY][lL][eE]\s*>)"#,
        r#"(?:<\s*(?P<left>\w+)(?:\s*(?P<attribute>[^>]+)(?:=(?:"(?P<quoted>[^"]+)"|(?P<unquoted>\w+)))?)*)"#,
        r#"(?P<closing></)"#,
        r#"(?P<selfclosing>/\s*>)"#,
        r#"(?P<right>>)"#,
        r#"(?P<eq>=)"#,
        r#"(?P<white>\s+)"#,
        r#"(?P<text>[^<>/=]+)"#,
    ].join("|")).unwrap()
});

/// Tokenizes a raw HTML dcument.
fn lex_document(raw: &str) -> Vec<HtmlToken> {
    let mut tokens = Vec::new();
    for raw_token in HTML_LEXER.captures_iter(raw) {
        if let Some(doctype) = raw_token.name("doctype") {
            tokens.push(HtmlToken::Doctype(doctype.as_str().to_owned()));
        } else if let Some(_comment) = raw_token.name("comment") {
            // TODO: Track comments (for now it's more convenient to ignore them)
            // tokens.push(HtmlToken::Comment(comment.as_str().to_owned()));
        } else if let Some(script) = raw_token.name("script") {
            tokens.push(HtmlToken::Script(script.as_str().to_owned()));
        } else if let Some(style) = raw_token.name("style") {
            tokens.push(HtmlToken::Style(style.as_str().to_owned()));
        } else if let Some(tag_name) = raw_token.name("left") {
            tokens.push(HtmlToken::Left(tag_name.as_str().to_owned()));
        } else if raw_token.name("closing").is_some() {
            tokens.push(HtmlToken::Closing);
        } else if raw_token.name("selfclosing").is_some() {
            tokens.push(HtmlToken::SelfClosing);
        } else if raw_token.name("right").is_some() {
            tokens.push(HtmlToken::Right);
        } else if let Some(attribute) = raw_token.name("attribute") {
            tokens.push(HtmlToken::AttributeKey(attribute.as_str().to_owned()));
        } else if let Some(quoted) = raw_token.name("quoted").or_else(|| raw_token.name("unquoted")) {
            tokens.push(HtmlToken::AttributeValue(quoted.as_str().to_owned()));
        } else if raw_token.name("white").is_some() {
            // We ignore whitespace
        } else if let Some(text) = raw_token.name("text") {
            tokens.push(HtmlToken::Text(text.as_str().to_owned()));
        }
    }
    tokens
}

struct Opening {
    tag_name: String,
    attributes: HashMap<String, String>,
    self_closing: bool,
}

static SINGLETON_TAGS: Lazy<HashSet<&str>> = Lazy::new(|| {
    let mut set = HashSet::new();
    set.insert("area");
    set.insert("base");
    set.insert("br");
    set.insert("col");
    set.insert("command");
    set.insert("embed");
    set.insert("hr");
    set.insert("img");
    set.insert("input");
    set.insert("keygen");
    set.insert("link");
    set.insert("meta");
    set.insert("param");
    set.insert("source");
    set.insert("track");
    set.insert("wbr");
    set
});

// A recursive descent parser for HTML.

impl HtmlParser {
    /// Parses an HTML document.
    pub fn parse(&self, raw: &str) -> Result<Document> {
        let mut tokens = Tokens::new(lex_document(raw));
        self.parse_document(&mut tokens)
    }

    fn parse_document(&self, tokens: &mut Tokens<HtmlToken>) -> Result<Document> {
        let mut doc = Document::new();

        if let HtmlToken::Doctype(_) = tokens.peek()? {
            // Ignore doctype for now
            tokens.next()?;
        }

        // Parse <html> ... </html>
        doc.root_mut().add_child(self.parse_node(tokens)?);

        Ok(doc)
    }

    /// Parse an element or simply text (between tags).
    fn parse_node(&self, tokens: &mut Tokens<HtmlToken>) -> Result<Node> {
        if let HtmlToken::Text(txt) = tokens.peek()?.clone() {
            tokens.next()?;
            Ok(Node::Text(txt))
        } else {
            Ok(Node::Element(self.parse_element(tokens)?))
        }
    }

    /// Parse `<tag> ... </tag>` (or only the opening tag for some tags)
    fn parse_element(&self, tokens: &mut Tokens<HtmlToken>) -> Result<Element> {
        // Match specially treated tags first
        match tokens.peek()?.clone() {
            HtmlToken::Script(script) => {
                tokens.next()?;
                return Ok(Element::new("script", HashMap::new(), vec![
                    Node::Text(script)
                ]));
            },
            HtmlToken::Style(style) => {
                tokens.next()?;
                return Ok(Element::new("style", HashMap::new(), vec![
                    Node::Text(style)
                ]));
            },
            _ => {},
        };

        let opening = self.parse_opening(tokens)?;
        let mut children = Vec::new();

        if !opening.self_closing && !SINGLETON_TAGS.contains(opening.tag_name.as_str()) {
            while tokens.peek()? != &HtmlToken::Closing {
                let child = self.parse_node(tokens)?;
                children.push(child);
            }

            tokens.expect(&HtmlToken::Closing)?;
            tokens.expect(&HtmlToken::Text(opening.tag_name.clone()))?;
            tokens.expect(&HtmlToken::Right)?;
        }

        Ok(Element::new(&opening.tag_name, opening.attributes, children))
    }

    /// Parse `<tag attr="value" ...>`
    fn parse_opening(&self, tokens: &mut Tokens<HtmlToken>) -> Result<Opening> {
        let tag_name = match tokens.next()? {
            HtmlToken::Left(name) => Ok(name),
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
        if let HtmlToken::AttributeKey(key) = token.clone() {
            tokens.next()?;
            let value = if let HtmlToken::AttributeValue(value) = tokens.peek()?.clone() {
                tokens.next()?;
                value
            } else {
                "".to_owned()
            };
            Ok(Some((key, value)))
        } else {
            Ok(None)
        }
    }
}
