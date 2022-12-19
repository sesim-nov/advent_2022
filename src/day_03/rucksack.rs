use std::collections::HashSet;

pub struct Rucksack {
    compartments: Vec<HashSet<char>>
}

impl Rucksack {
    pub fn get_common(&self) -> Vec<char> {
        self.compartments[0].intersection(&self.compartments[1]).copied().collect()
    }
}

impl From<&str> for Rucksack {
    fn from(s: &str) -> Self {
        let len = s.chars().count();
        let midpoint = len/2;
        let lhs = s[..midpoint].chars().collect();
        let rhs = s[midpoint..].chars().collect();
        Self{
            compartments: vec![lhs, rhs],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        //Arrange
        let test_str = "aq";

        //Act
        let sack = Rucksack::from(test_str);

        //Assert
        assert!(sack.compartments[0].contains(&'a'));
        assert!(sack.compartments[1].contains(&'q'));
    }

    #[test]
    fn test_intersect() {
        //Arrange
        let test_str = "abcarf";

        //Act
        let sack = Rucksack::from(test_str);
        let letter = sack.get_common();

        //Assert
        assert_eq!(letter[0], 'a');
    }
}
