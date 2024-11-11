use std::{
    collections::{BinaryHeap, HashSet},
    marker::PhantomData,
};

use super::{
    board::Board,
    move_sequence::{Move, MoveSequence},
    robot_positions::RobotPositions,
    wall_configuration::WallConfiguration,
    Position,
};

#[derive(Eq, PartialEq)]
struct Cost {
    moves: usize,
    robot_change: usize, // it's better to continuously move the same robot if possible (and not switch between robots all the time)
}

impl Ord for Cost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // XXX: BinaryHeap is a max-heap, and this cannot be changed.
        //  We therefore have to flip the ordering itself, hence why `self` and
        // `other` are reversed here.
        other
            .moves
            .cmp(&self.moves)
            .then_with(|| other.robot_change.cmp(&self.robot_change))
    }
}
impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct SequenceWithCost<P, M>
where
    P: RobotPositions,
    M: MoveSequence<P>,
{
    move_sequence: M,
    cost: Cost,

    // We are only using the type parameter P as a constraint on the type parameter
    // M, which leads the compiler to believe it's unused by this type.
    // This fake fields makes the compiler thinks P is actually used in the type
    // definition.
    phantom_position: PhantomData<P>,
}

impl<P, M> SequenceWithCost<P, M>
where
    P: RobotPositions,
    M: MoveSequence<P>,
{
    pub fn moves(&self) -> Vec<(Move, P)> {
        self.move_sequence.clone().to_vec()
    }
}

impl<P, M> PartialEq for SequenceWithCost<P, M>
where
    P: RobotPositions,
    M: MoveSequence<P>,
{
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<P, M> Eq for SequenceWithCost<P, M>
where
    P: RobotPositions,
    M: MoveSequence<P>,
{
}

impl<P, M> Ord for SequenceWithCost<P, M>
where
    P: RobotPositions,
    M: MoveSequence<P>,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}
impl<P, M> PartialOrd for SequenceWithCost<P, M>
where
    P: RobotPositions,
    M: MoveSequence<P>,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solve<W: WallConfiguration, P: RobotPositions, M: MoveSequence<P>>(
    board: &Board<W>,
    robot_positions: P,
    empty_move_sequence: M,
    target: (usize, Position),
) -> Option<SequenceWithCost<P, M>> {
    let mut seen = HashSet::new();
    seen.insert(robot_positions.clone());

    let mut queue = BinaryHeap::new();
    queue.push(SequenceWithCost {
        cost: Cost {
            moves: 0,
            robot_change: 0,
        },
        move_sequence: empty_move_sequence,
        phantom_position: PhantomData,
    });
    while let Some(sequence) = queue.pop() {
        let current_robot_positions = sequence
            .move_sequence
            .last()
            .map(|e| e.1)
            .unwrap_or(&robot_positions);
        if current_robot_positions.get_robot_position(target.0) == &target.1 {
            println!(
                "found solution in {} moves, {} positions explored",
                sequence.move_sequence.clone().to_vec().len(),
                seen.len()
            );
            return Some(sequence);
        }
        let valid_moves: Vec<_> = (0..current_robot_positions.num_robots())
            .flat_map(|robot| {
                board
                    .get_valid_moves_for_robot(robot, current_robot_positions)
                    .iter()
                    .map(|(direction, next_robot_position)| {
                        (
                            Move {
                                robot,
                                direction: *direction,
                            },
                            current_robot_positions.update(robot, next_robot_position.clone()),
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .filter(|(_, next_robot_positions)| !seen.contains(next_robot_positions))
            .collect();
        for (move_, next_robot_positions) in valid_moves {
            let updated_sequence_with_cost = SequenceWithCost {
                move_sequence: sequence
                    .move_sequence
                    .append(move_.clone(), next_robot_positions.clone()),
                cost: Cost {
                    moves: sequence.cost.moves + 1,
                    robot_change: sequence.cost.robot_change
                        + match sequence.move_sequence.last() {
                            Some((previous_move, _)) => {
                                if move_.robot == previous_move.robot {
                                    0
                                } else {
                                    1
                                }
                            }
                            None => 0,
                        },
                },
                phantom_position: PhantomData,
            };
            seen.insert(next_robot_positions.clone());
            queue.push(updated_sequence_with_cost);
        }
    }
    println!(
        "could not find a solution, {} positions explored",
        seen.len()
    );
    None
}
#[cfg(test)]
mod tests {
    use crate::solver::move_sequence::MoveSequenceLinkedList;
    use crate::solver::robot_positions::RobotPositionsVec;
    use crate::solver::wall_configuration::WallConfigurationVecVec;

    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn test_solve_with_solution() {
        let board = Board::new(WallConfigurationVecVec {
            height: 6,
            width: 5,
            right_walls: vec![vec![], vec![2], vec![], vec![], vec![], vec![]],
            bottom_walls: vec![vec![], vec![], vec![1], vec![], vec![]],
        });
        let robot_positions = RobotPositionsVec::new(vec![
            Position::new(0, 0),
            Position::new(1, 0),
            Position::new(1, 2),
            Position::new(1, 4),
        ]);
        let empty_move_sequence = MoveSequenceLinkedList::<RobotPositionsVec>::empty();
        let cases: Vec<(usize, Position, usize)> = vec![
            (0, Position::new(0, 0), 0),
            (0, Position::new(0, 4), 1),
            (0, Position::new(0, 3), 2),
            (0, Position::new(4, 3), 5),
            (3, Position::new(0, 0), 4),
            (0, Position::new(3, 1), 8),
            (1, Position::new(2, 3), 6),
            (0, Position::new(2, 3), 6),
            (0, Position::new(3, 2), 8),
            (0, Position::new(2, 2), 6),
            (0, Position::new(3, 3), 7),
        ];
        for (robot, target_position, moves) in cases {
            let solution = solve(
                &board,
                robot_positions.clone(),
                empty_move_sequence.clone(),
                (robot, target_position),
            )
            .unwrap();
            dbg!(solution
                .move_sequence
                .clone()
                .to_vec()
                .iter()
                .map(|(move_, _)| move_)
                .collect::<Vec<_>>());

            // assert correct number of moves
            assert_eq!(solution.move_sequence.clone().to_vec().len(), moves);

            // last move should always be made with the target robot
            if moves > 0 {
                assert_eq!(solution.move_sequence.last().unwrap().0.robot, robot);
            }
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_solve_with_no_solution() {
        let board = Board::new(WallConfigurationVecVec {
            height: 3,
            width: 3,
            right_walls: vec![vec![], vec![], vec![]],
            bottom_walls: vec![vec![], vec![], vec![]],
        });
        let robot_positions =
            RobotPositionsVec::new(vec![Position::new(0, 0), Position::new(1, 1)]);
        let empty_move_sequence = MoveSequenceLinkedList::<RobotPositionsVec>::empty();

        assert!(solve(
            &board,
            robot_positions.clone(),
            empty_move_sequence.clone(),
            (0, Position::new(1, 1)), // there's no way for the robot to reach the center of the board!
        )
        .is_none());
    }
}
