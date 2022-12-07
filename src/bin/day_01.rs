use advent_2022::read_file;
use std::collections::VecDeque;
use std::cmp::max;

fn main() {
    let in_file = "input/01.txt";
    let ans_01 = part_01(in_file);
    let ans_02 = part_02(in_file);

    println!("Part 1: {}", ans_01);
    println!("Part 2: {}", ans_02);
}

fn part_01(f: &str) -> usize {
    let file_lines = read_file::get_lines(f);
    let mut max_val = 0;
    let mut running_val = 0;
    for line in file_lines {
        let num_str = line.expect("Failed to read input file.");
        if num_str.is_empty() {
            max_val = max(max_val, running_val);
            running_val = 0;
            continue;
        }
        running_val += num_str.parse::<usize>().expect("Failed to parse number.");
    }
    max_val
}

fn part_02(f: &str) -> usize {
    let file_lines = read_file::get_lines(f);
    let mut max_vals = VecDeque::from([0,0,0]);
    let mut running_val = 0;
    for line in file_lines {
        let num_str = line.expect("Failed to read input file.");
        if num_str.is_empty() {
            update_vec(&mut max_vals, running_val);
            running_val = 0;
        } else {
            running_val += num_str.parse::<usize>().expect("Failed to parse number.");
        }
    }
    update_vec(&mut max_vals, running_val);

    max_vals.iter().sum()
}

fn update_vec (v: &mut VecDeque<usize>, new_value: usize) {
    let insert_point = match v.binary_search(&new_value) {
        Ok(o) => o,
        Err(e) => e,
    };
    v.insert(insert_point, new_value);
    v.pop_front();
}

#[cfg(test)]
mod day_01 {
    use super::*;

    #[test]
    fn test_part_1() {
        // Arrange
        let in_file = "test_input/01.input";
        let expect_file = "test_input/01a.expect";

        // Act
        let lhs = part_01(in_file);
        let rhs = read_file::get_lines(expect_file)
            .next()
            .unwrap()
            .unwrap()
            .parse()
            .unwrap();

        // Assert
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_part_2() {
        // Arrange
        let in_file = "test_input/01.input";
        let expect_file = "test_input/01b.expect";

        // Act
        let lhs = part_02(in_file);
        let rhs = read_file::get_lines(expect_file)
            .next()
            .unwrap()
            .unwrap()
            .parse()
            .unwrap();

        // Assert
        assert_eq!(lhs, rhs);
    }
}
