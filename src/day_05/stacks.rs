use crate::day_05::parsing;
use crate::day_05::instruction::Instruction;
use std::str::FromStr;

pub struct Stacks(Vec<Vec<char>>);

impl FromStr for Stacks {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = parsing::get_stack_width(s);
        let indices: Vec<usize> = (0..width).map(|x| 1 + (4 * x)).collect();
        let mut stacks: Vec<Vec<char>> = vec![vec![]; width];
        let split = s.split("\n");
        for line in split {
            for (i_stk, i_str) in indices.iter().enumerate() {
                let c: char = line.as_bytes()[*i_str].into();
                if c != ' ' {
                    stacks[i_stk].push(c);
                }
            }
        }
        for s in stacks.iter_mut() {
            s.reverse();
        }
        Ok(Self(stacks))
    }
}

impl Stacks {
    pub fn process(&mut self, inst: Instruction) {
        let height = self.0[inst.from].len();
        let remaining = height.saturating_sub(inst.amount);
        let mut moving: Vec<char> = self.0[inst.from].drain(remaining..).collect();
        self.0[inst.to].append(&mut moving);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_works() {
        // Arrange
        let inst = Instruction {
            from: 0,
            to: 1, 
            amount: 1,
        };
        let mut stacks = Stacks(vec![vec!['a', 'b', 'c'], vec!['d', 'e']]);

        // Act
        stacks.process(inst);


        // Assert
        assert_eq!(stacks.0[0][0], 'a');
        assert_eq!(stacks.0[1][2], 'c');
    }
}
