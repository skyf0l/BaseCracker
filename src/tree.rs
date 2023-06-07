use std::fmt;
use std::{cell::RefCell, rc::Rc};

/// Reference to a node.
pub type RefNode<T> = Rc<RefCell<Node<T>>>;

/// Tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tree<T> {
    root: RefNode<T>,
}

impl<T> Tree<T> {
    /// Create a new tree.
    pub fn new(seed: T) -> Self {
        Self {
            root: Node::new(seed),
        }
    }

    /// Get the root node.
    pub fn root(&mut self) -> RefNode<T> {
        self.root.clone()
    }
}

/// Node.
#[derive(Clone, PartialEq, Eq)]
pub struct Node<T> {
    children: Vec<RefNode<T>>,
    parent: Option<RefNode<T>>,
    data: Option<T>,
}

impl<T> fmt::Debug for Node<T>
where
    T: fmt::Debug,
{
    /// Remove the parent field from the debug output and avoid infinite recursion.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            // Remove `RefCell` from the debug output.
            .field(
                "children",
                &self.children.iter().map(|c| c.borrow()).collect::<Vec<_>>(),
            )
            .field("data", &self.data)
            .finish()
    }
}

impl<T> Node<T> {
    /// Create a new node.
    pub fn new(data: T) -> RefNode<T> {
        Rc::new(RefCell::new(Self {
            children: Vec::new(),
            parent: None,
            data: Some(data),
        }))
    }

    /// Create a new node with parent.
    pub fn new_with_parent(data: T, parent: RefNode<T>) -> RefNode<T> {
        Rc::new(RefCell::new(Self {
            children: Vec::new(),
            parent: Some(parent),
            data: Some(data),
        }))
    }

    /// Get the children nodes.
    pub fn children(&self) -> &[RefNode<T>] {
        &self.children
    }

    /// Get the data.
    pub fn data(&self) -> Option<&T> {
        self.data.as_ref()
    }
}

/// Add a child node.
pub fn add_child<T>(node: &RefNode<T>, data: T) -> RefNode<T> {
    let child = Node::new_with_parent(data, Rc::clone(node));
    node.borrow_mut().children.push(child.clone());
    child
}
