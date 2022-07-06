use std::collections::HashMap;

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
    tag: String,
    attributes: HashMap<String, String>,
    children: Vec<Node>,
}

impl Element {
    /// Creates a new element with the given tag name, attributes and children.
    pub fn new(tag: &str, attributes: HashMap<String, String>, children: Vec<Node>) -> Self {
        Self {
            tag: tag.to_owned(),
            attributes,
            children
        }
    }

    /// Creates a new element with the given tag name.
    pub fn tag(tag: &str) -> Self {
        Self {
            tag: tag.to_owned(),
            attributes: HashMap::new(),
            children: Vec::new(),
        }
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
