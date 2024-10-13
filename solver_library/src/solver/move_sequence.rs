use std::rc::Rc;

use super::{robot_positions::RobotPositions, Direction};

// Immutable container for a robot move
#[derive(Clone, Debug, PartialEq)]
pub struct Move {
    pub robot: usize,
    pub direction: Direction,
}

// Trait for immutable containers holding a sequence of moves
// TODO: all types implementing this trait should also implement IntoIterator<(Move, P)>
trait MoveSequence<P>
where
    P: RobotPositions,
{
    fn empty() -> Self;
    fn append(&self, move_: Move, next_positions: P) -> Self;
}

// Move sequence backed by a Vec that's duplicated on append
struct MoveSequenceVec<P>
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
}

// Move sequence backed by a linked list leveraging Rc for structural sharing
// This should be way faster than MoveSequenceVec when the search is dealing
// with very long move sequences
struct MoveSequenceLinkedList<P>
where
    P: RobotPositions,
{
    last_position: Option<(Move, P, Rc<MoveSequenceLinkedList<P>>)>,
}

impl<P: RobotPositions> MoveSequence<P> for Rc<MoveSequenceLinkedList<P>> {
    fn empty() -> Self {
        Rc::new(MoveSequenceLinkedList {
            last_position: None,
        })
    }

    fn append(&self, move_: Move, next_positions: P) -> Self {
        Rc::new(MoveSequenceLinkedList {
            last_position: Some((move_, next_positions, self.clone())),
        })
    }
}
