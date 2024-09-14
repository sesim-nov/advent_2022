use crate::read_file;

struct CrateStacks {
    stacks: Vec<char>,
}

pub fn part_01(fname: &std::path::Path) -> String{
    let mut lines = read_file::get_lines(&fname.to_str().unwrap());
    let first_line = lines.next().unwrap().unwrap();

    println!("There are {} columns in this input", (first_line.len() + 1) / 4);

    "hi".to_string()
}

pub fn part_02(fname: &std::path::Path) -> String{
    "hi".to_string()
}