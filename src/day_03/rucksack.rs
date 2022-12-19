use std::collections::HashSet;

pub struct Rucksack {
    compartments: Vec<HashSet<char>>
}

impl Rucksack {
    pub fn get_common(&self) -> Vec<char> {
        self.compartments[0].intersection(&self.compartments[1]).copied().collect()
    }
    pub fn get_all_contents(&self) -> HashSet<char> {
        self.compartments[0].union(&self.compartments[1]).copied().collect()
    }
}

impl From<String> for Rucksack {
    fn from(s: String) -> Self {
        let len = s.chars().count();
        let midpoint = len/2;
        let lhs = s[..midpoint].chars().collect();
        let rhs = s[midpoint..].chars().collect();
        Self{
            compartments: vec![lhs, rhs],
        }
    }
}

fn intersect_all(a: Vec<Rucksack>) -> char {
    let hashes: Vec<HashSet<char>> = a.iter().map(|x| x.get_all_contents()).collect();
    hashes.into_iter().reduce(|acc, e| {
        acc.intersection(&e).copied().collect()
    })
        .unwrap()
        .iter()
        .next()
        .copied()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        //Arrange
        let test_str = "aq".to_string();

        //Act
        let sack = Rucksack::from(test_str);

        //Assert
        assert!(sack.compartments[0].contains(&'a'));
        assert!(sack.compartments[1].contains(&'q'));
    }

    #[test]
    fn test_intersect() {
        //Arrange
        let test_str = "abcarf".to_string();

        //Act
        let sack = Rucksack::from(test_str);
        let letter = sack.get_common();

        //Assert
        assert_eq!(letter[0], 'a');
    }

    #[test]
    fn test_get_all() {
        //Arrange
        let test_str = "abcarf".to_string();

        //Act
        let sack = Rucksack::from(test_str);
        let all_contents = sack.get_all_contents();

        //Assert
        assert!(all_contents.contains(&'a'));
        assert!(all_contents.contains(&'b'));
        assert!(all_contents.contains(&'c'));
        assert!(all_contents.contains(&'f'));
        assert!(all_contents.contains(&'r'));
        
    }

    #[test]
    fn test_intersect_all() {
        //Arrange
        let test_strs: Vec<String> = vec!["abc", "ade", "afg"].iter().map(|x| x.to_string()).collect();
        
        //Act
        let sacks: Vec<Rucksack> = test_strs.into_iter().map(|x| Rucksack::from(x)).collect();
        let inter = intersect_all(sacks);

        //Assert
        assert_eq!(inter, 'a');
    }
}
