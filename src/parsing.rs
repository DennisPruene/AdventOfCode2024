use crate::DynResult;
use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_file_to_string<P: AsRef<Path>>(path: P) -> DynResult<String> {
    let mut file = File::open(path)?;
    let mut result = String::new();
    file.read_to_string(&mut result)?;
    Ok(result)
}

pub fn parse_integers<P: AsRef<Path>>(path: P) -> DynResult<Vec<Vec<i32>>> {
    let integer_matcher = Regex::new("[+-]?[1-9][0-9]*").unwrap();

    let lines = read_file_to_string(path)?;
    let mut result = vec![];
    for line in lines.lines() {
        let parsed_line: Vec<i32> = integer_matcher
            .find_iter(line)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        result.push(parsed_line);
    }
    Ok(result)
}

pub fn process_input_for_matches<P: AsRef<Path>>(path: P, pattern: &str) -> DynResult<Vec<String>> {
    let input = read_file_to_string(path)?;
    let pattern_matcher = Regex::new(pattern)?;
    let result = pattern_matcher
        .find_iter(&input)
        .map(|m| m.as_str().to_string())
        .collect();
    Ok(result)
}
