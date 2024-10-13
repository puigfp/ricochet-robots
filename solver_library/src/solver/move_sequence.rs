use std::rc::Rc;

use super::{robot_positions::RobotPositions, Direction};

#[derive(Clone, Debug, PartialEq)]
pub struct Move {
    pub robot: usize,
    pub direction: Direction,
}

trait MoveSequence<P>
where
    P: RobotPositions,
{
    fn empty() -> Self;
    fn append(&self, move_: Move, next_positions: P) -> Self;
}

// Move sequence backed by a vec that's duplicated on append
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
