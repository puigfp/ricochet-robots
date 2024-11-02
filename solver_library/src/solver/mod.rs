mod board;
mod move_sequence;
mod robot_positions;
mod wall_configuration;

#[derive(Clone, Debug, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}
