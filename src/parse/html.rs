use std::collections::{HashMap, HashSet};

use log::warn;
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
    Script(String), // <script> ... </script>
    Style(String), // <style> ... </style>
    Opening { tag_name: String, attributes: HashMap<String, String>, self_closing: bool }, // <tag attr="value">
    Closing { tag_name: String }, // </tag>
    Text(String), // e.g. tag names, attribute names, values, ...
}

static HTML_LEXER: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&[
        // Doctype, i.e. <!DOCTYPE ...>
        r#"(?:<![dD][oO][cC][tT][yY][pP][eE]\s+(?P<doctype>[^>]+)>)"#,
        // Comments, i.e. <!-- ... -->
        r#"(?:<!--(?P<comment>-?[^-]+|--[^>])*-->)"#,
        // Script tags, i.e. <script> ... </script>
        r#"(?:<\s*[sS][cC][rR][iI][pP][tT][^>]*>(?P<script>[\s\S]*?)</\s*[sS][cC][rR][iI][pP][tT]\s*>)"#,
        // Style tags, i.e. <style> ... </style>
        r#"(?:<\s*[sS][tT][yY][lL][eE][^>]*>(?P<style>[\s\S]*?)</\s*[sS][tT][yY][lL][eE]\s*>)"#,
        // Opening/self-closing tags, e.g. <meta charset="utf-8" />
        r#"(?:<\s*(?P<openingtag>\w+)(?P<attributes>(?:\s+[\w\-]+\s*(?:=\s*(?:"[^"]*"|'[^']*'|[\w:%]+))?)*)\s*(?P<selfclosing>/)?\s*>)"#,
        // Closing tags, e.g. </html>
        r#"(?:<\s*/\s*(?P<closingtag>\w+)\s*>)"#,
        // Whitespace
        r#"(?P<white>\s+)"#,
        // Any other text
        r#"(?P<text>[^<]+)"#,
    ].join("|")).unwrap()
});

static ATTRIBUTE_LEXER: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?P<key>[\w\-]+)\s*(?:=\s*(?:"(?P<doublequoted>[^"]*)"|'(?P<singlequoted>[^']*)'|(?P<unquoted>[\w:%]+)))?"#).unwrap()
});

/// Tokenizes a raw HTML dcument.
fn lex_document(raw: &str) -> Vec<HtmlToken> {
    let mut tokens = Vec::new();
    let mut last_end: usize = 0;

    for raw_token in HTML_LEXER.captures_iter(raw) {
        let range = raw_token.get(0).unwrap().range();
        if last_end != range.start {
            let window = 80;
            warn!(
                "Skipping '{}' (context: '...{}...')",
                &raw[last_end..range.start],
                &raw[(last_end - window).max(0)..(range.start + window).min(raw.len())]
            );
        }
        last_end = range.end;

        if let Some(doctype) = raw_token.name("doctype") {
            tokens.push(HtmlToken::Doctype(doctype.as_str().to_owned()));
        } else if let Some(_comment) = raw_token.name("comment") {
            // TODO: Track comments (for now it's more convenient to ignore them)
            // tokens.push(HtmlToken::Comment(comment.as_str().to_owned()));
        } else if let Some(script) = raw_token.name("script").map(|m| m.as_str().to_owned()) {
            tokens.push(HtmlToken::Script(script));
        } else if let Some(style) = raw_token.name("style").map(|m| m.as_str().to_owned()) {
            tokens.push(HtmlToken::Style(style));
        } else if let Some(tag_name) = raw_token.name("openingtag").map(|m| m.as_str().to_owned()) {
            let raw_attributes = &raw_token["attributes"];
            let self_closing = raw_token.name("selfclosing").is_some();
            let mut attributes = HashMap::new();
            for raw_attribute in ATTRIBUTE_LEXER.captures_iter(raw_attributes) {
                let key = raw_attribute["key"].to_owned();
                let value = raw_attribute.name("doublequoted").map(|m| m.as_str())
                    .or_else(|| raw_attribute.name("singlequoted").map(|m| m.as_str()))
                    .or_else(|| raw_attribute.name("unquoted").map(|m| m.as_str()))
                    .unwrap_or_else(|| "")
                    .to_owned();
                attributes.insert(key, value);
            }
            tokens.push(HtmlToken::Opening { tag_name, attributes, self_closing });
        } else if let Some(tag_name) = raw_token.name("closingtag").map(|m| m.as_str().to_owned()) {
            tokens.push(HtmlToken::Closing { tag_name });
        } else if let Some(text) = raw_token.name("text").map(|m| m.as_str().to_owned()) {
            tokens.push(HtmlToken::Text(text));
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
            loop {
                match tokens.peek()?.clone() {
                    HtmlToken::Closing { tag_name } => {
                        tokens.next()?;
                        if tag_name == opening.tag_name {
                            break
                        } else {
                            return Err(Error::UnexpectedToken(format!("Expected </{}>, but was </{}>", opening.tag_name, tag_name)));
                        }
                    },
                    _ => {
                        let child = self.parse_node(tokens)?;
                        children.push(child);
                    }
                }
            }
        }

        Ok(Element::new(&opening.tag_name, opening.attributes, children))
    }

    /// Parse `<tag attr="value" ...>`
    fn parse_opening(&self, tokens: &mut Tokens<HtmlToken>) -> Result<Opening> {
        match tokens.next()? {
            HtmlToken::Opening { tag_name, attributes, self_closing } => Ok(Opening { tag_name, attributes, self_closing }),
            token => Err(Error::UnexpectedToken(format!("Expected tag name but got {:?}", token))),
        }
    }
}
