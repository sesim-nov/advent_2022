use std::{collections::VecDeque, fs::File, io::{BufReader, Lines}};

#[derive(Debug)]
pub struct CrateStacks {
    stacks: Vec<VecDeque<char>>,
}

impl CrateStacks {
    pub fn from_lines(lines: &mut Lines<BufReader<File>>) -> Self {
        let mut this_line: Vec<char> = lines
            .next()
            .unwrap()
            .unwrap()
            .chars()
            .collect();
        let num_stacks = (this_line.len() + 1) / 4;
        let indices: Vec<usize> = (0..num_stacks).map(|x| -> usize {
            1 + (x * 4)
        }).collect();
        let mut stacks = vec![VecDeque::<char>::new(); num_stacks];

        while this_line.len() > 1 {
            for (i,j) in indices.iter().enumerate() {
                match this_line[*j]{
                    ' ' => (),
                    c => stacks[i].push_back(c),
                }
            }
            this_line = lines
            .next()
            .unwrap()
            .unwrap()
            .chars()
            .collect();
        }

        //The above also captures the column labels. Delete them. 
        for i in stacks.iter_mut() {
            i.pop_back();
        }

        Self { 
            stacks
        }
    }

    fn process_cmd(&mut self, cmd: MoveCommand) {
        let mut tmp_stack: Vec<char> = Vec::new();
        for _ in 0..cmd.count {
            tmp_stack.push(self.stacks[cmd.source - 1].pop_front().unwrap());
        }
        let mut reversed: VecDeque<char> = tmp_stack.into_iter().rev().collect();
        self.stacks[cmd.dest - 1].append(&mut reversed)
    }
}

pub struct MoveCommand{
    pub count: usize,
    pub source: usize,
    pub dest: usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_command() {
        //Arrange
        let stack_01 = VecDeque::from(vec!['A', 'B']);
        let stack_02 = VecDeque::from(vec!['C']);
        let stacks = vec![stack_01, stack_02];
        let mut crate_stack = CrateStacks{
            stacks
        };
        let command = MoveCommand {
            count: 2,
            source: 1,
            dest: 2,
        };

        //Act
        crate_stack.process_cmd(command);

    }
}