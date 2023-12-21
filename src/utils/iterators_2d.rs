use super::Point;

pub trait Indexed2D<'a, T> {
    type Iter;

    fn get_point(&'a self, pt: Point) -> Option<&'a T>;
    fn swap_points(&'a mut self, pt1: Point, pt2: Point);
    fn get_point_mut(&'a mut self, pt: Point) -> Option<&'a mut T>;
    fn iter_2d(&'a self) -> Self::Iter;
}

impl<'a, T: std::default::Default + 'a> Indexed2D<'a, T> for [Vec<T>] {
    type Iter = VecIter2D<'a, T>;

    fn get_point(&'a self, pt: Point) -> Option<&'a T> {
        let row = self.get(pt.y)?;
        row.get(pt.x)
    }

    fn swap_points(&'a mut self, pt1: Point, pt2: Point) {
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
}
