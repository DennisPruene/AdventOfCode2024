use std::ascii::Char;
use std::collections::{HashMap, HashSet};
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
    up_positions: HashMap<usize, HashSet<usize>>,
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
            up_positions: HashMap::new()
        }
    }

    pub fn get_map(&self) -> &Array2<Char> {
        &self.map
    }

    pub fn get_patrol_position(&self) -> (usize, usize) {
        (self.patrol_position[0] as usize, self.patrol_position[1] as usize)
    }

    fn is_position_in_up_positions(&self, pos: &Array1<isize>) -> bool {
        if self.up_positions.contains_key(&(pos[0] as usize)) {
            self.up_positions.get(&(pos[0] as usize)).unwrap().contains(&(pos[1] as usize))
        } else {
            false
        }
    }

    fn add_position_to_up_positions(&mut self, pos: &Array1<isize>) {
        self.up_positions.entry(pos[0] as usize).or_default().insert(pos[1] as usize);
    }

    fn get_ray(&self) -> ArrayView<Char, Ix1> {
        match (self.patrol_direction[0], self.patrol_direction[1]) {
            (0, 1) => {
                self.map.slice(s![self.patrol_position[0], self.patrol_position[1]+1..])
            },
            (0, -1) => {
                self.map.slice(s![self.patrol_position[0], ..self.patrol_position[1];-1])
            }
            (1, 0) => {
                self.map.slice(s![self.patrol_position[0]+1.., self.patrol_position[1]])
            },
            (-1, 0) => {
                self.map.slice(s![..self.patrol_position[0];-1, self.patrol_position[1]])
            }
            _ => unreachable!()
        }
    }

    fn get_ray_mut(&mut self) -> ArrayViewMut<Char, Ix1> {
        match (self.patrol_direction[0], self.patrol_direction[1]) {
            (0, 1) => {
                self.map.slice_mut(s![self.patrol_position[0], self.patrol_position[1]..])
            },
            (0, -1) => {
                self.map.slice_mut(s![self.patrol_position[0], ..=self.patrol_position[1];-1])
            }
            (1, 0) => {
                self.map.slice_mut(s![self.patrol_position[0].., self.patrol_position[1]])
            },
            (-1, 0) => {
                self.map.slice_mut(s![..=self.patrol_position[0];-1, self.patrol_position[1]])
            }
            _ => unreachable!()
        }
    }

    pub fn take_step(&mut self, mark_path: bool) -> StepResult {
        let step_size = self.get_ray().iter().position(|c| *c == '#'.as_ascii().unwrap());
        match step_size {
            None => {
                if mark_path {
                    self.get_ray_mut().fill('X'.as_ascii().unwrap());
                }
                StepResult::OutOfBounds
            },
            Some(step_size) => {
                if mark_path {
                    self.get_ray_mut().slice_mut(s![..=step_size]).fill('X'.as_ascii().unwrap());
                }

                self.patrol_position[0] += ((step_size as isize) * &self.patrol_direction)[0];
                self.patrol_position[1] += ((step_size as isize) * &self.patrol_direction)[1];
                self.patrol_direction = RIGHT_TURN_MATRIX.dot(&self.patrol_direction);
                if self.patrol_direction[0] == -1 {
                    if self.is_position_in_up_positions(&self.patrol_position) {
                        return StepResult::LoopDetected;
                    }
                    self.add_position_to_up_positions(&self.patrol_position.clone());
                }
                StepResult::KeepGoing
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