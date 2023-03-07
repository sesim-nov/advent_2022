use std::str::FromStr;
use crate::day_05::instruction::Instruction;
use crate::day_05::parsing;

pub struct Problem {
    pub stack: Vec<Vec<char>>,
    pub instructions: Vec<Instruction>,
}

impl FromStr for Problem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = parsing::split_input(s);
        let stacks = parts.stacks;
        let instructions: Result<Vec<Instruction>, String> = parts.procedure
            .split("\n")
            .map(|x| Instruction::from_str(x))
            .collect();
        let instructions = instructions?;
        todo!();
    }
}

