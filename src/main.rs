use structopt::StructOpt;
use std::io::{BufReader, BufRead};

const ABOUT: &str = r#"
Filters lines based on the given range expression.

Examples:

'3'       - line 3 only
'2..6'    - lines 2 to 6 exclusive
'2...6'   - lines 2 to 6 inclusive
'3..'     - lines 3 onwards
'..4'     - lines 1 to 4 exclusive
"#;


#[derive(StructOpt, Debug)]
#[structopt(
    name = "btwn",
    about = ABOUT
)]
struct Arguments {
    /// A range filter expression, e.g. '1..5'
    #[structopt(short, long)]
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
    let args = Arguments::from_args();
    let (start, end) = parse_range(args.range);
    let reader = BufReader::new(std::io::stdin());

    for line in reader.lines().skip(start).take(end - start) {
        let text = line.unwrap();
        println!("{}", text);
    }
}
