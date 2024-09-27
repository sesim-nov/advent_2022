use std::collections::HashSet;
use crate::read_file;

pub(crate) mod command_parser;
use command_parser::*;

pub(crate) mod file_tree;
use file_tree::*;

pub struct Answer{
    pub pt_1: String,
    pub pt_2: String,
}

pub fn solve(fname: &std::path::Path) -> Answer{
    let lines = read_file::get_lines(fname.to_str().unwrap());
    let mut tree = FSTree::new();
    let result = lines.into_iter().map(|line| {
        let line = line.expect("Error reading file");
        match command_parser::parse_cmd(&line) {
            Ok(e) => Ok(e),
            Err(e) => Err(e.to_string()),
        }
    });
    for x in result {
        tree.execute_command(x.unwrap()).unwrap();
    }
    let res = tree.get_size_10k();
    let res2 = tree.solve_pt2();
    Answer{
        pt_1: res.to_string(),
        pt_2: res2.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;

    #[test]
    fn test_part_01() {
        // Arrange
        let in_file = Path::new("test_input/07.input");
        let expect_file_1 = "test_input/07a.expect";
        let expect_file_2 = "test_input/07b.expect";

        // Act
        let lhs = solve(in_file);
        let rhs_pt1 = read_file::get_lines(expect_file_1)
            .next()
            .unwrap()
            .unwrap();
        let rhs_pt2 = read_file::get_lines(expect_file_2)
            .next()
            .unwrap()
            .unwrap();


        // Assert
        assert_eq!(lhs.pt_1, rhs_pt1);
        assert_eq!(lhs.pt_2, rhs_pt2);
    }

    // #[test]
    // fn test_parse_cmd() {
    //     //Arrange
    //     let cmd = "$ cd ..";

    //     //Act
    //     let rhs = parse_cmd(cmd);

    //     //Assert
    //     assert_eq!(DirCommand::ChangeDir("..".to_string()), rhs);
    // }


    // #[test]
    // fn test_part_02() {
    //     // Arrange
    //     let in_file = Path::new("test_input/07.input");
    //     let expect_file = "test_input/07b.expect";

    //     // Act
    //     let lhs = part_02(in_file);
    //     let rhs = read_file::get_lines(expect_file)
    //         .next()
    //         .unwrap()
    //         .unwrap();

    //     // Assert
    //     assert_eq!(lhs, rhs);
    // }
}