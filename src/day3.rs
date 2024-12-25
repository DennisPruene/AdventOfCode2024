use crate::DynResult;
use crate::parsing::process_input_for_matches;
use once_cell::sync::Lazy;
use regex::Regex;
use std::path::Path;

const MUL_INSTRUCTION: &str = r"mul\((\d*),(\d*)\)";
static MUL_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(MUL_INSTRUCTION).unwrap());
const DO_INSTRUCTION: &str = r"do\(\)";
static DO_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(DO_INSTRUCTION).unwrap());
const DONT_INSTRUCTION: &str = r"don't\(\)";
static DONT_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(DONT_INSTRUCTION).unwrap());

pub fn solve_day3<P: AsRef<Path>>(path: P) -> DynResult<()> {
    println!(
        "Solving Day 3 Part 1 on the input {}",
        path.as_ref().to_str().unwrap()
    );
    let input = process_input_for_matches(&path, MUL_INSTRUCTION)?;
    let mut first_result = 0;
    for instruction in input.iter() {
        first_result += compute_multiplication(instruction)?;
    }
    println!("The solution is: {}", first_result);

    println!("Solving Day 3 Part 2 on the same input");
    let input = process_input_for_matches(
        path,
        &format!(
            "{}|{}|{}",
            MUL_INSTRUCTION, DO_INSTRUCTION, DONT_INSTRUCTION
        ),
    )?;
    let mut second_result = 0;
    let mut is_mul_enabled = true;
    for instruction in input.iter() {
        if DO_MATCHER.is_match(instruction) {
            is_mul_enabled = true;
        } else if DONT_MATCHER.is_match(instruction) {
            is_mul_enabled = false;
        } else if is_mul_enabled {
            if MUL_MATCHER.is_match(instruction) {
                second_result += compute_multiplication(instruction)?;
            }
        }
    }
    println!("The solution is: {}", second_result);
    Ok(())
}

fn compute_multiplication(instruction: &str) -> DynResult<i32> {
    let captures = MUL_MATCHER.captures(instruction).unwrap();
    let first_number: i32 = captures.get(1).unwrap().as_str().parse()?;
    let second_number: i32 = captures.get(2).unwrap().as_str().parse()?;
    Ok(first_number * second_number)
}
