pub mod board;
pub mod move_sequence;
pub mod robot_positions;
pub mod solver;
pub mod wall_configuration;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}
