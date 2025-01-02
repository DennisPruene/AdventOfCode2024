use crate::DynResult;
use once_cell::sync::Lazy;
use std::ascii::Char;
use std::collections::HashMap;
use std::path::Path;
use self::patrol_tracker::StepResult;
use self::patrol_tracker::PatrolTracker;
use crate::parsing::read_file_to_char_matrix;

mod patrol_tracker;

static CLOCKWISE_ROTATION: Lazy<HashMap<Char, Char>> = Lazy::new(|| {
    [
        ('^'.as_ascii().unwrap(), '>'.as_ascii().unwrap()),
        ('>'.as_ascii().unwrap(), 'v'.as_ascii().unwrap()),
        ('v'.as_ascii().unwrap(), '<'.as_ascii().unwrap()),
        ('<'.as_ascii().unwrap(), '^'.as_ascii().unwrap()),
    ]
    .into()
});

pub fn solve_day6<P: AsRef<Path>>(path: P) -> DynResult<()> {
    println!(
        "Solving Day 6 on the input {}",
        path.as_ref().to_str().unwrap()
    );

    let map = read_file_to_char_matrix(path)?;
    let mut patrol_tracker = PatrolTracker::new(map.clone());
    while patrol_tracker.take_step(true) == StepResult::KeepGoing {}
    println!("The solution to part 1 is: {}", patrol_tracker.get_map().iter().filter(|c| **c == 'X'.as_ascii().unwrap()).count());

    let mut loop_tracker = PatrolTracker::new(map);
    let (start_x, start_y) = loop_tracker.get_patrol_position();
    let possible_obstacle_positions: Vec<_> = patrol_tracker.get_map()
        .indexed_iter()
        .filter(|(_, c)| **c == 'X'.as_ascii().unwrap())
        .filter(|((x, y), _)| start_x != *x || start_y != *y)
        .map(|(i, _)| i)
        .collect();
    let mut second_result = 0;
    for possible_obstacle_position in possible_obstacle_positions {
        if loop_tracker.would_obstacle_create_looping_patrol_path(possible_obstacle_position) {
            second_result += 1;
        }
    }
    println!("{loop_tracker:#?}");
    println!("The solution to part 2 is: {second_result}");
    Ok(())
}