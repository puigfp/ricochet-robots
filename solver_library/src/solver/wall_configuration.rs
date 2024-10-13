use super::Position;

trait WallConfiguration {
    fn next_wall_up(&self, position: Position) -> usize;
    fn next_wall_down(&self, position: Position) -> usize;
    fn next_wall_right(&self, position: Position) -> usize;
    fn next_wall_left(&self, position: Position) -> usize;
}

struct WallConfigurationVecVec {
    height: usize,
    width: usize,
    right_walls: Vec<Vec<usize>>,
    bottom_walls: Vec<Vec<usize>>,
}

impl WallConfigurationVecVec {
    fn next_wall(walls: &[usize], position: usize, diff: isize, default: usize) -> usize {
        let candidate_walls = walls.iter().filter(|pos| match diff {
            1 => **pos >= position,
            -1 => **pos < position,
            _ => unreachable!(),
        });
        let first_wall = match diff {
            1 => candidate_walls.min().copied(),
            -1 => candidate_walls.max().map(|pos| pos + 1),
            _ => unreachable!(),
        };
        first_wall.unwrap_or(default)
    }
}

impl WallConfiguration for WallConfigurationVecVec {
    fn next_wall_up(&self, position: Position) -> usize {
        WallConfigurationVecVec::next_wall(
            self.bottom_walls.get(position.col).unwrap(),
            position.row,
            -1,
            0,
        )
    }

    fn next_wall_down(&self, position: Position) -> usize {
        WallConfigurationVecVec::next_wall(
            self.bottom_walls.get(position.col).unwrap(),
            position.row,
            1,
            self.height - 1,
        )
    }

    fn next_wall_right(&self, position: Position) -> usize {
        WallConfigurationVecVec::next_wall(
            self.right_walls.get(position.row).unwrap(),
            position.col,
            1,
            self.width - 1,
        )
    }

    fn next_wall_left(&self, position: Position) -> usize {
        WallConfigurationVecVec::next_wall(
            self.right_walls.get(position.row).unwrap(),
            position.col,
            -1,
            0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wall_configuration() {
        let wall_configuration = WallConfigurationVecVec {
            height: 6,
            width: 5,
            right_walls: vec![vec![1, 2], vec![], vec![], vec![], vec![]],
            bottom_walls: vec![vec![1, 2], vec![], vec![], vec![], vec![], vec![]],
        };

        // clear row and column
        assert_eq!(wall_configuration.next_wall_right(Position::new(2, 2)), 4); // hit the right side of the board
        assert_eq!(wall_configuration.next_wall_left(Position::new(2, 2)), 0); // hit the left side of the board
        assert_eq!(wall_configuration.next_wall_up(Position::new(2, 2)), 0); // hit the top side of the board
        assert_eq!(wall_configuration.next_wall_down(Position::new(2, 2)), 5); // hit the bottom side of the board

        assert_eq!(wall_configuration.next_wall_right(Position::new(0, 0)), 1); // hit a wall
        assert_eq!(wall_configuration.next_wall_left(Position::new(0, 0)), 0); // hit the left side of the board

        assert_eq!(wall_configuration.next_wall_right(Position::new(0, 3)), 4); // hit the right side of the board
        assert_eq!(wall_configuration.next_wall_left(Position::new(0, 3)), 3); // hit a wall

        assert_eq!(wall_configuration.next_wall_right(Position::new(0, 2)), 2); // hit a wall
        assert_eq!(wall_configuration.next_wall_left(Position::new(0, 2)), 2); // hit a wall

        assert_eq!(wall_configuration.next_wall_down(Position::new(0, 0)), 1); // hit a wall
        assert_eq!(wall_configuration.next_wall_up(Position::new(0, 0)), 0); // hit the top side of the board

        assert_eq!(wall_configuration.next_wall_down(Position::new(3, 0)), 5); // hit the bottom side of the board
        assert_eq!(wall_configuration.next_wall_up(Position::new(3, 0)), 3); // hit a wall

        assert_eq!(wall_configuration.next_wall_down(Position::new(2, 0)), 2); // hit a wall
        assert_eq!(wall_configuration.next_wall_up(Position::new(2, 0)), 2); // hit a wall
    }
}
