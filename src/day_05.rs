use crate::read_file;

mod crate_stacks;
use crate_stacks::*;

pub fn part_01(fname: &std::path::Path) -> String{
    let mut lines = read_file::get_lines(&fname.to_str().unwrap());
    let mut stacks = CrateStacks::from_lines(&mut lines);
    let commands: Vec<MoveCommand> = lines.map(|x| -> MoveCommand {
        let line: Vec<String> = x
            .unwrap()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect();
        let count:usize = line[1].parse().unwrap();
        let source:usize = line[3].parse().unwrap();
        let dest:usize = line[5].parse().unwrap();
        MoveCommand{
            count,
            source,
            dest,
        }
    }).collect();

    for command in commands {
        stacks.process_cmd_9000(command);
    }

    stacks.make_top_string()
}

pub fn part_02(fname: &std::path::Path) -> String{
    let mut lines = read_file::get_lines(&fname.to_str().unwrap());
    let mut stacks = CrateStacks::from_lines(&mut lines);
    let commands: Vec<MoveCommand> = lines.map(|x| -> MoveCommand {
        let line: Vec<String> = x
            .unwrap()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect();
        let count:usize = line[1].parse().unwrap();
        let source:usize = line[3].parse().unwrap();
        let dest:usize = line[5].parse().unwrap();
        MoveCommand{
            count,
            source,
            dest,
        }
    }).collect();

    for command in commands {
        stacks.process_cmd_9001(command);
    }

    stacks.make_top_string()
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;

    #[test]
    fn test_part_01() {
        // Arrange
        let in_file = Path::new("test_input/05.input");
        let expect_file = "test_input/05a.expect";

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
    fn test_part_02() {
        // Arrange
        let in_file = Path::new("test_input/05.input");
        let expect_file = "test_input/05b.expect";

        // Act
        let lhs = part_02(in_file);
        let rhs = read_file::get_lines(expect_file)
            .next()
            .unwrap()
            .unwrap();

        // Assert
        assert_eq!(lhs, rhs);
    }
}