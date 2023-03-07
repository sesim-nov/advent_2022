use std::str::FromStr;
use crate::day_05::instruction::Instruction;

pub struct Problem {
    pub stack: Vec<Vec<char>>,
    pub instructions: Vec<Instruction>,
}

impl FromStr for Problem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!();
    }
}

