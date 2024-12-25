#![allow(dead_code)]

use day3::solve_day3;
use parsing::parse_integers;
use std::error::Error;
use std::path::Path;

mod day2;
mod day3;
mod parsing;

pub type DynResult<T> = Result<T, Box<dyn Error>>;

fn main() -> DynResult<()> {
    solve_day3("input/day3/input.txt")?;
    Ok(())
}

fn solve_day1<P: AsRef<Path>>(path: P) -> DynResult<()> {
    println!(
        "Solving Day 1 Part 1 on the input {}",
        path.as_ref().to_str().unwrap()
    );
    let input = parse_integers(path)?;
    let mut left_list: Vec<i32> = input.iter().map(|ints| ints[0]).collect();
    left_list.sort();
    let mut right_list: Vec<i32> = input.iter().map(|ints| ints[1]).collect();
    right_list.sort();
    let mut first_result = 0;
    for (left, right) in left_list.iter().zip(right_list.iter()) {
        if left < right {
            first_result += right - left;
        } else {
            first_result += left - right;
        }
    }
    println!("The solution is: {}", first_result);

    println!("Solving Day 1 Part 2 on the same input");
    let mut second_result = 0;
    for left_entry in left_list.iter() {
        let right_occurrences = right_list.iter().filter(|r| *r == left_entry).count();
        second_result += left_entry * right_occurrences as i32;
    }
    println!("The solution is: {}", second_result);
    Ok(())
}
