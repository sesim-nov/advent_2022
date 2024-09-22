
#[derive(Debug, PartialEq)]
enum DirCommand{
    ChangeDir(String),
    ParentDir,
    AddFile(String),
    AddDirectory(String),
    DoNothing,
}

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

// fn parse_cmd(cmd: &str) -> Result<DirCommand, String> {
//     let split_cmd: Vec<&str> = cmd.split_whitespace().collect();
//     match split_cmd[0] {
//         "$" => match split_cmd[1] {
//             "cd" => {
//                 match split_cmd.get(2) {
//                     Some(dest) => Ok(DirCommand::ChangeDir(dest)),
//                     None => Err("No destination provided".to_string())
//                 }
//             },
//             "ls" => Ok(DirCommand::DoNothing),
//             _ => Err("Unknown Command".to_string()),
//         },
//         "dir" => DirCommand::AddDirectory(split_cmd[1].to_string()),
//         _ => DirCommand::AddFile(split_cmd[1].to_string()),
//     }
// }
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