mod board;
mod move_sequence;
mod robot_positions;
mod solver;
mod wall_configuration;

#[derive(Clone, Debug, Hash, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}
