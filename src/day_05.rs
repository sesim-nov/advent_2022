use crate::read_file;

mod crate_stacks;
use crate_stacks::*;

pub fn part_01(fname: &std::path::Path) -> String{
    let mut lines = read_file::get_lines(&fname.to_str().unwrap());
    let stacks = CrateStacks::from_lines(&mut lines);
    let commands: Vec<MoveCommand> = lines.map(|x| -> MoveCommand {
        let line: Vec<char> = x
            .unwrap()
            .chars()
            .collect();
        let count = usize::try_from(line[5].to_digit(10).unwrap()).unwrap();
        let source = usize::try_from(line[12].to_digit(10).unwrap()).unwrap();
        let dest = usize::try_from(line[17].to_digit(10).unwrap()).unwrap();
        MoveCommand{
            count,
            source,
            dest,
        }
    }).collect();

    "hi".to_string()
}

pub fn part_02(fname: &std::path::Path) -> String{
    "hi".to_string()
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

    //#[test]
    // fn test_part_02() {
    //     // Arrange
    //     let in_file = "test_input/04.input";
    //     let expect_file = "test_input/04b.expect";

    //     // Act
    //     let lhs = part_02(in_file);
    //     let rhs = read_file::get_lines(expect_file)
    //         .next()
    //         .unwrap()
    //         .unwrap()
    //         .parse()
    //         .unwrap();

    //     // Assert
    //     assert_eq!(lhs, rhs);
    // }
}