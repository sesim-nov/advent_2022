use advent_2022::day_05::*;

fn main() {
    let path = std::path::Path::new("input/05.txt");
    let part_1 = part_01(path);
    let part_2 = part_02(path);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}