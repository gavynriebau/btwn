use argh::FromArgs;
use std::io::{BufReader, BufRead};

#[derive(FromArgs)]
#[argh(
    description = "Filters lines based on the given range expression",
    example = "'3' - line 3 only\n'2..6' - lines 2 to 6 exclusive\n'2...6' - lines 2 to 6 inclusive\n'3..' - lines 3 onwards\n'..4' - lines 1 to 4 exclusive"
)]
struct Arguments {

    #[argh(positional)]
    range: String
}

fn parse_inclusive_range(range: String) -> (usize, usize) {
    let parts: Vec<String> = range.split("...")
        .map(|s| String::from(s))
        .collect();

    let mut start = usize::min_value();
    let mut end = usize::max_value();

    if !parts[0].is_empty() {
        start = parts[0].parse().expect("Failed to parse range");
        start -= 1;
    }
    if !parts[1].is_empty() {
        end = parts[1].parse().expect("Failed to parse range");
    }

    (start, end)
}

fn parse_exclusive_range(range: String) -> (usize, usize) {
    let parts: Vec<String> = range.split("..")
        .map(|s| String::from(s))
        .collect();

    let mut start = usize::min_value();
    let mut end = usize::max_value();

    if !parts[0].is_empty() {
        start = parts[0].parse().expect("Failed to parse range");
        start -= 1;
    }
    if !parts[1].is_empty() {
        end = parts[1].parse().expect("Failed to parse range");
        end -= 1;
    }

    (start, end)
}

fn parse_range(range: String) -> (usize, usize) {
    if range.contains("...") {
        return parse_inclusive_range(range);
    }

    if range.contains("..") {
        return parse_exclusive_range(range);
    }

    let value: usize = range.parse().expect("Failed to parse range");
    (value-1, value)
}

fn main() {
    let args: Arguments = argh::from_env();
    let (start, end) = parse_range(args.range);
    let reader = BufReader::new(std::io::stdin());

    for line in reader.lines().skip(start).take(end - start) {
        let text = line.unwrap();
        println!("{}", text);
    }
}
