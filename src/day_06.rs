use crate::read_file;

fn look_for_packet(pkt: &str) -> i32 {
    let stream: Vec<char> = pkt.chars().collect();

    let mut packet_stack: Vec<char> = Vec::new();
    let mut n = -1;
    
    //TODO: This method is wrong. I don't need to discard the stack when the check fails. I just need to keep looking. Need to switch to std::slice::Windows. 
    for c in stream {
        n += 1;
        if packet_stack.iter().all(|x| *x != c) {
            packet_stack.push(c);
        } else {
            packet_stack.clear();
        }

        if packet_stack.len() > 3 {
            break;
        }
    }
    n
}

pub fn part_01(fname: &std::path::Path) -> String{
    let pkt = 
        read_file::read_to_string(fname.to_str().unwrap())
        .expect("File read error");

    let n = look_for_packet(&pkt);

    n.to_string()
}
  

pub fn part_02(fname: &std::path::Path) -> String{
    "stub".to_string()
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
        let n = look_for_packet(pkt);

        //Assert
        assert_eq!(n, 10);
    }

    #[test]
    fn test_part_01c() {
        //Arrange
        let pkt = "bvwbjplbgvbhsrlpgdmjqwftvncz";

        //Act
        let n = look_for_packet(pkt);

        //Assert
        assert_eq!(n, 5);
    }


    // #[test]
    // fn test_part_02() {
    //     // Arrange
    //     let in_file = Path::new("test_input/06.input");
    //     let expect_file = "test_input/06b.expect";

    //     // Act
    //     let lhs = part_02(in_file);
    //     let rhs = read_file::get_lines(expect_file)
    //         .next()
    //         .unwrap()
    //         .unwrap();

    //     // Assert
    //     assert_eq!(lhs, rhs);
    // }
}