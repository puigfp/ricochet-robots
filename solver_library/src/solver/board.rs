use super::robot_positions::RobotPositions;
use super::wall_configuration::WallConfiguration;
use super::{Direction, Position};

// Immutable container for the board setup
pub struct Board<W: WallConfiguration> {
    wall_configuration: W,
    // TODO: add field for mirrors
    // TODO: add field for targets
}

impl<W> Board<W>
where
    W: WallConfiguration,
{
    pub fn new(wall_configuration: W) -> Self {
        Board { wall_configuration }
    }
    fn get_valid_up_move<P: RobotPositions>(
        &self,
        robot: usize,
        robot_positions: &P,
    ) -> Option<Position> {
        let position = robot_positions.get_robot_position(robot);
        let up_wall = self.wall_configuration.next_wall_up(position);
        let up_robot = robot_positions.next_robot_up(position);
        [up_wall, up_robot, Some(0)]
            .iter()
            .flatten()
            .max()
            .filter(|row| **row != position.row)
            .map(|row| Position {
                col: position.col,
                row: *row,
            })
    }

    fn get_valid_down_move<P: RobotPositions>(
        &self,
        robot: usize,
        robot_positions: &P,
    ) -> Option<Position> {
        let position = robot_positions.get_robot_position(robot);
        let down_wall = self.wall_configuration.next_wall_down(position);
        let down_robot = robot_positions.next_robot_down(position);
        [
            down_wall,
            down_robot,
            Some(self.wall_configuration.get_height() - 1),
        ]
        .iter()
        .flatten()
        .min()
        .filter(|row| **row != position.row)
        .map(|row| Position {
            col: position.col,
            row: *row,
        })
    }
    fn get_valid_right_move<P: RobotPositions>(
        &self,
        robot: usize,
        robot_positions: &P,
    ) -> Option<Position> {
        let position = robot_positions.get_robot_position(robot);
        let right_wall = self.wall_configuration.next_wall_right(position);
        let right_robot = robot_positions.next_robot_right(position);
        [
            right_wall,
            right_robot,
            Some(self.wall_configuration.get_width() - 1),
        ]
        .iter()
        .flatten()
        .min()
        .filter(|col| **col != position.col)
        .map(|col| Position {
            col: *col,
            row: position.row,
        })
    }

    fn get_valid_left_move<P: RobotPositions>(
        &self,
        robot: usize,
        robot_positions: &P,
    ) -> Option<Position> {
        let position = robot_positions.get_robot_position(robot);
        let left_wall = self.wall_configuration.next_wall_left(position);
        let left_robot = robot_positions.next_robot_left(position);
        [left_wall, left_robot, Some(0)]
            .iter()
            .flatten()
            .max()
            .filter(|col| **col != position.col)
            .map(|col| Position {
                col: *col,
                row: position.row,
            })
    }
    pub fn get_valid_moves_for_robot<P: RobotPositions>(
        &self,
        robot: usize,
        robot_positions: &P,
    ) -> Vec<(Direction, Position)> {
        [
            (
                Direction::Up,
                self.get_valid_up_move(robot, robot_positions),
            ),
            (
                Direction::Down,
                self.get_valid_down_move(robot, robot_positions),
            ),
            (
                Direction::Right,
                self.get_valid_right_move(robot, robot_positions),
            ),
            (
                Direction::Left,
                self.get_valid_left_move(robot, robot_positions),
            ),
        ]
        .into_iter()
        .flat_map(|(direction, next_position)| {
            next_position.map(|next_position| (direction, next_position))
        })
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::solver::{
        board::Board, robot_positions::RobotPositionsVec,
        wall_configuration::WallConfigurationVecVec, Direction, Position,
    };

    #[test]
    #[wasm_bindgen_test]
    fn test_board() {
        let board = Board {
            wall_configuration: WallConfigurationVecVec {
                height: 6,
                width: 5,
                right_walls: vec![vec![], vec![2], vec![], vec![], vec![], vec![]],
                bottom_walls: vec![vec![], vec![], vec![1], vec![], vec![]],
            },
        };
        let robot_positions = RobotPositionsVec::new(vec![
            Position::new(0, 0),
            Position::new(1, 0),
            Position::new(1, 2),
            Position::new(1, 4),
        ]);
        type Case = (usize, Vec<(Direction, Position)>);
        let cases: Vec<Case> = vec![
            (0, vec![(Direction::Right, Position::new(0, 4))]),
            (
                1,
                vec![
                    (Direction::Down, Position::new(5, 0)),
                    (Direction::Right, Position::new(1, 1)),
                ],
            ),
            (
                2,
                vec![
                    (Direction::Up, Position::new(0, 2)),
                    (Direction::Left, Position::new(1, 1)),
                ],
            ),
            (
                3,
                vec![
                    (Direction::Up, Position::new(0, 4)),
                    (Direction::Down, Position::new(5, 4)),
                    (Direction::Left, Position::new(1, 3)),
                ],
            ),
        ];
        for (robot, expected) in cases {
            let result = board.get_valid_moves_for_robot(robot, &robot_positions);
            assert_eq!(result, expected)
        }
    }
}
