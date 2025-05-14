use std::collections::HashMap;
use std::fs::read_to_string;

fn read_lines(file: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for line in read_to_string(file).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

fn load_file(string_vector: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();

    for element in string_vector.iter() {
        let parts: Vec<&str> = element.split_whitespace().collect();
        vec1.push(parts[0].parse::<i32>().unwrap());
        vec2.push(parts[1].parse::<i32>().unwrap());
    }

    (vec1, vec2)
}

pub fn solve_day1_part1() -> i32 {
    let lines: Vec<String> = read_lines("src/input/day1.txt");
    let (mut left, mut right) = load_file(lines);

    left.sort();
    right.sort();

    let result: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    result
}

pub fn solve_day1_part2() -> i32 {
    let lines: Vec<String> = read_lines("src/input/day1.txt");
    let (left, right) = load_file(lines);

    let mut counts = HashMap::new();
    for num in right {
        *counts.entry(num).or_insert(0) += 1;
    }

    let mut result: i32 = 0;
    for num in left {
        let count = counts.get(&num).unwrap_or(&0);
        result += num * *count;
    }
    result
}
