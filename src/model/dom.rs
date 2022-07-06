use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;

/// An HTML document.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Document {
    root: Element,
}

impl Document {
    /// Creates a new (empty) document.
    pub fn new() -> Self {
        Self {
            root: Element::tag("$root")
        }
    }

    /// The root node.
    pub fn root(&self) -> &Element { &self.root }

    /// The root node, mutably.
    pub fn root_mut(&mut self) -> &mut Element { &mut self.root }
}

/// A node in the DOM tree.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Node {
    Text(String),
    Element(Element),
}

/// An HTML element.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Element {
    tag_name: String,
    attributes: HashMap<String, String>,
    children: Vec<Node>,
}

static HEADING_TAG: Lazy<Regex> = Lazy::new(|| Regex::new(r"h\d+").unwrap());

impl Element {
    /// Creates a new element with the given tag name, attributes and children.
    pub fn new(tag_name: &str, attributes: HashMap<String, String>, children: Vec<Node>) -> Self {
        Self {
            tag_name: tag_name.to_owned(),
            attributes,
            children
        }
    }

    /// Creates a new element with the given tag name.
    pub fn tag(tag_name: &str) -> Self {
        Self {
            tag_name: tag_name.to_owned(),
            attributes: HashMap::new(),
            children: Vec::new(),
        }
    }

    /// Fetches the tag name.
    pub fn tag_name(&self) -> &str {
        &self.tag_name
    }

    /// Whether this is a heading tag.
    pub fn is_heading(&self) -> bool {
        HEADING_TAG.is_match(&self.tag_name)
    }

    /// Iterates the children.
    pub fn children(&self) -> impl Iterator<Item=&Node> {
        self.children.iter()
    }

    /// Adds a new child to the element.
    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
}
