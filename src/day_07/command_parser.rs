
#[derive(Debug)]
pub enum Action<T> {
    /// If the next iteration returns None, return value passed as T
    Request(T),

    /// Require more information. Return None if next cycle returns None
    Require,

    /// Immediately return T
    Return(T),
}

struct CommandParser {
    cursor: usize,
    array: Vec<char>,
}

impl CommandParser{
    pub fn new(cmd: &str) -> Self {
        Self{
            cursor: 0,
            array: cmd.chars().collect()
        }
    }

    pub fn peek(&self) -> Option<&char> {
        self.array.get(self.cursor)
    }

    pub fn pop(&mut self) -> Option<&char> {
        //Why can't i use peek here without pissing off the borrow checker? 
        match self.array.get(self.cursor) {
            None => None,
            Some(c) => {
                self.cursor += 1;
                Some(c)
            }
        }
    }

    pub fn take(&mut self, target: &char) -> bool {
        match self.peek() {
            Some(c) => {
                if c == target {
                    self.cursor += 1;
                    true
                } else {
                    false
                }
            }
            None => false
        }
    }

    ///Get remainder of parser contents.
    pub fn get_remainder(&self) -> Option<String>{
        let arr: String = self.array.get(self.cursor..self.array.len())?
            .into_iter()
            .collect();
        Some(arr)
    }

    pub fn scan<T,F>(&mut self, f: F) -> Option<T> where
        F: Fn(&str) -> Option<Action<T>>  {
        let mut sequence = String::new();
        let mut require = false;
        let mut request_val = None;

        loop {
            match self.array.get(self.cursor) {
                Some(next_char) => {    
                    sequence.push(*next_char);
                    self.cursor += 1;        
                    match f(&sequence) {
                        Some(Action::Request(r)) => {
                            //on the next loop, if we return None, return T
                            require = false;
                            request_val = Some(r);
                        },
                        Some(Action::Require) => {
                            require = true;
                            request_val = None;
                        }
                        Some(Action::Return(ret)) => {
                            break Some(ret)
                        }
                        None => {
                            self.cursor -= 1;
                            if !require {
                                break request_val
                            } else {
                                break None
                            }
                        }
                    }
                },
                None => break None
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum DirCommand{
    ChangeDir(String),
    ParentDir,
    AddFile(String),
    AddDirectory(String),
    DoNothing,
}

#[derive(Debug)]
enum CommandName {
    Cd,
    Ls,
    Dir,
    File,
}

pub fn parse_cmd(cmd: &str) -> Result<DirCommand, &str> {
    let mut parser = CommandParser::new(cmd);
    let phase_01 = parser.scan(|x| -> Option<Action<CommandName>> {
        if x.chars().next().unwrap().is_digit(10) {
            Some(Action::Return(CommandName::File))
        } else {
            match x {
                "$" => Some(Action::Require),
                "$ " => Some(Action::Require),
                "$ c" => Some(Action::Require),
                "$ cd" => Some(Action::Require),
                "$ cd " => Some(Action::Return(CommandName::Cd)),
                "$ l" => Some(Action::Require),
                "$ ls" => Some(Action::Return(CommandName::Ls)),
                "d" => Some(Action::Require),
                "di" => Some(Action::Require),
                "dir" => Some(Action::Require),
                "dir " => Some(Action::Return(CommandName::Dir)),
                _ => None,
            }
        }
    });
    let phase_02 = match phase_01 {
        Some(CommandName::Cd) => {
            let remainder = parser.get_remainder().ok_or("Failed to get remainder")?;
            Ok(DirCommand::ChangeDir(remainder))
        },
        Some(CommandName::Dir) => {
            let remainder = parser.get_remainder().ok_or("Failed to get remainder")?;
            Ok(DirCommand::AddDirectory(remainder))
        },
        Some(CommandName::Ls) => {
            Ok(DirCommand::DoNothing)
        },
        Some(CommandName::File) => {
            Ok(DirCommand::AddFile(cmd.to_string()))
        },
        None => Err("Phase 02 parsing failed")
    };
    // println!("{:?}", phase_01);
    println!("{:?}", phase_02);
    Ok(DirCommand::DoNothing)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan() {
        //Arrange
        let cmd = "find";
        let cmd_2 = "finf";
        let mut parser = CommandParser::new(cmd);
        let mut parser_2 = CommandParser::new(cmd_2);
        fn scan_fn(x: &str) -> Option<Action<String>> {
            match x {
                "f" => Some(Action::Require),
                "fi" => Some(Action::Require),
                "fin" => Some(Action::Request("Derp".to_string())),
                "find" => Some(Action::Return("done".to_string())),
                _ => None,
            }
        }
        //Act
        let rhs = parser.scan(scan_fn);
        let rhs_2 = parser_2.scan(scan_fn);

        //Assert
        assert_eq!(Some("done".to_string()), rhs);
        assert_eq!(Some("Derp".to_string()), rhs_2);
    }
}