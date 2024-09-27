use advent_2022::day_07::*;

fn main() {
    let path = std::path::Path::new("input/07.txt");
    let ans = solve(path);
    //let part_2 = part_02(path);

    println!("Part 1: {}", ans.pt_1);
    println!("Part 2: {}", ans.pt_2);
}