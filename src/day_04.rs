use crate::read_file;
use std::str::FromStr;

mod section_range;
use section_range::*;

pub fn part_01(path: &str) -> usize {
    let lines = read_file::get_lines(path);
    lines
        .map(|line| {
            let ranges: Vec<SectionRange> = line
                .unwrap()
                .split(',')
                .map(|x| SectionRange::from_str(x).unwrap())
                .collect();
            (ranges[0].contains(&ranges[1]) || ranges[1].contains(&ranges[0])) as usize
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_01() {
        // Arrange
        let in_file = "test_input/04.input";
        let expect_file = "test_input/04a.expect";

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
