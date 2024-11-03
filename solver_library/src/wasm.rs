use std::collections::HashMap;

use wasm_bindgen::prelude::*;

use crate::solver::board::Board;
use crate::solver::move_sequence::{MoveSequence, MoveSequenceLinkedList};
use crate::solver::robot_positions::{self, RobotPositionsVec};
use crate::solver::wall_configuration::{WallConfiguration, WallConfigurationVecVec};
use crate::solver::{board, solver};
use crate::solver::{Direction, Position};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn fib(i: usize) -> usize {
    assert!(i < 100); // fake error to work on web worker error handling
    if i < 2 {
        return 1;
    }
    fib(i - 1) + fib(i - 2)
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct RobotPosition {
    pub x: usize,
    pub y: usize,
}

#[wasm_bindgen]
impl RobotPosition {
    #[wasm_bindgen(constructor)]
    pub fn new(x: usize, y: usize) -> RobotPosition {
        RobotPosition { x, y }
    }

    // We can't generate WASM bindings for functions of the Display trait!
    #[wasm_bindgen]
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Move {
    pub robot: usize,
    pub direction: usize,
}

#[wasm_bindgen]
pub fn solve(
    robot_positions: Vec<RobotPosition>,
    height: usize,
    width: usize,
    right_walls: Vec<RobotPosition>,
    bottom_walls: Vec<RobotPosition>,
    target: RobotPosition,
    target_robot: Option<usize>,
) -> Vec<Move> {
    let mut right_walls_vec_vec = vec![];
    for _ in 0..height {
        right_walls_vec_vec.push(vec![]);
    }
    for p in right_walls.iter() {
        right_walls_vec_vec[p.x].push(p.y);
    }
    let mut bottom_walls_vec_vec = vec![];
    for _ in 0..width {
        bottom_walls_vec_vec.push(vec![]);
    }
    for p in bottom_walls.iter() {
        bottom_walls_vec_vec[p.y].push(p.x);
    }
    let wall_configuration = WallConfigurationVecVec {
        right_walls: right_walls_vec_vec,
        bottom_walls: bottom_walls_vec_vec,
        height,
        width,
    };
    assert!(wall_configuration.is_valid());

    let board = Board::new(wall_configuration);
    let robot_positions = RobotPositionsVec::new(
        robot_positions
            .into_iter()
            .map(|p| Position::new(p.x, p.y))
            .collect(),
    );
    let solution = solver::solve(
        &board,
        robot_positions,
        MoveSequenceLinkedList::empty(),
        (target_robot.unwrap(), Position::new(target.x, target.y)),
    );
    match solution {
        Some(sequence) => sequence
            .moves()
            .iter()
            .map(|(move_, _)| Move {
                robot: move_.robot,
                direction: (match move_.direction {
                    Direction::Up => 0,
                    Direction::Left => 1,
                    Direction::Down => 2,
                    Direction::Right => 3,
                }),
            })
            .collect(),
        None => vec![],
    }
}
