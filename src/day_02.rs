use crate::read_file;

mod game_result;
use game_result::*;

mod rps_choice;
use rps_choice::*;

pub fn part_01(path: &str) -> usize {
    let lines = read_file::get_lines(path);
    lines
        .map(|line| {
            let s = line.unwrap();
            let mut c = s.chars();
            let opponent = RPSChoice::try_from(c.next().unwrap()).expect("Parsing error.");
            c.next();
            let player = RPSChoice::try_from(c.next().unwrap()).expect("Parsing error.");
            p1_play_game(player, opponent)
        })
        .sum()
}

fn p1_play_game(player: RPSChoice, opponent: RPSChoice) -> usize {
    let game_result = player.play_against(&opponent);
    score_game(&player, &game_result)
}

fn score_game(player: &RPSChoice, result: &GameResult) -> usize {
    let game_score = match result {
        GameResult::Win => 6,
        GameResult::Draw => 3,
        GameResult::Loss => 0,
    };
    let choice_score = match player {
        RPSChoice::Rock => 1,
        RPSChoice::Paper => 2,
        RPSChoice::Scissors => 3,
    };
    game_score + choice_score
}

pub fn part_02(path: &str) -> usize {
    let lines = read_file::get_lines(path);
    lines
        .map(|line| {
            let s = line.unwrap();
            let mut c = s.chars();
            let opponent = RPSChoice::try_from(c.next().unwrap()).expect("Parsing error.");
            c.next();
            let result = GameResult::try_from(c.next().unwrap()).expect("Parsing error.");
            p2_play_game(result, opponent)
        })
        .sum()
}

fn p2_play_game(result: GameResult, opponent: RPSChoice) -> usize {
    let player = opponent.force_result(&result);
    score_game(&player, &result)
}

#[cfg(test)]
mod day_02_tests {
    use super::*;
    use RPSChoice::*;

    #[test]
    fn test_scoring() {
        assert_eq!(p1_play_game(Rock, Paper), 1);
        assert_eq!(p1_play_game(Paper, Rock), 8);
        assert_eq!(p1_play_game(Scissors, Paper), 9);
        assert_eq!(p1_play_game(Scissors, Scissors), 6);
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
    #[test]
    fn test_part_2() {
        // Arrange
        let in_file = "test_input/02.input";
        let expect_file = "test_input/02b.expect";

        // Act
        let lhs = part_02(in_file);
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
