use std::{cell::{Ref, RefCell}, cmp::min, rc::{Rc, Weak}};

use super::command_parser::DirCommand;

#[derive(Debug)]
pub enum FSNode {
    FileNode(File),
    DirectoryNode(Directory),
}

impl FSNode {
    #[cfg(test)]
    fn num_children(&self) -> usize {
        match self {
            Self::FileNode(_) => 0,
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
    fn get_name(&self) -> &str {
        match self {
            Self::DirectoryNode(e) => e.get_name(),
            Self::FileNode(e) => e.get_name(),
        }
    }
    fn get_children(&self) -> Option<Ref<'_, Vec<Rc<FSNode>>>> {
        match self {
            Self::FileNode(_) => None,
            Self::DirectoryNode(e) => Some(e.get_children()),
        }
    }
    fn set_parent(&self, parent: Rc<FSNode>) {
        match self {
            Self::FileNode(a) => a.set_parent(parent),
            Self::DirectoryNode(b) => b.set_parent(parent),
        }
    }
    fn get_parent(&self) -> Weak<FSNode>{
        match self {
            Self::FileNode(a) => a.get_parent(),
            Self::DirectoryNode(b) => b.get_parent(),
        }
    }
    fn get_size(&self) -> usize {
        match self {
            Self::FileNode(a) => a.get_size(),
            Self::DirectoryNode(b) => b.get_size()
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
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_parent(&self) -> Weak<FSNode> {
        Weak::clone(&self.parent.borrow())
    }
    pub fn get_size(&self) -> usize {
        self.size
    }
}

#[derive(Debug)]
pub struct Directory {
    pub name: String,
    pub size: RefCell<usize>,
    pub children: RefCell<Vec<Rc<FSNode>>>,
    pub parent: RefCell<Weak<FSNode>>,
}

impl Directory {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            size: RefCell::new(0),
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
        }
    }
    #[cfg(test)]
    fn num_children(&self) -> usize {
        self.children
            .borrow()
            .len()
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    fn get_children(&self) -> Ref<'_, Vec<Rc<FSNode>>> {
        self.children.borrow()
    }
    fn add_child(&self, child: Rc<FSNode>) {
        self.children
            .borrow_mut()
            .push(child);
    }
    pub fn set_parent(&self, parent: Rc<FSNode>) {
        *self.parent.borrow_mut() = Rc::downgrade(&parent);
    }
    pub fn get_parent(&self) -> Weak<FSNode> {
        Weak::clone(&self.parent.borrow())
    }
    pub fn get_size(&self) -> usize {
        if self.size == RefCell::new(0) {
            let mut size = 0;
            for child in self.children.borrow().iter() {
                size += child.get_size();
            }
            let mut self_size = self.size.borrow_mut();
            *self_size = size;
        }
        self.size.borrow().clone()
    }
    pub fn find_100k(&self, list: &mut usize ) {
        let size = self.get_size();
        if size <= 100000 {
            *list += size;
        }
        for child in self.children.borrow().iter() {
            if let FSNode::DirectoryNode(e) = &**child {
                e.find_100k(list);
            }
        }
    }
    pub fn find_min_dir_ge_val(&self, val: &usize, acc: &mut usize) {
        let this_val = *self.size.borrow();
        if this_val >= *val {
            *acc = min(this_val, *acc);
        }
        for child in self.children.borrow().iter() {
            if let FSNode::DirectoryNode(e) = &**child {
                e.find_min_dir_ge_val(val, acc);
            }
        }
    }
}

pub struct FSTree {
    root: Rc<FSNode>,
    cursor: Rc<FSNode>,
}

impl FSTree{
    pub fn new() -> Self {
        let root = Rc::new(FSNode::DirectoryNode(Directory::new("/")));
        let cursor = Rc::clone(&root);
        Self{
            root,
            cursor,
        }
    }

    pub fn cd(&mut self, dirname: &str) -> Result<(), &str> {
        if dirname == "/" {
            self.cursor = Rc::clone(&self.root);
            Ok(())
        } else {
            let tmp_cursor = Rc::clone(&self.cursor);
            let children = tmp_cursor
                .get_children()
                .ok_or("No children.")?;
            let new_cursor = children
                .iter()
                .filter(|x|  x.get_name() == dirname)
                .next()
                .ok_or("No matching directory name")?;
            self.cursor = Rc::clone(new_cursor);
            Ok(())
        }
    }

    pub fn cd_parent(&mut self) -> Result<(), &str>{
        let parent = self.cursor.get_parent();
        let parent = Weak::upgrade(&parent).ok_or("Failed to upgrade weak reference")?;
        self.cursor = parent;
        Ok(())
    }

    pub fn make_node(&self, node: Rc<FSNode>) -> Result<(), &str> {
        node.set_parent(Rc::clone(&self.cursor));
        self.cursor.add_child(node)
    }

    pub fn execute_command(&mut self, cmd: DirCommand) -> Result<(), &str> {
        match cmd {
            DirCommand::AddDirectory(e) => {
                self.make_node(
                    Rc::new(FSNode::DirectoryNode(
                        Directory::new(&e))))
            },
            DirCommand::AddFile(e) => {
                self.make_node(
                    Rc::new(FSNode::FileNode(
                        e
                    ))
                )
            }
            DirCommand::ParentDir => self.cd_parent(),
            DirCommand::ChangeDir(e) => self.cd(&e),
            DirCommand::DoNothing => Ok(()),
        }
    }

    pub fn get_size_root(&self) -> usize {
        self.root.get_size()
    }

    pub fn get_size_10k(&self) -> usize {
        if let FSNode::DirectoryNode(e) = &*self.root {
            let mut val = 0;
            e.find_100k(&mut val);
            val
        } else {0}
    }

    pub fn solve_pt2(&self) -> usize {
        if let FSNode::DirectoryNode(e) = &*self.root {
            let total = 70000000;
            let used = self.get_size_root();
            let available = total - used;
            let required = 30000000 - available;
            let mut acc = total;
            e.find_min_dir_ge_val(&required, &mut acc);
            acc
        } else {0}
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