/// Tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tree<T> {
    root: Node<T>,
}

impl<T> Tree<T> {
    /// Create a new tree.
    pub fn new(seed: T) -> Self {
        Self {
            root: Node::new(seed),
        }
    }

    /// Get the root node mutably.
    pub fn root_mut(&mut self) -> &mut Node<T> {
        &mut self.root
    }
}

/// Crack node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node<T> {
    children: Vec<Node<T>>,
    data: Option<T>,
}

impl<T> Node<T> {
    /// Create a new node.
    pub fn new(data: T) -> Self {
        Self {
            children: Vec::new(),
            data: Some(data),
        }
    }

    /// Get the children nodes.
    pub fn children(&self) -> &[Node<T>] {
        &self.children
    }

    /// Get the data.
    pub fn data(&self) -> Option<&T> {
        self.data.as_ref()
    }

    /// Add a child node.
    pub fn add_child(&mut self, data: T) -> &mut Node<T> {
        let child = Node {
            children: Vec::new(),
            data: Some(data),
        };
        self.children.push(child);
        self.children.last_mut().unwrap()
    }
}
