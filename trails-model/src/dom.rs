use std::{collections::HashMap, borrow::Cow};

use trails_base::once_cell::sync::Lazy;
use trails_base::regex::Regex;

/// An HTML document.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Document {
    root: Element,
}

impl Document {
    /// Creates a new (empty) document.
    pub fn new() -> Self {
        Self {
            root: Element::root(),
        }
    }

    /// Creates a new document from the given element.
    pub fn from_root(root: Element) -> Self {
        Self {
            root,
        }
    }

    /// The root node.
    pub fn root(&self) -> &Element { &self.root }
}

/// A node in the DOM tree.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Node {
    Text(String),
    Element(Element),
}

impl Node {
    /// The tag name if this is an element.
    pub fn tag_name(&self) -> Option<&str> {
        match self {
            Self::Element(element) => Some(element.tag_name()),
            _ => None,
        }
    }

    /// The combined text under this tree. Cheap if this is a text node.
    pub fn text(&self) -> Cow<str> {
        match self {
            Node::Text(text) => Cow::Borrowed(text),
            Node::Element(element) => Cow::Owned(element.text()),
        }
    }
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
    /// Creates a new root element.
    pub fn root() -> Self {
        Self::tag("$root")
    }

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

    /// The combined text under this tree.
    pub fn text(&self) -> String {
        self.children.iter().map(|c| c.text()).collect::<Vec<_>>().join(" ")
    }

    /// Whether this is a heading tag.
    pub fn is_heading(&self) -> bool {
        HEADING_TAG.is_match(&self.tag_name)
    }

    /// Iterates the children.
    pub fn children(&self) -> impl Iterator<Item=&Node> {
        self.children.iter()
    }

    /// Fetches an attribute.
    pub fn attribute(&self, key: &str) -> Option<&str> {
        self.attributes.get(key).map(|s| s.as_str())
    }

    /// Adds a new child to the element.
    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
}
