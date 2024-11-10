use wasm_bindgen::prelude::*;

use serde::Serialize;

use crate::solver::board::Board;
use crate::solver::move_sequence::{MoveSequence, MoveSequenceLinkedList};
use crate::solver::robot_positions::{RobotPositions, RobotPositionsVec};
use crate::solver::solver;
use crate::solver::wall_configuration::{WallConfiguration, WallConfigurationVecVec};
use crate::solver::Direction;

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
#[derive(Debug, Serialize)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[wasm_bindgen]
impl Position {
    #[wasm_bindgen(constructor)]
    pub fn new(row: usize, col: usize) -> Position {
        Position { row, col }
    }

    // We can't generate WASM bindings for functions of the Display trait!
    #[wasm_bindgen]
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Serialize)]
pub struct Move {
    pub robot: usize,
    pub direction: usize,
    pub robot_positions: Vec<Position>,
}

#[wasm_bindgen]
pub fn solve(
    robot_positions: Vec<Position>,
    height: usize,
    width: usize,
    right_walls: Vec<Position>,
    bottom_walls: Vec<Position>,
    target: Position,
    target_robot: Option<usize>,
) -> JsValue {
    let mut right_walls_vec_vec = vec![];
    for _ in 0..height {
        right_walls_vec_vec.push(vec![]);
    }
    for p in right_walls.iter() {
        right_walls_vec_vec[p.row].push(p.col);
    }
    let mut bottom_walls_vec_vec = vec![];
    for _ in 0..width {
        bottom_walls_vec_vec.push(vec![]);
    }
    for p in bottom_walls.iter() {
        bottom_walls_vec_vec[p.col].push(p.row);
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
            .map(|p| crate::solver::Position::new(p.row, p.col))
            .collect(),
    );
    let solution = solver::solve(
        &board,
        robot_positions,
        MoveSequenceLinkedList::empty(),
        (
            target_robot.unwrap(),
            crate::solver::Position::new(target.row, target.col),
        ),
    );
    let output = match solution {
        Some(sequence) => sequence
            .moves()
            .iter()
            .map(|(move_, robot_positions)| Move {
                robot: move_.robot,
                direction: (match move_.direction {
                    Direction::Up => 0,
                    Direction::Left => 1,
                    Direction::Down => 2,
                    Direction::Right => 3,
                }),
                robot_positions: (0..robot_positions.num_robots())
                    .map(|i| robot_positions.get_robot_position(i).to_owned())
                    .map(|p| Position {
                        row: p.row,
                        col: p.col,
                    })
                    .collect(),
            })
            .collect(),
        None => vec![],
    };
    serde_wasm_bindgen::to_value(&output).unwrap()
}
