use advent_2022::read_file;

fn main() {
    println!("Hello world!");
}

#[derive(Eq, PartialEq, Debug)]
enum GameResult {
    Win,
    Draw,
    Loss
}

#[derive(Debug, PartialEq)]
enum RPSChoice {
    Rock,
    Paper,
    Scissors
}

impl RPSChoice {
    fn play_against(&self, other: &RPSChoice) -> GameResult {
        use RPSChoice::*;
        match (self, other) {
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => GameResult::Win,
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => GameResult::Draw,
            _ => GameResult::Loss,
        }
    }
}

impl TryFrom<char> for RPSChoice {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' | 'X' => Ok(RPSChoice::Rock),
            'B' | 'Y' => Ok(RPSChoice::Paper),
            'C' | 'Z' => Ok(RPSChoice::Scissors),
            _ => Err("Invalid character supplied.".into()),
        }
    }
}

fn part_01(path: &str) -> usize {
    42
}

#[cfg(test)]
mod day_02 {
    use super::*;
    use RPSChoice::*;

    #[test]
    fn test_gameplay() {
        assert_eq!(Rock.play_against(&Scissors), GameResult::Win);
        assert_eq!(Rock.play_against(&Paper), GameResult::Loss);
        assert_eq!(Rock.play_against(&Rock), GameResult::Draw);
        assert_eq!(Scissors.play_against(&Paper), GameResult::Win);
        assert_eq!(Scissors.play_against(&Rock), GameResult::Loss);
        assert_eq!(Scissors.play_against(&Scissors), GameResult::Draw);
        assert_eq!(Paper.play_against(&Rock), GameResult::Win);
        assert_eq!(Paper.play_against(&Scissors), GameResult::Loss);
        assert_eq!(Paper.play_against(&Paper), GameResult::Draw);
    }
    #[test]
    fn test_parse() {
        assert_eq!(RPSChoice::try_from('A'), Ok(Rock));
        assert_eq!(RPSChoice::try_from('B'), Ok(Paper));
        assert_eq!(RPSChoice::try_from('C'), Ok(Scissors));
        assert_eq!(RPSChoice::try_from('X'), Ok(Rock));
        assert_eq!(RPSChoice::try_from('Y'), Ok(Paper));
        assert_eq!(RPSChoice::try_from('Z'), Ok(Scissors));
        assert!(RPSChoice::try_from('H').is_err());
    }

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
