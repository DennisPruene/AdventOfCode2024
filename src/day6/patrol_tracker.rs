use std::ascii::Char;
use std::collections::HashMap;
use ndarray::Ix;
use once_cell::sync::Lazy;
use ndarray::prelude::*;

static RIGHT_TURN_MATRIX: Lazy<Array2<isize>> = Lazy::new(|| {
    arr2(&[[0, 1],
              [-1, 0]])
});

static ARROW_TO_DIRECTION: Lazy<HashMap<Char, Array1<isize>>> = Lazy::new(|| {
    [
        ('^'.as_ascii().unwrap(), arr1(&[-1, 0])),
        ('>'.as_ascii().unwrap(), arr1(&[0, 1])),
        ('v'.as_ascii().unwrap(), arr1(&[1, 0])),
        ('<'.as_ascii().unwrap(), arr1(&[0, -1]))
    ]
        .into()
});

#[derive(Debug, Clone)]
pub struct PatrolTracker {
    map: Array2<Char>,
    patrol_position: Array1<isize>,
    patrol_direction: Array1<isize>,
    up_positions: Vec<Array1<isize>>,
}

impl PatrolTracker {
    pub fn new(mut map: Array2<Char>) -> Self {
        let (patrol_x, patrol_y) = map.indexed_iter()
            .filter(|(_, c)| ARROW_TO_DIRECTION.contains_key(*c))
            .map(|(i, _)| i)
            .next().unwrap();
        let patrol_position = array![patrol_x as isize, patrol_y as isize];
        let patrol_direction = ARROW_TO_DIRECTION.get(map.get((patrol_x, patrol_y)).unwrap()).unwrap().clone();
        map[(patrol_x, patrol_y)] = '.'.as_ascii().unwrap();
        Self {
            map,
            patrol_position,
            patrol_direction,
            up_positions: vec![]
        }
    }

    pub fn get_map(&self) -> &Array2<Char> {
        &self.map
    }

    pub fn get_patrol_position(&self) -> (usize, usize) {
        (self.patrol_position[0] as usize, self.patrol_position[1] as usize)
    }

    pub fn take_step(&mut self, mark_path: bool) -> StepResult {
        let next_position = &self.patrol_position + &self.patrol_direction;
        match self.map.get((next_position[0] as Ix, next_position[1] as Ix)) {
            Some(c) =>  {
                if c.as_str() == "#" {
                    self.patrol_direction = RIGHT_TURN_MATRIX.dot(&self.patrol_direction);
                    if self.patrol_direction[1] == -1 {
                        if self.up_positions.contains(&self.patrol_position) {
                            return StepResult::LoopDetected;
                        }
                        self.up_positions.push(self.patrol_position.clone());
                    }
                } else {
                    if mark_path {
                        self.map[(self.patrol_position[0] as Ix, self.patrol_position[1] as Ix)] = 'X'.as_ascii().unwrap();
                    }
                    self.patrol_position = next_position;
                }
                StepResult::KeepGoing
            },
            None => {
                if mark_path {
                    self.map[(self.patrol_position[0] as Ix, self.patrol_position[1] as Ix)] = 'X'.as_ascii().unwrap();
                }
                StepResult::OutOfBounds
            }
        }
    }

    pub fn would_obstacle_create_looping_patrol_path(&self, obstacle_position: (usize, usize)) -> bool {
        let mut tracker = self.clone();
        tracker.map[obstacle_position] = '#'.as_ascii().unwrap();
        let mut patrol_result = StepResult::KeepGoing;
        while patrol_result == StepResult::KeepGoing {
            patrol_result = tracker.take_step(false);
        }
        match patrol_result {
            StepResult::LoopDetected => true,
            StepResult::OutOfBounds => false,
            StepResult::KeepGoing => unreachable!()
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum StepResult {
    KeepGoing,
    OutOfBounds,
    LoopDetected
}