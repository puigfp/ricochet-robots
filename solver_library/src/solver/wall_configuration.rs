use super::Position;

// Trait for immutable containers holding the layout of the walls
pub trait WallConfiguration {
    fn is_valid(&self) -> bool;

    // XXX: this probably should not live here, it's shared by the entire board
    fn get_height(&self) -> usize;
    fn get_width(&self) -> usize;

    // How far can a robot travel until hitting a wall?
    fn next_wall_up(&self, position: &Position) -> Option<usize>;
    fn next_wall_down(&self, position: &Position) -> Option<usize>;
    fn next_wall_right(&self, position: &Position) -> Option<usize>;
    fn next_wall_left(&self, position: &Position) -> Option<usize>;
}

// Immutable container storing the layout of the walls in a Vec<Vec<usize>>
pub struct WallConfigurationVecVec {
    // TODO: make these private
    pub height: usize,
    pub width: usize,
    pub right_walls: Vec<Vec<usize>>,
    pub bottom_walls: Vec<Vec<usize>>,
}

impl WallConfigurationVecVec {
    fn next_wall(walls: &[usize], position: usize, diff: isize) -> Option<usize> {
        let candidate_walls = walls.iter().filter(|pos| match diff {
            1 => **pos >= position,
            -1 => **pos < position,
            _ => unreachable!(),
        });
        match diff {
            1 => candidate_walls.min().copied(),
            -1 => candidate_walls.max().map(|pos| pos + 1),
            _ => unreachable!(),
        }
    }
}

impl WallConfiguration for WallConfigurationVecVec {
    fn get_height(&self) -> usize {
        self.height
    }

    fn get_width(&self) -> usize {
        self.width
    }

    fn is_valid(&self) -> bool {
        self.right_walls.len() == self.height
            && self.bottom_walls.len() == self.width
            && self
                .right_walls
                .iter()
                .flatten()
                .all(|col| *col < self.width)
            && self
                .bottom_walls
                .iter()
                .flatten()
                .all(|row| *row < self.height)
    }

    fn next_wall_up(&self, position: &Position) -> Option<usize> {
        WallConfigurationVecVec::next_wall(
            self.bottom_walls.get(position.col).unwrap(),
            position.row,
            -1,
        )
    }

    fn next_wall_down(&self, position: &Position) -> Option<usize> {
        WallConfigurationVecVec::next_wall(
            self.bottom_walls.get(position.col).unwrap(),
            position.row,
            1,
        )
    }

    fn next_wall_right(&self, position: &Position) -> Option<usize> {
        WallConfigurationVecVec::next_wall(
            self.right_walls.get(position.row).unwrap(),
            position.col,
            1,
        )
    }

    fn next_wall_left(&self, position: &Position) -> Option<usize> {
        WallConfigurationVecVec::next_wall(
            self.right_walls.get(position.row).unwrap(),
            position.col,
            -1,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn test_wall_configuration() {
        let wall_configuration = WallConfigurationVecVec {
            height: 6,
            width: 5,
            right_walls: vec![vec![1, 2], vec![], vec![], vec![], vec![], vec![]],
            bottom_walls: vec![vec![1, 2], vec![], vec![], vec![], vec![]],
        };
        assert!(wall_configuration.is_valid());
        type Case = (
            Position,
            Option<usize>,
            Option<usize>,
            Option<usize>,
            Option<usize>,
        );
        let cases: Vec<Case> = vec![
            (Position::new(2, 2), None, None, None, None), // clear row and column
            (Position::new(0, 0), None, Some(1), Some(1), None),
            (Position::new(0, 3), None, None, None, Some(3)),
            (Position::new(3, 0), Some(3), None, None, None),
            (Position::new(0, 2), None, None, Some(2), Some(2)), // horizontally stuck between two walls
            (Position::new(2, 0), Some(2), Some(2), None, None), // vertically stuck between two walls
        ];

        for (position, up, down, right, left) in cases {
            assert_eq!(wall_configuration.next_wall_up(&position), up);
            assert_eq!(wall_configuration.next_wall_down(&position), down);
            assert_eq!(wall_configuration.next_wall_right(&position), right);
            assert_eq!(wall_configuration.next_wall_left(&position), left);
        }
    }
}
