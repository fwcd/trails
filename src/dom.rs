/// An HTML document.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Document {
    root: Element,
}

impl Document {
    pub fn new() -> Self {
        Self {
            root: Element::tag("$root")
        }
    }
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
    children: Vec<Node>,
}

impl Element {
    /// Creates a new element with the given tag name.
    pub fn tag(tag: &str) -> Self {
        Self {
            tag: tag.to_owned(),
            children: Vec::new(),
        }
    }

    /// Adds a new child to the element.
    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
}
