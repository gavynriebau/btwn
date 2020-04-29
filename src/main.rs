use std::io::{BufReader, BufRead};
use clap::*;
use std::fs::File;

const ABOUT: &str = r#"
Filters lines based on the given range expression.

Examples:

'3'       - line 3 only
'2..6'    - lines 2 to 6 exclusive
'2...6'   - lines 2 to 6 inclusive
'3..'     - lines 3 onwards
'..4'     - lines 1 to 4 exclusive
"#;

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
    let matches = App::new("btwn - command line range filter tool")
        .about(ABOUT)
        .arg(Arg::with_name("input")
            .help("Input file (defaults to stdin)")
            .short("i")
            .long("input")
            .value_name("FILE"))
        .arg(Arg::with_name("range")
            .help("A range filter expression, e.g. '1..5'")
            .required(true))
        .get_matches();

    let (start, end) = parse_range(matches.value_of("range").unwrap().to_string());
    let reader: Box<dyn BufRead> = match matches.value_of("input") {
        Some(file_name) => Box::new(BufReader::new(File::open(file_name).unwrap())),
        None => Box::new(BufReader::new(std::io::stdin()))
    };

    for line in reader.lines().skip(start).take(end - start) {
        let text = line.unwrap();
        println!("{}", text);
    }
}
