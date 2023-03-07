use crate::read_file;
use std::str::FromStr;

mod instruction;
mod parsing;
mod problem;

pub fn part_01(path: &str) -> String {
    "42".to_string()
}

pub fn part_02(path: &str) -> usize {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_01() {
        // Arrange
        let in_file = "test_input/05.input";
        let expect_file = "test_input/05a.expect";

        // Act
        let lhs = part_01(in_file);
        let rhs = read_file::get_lines(expect_file).next().unwrap().unwrap();

        // Assert
        assert_eq!(lhs, rhs);
    }

    //#[test]
    //fn test_part_02() {
    //    // Arrange
    //    let in_file = "test_input/05.input";
    //    let expect_file = "test_input/05b.expect";

    //    // Act
    //    let lhs = part_02(in_file);
    //    let rhs = read_file::get_lines(expect_file)
    //        .next()
    //        .unwrap()
    //        .unwrap()
    //        .parse()
    //        .unwrap();

    //    // Assert
    //    assert_eq!(lhs, rhs);
    //}
}
