use std::{collections::HashMap, fs::read_to_string};

fn _main() {
    let file_vector: Vec<String> = read_to_string("input")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut file_tuple: (Vec<i32>, Vec<i32>) = file_vector
        .iter()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap().parse::<i32>().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .unzip();

    fn part1(file_tuple: &mut (Vec<i32>, Vec<i32>)) -> i32 {
        file_tuple.0.sort();
        file_tuple.1.sort();

        file_tuple
            .0
            .iter()
            .zip(file_tuple.1.iter())
            .map(|(a, b)| (a - b).abs())
            .sum()
    }

    println!("Part 1 Answer : {}", part1(&mut file_tuple));

    fn part2(file_tuple: &(Vec<i32>, Vec<i32>)) -> i32 {
        let mut counts = HashMap::new();

        for &num in &file_tuple.1 {
            *counts.entry(num).or_insert(0) += 1;
        }

        let mut result = 0;
        for &num in &file_tuple.0 {
            if let Some(&count) = counts.get(&num) {
                result += num * count;
            }
        }

        result
    }

    println!("Part 2 Answer : {}", part2(&mut file_tuple));
}
