#[derive(Eq, PartialEq, Debug)]
pub enum GameResult {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_gameresult() {
        assert_eq!(GameResult::try_from('X'), Ok(GameResult::Loss));
        assert_eq!(GameResult::try_from('Y'), Ok(GameResult::Draw));
        assert_eq!(GameResult::try_from('Z'), Ok(GameResult::Win));
        assert!(GameResult::try_from('H').is_err());
    }
}
