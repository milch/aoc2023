use super::{OptionFlatMap, Point, Vector2D};

pub trait Indexed2D<'a, T> {
    type Iter;

    fn get_point(&'a self, pt: Point) -> Option<&'a T>;
    fn get_point_wrap(&'a self, pt: Vector2D<isize>) -> &'a T;
    fn swap_points(&'a mut self, pt1: Point, pt2: Point)
    where
        T: Default;
    fn get_point_mut(&'a mut self, pt: Point) -> Option<&'a mut T>;
    fn iter_2d(&'a self) -> Self::Iter;
}

impl<'a, T: 'a> Indexed2D<'a, T> for [Vec<T>] {
    type Iter = VecIter2D<'a, T>;

    fn get_point(&'a self, pt: Point) -> Option<&'a T> {
        let row = self.get(pt.y)?;
        row.get(pt.x)
    }

    fn swap_points(&'a mut self, pt1: Point, pt2: Point)
    where
        T: Default,
    {
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

    fn get_point_mut(&'a mut self, pt: Point) -> Option<&'a mut T> {
        let row = self.get_mut(pt.y)?;
        row.get_mut(pt.x)
    }

    fn iter_2d(&'a self) -> Self::Iter {
        VecIter2D {
            col_iter: None,
            row_iter: self.iter(),
            row: 0,
            col: 0,
        }
    }

    fn get_point_wrap(&'a self, pt: Vector2D<isize>) -> &'a T {
        let bounds = (self.len() as isize, self[0].len() as isize);
        let x = if pt.x >= 0 {
            pt.x % bounds.1
        } else {
            (bounds.1 - 1) - ((pt.x + 1) % bounds.1).abs()
        };
        let y = if pt.y >= 0 {
            pt.y % bounds.0
        } else {
            (bounds.0 - 1) - ((pt.y + 1) % bounds.0).abs()
        };

        self.get(y as usize)
            .flat_map(|row| row.get(x as usize))
            .unwrap()
    }
}

pub struct VecIter2D<'a, T> {
    row_iter: std::slice::Iter<'a, Vec<T>>,
    col_iter: Option<std::slice::Iter<'a, T>>,
    row: usize,
    col: usize,
}

impl<'a, T> Iterator for VecIter2D<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(col) = &mut self.col_iter {
            let next = col.next();
            if next.is_some() {
                self.col += 1;
                return next;
            }
        }
        self.col_iter = match self.row_iter.next() {
            Some(next_col) => {
                self.col = 0;
                self.row += 1;
                Some(next_col.iter())
            }
            None => return None,
        };

        match &mut self.col_iter {
            Some(col) => {
                self.col += 1;
                col.next()
            }
            None => None,
        }
    }
}

pub trait Enumerable2D<'a, T> {
    fn enumerate_2d(self) -> Enumerate2D<'a, T>
    where
        Self: Sized;
}

impl<'a, T> Enumerable2D<'a, T> for VecIter2D<'a, T> {
    fn enumerate_2d(self) -> Enumerate2D<'a, T>
    where
        Self: Sized,
    {
        Enumerate2D { iter: self }
    }
}

pub struct Enumerate2D<'a, T> {
    iter: VecIter2D<'a, T>,
}

impl<'a, T> Iterator for Enumerate2D<'a, T> {
    type Item = (Point, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next()?;
        // Counts are off by one
        let point = Point::new(self.iter.col - 1, self.iter.row - 1);

        Some((point, next))
    }
}

pub trait ToMatrix<T> {
    fn matrix(&self) -> Vec<Vec<T>>;
}

pub trait ToMatrixParse<T> {
    fn matrix_parse(&self, parse: fn(char) -> T) -> Vec<Vec<T>>;
}

impl<T> ToMatrix<T> for &str
where
    T: From<char>,
{
    fn matrix(&self) -> Vec<Vec<T>> {
        self.matrix_parse(T::from)
    }
}

impl<T> ToMatrixParse<T> for &str {
    fn matrix_parse(&self, parse: fn(char) -> T) -> Vec<Vec<T>> {
        self.lines()
            .map(|l| l.chars().map(parse).collect())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_iterate_2d() {
        let matrix = vec![vec![1, 2], vec![6], vec![8, 9, 10]];
        let mut matrix_it = matrix.iter_2d();

        assert_eq!(matrix_it.next(), Some(&1));
        assert_eq!(matrix_it.next(), Some(&2));
        assert_eq!(matrix_it.next(), Some(&6));
        assert_eq!(matrix_it.next(), Some(&8));
        assert_eq!(matrix_it.next(), Some(&9));
        assert_eq!(matrix_it.next(), Some(&10));
    }

    #[test]
    fn test_enumerate() {
        let matrix = vec![vec![1, 2], vec![6], vec![8, 9, 10]];
        let mut matrix_it = matrix.iter_2d().enumerate_2d().map(|(pt, _)| pt);

        assert_eq!(matrix_it.next(), Some(Point::new(0usize, 0)));
        assert_eq!(matrix_it.next(), Some(Point::new(1usize, 0)));
        assert_eq!(matrix_it.next(), Some(Point::new(0usize, 1)));
        assert_eq!(matrix_it.next(), Some(Point::new(0usize, 2)));
        assert_eq!(matrix_it.next(), Some(Point::new(1usize, 2)));
        assert_eq!(matrix_it.next(), Some(Point::new(2usize, 2)));
    }

    #[test]
    fn test_get_point_wrap() {
        let matrix = vec![vec![1, 2], vec![4, 5], vec![7, 8]];

        // Normal indices
        assert_eq!(matrix.get_point_wrap(Vector2D::new(0isize, 0)), &1);
        assert_eq!(matrix.get_point_wrap(Vector2D::new(1isize, 0)), &2);
        assert_eq!(matrix.get_point_wrap(Vector2D::new(0isize, 1)), &4);
        assert_eq!(matrix.get_point_wrap(Vector2D::new(1isize, 1)), &5);
        assert_eq!(matrix.get_point_wrap(Vector2D::new(0isize, 2)), &7);
        assert_eq!(matrix.get_point_wrap(Vector2D::new(1isize, 2)), &8);

        // Negative wraparound
        assert_eq!(matrix.get_point_wrap(Vector2D::new(-1isize, 0)), &2);
        assert_eq!(matrix.get_point_wrap(Vector2D::new(-2isize, 0)), &1);
        assert_eq!(matrix.get_point_wrap(Vector2D::new(-3isize, 0)), &2);
        assert_eq!(matrix.get_point_wrap(Vector2D::new(-4isize, 0)), &1);
        assert_eq!(matrix.get_point_wrap(Vector2D::new(-4isize, -1)), &7);
        assert_eq!(matrix.get_point_wrap(Vector2D::new(-4isize, -2)), &4);
        assert_eq!(matrix.get_point_wrap(Vector2D::new(-4isize, -3)), &1);
        assert_eq!(matrix.get_point_wrap(Vector2D::new(-4isize, -4)), &7);

        // Positive wraparound
        assert_eq!(matrix.get_point_wrap(Vector2D::new(2isize, 0)), &1);
        assert_eq!(matrix.get_point_wrap(Vector2D::new(3isize, 0)), &2);
        assert_eq!(matrix.get_point_wrap(Vector2D::new(2isize, 3)), &1);
        assert_eq!(matrix.get_point_wrap(Vector2D::new(2isize, 4)), &4);
        assert_eq!(matrix.get_point_wrap(Vector2D::new(2isize, 5)), &7);
    }
}
