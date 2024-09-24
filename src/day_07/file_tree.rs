use std::{cell::RefCell, rc::{Rc, Weak}};

use super::command_parser::DirCommand;

#[derive(Debug)]
enum FSNode {
    FileNode(File),
    DirectoryNode(Directory),
}

impl FSNode {
    fn num_children(&self) -> usize {
        match self {
            Self::FileNode(e) => 0,
            Self::DirectoryNode(d) => d.num_children(),
        }
    }
    fn add_child(&self, child: Rc<FSNode>) -> Result<(),&str> {
        match self {
            Self::FileNode(_) => Err("Add Children called on regular file"),
            Self::DirectoryNode(d) => {
                d.add_child(child);
                Ok(())
            },
        }
    }
    fn set_parent(&self, parent: Rc<FSNode>) {
        match self {
            Self::FileNode(a) => a.set_parent(parent),
            Self::DirectoryNode(b) => b.set_parent(parent),
        }
    }
}

#[derive(Debug)]
pub struct File {
    pub size: usize,
    pub name: String,
    pub parent: RefCell<Weak<FSNode>>,
}

impl File {
    pub fn new(name: &str, size: usize) -> Self{
        Self {
            size,
            name: name.to_string(),
            parent: RefCell::new(Weak::new()),
        }
    }
    pub fn set_parent(&self, parent: Rc<FSNode>) {
        *self.parent.borrow_mut() = Rc::downgrade(&parent);
    }
}

#[derive(Debug)]
pub struct Directory {
    pub name: String,
    pub size: Option<usize>,
    pub children: RefCell<Vec<Rc<FSNode>>>,
    pub parent: RefCell<Weak<FSNode>>,
}

impl Directory {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            size: None,
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
        }
    }
    fn num_children(&self) -> usize {
        self.children
            .borrow()
            .len()
    }
    fn add_child(&self, child: Rc<FSNode>) {
        self.children
            .borrow_mut()
            .push(child);
    }
    pub fn set_parent(&self, parent: Rc<FSNode>) {
        *self.parent.borrow_mut() = Rc::downgrade(&parent);
    }
}

struct FSTree {
    root: Rc<FSNode>,
    cursor: Rc<FSNode>,
    path: String,
}

impl FSTree{
    pub fn new() -> Self {
        let root = Rc::new(FSNode::DirectoryNode(Directory::new("/")));
        let cursor = Rc::clone(&root);
        Self{
            root,
            cursor,
            path: "".to_string(),
        }
    }

    pub fn cd(&mut self, dirname: &str) {
        if dirname == "/" {
            self.cursor = Rc::clone(&self.root);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_child() {
        //Arrange
        let node = FSNode::DirectoryNode(
            Directory::new("a")
        );
        let child_node = Rc::new(FSNode::DirectoryNode(Directory::new("b")));
        let grandchild_node = Rc::new(FSNode::FileNode(File::new("c.txt", 4)));

        //Act
        node.add_child(Rc::clone(&child_node)).expect("Error adding child");
        child_node.add_child(grandchild_node).expect("Error adding grandchild");

        //Assert
        assert_eq!(node.num_children(), 1);
        assert_eq!(child_node.num_children(), 1);
    }
}