
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
