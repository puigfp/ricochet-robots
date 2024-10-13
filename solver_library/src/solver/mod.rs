mod wall_configuration;

struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}
