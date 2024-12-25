use crate::DynResult;
use crate::parsing::parse_integers;
use std::cmp::Ordering;
use std::path::Path;

pub fn solve_day2<P: AsRef<Path>>(path: P) -> DynResult<()> {
    println!(
        "Solving Day 2 Part 1 on the input {}",
        path.as_ref().to_str().unwrap()
    );
    let input = parse_integers(path)?;
    let mut safe_report_count = 0;
    for report in input.iter() {
        if is_safe(report) {
            safe_report_count += 1;
        }
    }
    println!("The number of safe reports is: {}", safe_report_count);

    println!("Solving Day 2 Part 2 on the same input");
    let mut second_result = 0;
    for report in input.iter() {
        if is_safe_with_dampener(report) {
            second_result += 1;
        }
    }
    println!("The solution is: {}", second_result);
    Ok(())
}

fn is_safe(report: &[i32]) -> bool {
    match report[0].cmp(&report[1]) {
        Ordering::Less => is_iterator_safe(report.iter()),
        Ordering::Greater => is_iterator_safe(report.iter().rev()),
        Ordering::Equal => false,
    }
}

fn is_iterator_safe<'a, T: Iterator<Item = &'a i32> + Clone>(report: T) -> bool {
    for (cur, prev) in report.clone().skip(1).zip(report) {
        let step = cur - prev;
        if step < 1 || step > 3 {
            return false;
        }
    }
    true
}

fn is_safe_with_dampener(report: &Vec<i32>) -> bool {
    if is_safe(report) {
        return true;
    }
    for i in 0..report.len() {
        let mut report_clone = report.clone();
        report_clone.remove(i);
        if is_safe(&report_clone) {
            return true;
        }
    }
    false
}
