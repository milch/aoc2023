use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Copy, Clone)]
pub struct Direction {
    pub(crate) dx: isize,
    pub(crate) dy: isize,
}

impl Direction {
    pub fn north() -> Self {
        Self { dx: 0, dy: -1 }
    }
    pub fn south() -> Self {
        Self { dx: 0, dy: 1 }
    }
    pub fn east() -> Self {
        Self { dx: 1, dy: 0 }
    }
    pub fn west() -> Self {
        Self { dx: -1, dy: 0 }
    }

    pub fn opposite(&self) -> Self {
        Self {
            dx: -self.dx,
            dy: -self.dy,
        }
    }

    pub fn is_y(&self) -> bool {
        self.dy != 0
    }

    pub fn is_x(&self) -> bool {
        self.dx != 0
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl Point {
    pub fn new<N>(col_idx: N, row_idx: N) -> Point
    where
        N: Into<usize>,
    {
        Point {
            x: col_idx.into(),
            y: row_idx.into(),
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Self::Output {
        Point {
            x: self.x.checked_add_signed(rhs.dx).unwrap_or(0),
            y: self.y.checked_add_signed(rhs.dy).unwrap_or(0),
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, rhs: Direction) {
        self.x = self.x.checked_add_signed(rhs.dx).unwrap_or(0);
        self.y = self.y.checked_add_signed(rhs.dy).unwrap_or(0);
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.y -= rhs.y;
        self.y -= rhs.y;
    }
}

pub trait Indexed2D<T> {
    fn get_point(&self, pt: Point) -> Option<&T>;
    fn swap_points(&mut self, pt1: Point, pt2: Point);
    fn get_point_mut(&mut self, pt: Point) -> Option<&mut T>;
}

impl<T: std::default::Default> Indexed2D<T> for [Vec<T>] {
    fn get_point(&self, pt: Point) -> Option<&T> {
        let row = self.get(pt.y)?;
        row.get(pt.x)
    }

    fn get_point_mut(&mut self, pt: Point) -> Option<&mut T> {
        let row = self.get_mut(pt.y)?;
        row.get_mut(pt.x)
    }

    fn swap_points(&mut self, pt1: Point, pt2: Point) {
        if pt1.y == pt2.y {
            // If the elements are in the same row, we can just swap them directly
            self[pt1.y].swap(pt1.x, pt2.x);
        } else {
            // ... Otherwise take out the elements and place them back in reverse order
            let item1 = std::mem::take(self.get_point_mut(pt1).unwrap());
            let item2 = std::mem::take(self.get_point_mut(pt2).unwrap());
            self[pt1.y][pt1.x] = item2;
            self[pt2.y][pt2.x] = item1;
        }
    }
}
