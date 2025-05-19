use regex::Regex;
use std::fs;

fn read_file(input_name: String) -> String {
    fs::read_to_string(input_name).expect("Failed to read input file")
}

fn pattern_match(str_input: String) -> Vec<(isize, isize)> {
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    pattern
        .captures_iter(&str_input)
        .filter_map(|cap| {
            let x = cap[1].parse::<isize>().ok()?;
            let y = cap[2].parse::<isize>().ok()?;
            Some((x, y))
        })
        .collect()
}

fn pattern_match_with_condition(str_input: String) -> Vec<(isize, isize)> {
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    let mut result = Vec::new();

    for cap in pattern.find_iter(&str_input) {
        let text = cap.as_str();
        match text {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ if text.starts_with("mul(") && enabled => {
                let inner = &text[4..text.len() - 1]; // extract "X,Y"
                let mut parts = inner.split(',');
                if let (Some(x), Some(y)) = (parts.next(), parts.next()) {
                    if let (Ok(a), Ok(b)) = (x.parse(), y.parse()) {
                        result.push((a, b));
                    }
                }
            }
            _ => {}
        }
    }

    result
}

fn multiply_sum(result_vector: Vec<(isize, isize)>) -> isize {
    result_vector.iter().map(|(x, y)| x * y).sum()
}

fn main() {
    let result = multiply_sum(pattern_match_with_condition(read_file("input".to_string())));
    println!("{result}");
}
