pub struct InputParts {
    pub stacks: String,
    pub procedure: String,
}

pub fn split_input(inp: String) -> InputParts {
    let cut_point = inp.find("\n\n").expect("Malformed puzzle input");
    let (stacks, procedure) = inp.split_at(cut_point);
    let stacks = stacks.to_string();
    let procedure = procedure[2..].to_string();
    InputParts{
        stacks,
        procedure
    }
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
}
