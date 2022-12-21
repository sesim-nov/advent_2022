use std::str::FromStr;

pub struct SectionRange {
    start: usize,
    end: usize,
}

impl FromStr for SectionRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let spl: Vec<&str> = s.split('-').collect();
        if spl.len() != 2 {
            return Err("Incorrect input".into());
        }
        let start: usize = spl[0].parse().unwrap();
        let end: usize = spl[1].parse().unwrap();
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
        let p = "1-3-4";

        //Act
        let sr = SectionRange::from_str(s).unwrap();
        let sr_err = SectionRange::from_str(p);

        //Assert
        assert_eq!(sr.start, 1);
        assert_eq!(sr.end, 547);
        assert!(sr_err.is_err());
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
