use std::rc::Rc;

use super::{robot_positions::RobotPositions, Direction};

// Immutable container for a robot move
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Move {
    pub robot: usize,
    pub direction: Direction,
}

// Trait for immutable containers holding a sequence of moves
pub trait MoveSequence<P>: Clone
where
    P: RobotPositions,
{
    fn empty() -> Self;
    fn append(&self, move_: Move, next_positions: P) -> Self;
    fn last(&self) -> Option<(&Move, &P)>;
    fn to_vec(self) -> Vec<(Move, P)>;
}

// Move sequence backed by a Vec that's duplicated on append
#[derive(Clone)]
pub struct MoveSequenceVec<P>
where
    P: RobotPositions,
{
    path: Vec<(Move, P)>,
}

impl<P: RobotPositions> MoveSequence<P> for MoveSequenceVec<P> {
    fn empty() -> Self {
        MoveSequenceVec { path: vec![] }
    }

    fn append(&self, move_: Move, next_positions: P) -> Self {
        let mut path_cloned = self.path.clone();
        path_cloned.push((move_, next_positions));
        MoveSequenceVec { path: path_cloned }
    }

    fn last(&self) -> Option<(&Move, &P)> {
        self.path.last().map(|(move_, position)| (move_, position))
    }

    fn to_vec(self) -> Vec<(Move, P)> {
        self.path
    }
}

// Move sequence backed by a linked list leveraging Rc for structural sharing
// This should be way faster than MoveSequenceVec when the search is dealing
// with very long move sequences
enum MoveSequenceLinkedListInner<P>
where
    P: RobotPositions,
{
    Nil,
    Cons(Move, P, Rc<MoveSequenceLinkedListInner<P>>),
}

#[derive(Clone)]
pub struct MoveSequenceLinkedList<P>(Rc<MoveSequenceLinkedListInner<P>>)
where
    P: RobotPositions;

impl<P: RobotPositions> MoveSequence<P> for MoveSequenceLinkedList<P> {
    fn empty() -> Self {
        MoveSequenceLinkedList(Rc::new(MoveSequenceLinkedListInner::Nil))
    }

    fn append(&self, move_: Move, next_positions: P) -> Self {
        MoveSequenceLinkedList(Rc::new(MoveSequenceLinkedListInner::Cons(
            move_,
            next_positions,
            self.0.clone(),
        )))
    }

    fn last(&self) -> Option<(&Move, &P)> {
        match self.0.as_ref() {
            MoveSequenceLinkedListInner::Nil => None,
            MoveSequenceLinkedListInner::Cons(move_, positions, _) => Some((move_, positions)),
        }
    }

    fn to_vec(self) -> Vec<(Move, P)> {
        let mut result = vec![];
        let mut current = self.0;
        while let MoveSequenceLinkedListInner::Cons(move_, position, next) = current.as_ref() {
            result.push((move_.clone(), position.clone()));
            current = next.clone();
        }
        result.reverse();
        result
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::solver::{
        move_sequence::Move, robot_positions::RobotPositionsVec, Direction, Position,
    };

    use super::{MoveSequence, MoveSequenceLinkedList, MoveSequenceVec};

    #[test]
    #[wasm_bindgen_test]
    fn test_move_sequences() {
        fn helper<S: MoveSequence<RobotPositionsVec>>(empty: S) {
            assert!(empty.last().is_none());
            assert_eq!(
                empty.clone().to_vec().into_iter().collect::<Vec<_>>(),
                vec![]
            );

            let positions_1 = RobotPositionsVec::new(vec![Position::new(0, 0)]);
            let appended_once = empty.append(
                Move {
                    robot: 0,
                    direction: Direction::Up,
                },
                positions_1.clone(),
            );
            assert_eq!(appended_once.last().unwrap().1, &positions_1);
            assert_eq!(
                appended_once
                    .clone()
                    .to_vec()
                    .into_iter()
                    .map(|(_, p)| p)
                    .collect::<Vec<_>>(),
                vec![positions_1.clone()]
            );

            let positions_2 = RobotPositionsVec::new(vec![Position::new(1, 1)]);
            let appended_twice = appended_once.append(
                Move {
                    robot: 0,
                    direction: Direction::Down,
                },
                positions_2.clone(),
            );
            assert_eq!(appended_twice.last().unwrap().1, &positions_2);
            assert_eq!(
                appended_twice
                    .clone()
                    .to_vec()
                    .into_iter()
                    .map(|(_, p)| p)
                    .collect::<Vec<_>>(),
                vec![positions_1.clone(), positions_2.clone()]
            );
        }
        helper(MoveSequenceVec::empty());
        helper(MoveSequenceLinkedList::empty());
    }
}
