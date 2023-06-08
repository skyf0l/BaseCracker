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

    /// Get leaves of the tree.
    pub fn leaves(&self) -> Vec<RefNode<T>> {
        let mut leaves = Vec::new();

        // Ignore the root node.
        for child in &self.root.borrow().children {
            node_leaves(child, &mut leaves);
        }
        leaves
    }
}

/// Node.
#[derive(Clone, PartialEq, Eq)]
pub struct Node<T> {
    pub children: Vec<RefNode<T>>,
    pub parent: Option<RefNode<T>>,
    pub data: Rc<T>,
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
            data: Rc::new(data),
        }))
    }

    /// Create a new node with parent.
    pub fn new_with_parent(data: T, parent: RefNode<T>) -> RefNode<T> {
        Rc::new(RefCell::new(Self {
            children: Vec::new(),
            parent: Some(parent),
            data: Rc::new(data),
        }))
    }
}

/// Add a child node.
pub fn add_child<T>(node: &RefNode<T>, data: T) -> RefNode<T> {
    let child = Node::new_with_parent(data, Rc::clone(node));
    node.borrow_mut().children.push(child.clone());
    child
}

/// Get leaves of the tree.
fn node_leaves<T>(node: &RefNode<T>, leaves: &mut Vec<RefNode<T>>) {
    if node.borrow().children.is_empty() {
        leaves.push(Rc::clone(node));
    } else {
        for child in &node.borrow().children {
            node_leaves(child, leaves);
        }
    }
}
