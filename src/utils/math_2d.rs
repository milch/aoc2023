use std::{
    fmt::{self, Display},
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub const ALL: [Direction; 4] = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct Vector2D<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

impl<T> From<Vector2D<T>> for Direction
where
    T: Into<i8>,
{
    fn from(vector: Vector2D<T>) -> Self {
        match (vector.x.into(), vector.y.into()) {
            (1, 0) => Direction::East,
            (-1, 0) => Direction::West,
            (0, 1) => Direction::South,
            (0, -1) => Direction::North,
            _ => unreachable!(),
        }
    }
}

impl<T> From<Direction> for Vector2D<T>
where
    T: num::Signed,
{
    fn from(val: Direction) -> Self {
        match val {
            Direction::East => Vector2D::new(T::one(), T::zero()),
            Direction::West => Vector2D::new(-T::one(), T::zero()),
            Direction::South => Vector2D::new(T::zero(), T::one()),
            Direction::North => Vector2D::new(T::zero(), -T::one()),
        }
    }
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    pub fn rotate(self, angle: f32) -> Self {
        let rads = -angle.to_radians();
        let vector: Vector2D<i8> = (self).into();
        Vector2D {
            x: (vector.x as f32 * rads.cos() - vector.y as f32 * rads.sin()) as i8,
            y: (vector.x as f32 * rads.sin() + vector.y as f32 * rads.cos()) as i8,
        }
        .into()
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::North => f.write_str("north"),
            Direction::South => f.write_str("south"),
            Direction::East => f.write_str("east"),
            Direction::West => f.write_str("west"),
        }
    }
}

pub type Point = Vector2D<usize>;

impl<T: num::Num> Vector2D<T> {
    pub fn new<N>(col_idx: N, row_idx: N) -> Vector2D<T>
    where
        N: Into<T>,
    {
        Vector2D {
            x: col_idx.into(),
            y: row_idx.into(),
        }
    }

    pub fn origin() -> Vector2D<T> {
        Vector2D {
            x: T::zero(),
            y: T::zero(),
        }
    }
}

pub trait OrdDistance<T> {
    fn distance(&self, other: &Vector2D<T>) -> T;
}

impl<T: Copy + PartialOrd + num::Num> OrdDistance<T> for Vector2D<T> {
    fn distance(&self, other: &Vector2D<T>) -> T {
        let x_diff = if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };
        let y_diff = match self.y > other.y {
            true => self.y - other.y,
            false => other.y - self.y,
        };
        x_diff + y_diff
    }
}

impl<T: num::Signed> Vector2D<T> {
    pub fn distance(&self, other: &Vector2D<T>) -> T {
        self.x.abs_sub(&other.x) + self.y.abs_sub(&other.y)
    }
}

impl<T> Display for Vector2D<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.y, self.x))
    }
}

impl<U, T: Add<Output = U>> Add for Vector2D<T> {
    type Output = Vector2D<U>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Direction> for Point {
    type Output = Option<Point>;

    fn add(self, rhs: Direction) -> Self::Output {
        let rhs_vector: Vector2D<i8> = rhs.into();
        Some(Point {
            x: self.x.checked_add_signed(rhs_vector.x as isize)?,
            y: self.y.checked_add_signed(rhs_vector.y as isize)?,
        })
    }
}

impl<T: Sub<Output = U>, U> Sub for Vector2D<T> {
    type Output = Vector2D<U>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: AddAssign> AddAssign for Vector2D<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, rhs: Direction) {
        let rhs_vector: Vector2D<i8> = rhs.into();
        self.x = self
            .x
            .checked_add_signed(rhs_vector.x as isize)
            .unwrap_or(0);
        self.y = self
            .y
            .checked_add_signed(rhs_vector.y as isize)
            .unwrap_or(0);
    }
}

impl<T: SubAssign> SubAssign for Vector2D<T>
where
    T: Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.y -= rhs.y;
        self.y -= rhs.y;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_rotate() {
        assert_eq!(Direction::North.rotate(90.0), Direction::West);
        assert_eq!(Direction::North.rotate(180.0), Direction::South);
        assert_eq!(Direction::North.rotate(270.0), Direction::East);
        assert_eq!(Direction::North.rotate(-90.0), Direction::East);

        assert_eq!(Direction::West.rotate(90.0), Direction::South);
        assert_eq!(Direction::West.rotate(180.0), Direction::East);
        assert_eq!(Direction::West.rotate(270.0), Direction::North);
        assert_eq!(Direction::West.rotate(-90.0), Direction::North);
    }

    #[test]
    fn test_distance() {
        assert_eq!(Point::new(0usize, 0).distance(&Point::new(1usize, 1)), 2);
        assert_eq!(Point::new(10usize, 0).distance(&Point::new(0usize, 0)), 10);
    }
}
