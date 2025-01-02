use crate::day4::matrix::{Matrix, MutMatrix, base::MatrixBase, sparse::SparseMatrix};
use crate::parsing::read_file_to_string;
use crate::DynResult;
use once_cell::sync::Lazy;
use std::ascii::Char;
use std::collections::HashMap;
use std::convert::TryInto;
use std::path::Path;

static ARROW_TO_DIRECTION: Lazy<HashMap<Char, (i32, i32)>> = Lazy::new(|| {
    [
        ('^'.as_ascii().unwrap(), (0, -1)),
        ('>'.as_ascii().unwrap(), (1, 0)),
        ('v'.as_ascii().unwrap(), (0, 1)),
        ('<'.as_ascii().unwrap(), (-1, 0)),
    ]
    .into()
});
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

    let map: MatrixBase<Char> = read_file_to_string(path)?.try_into()?;
    let mut map = SparseMatrix::from_matrix(map, '.'.as_ascii().unwrap());
    println!("{map:#?}");
    println!("{map}");
    let (mut patrol_x, mut patrol_y) = map
        .indexed_iter()
        .filter(|(_, c)| vec!["^", "<", "v", ">"].contains(&c.as_str()))
        .map(|(i, _)| i)
        .next()
        .unwrap();
    loop {
        if let Some((next_patrol_x, next_patrol_y)) = take_step(&mut map, patrol_x, patrol_y) {
            patrol_x = next_patrol_x;
            patrol_y = next_patrol_y;
        } else {
            break;
        }
    }
    println!("The solution to part 1 is: {}", map.count_where(|c| c.as_str() == "X"));


    Ok(())
}

fn take_step(
    map: &mut SparseMatrix<Char>,
    patrol_x: usize,
    patrol_y: usize,
) -> Option<(usize, usize)> {
    let patrol_arrow = map.get(patrol_x, patrol_y).unwrap();
    let (dir_x, dir_y) = *ARROW_TO_DIRECTION.get(&patrol_arrow).unwrap();
    let next_patrol_x = patrol_x as i32 + dir_x;
    let next_patrol_y = patrol_y as i32 + dir_y;
    if next_patrol_x < 0 || next_patrol_y < 0 {
        return None;
    }
    let next_patrol_x = next_patrol_x as usize;
    let next_patrol_y = next_patrol_y as usize;
    if let Some(c) = map.get(next_patrol_x, next_patrol_y) {
        return if c.as_str() == "#" {
            map.set(
                patrol_x,
                patrol_y,
                *CLOCKWISE_ROTATION.get(&patrol_arrow).unwrap(),
            );
            Some((patrol_x, patrol_y))
        } else {
            map.set(
                next_patrol_x,
                next_patrol_y,
                patrol_arrow,
            );
            map.set(patrol_x, patrol_y, 'X'.as_ascii().unwrap());
            Some((next_patrol_x, next_patrol_y))
        };
    }
    map.set(patrol_x, patrol_y, 'X'.as_ascii().unwrap());
    None
}
