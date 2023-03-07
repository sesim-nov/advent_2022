use crate::day_05::parsing::convert_next_to_usize;
use std::str::FromStr;

pub struct Instruction {
    pub amount: usize,
    pub from: usize,
    pub to: usize,
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");
        let _ = split.next();
        let amount = convert_next_to_usize(split.next())?;
        let _ = split.next();
        let from = convert_next_to_usize(split.next())?;
        let _ = split.next();
        let to = convert_next_to_usize(split.next())?;
        Ok(Self {
            amount,
            from,
            to,
        })
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn parse_instruction_works() {
        // Arrange
        let inst = "move 4 from 2 to 8";

        // Act
        let res = Instruction::from_str(inst).unwrap();

        // Assert
        assert_eq!(res.amount, 4);
        assert_eq!(res.from, 2);
        assert_eq!(res.to, 8);
    }
}
