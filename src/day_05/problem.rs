use crate::day_05::instruction::Instruction;
use crate::day_05::parsing;
use crate::day_05::stacks::Stacks;

use std::str::FromStr;

pub struct Problem {
    pub stacks: Stacks,
    pub instructions: Vec<Instruction>,
}

impl FromStr for Problem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = parsing::split_input(s);
        let stacks = Stacks::from_str(&parts.stacks)?;
        let instructions: Result<Vec<Instruction>, String> = parts
            .procedure
            .split("\n")
            .map(|x| Instruction::from_str(x))
            .collect();
        let instructions = instructions?;
        Ok(Self {
            stacks,
            instructions,
        })
    }
}
