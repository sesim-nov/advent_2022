use crate::day_02::game_result::*;
#[derive(Clone, Debug, PartialEq)]
pub enum RPSChoice {
    Rock,
    Paper,
    Scissors,
}

impl RPSChoice {
    pub fn play_against(&self, other: &RPSChoice) -> GameResult {
        use RPSChoice::*;
        match (self, other) {
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => GameResult::Win,
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => GameResult::Draw,
            _ => GameResult::Loss,
        }
    }

    pub fn get_winner(&self) -> RPSChoice {
        match self {
            RPSChoice::Rock => RPSChoice::Paper,
            RPSChoice::Paper => RPSChoice::Scissors,
            RPSChoice::Scissors => RPSChoice::Rock,
        }
    }

    pub fn get_loser(&self) -> RPSChoice {
        match self {
            RPSChoice::Rock => RPSChoice::Scissors,
            RPSChoice::Paper => RPSChoice::Rock,
            RPSChoice::Scissors => RPSChoice::Paper,
        }
    }

    pub fn force_result(&self, desired_result: &GameResult) -> RPSChoice {
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

#[cfg(test)]
mod tests{
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
    fn test_force_result() {
        assert_eq!(
            RPSChoice::Rock.force_result(&GameResult::Win),
            RPSChoice::Paper
        );
        assert_eq!(
            RPSChoice::Rock.force_result(&GameResult::Loss),
            RPSChoice::Scissors
        );
        assert_eq!(
            RPSChoice::Rock.force_result(&GameResult::Draw),
            RPSChoice::Rock
        );
        assert_eq!(
            RPSChoice::Paper.force_result(&GameResult::Win),
            RPSChoice::Scissors
        );
        assert_eq!(
            RPSChoice::Paper.force_result(&GameResult::Loss),
            RPSChoice::Rock
        );
        assert_eq!(
            RPSChoice::Paper.force_result(&GameResult::Draw),
            RPSChoice::Paper
        );
        assert_eq!(
            RPSChoice::Scissors.force_result(&GameResult::Win),
            RPSChoice::Rock
        );
        assert_eq!(
            RPSChoice::Scissors.force_result(&GameResult::Loss),
            RPSChoice::Paper
        );
        assert_eq!(
            RPSChoice::Scissors.force_result(&GameResult::Draw),
            RPSChoice::Scissors
        );
    }
}
