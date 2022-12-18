use advent_2022::read_file;

fn main() {
    let path = "input/02.txt";
    let part_1 = part_01(path);

    println!("Part 1: {}", part_1);
}

#[derive(Eq, PartialEq, Debug)]
enum GameResult {
    Win,
    Draw,
    Loss,
}

impl TryFrom<char> for GameResult {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'X' => Ok(GameResult::Loss),
            'Y' => Ok(GameResult::Draw),
            'Z' => Ok(GameResult::Win),
            _ => Err("Invalid character supplied.".into()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum RPSChoice {
    Rock,
    Paper,
    Scissors,
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

    fn get_winner(&self) -> RPSChoice {
        match self {
            RPSChoice::Rock => RPSChoice::Paper,
            RPSChoice::Paper => RPSChoice::Scissors,
            RPSChoice::Scissors => RPSChoice::Rock,
        }
    }

    fn get_loser(&self) -> RPSChoice {
        match self {
            RPSChoice::Rock => RPSChoice::Scissors,
            RPSChoice::Paper => RPSChoice::Rock,
            RPSChoice::Scissors => RPSChoice::Paper,
        }
    }

    fn force_result(&self, desired_result: &GameResult) -> RPSChoice {
        match desired_result {
            GameResult::Win => self.get_winner(),
            GameResult::Loss => self.get_loser(),
            GameResult::Draw => self.clone(),
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

fn part_02(path: &str) -> usize {
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
    fn test_scoring() {
        assert_eq!(p1_play_game(Rock, Paper), 1);
        assert_eq!(p1_play_game(Paper, Rock), 8);
        assert_eq!(p1_play_game(Scissors, Paper), 9);
        assert_eq!(p1_play_game(Scissors, Scissors), 6);
    }
    #[test]
    fn test_parse_rpschoice() {
        assert_eq!(RPSChoice::try_from('A'), Ok(Rock));
        assert_eq!(RPSChoice::try_from('B'), Ok(Paper));
        assert_eq!(RPSChoice::try_from('C'), Ok(Scissors));
        assert_eq!(RPSChoice::try_from('X'), Ok(Rock));
        assert_eq!(RPSChoice::try_from('Y'), Ok(Paper));
        assert_eq!(RPSChoice::try_from('Z'), Ok(Scissors));
        assert!(RPSChoice::try_from('H').is_err());
    }
    #[test]
    fn test_parse_gameresult() {
        assert_eq!(GameResult::try_from('X'), Ok(GameResult::Loss));
        assert_eq!(GameResult::try_from('Y'), Ok(GameResult::Draw));
        assert_eq!(GameResult::try_from('Z'), Ok(GameResult::Win));
        assert!(GameResult::try_from('H').is_err());
    }
    #[test]
    fn test_force_result() {
        assert_eq!(RPSChoice::Rock.force_result(&GameResult::Win), RPSChoice::Paper);
        assert_eq!(RPSChoice::Rock.force_result(&GameResult::Loss), RPSChoice::Scissors);
        assert_eq!(RPSChoice::Rock.force_result(&GameResult::Draw), RPSChoice::Rock);
        assert_eq!(RPSChoice::Paper.force_result(&GameResult::Win), RPSChoice::Scissors);
        assert_eq!(RPSChoice::Paper.force_result(&GameResult::Loss), RPSChoice::Rock);
        assert_eq!(RPSChoice::Paper.force_result(&GameResult::Draw), RPSChoice::Paper);
        assert_eq!(RPSChoice::Scissors.force_result(&GameResult::Win), RPSChoice::Rock);
        assert_eq!(RPSChoice::Scissors.force_result(&GameResult::Loss), RPSChoice::Paper);
        assert_eq!(RPSChoice::Scissors.force_result(&GameResult::Draw), RPSChoice::Scissors);
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
