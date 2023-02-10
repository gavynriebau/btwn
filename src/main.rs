use std::io::{BufReader, BufRead, Write};
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

fn parse_inclusive_range(range: &str) -> (usize, usize) {
    let parts: Vec<String> = range.split("...")
        .map(|s| String::from(s))
        .collect();

    let mut start = usize::min_value();
    let mut end = usize::max_value();

    if !parts[0].is_empty() {
        start = parts[0].parse().expect("Failed to parse range");
    }
    if !parts[1].is_empty() {
        end = parts[1].parse().expect("Failed to parse range");
    }

    (start, end)
}

fn parse_exclusive_range(range: &str) -> (usize, usize) {
    let parts: Vec<String> = range.split("..")
        .map(|s| String::from(s))
        .collect();

    let mut start = usize::min_value();
    let mut end = usize::max_value();

    if !parts[0].is_empty() {
        start = parts[0].parse().expect("Failed to parse range");
    }
    if !parts[1].is_empty() {
        end = parts[1].parse().expect("Failed to parse range");
        end -= 1;
    }

    (start, end)
}

fn parse_range(range: &str) -> (usize, usize) {
    if range.contains("...") {
        return parse_inclusive_range(range);
    }

    if range.contains("..") {
        return parse_exclusive_range(range);
    }

    let value: usize = range.parse().expect("Failed to parse range");
    (value, value)
}

fn get_reader_from_filename(filename: Option<&str>) -> Box<dyn BufRead> {
    let reader: Box<dyn BufRead> = match filename {
        Some(filename) => Box::new(BufReader::new(File::open(filename).unwrap())),
        None => Box::new(BufReader::new(std::io::stdin()))
    };

    reader
}

fn process_range(range_arg: &str, reader: Box<dyn BufRead>, writer: &mut dyn Write) -> std::io::Result<()> {
    let (start, end) = parse_range(range_arg);

    let skip = start.checked_sub(1).unwrap_or(0);

    let text = reader.lines()
        .skip(skip)
        .take(end - skip)
        .filter_map(|x| x.ok())
        .collect::<Vec<_>>()
        .as_slice()
        .join("\n");

    writer.write_fmt(format_args!("{}\n", text))
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

    let range_arg: &str = matches.value_of("range").unwrap();
    let file_name: Option<&str> = matches.value_of("input");

    let reader = get_reader_from_filename(file_name);
    let mut writer: Box<dyn Write> = Box::new(std::io::stdout());

    process_range(range_arg, reader, &mut writer).unwrap();
}

#[cfg(test)]
mod tests {

    mod range_parsing {
        use crate::parse_range;

        #[test]
        fn it_parses_inclusive_ranges_correctly() {
            let range: &str = "1...5";

            let (start, end) = parse_range(range);

            assert_eq!(start, 1);
            assert_eq!(end, 5);
        }

        #[test]
        fn it_parses_exclusive_ranges_correctly() {
            let range: &str = "1..5";

            let (start, end) = parse_range(range);

            assert_eq!(start, 1);
            assert_eq!(end, 4);
        }
    }

    mod text_printing {
        use std::io::{Cursor, Read, BufRead, Seek, SeekFrom};

        use crate::process_range;

        #[test]
        fn it_prints_inclusive_ranges_correctly() {
            let range: &str = "2...5";
            let reader: Box<dyn BufRead> = Box::new(r#"a
b
c
d
e
f
g"#.as_bytes());

            let mut writer = Box::new(Cursor::new(vec![0_u8; 0]));

            process_range(range, reader, &mut writer).unwrap();

            let mut output = String::new();

            writer.seek(SeekFrom::Start(0)).unwrap();
            writer.read_to_string(&mut output).unwrap();

            assert_eq!(output, r#"b
c
d
e
"#);
        }

        #[test]
        fn it_prints_exclusive_ranges_correctly() {
            let range: &str = "2..5";
            let reader: Box<dyn BufRead> = Box::new(r#"a
b
c
d
e
f
g"#.as_bytes());

            let mut writer = Box::new(Cursor::new(vec![0_u8; 0]));

            process_range(range, reader, &mut writer).unwrap();

            let mut output = String::new();

            writer.seek(SeekFrom::Start(0)).unwrap();
            writer.read_to_string(&mut output).unwrap();

            assert_eq!(output, r#"b
c
d
"#);
        }

        #[test]
        fn it_prints_single_line_correctly() {
            let range: &str = "4";
            let reader: Box<dyn BufRead> = Box::new(r#"a
b
c
d
e
f
g"#.as_bytes());

            let mut writer = Box::new(Cursor::new(vec![0_u8; 0]));

            process_range(range, reader, &mut writer).unwrap();

            let mut output = String::new();

            writer.seek(SeekFrom::Start(0)).unwrap();
            writer.read_to_string(&mut output).unwrap();

            assert_eq!(output, r#"d
"#);
        }

        // TODO: Add tests for open/closed start/end ranges
        
    }

    

}
