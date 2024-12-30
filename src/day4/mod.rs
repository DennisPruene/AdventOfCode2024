use self::matrix::Matrix;
use self::matrix::base::MatrixBase;
use crate::DynResult;
use crate::parsing::read_file_to_string;
use regex::Regex;
use std::ascii::Char;
use std::convert::TryInto;
use std::path::Path;

pub mod matrix;

pub fn solve_day4<P: AsRef<Path>>(path: P) -> DynResult<()> {
    println!(
        "Solving Day 4 Part 1 on the input {}",
        path.as_ref().to_str().unwrap()
    );
    let input: MatrixBase<Char> = read_file_to_string(path)?.try_into()?;
    let xmas_matcher = Regex::new("XMAS")?;
    let mut first_result = 0;
    for line in input.word_search() {
        let mut line: Vec<Char> = line.collect();
        first_result += xmas_matcher.find_iter(line.as_str()).count();
        line.reverse();
        first_result += xmas_matcher.find_iter(line.as_str()).count();
    }
    println!("The first result is: {first_result}");

    println!("Solving Day 4 Part 2 on the same input");
    let second_result = input
        .convolve(3, 3, |m| {
            let down_diagonal: String = m.down_diagonal(0).unwrap().map(|c| c.to_char()).collect();
            let up_diagonal: String = m.up_diagonal(0).unwrap().map(|c| c.to_char()).collect();
            (down_diagonal == "MAS" || down_diagonal == "SAM")
                && (up_diagonal == "MAS" || up_diagonal == "SAM")
        })
        .count_non_zero();
    println!("The second result is: {second_result}");
    Ok(())
}
