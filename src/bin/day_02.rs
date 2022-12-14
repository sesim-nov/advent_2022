use advent_2022::read_file;

fn main() {
    println!("Hello world!");
}

fn part_01(path: &str) -> usize {
    42
}

#[cfg(test)]
mod day_02 {
    use super::*;

    #[test]
    fn test_part_1() {
        // Arrange
        let in_file = "test_input/02.input";
        let expect_file = "test_input/02a.expect";

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
