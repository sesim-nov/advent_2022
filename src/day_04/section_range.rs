use std::str::FromStr;

pub struct SectionRange {
    start: usize,
    end: usize,
}

impl FromStr for SectionRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = Vec::from_iter(s.chars());
        let start: usize = s[0].to_digit(10).unwrap().try_into().unwrap();
        let end: usize = s[2].to_digit(10).unwrap().try_into().unwrap();
        Ok(Self { start, end })
    }
}

impl SectionRange {
    fn contains(&self, other: &SectionRange) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_section_range() {
        //Arrange
        let s = "1-547";

        //Act
        let sr = SectionRange::from_str(s).unwrap();

        //Assert
        assert_eq!(sr.start, 1);
        assert_eq!(sr.end, 547);
    }

    #[test]
    fn test_contains_check() {
        //Arrange
        let sa = "1-5";
        let sb = "2-5";
        let sc = "9-24";

        //Act
        let rng_a = SectionRange::from_str(sa).unwrap();
        let rng_b = SectionRange::from_str(sb).unwrap();
        let rng_c = SectionRange::from_str(sc).unwrap();

        //Assert
        assert!(rng_a.contains(&rng_b));
        assert!(!rng_b.contains(&rng_a));
        assert!(!rng_b.contains(&rng_c));
        assert!(!rng_a.contains(&rng_c));
    }
}
