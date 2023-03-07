use crate::day_05::parsing;
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
