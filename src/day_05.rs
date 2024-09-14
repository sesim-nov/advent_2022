use crate::read_file;

pub fn part_01(fname: &std::path::Path) -> String{
    let lines = read_file::get_lines(&fname.to_str().unwrap());
    for line in lines {
        println!("{}", line.unwrap());
    }
    "hi".to_string()
}

pub fn part_02(fname: &std::path::Path) -> String{
    "hi".to_string()
}