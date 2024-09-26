use std::{cell::{Ref, RefCell}, rc::{Rc, Weak}};

use super::command_parser::DirCommand;

#[derive(Debug)]
pub enum FSNode {
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

    pub fn execute_commands(&mut self, cmds: Vec<DirCommand>) -> Result<(), &str> {
        for x in cmds {
            match self.execute_command(x){
                Ok(e) => e,
                Err(_) => return Err("Failed to execute command")
            }
        }
        Ok(())
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