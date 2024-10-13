use move_sequence::Move;
use robot_positions::RobotPositions;
use wall_configuration::WallConfiguration;

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

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

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
    fn get_valid_moves_for_robot<P: RobotPositions>(
        &self,
        robot: usize,
        robot_positions: P,
    ) -> Vec<(Direction, Position)> {
        [
            (
                Direction::Up,
                self.get_valid_up_move(robot, &robot_positions),
            ),
            (
                Direction::Down,
                self.get_valid_down_move(robot, &robot_positions),
            ),
            (
                Direction::Right,
                self.get_valid_right_move(robot, &robot_positions),
            ),
            (
                Direction::Left,
                self.get_valid_left_move(robot, &robot_positions),
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
    use super::*;
    use robot_positions::RobotPositionsVec;
    use wall_configuration::WallConfigurationVecVec;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn test_board() {
        let board = Board {
            wall_configuration: WallConfigurationVecVec {
                height: 6,
                width: 5,
                right_walls: vec![vec![1, 2], vec![], vec![], vec![], vec![], vec![]],
                bottom_walls: vec![vec![1, 2], vec![], vec![], vec![], vec![]],
            },
        };
        let robot_positions = RobotPositionsVec {
            positions: vec![
                Position::new(0, 0),
                Position::new(0, 1),
                Position::new(1, 2),
                Position::new(1, 4),
            ],
        };
        type Case = (usize, Vec<(Direction, Position)>);
        assert_eq!(board.get_valid_moves_for_robot(2, robot_positions), vec![]);
    }
}
