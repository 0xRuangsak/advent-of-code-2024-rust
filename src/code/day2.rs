use std::fs::read_to_string;
fn level_check(input: &Vec<isize>) -> bool {
    let is_increasing = input.windows(2).all(|w| w[0] < w[1]);
    let is_decreasing = input.windows(2).all(|w| w[0] > w[1]);
    is_decreasing || is_increasing
}

fn diff_check(input: &Vec<isize>) -> bool {
    input
        .windows(2)
        .all(|w| (1..=3).contains(&(w[1] - w[0]).abs()))
}

fn valid_check(input: &Vec<isize>) -> isize {
    if level_check(&input) && diff_check(&input) {
        1
    } else {
        0
    }
}

fn dampened_check(input: &Vec<isize>) -> isize {
    for i in 0..input.len() {
        let dampened_input = input
            .iter()
            .enumerate()
            .filter(|(j, _)| i != *j)
            .map(|(_, v)| *v)
            .collect();
        if valid_check(&dampened_input) == 1 {
            return 1;
        }
    }
    0
}
fn main() {
    let input_file_vector: Vec<String> = read_to_string("input")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let part1_result: isize = input_file_vector
        .iter()
        .map(|line| {
            let nums: Vec<isize> = line
                .split_whitespace()
                .map(|s| s.parse::<isize>().unwrap())
                .collect();
            valid_check(&nums)
        })
        .sum();

    println!("Part 1 Answer : {part1_result}");

    let part2_result: isize = input_file_vector
        .iter()
        .map(|line| {
            let nums: Vec<isize> = line
                .split_whitespace()
                .map(|s| s.parse::<isize>().unwrap())
                .collect();
            dampened_check(&nums)
        })
        .sum();

    println!("Part 2 Answer : {part2_result}");
}
