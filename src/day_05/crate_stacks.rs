use std::{collections::VecDeque, fs::File, io::{BufReader, Lines}};

#[derive(Debug)]
pub struct CrateStacks {
    pub stacks: Vec<VecDeque<char>>,
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

    pub fn process_cmd_9000(&mut self, cmd: MoveCommand) {
        self.process_cmd(cmd, process_fwd);
    }

    pub fn process_cmd_9001(&mut self, cmd: MoveCommand) {
        self.process_cmd(cmd, process_rev);
    }

    pub fn process_cmd(&mut self, cmd: MoveCommand, process_fn: fn(Vec<char>) -> Vec<char>) {
        let mut tmp_stack: Vec<char> = Vec::new();
        for _ in 0..cmd.count {
            tmp_stack.push(self.stacks[cmd.source - 1].pop_front().unwrap());
        }
        let altered_stack = process_fn(tmp_stack);
        for x in altered_stack {
            self.stacks[cmd.dest - 1].push_front(x);
        }
    }

    pub fn make_top_string(&self) -> String {
        self.stacks.iter().map(|x| -> char {
            x[0]
        }).collect()
    }
}

fn process_fwd(stack: Vec<char>) -> Vec<char> {
    stack
}

fn process_rev(stack: Vec<char>) -> Vec<char> {
    stack.into_iter().rev().collect()
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
        let stack_01 = VecDeque::from(vec!['A', 'B', 'C']);
        let stack_02 = VecDeque::from(vec!['D']);
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
        crate_stack.process_cmd_9000(command);
        let result: String = crate_stack.stacks[1].iter().collect();

        //Assert
        assert_eq!(result, "BAD".to_string());
    }

    #[test]
    fn test_move_command_2() {
        //Arrange
        let stack_01 = VecDeque::from(vec!['A', 'B', 'C']);
        let stack_02 = VecDeque::from(vec!['D']);
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
        crate_stack.process_cmd_9001(command);
        let result: String = crate_stack.stacks[1].iter().collect();

        //Assert
        assert_eq!(result, "ABD".to_string());
    }

    #[test]
    fn test_make_string() {
        //Arrange
        let stack_01 = VecDeque::from(vec!['C', 'B', 'D']);
        let stack_02 = VecDeque::from(vec!['U']);
        let stack_03 = VecDeque::from(vec!['M', 'L']);
        let stacks = vec![stack_01, stack_02, stack_03];
        let crate_stack = CrateStacks{
            stacks
        };

        //Act
        let result = crate_stack.make_top_string();

        //Assert
        assert_eq!(result, "CUM".to_string());
    }
}