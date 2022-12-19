use crate::read_file;

mod rucksack;
use rucksack::*;

fn part_01(path:&str) -> usize {
    42
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_01() {
        // Arrange
        let in_file = "test_input/03.input";
        let expect_file = "test_input/03a.expect";

        // Act
        let lhs = part_01(in_file);
        let rhs = read_file::get_lines(expect_file)
            .next()
            .unwrap()
            .unwrap()
            .parse()
            .unwrap();

        // Assert
        assert_eq!(lhs, rhs);
    }
}
