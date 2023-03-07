use crate::day_05::problem;

pub struct InputParts {
    pub stacks: String,
    pub procedure: String,
}

pub fn split_input(inp: &str) -> InputParts {
    let cut_point = inp.find("\n\n").expect("Malformed puzzle input");
    let (stacks, procedure) = inp.split_at(cut_point);
    let stacks = stacks.to_string();
    let procedure = procedure[2..].to_string();
    InputParts { stacks, procedure }
}

pub fn get_stack_width(stk: &str) -> usize {
    let first_newline = stk.find("\n").expect("Malformed input");
    (first_newline + 1) / 4
}

pub fn convert_next_to_usize(r: Option<&str>) -> Result<usize, String> {
    let s = r.ok_or("Unpacking str value failed")?;
    s.parse::<usize>().map_err(|_| "Parsing string as usize failed.".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_input_splits_correctly() {
        // Arrange
        let a = "a\n\nb";

        // Act
        let b = split_input(a);

        // Assert
        assert_eq!(b.stacks, "a");
        assert_eq!(b.procedure, "b");
    }

    #[test]
    fn get_stack_width_works() {
        // Arrange
        let stk = r#"    [D]    \n\
                     [A] [N] [C]\n\
                     [Z] [M] [P]\n\
                      1   2   3 "#;

        // Act
        let res = get_stack_width(&stk.to_string());

        // Assert
        assert_eq!(res, 3);
    }

    #[test]
    fn convert_works() {
        // Arrange
        let a = Some("12");
        
        // Act
        let b = convert_next_to_usize(a);
        
        // Assert
        assert_eq!(b, Ok(12usize));
    }
}
