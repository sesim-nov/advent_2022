use std::collections::HashSet;

use crate::read_file;

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