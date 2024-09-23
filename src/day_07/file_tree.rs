use super::command_parser::DirCommand;

enum FSNode {
    FileNode(File),
    DirectoryNode(Directory),
}

#[derive(Debug, PartialEq)]
pub struct File {
    pub size: usize,
    pub name: String,
}

pub struct Directory {
    pub name: String,
    pub size: usize,
    pub children: Vec<FSNode>,
}

impl Directory {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            size: 0,
            children: Vec::new(),
        }
    }
}

struct FSTree {
    root: FSNode,
    //cursor: &FSNode,
    path: String,
}

impl FSTree{
    pub fn new() -> Self {
        let root = FSNode::DirectoryNode(Directory::new("/"));
        Self{
            root,
            //cursor: &root,
            path: "".to_string(),
        }
    }
}