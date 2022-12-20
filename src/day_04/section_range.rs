use std::str::FromStr;

pub struct SectionRange {
    start: usize,
    end: usize
}

impl FromStr for SectionRange {
    type Err = String; 

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = Vec::from_iter(s.chars());
        let start: usize = s[0].to_digit(10).unwrap().try_into().unwrap();
        let end: usize = s[2].to_digit(10).unwrap().try_into().unwrap();
        Ok(Self{
            start,
            end,
        })
    }
}
