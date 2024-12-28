use self::matrix::base::MatrixBase;
use self::matrix::Matrix;
use crate::DynResult;
use crate::parsing::read_file_to_string;
use std::ascii::Char;
use std::convert::TryInto;
use std::path::Path;
use regex::Regex;

mod matrix;

pub fn solve_day4<P: AsRef<Path>>(path: P) -> DynResult<()> {
    println!(
        "Solving Day 4 Part 1 on the input {}",
        path.as_ref().to_str().unwrap()
    );
    let input: MatrixBase<Char> = read_file_to_string(path)?.try_into()?;
    let xmas_matcher = Regex::new("XMAS")?;
    let mut first_result = 0;
    for line in input.word_search() {
        let mut line: Vec<Char> = line.map(|i| *i).collect();
        first_result += xmas_matcher.find_iter(line.as_str()).count();
        line.reverse();
        first_result += xmas_matcher.find_iter(line.as_str()).count();
    }
    println!("The first result is: {first_result}");
    Ok(())
}
