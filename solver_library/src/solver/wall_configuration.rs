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

impl WallConfiguration for WallConfigurationVecVec {
    fn next_wall_up(&self, position: Position) -> usize {
        self.bottom_walls
            .get(position.col)
            .unwrap()
            .iter()
            .filter(|row| **row < position.row)
            .max()
            .map(|row: &usize| row + 1)
            .unwrap_or(0)
    }

    fn next_wall_down(&self, position: Position) -> usize {
        self.bottom_walls
            .get(position.col)
            .unwrap()
            .iter()
            .filter(|row| **row >= position.row)
            .min()
            .copied()
            .unwrap_or(self.height - 1)
    }

    fn next_wall_right(&self, position: Position) -> usize {
        self.right_walls
            .get(position.row)
            .unwrap()
            .iter()
            .filter(|col| **col >= position.col)
            .min()
            .copied()
            .unwrap_or(self.width - 1)
    }

    fn next_wall_left(&self, position: Position) -> usize {
        self.right_walls
            .get(position.row)
            .unwrap()
            .iter()
            .filter(|col| **col < position.col)
            .max()
            .map(|col| col + 1)
            .unwrap_or(0)
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
