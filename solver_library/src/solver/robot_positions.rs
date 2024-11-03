use super::Position;

use std::hash::Hash;

// Immutable container for the positions of the robots
pub trait RobotPositions
where
    Self: Clone + std::fmt::Debug + Eq + Hash,
{
    fn get_robot_position(&self, robot: usize) -> &Position;
    fn num_robots(&self) -> usize;
    fn update(&self, robot: usize, position: Position) -> Self;

    // How far can a robot travel until hitting another robot?
    fn next_robot_up(&self, position: &Position) -> Option<usize>;
    fn next_robot_down(&self, position: &Position) -> Option<usize>;
    fn next_robot_right(&self, position: &Position) -> Option<usize>;
    fn next_robot_left(&self, position: &Position) -> Option<usize>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct RobotPositionsVec {
    positions: Vec<Position>,
}

impl RobotPositionsVec {
    pub fn new(positions: Vec<Position>) -> Self {
        RobotPositionsVec { positions }
    }
}

impl Eq for RobotPositionsVec {}

impl Hash for RobotPositionsVec {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.positions.hash(state);
    }
}
impl RobotPositions for RobotPositionsVec {
    fn get_robot_position(&self, robot: usize) -> &Position {
        self.positions.get(robot).unwrap()
    }

    fn num_robots(&self) -> usize {
        self.positions.len()
    }

    fn update(&self, robot: usize, position: Position) -> Self {
        // There are very few robots, so let's not overengineer this.
        let mut positions_cloned = self.positions.clone();
        positions_cloned[robot] = position;
        RobotPositionsVec {
            positions: positions_cloned,
        }
    }

    fn next_robot_up(&self, position: &Position) -> Option<usize> {
        self.positions
            .iter()
            .filter(|other_position| {
                other_position.col == position.col && other_position.row < position.row
            })
            .map(|other_position| other_position.row)
            .max()
            .map(|row| row + 1)
    }

    fn next_robot_down(&self, position: &Position) -> Option<usize> {
        self.positions
            .iter()
            .filter(|other_position| {
                other_position.col == position.col && other_position.row > position.row
            })
            .map(|other_position| other_position.row)
            .min()
            .map(|row| row - 1)
    }

    fn next_robot_right(&self, position: &Position) -> Option<usize> {
        self.positions
            .iter()
            .filter(|other_position| {
                other_position.row == position.row && other_position.col > position.col
            })
            .map(|other_position| other_position.col)
            .min()
            .map(|col| col - 1)
    }

    fn next_robot_left(&self, position: &Position) -> Option<usize> {
        self.positions
            .iter()
            .filter(|other_position| {
                other_position.row == position.row && other_position.col < position.col
            })
            .map(|other_position| other_position.col)
            .max()
            .map(|col| col + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn test_robot_positions() {
        let robot_positions = RobotPositionsVec {
            positions: vec![
                Position::new(0, 0),
                Position::new(0, 3),
                Position::new(0, 2),
                Position::new(3, 0),
            ],
        };

        type Case = (
            Position,
            Option<usize>,
            Option<usize>,
            Option<usize>,
            Option<usize>,
        );
        let cases: Vec<Case> = vec![
            (Position::new(0, 0), None, Some(2), Some(1), None),
            (Position::new(0, 5), None, None, None, Some(4)),
            (Position::new(4, 0), Some(4), None, None, None),
            (Position::new(3, 0), Some(1), None, None, None),
        ];

        for (position, up, down, right, left) in cases {
            assert_eq!(robot_positions.next_robot_up(&position), up);
            assert_eq!(robot_positions.next_robot_down(&position), down);
            assert_eq!(robot_positions.next_robot_right(&position), right);
            assert_eq!(robot_positions.next_robot_left(&position), left);
        }
    }
}
