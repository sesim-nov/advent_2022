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
    fn is_equal(&self, other: &RPSChoice) -> bool {
        let res = self.play_against(other);
        match res {
            GameResult::Draw => true,
            _ => false
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
