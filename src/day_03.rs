use crate::read_file;

mod rucksack;
use rucksack::*;

fn part_01(path:&str) -> usize {
    let lines = read_file::get_lines(path);
    lines.map( |line| {
        let sack = Rucksack::from(line.unwrap());
        score_char(sack.get_common()[0])
    })
    .sum()
}

fn score_char(l: char) -> usize {
    let a: usize = if l.is_uppercase() {
        'A' as usize - 26
    } else {
        'a' as usize
    };
    l as usize - a + 1
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

    #[test]
    fn test_char_math() {
        assert_eq!(score_char('a'),1);
        assert_eq!(score_char('z'),26);
        assert_eq!(score_char('A'),27);
        assert_eq!(score_char('Z'),52);
    }
}
