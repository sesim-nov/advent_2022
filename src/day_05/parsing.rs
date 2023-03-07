use crate::day_05::problem;

pub struct InputParts {
    pub stacks: String,
    pub procedure: String,
}

pub fn split_input(inp: String) -> InputParts {
    let cut_point = inp.find("\n\n").expect("Malformed puzzle input");
    let (stacks, procedure) = inp.split_at(cut_point);
    let stacks = stacks.to_string();
    let procedure = procedure[2..].to_string();
    InputParts { stacks, procedure }
}

pub fn get_stack_width(stk: &String) -> usize {
    let first_newline = stk.find("\n").expect("Malformed input");
    (first_newline + 1) / 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_input_splits_correctly() {
        // Arrange
        let a = "a\n\nb".to_string();

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
}
