use advent_2022::read_file;

fn main () {
    println!("Hello world!!");
}

fn day_01(f: &str) -> usize {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        // Arrange
        let in_file = "test_input/01.input";
        let expect_file = "test_input/01.expect";

        // Act
        let lhs = day_01(in_file);
        let rhs = read_file::get_lines(expect_file).next().unwrap().unwrap().parse().unwrap();

        // Assert
        assert_eq!(lhs, rhs);
    }
}
