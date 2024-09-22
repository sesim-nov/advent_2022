use std::collections::HashSet;

use crate::read_file;

#[derive(Debug, PartialEq)]
enum DirCommand{
    ChangeDir(String),
    ParentDir,
    AddFile(String),
    AddDirectory(String),
    DoNothing,
}

fn parse_cmd(cmd: &str) -> Result<DirCommand, String> {
    let split_cmd: Vec<&str> = cmd.split_whitespace().collect();
    match split_cmd[0] {
        "$" => match split_cmd[1] {
            "cd" => {
                match split_cmd.get(2) {
                    Some(dest) => Ok(Dircommans::ChangeDir(dest)),
                    None => Err("No destination provided".to_string())
                }
            },
            "ls" => Ok(DirCommand::DoNothing),
            _ => Err("Unknown Command".to_string()),
        },
        "dir" => DirCommand::AddDirectory(split_cmd[1].to_string()),
        _ => DirCommand::AddFile(split_cmd[1].to_string()),
    }
}

pub fn part_01(fname: &std::path::Path) -> String{
    "stub!".to_string()
}
  

pub fn part_02(fname: &std::path::Path) -> String{
    "stub!".to_string()
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;

    #[test]
    fn test_part_01() {
        // Arrange
        let in_file = Path::new("test_input/07.input");
        let expect_file = "test_input/07a.expect";

        // Act
        let lhs = part_01(in_file);
        let rhs = read_file::get_lines(expect_file)
            .next()
            .unwrap()
            .unwrap();

        // Assert
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_parse_cmd() {
        //Arrange
        let cmd = "$ cd ..";

        //Act
        let rhs = parse_cmd(cmd);

        //Assert
        assert_eq!(DirCommand::ChangeDir("..".to_string()), rhs);
    }


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