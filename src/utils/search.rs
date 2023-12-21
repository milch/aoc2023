use super::*;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Trace {
    current_point: Point,
    direction: Direction,
}

pub trait Traceable {
    fn new(start: Point, direction: Direction) -> Self;
    fn extend(&self, direction: Direction) -> Option<Self>
    where
        Self: Sized;
    fn get_point(&self) -> Point;
    fn get_direction(&self) -> Direction;
}

impl Traceable for Trace {
    fn extend(&self, direction: Direction) -> Option<Trace> {
        let next = self.current_point + direction;
        Some(Self {
            current_point: next?,
            direction,
        })
    }

    fn get_direction(&self) -> Direction {
        self.direction
    }

    fn get_point(&self) -> Point {
        self.current_point
    }

    fn new(start: Point, direction: Direction) -> Self {
        Self {
            current_point: start,
            direction,
        }
    }
}
