use std::collections::HashSet;

use crate::read_file;

fn look_for_packet(pkt: &str, window_size: usize) -> i32 {
    let stream: Vec<char> = pkt.chars().collect();

    let mut windows = stream.windows(window_size).enumerate();
    let res = loop {
        match windows.next() {
            None => {
                break -1
            },
            Some((idx,window)) => {
                let mut set:HashSet<char> = HashSet::new();
                for c in window{
                    set.insert(*c);
                }
                if set.len() == window_size {
                    break i32::try_from(idx).unwrap();
                }
            }
        }
    };
    res + i32::try_from(window_size).unwrap()
}

pub fn part_01(fname: &std::path::Path) -> String{
    let pkt = 
        read_file::read_to_string(fname.to_str().unwrap())
        .expect("File read error");

    let n = look_for_packet(&pkt,4);

    n.to_string()
}
  

pub fn part_02(fname: &std::path::Path) -> String{
    let pkt = 
        read_file::read_to_string(fname.to_str().unwrap())
        .expect("File read error");

    let n = look_for_packet(&pkt, 14);

    n.to_string()
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;

    #[test]
    fn test_part_01() {
        // Arrange
        let in_file = Path::new("test_input/06.input");
        let expect_file = "test_input/06a.expect";

        // Act
        let lhs = part_01(in_file);
        let rhs = read_file::get_lines(expect_file)
            .next()
            .unwrap()
            .unwrap();

        // Assert
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_part_01b() {
        //Arrange
        let pkt = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

        //Act
        let n = look_for_packet(pkt,4);

        //Assert
        assert_eq!(n, 10);
    }

    #[test]
    fn test_part_01c() {
        //Arrange
        let pkt = "bvwbjplbgvbhsrlpgdmjqwftvncz";

        //Act
        let n = look_for_packet(pkt,4);

        //Assert
        assert_eq!(n, 5);
    }


    #[test]
    fn test_part_02() {
        // Arrange
        let in_file = Path::new("test_input/06.input");
        let expect_file = "test_input/06b.expect";

        // Act
        let lhs = part_02(in_file);
        let rhs = read_file::get_lines(expect_file)
            .next()
            .unwrap()
            .unwrap();

        // Assert
        assert_eq!(lhs, rhs);
    }
}