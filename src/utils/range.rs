use std::ops::{Bound, Range, RangeBounds, RangeInclusive, RangeTo, RangeToInclusive, Sub};

pub trait RangeIntersection<Idx: Copy + Ord, Rhs = Self> {
    type RangeType;
    fn intersect(&self, other: &Rhs) -> Option<Self::RangeType>;
}

// Delicious copy-pasta thanks to range types all being incompatible with each other

impl<Idx: Copy + Ord, Rhs: RangeBounds<Idx>> RangeIntersection<Idx, Rhs> for Range<Idx> {
    type RangeType = Range<Idx>;

    fn intersect(&self, other: &Rhs) -> Option<Self::RangeType> {
        let start = match (self.start, other.start_bound()) {
            (l, Bound::Included(r)) => l.max(*r),
            (l, Bound::Unbounded) => l,

            _ => panic!("Unsupported combination of bounds"),
        };

        let end = match (self.end, other.end_bound()) {
            (l, Bound::Excluded(r)) => l.min(*r),
            (l, Bound::Unbounded) => l,

            _ => panic!("Unsupported combination of bounds"),
        };

        if start < end {
            Some(start..end)
        } else {
            None
        }
    }
}

impl<Idx: Copy + Ord, Rhs: RangeBounds<Idx>> RangeIntersection<Idx, Rhs> for RangeInclusive<Idx> {
    type RangeType = RangeInclusive<Idx>;

    fn intersect(&self, other: &Rhs) -> Option<Self::RangeType> {
        let start = match (self.start(), other.start_bound()) {
            (l, Bound::Included(r)) => l.max(r),
            (l, Bound::Unbounded) => l,

            _ => panic!("Unsupported combination of bounds"),
        };

        let end = match (self.end(), other.end_bound()) {
            (l, Bound::Included(r)) => l.min(r),
            (l, Bound::Unbounded) => l,

            _ => panic!("Unsupported combination of bounds"),
        };

        if start <= end {
            Some(*start..=*end)
        } else {
            None
        }
    }
}

impl<Idx: Copy + Ord> RangeIntersection<Idx, Range<Idx>> for RangeTo<Idx> {
    type RangeType = Range<Idx>;

    fn intersect(&self, other: &Range<Idx>) -> Option<Self::RangeType> {
        let start = other.start;
        let end = self.end.min(other.end);

        if start < end {
            Some(start..end)
        } else {
            None
        }
    }
}

impl<Idx: Copy + Ord> RangeIntersection<Idx, RangeInclusive<Idx>> for RangeToInclusive<Idx> {
    type RangeType = RangeInclusive<Idx>;

    fn intersect(&self, other: &RangeInclusive<Idx>) -> Option<Self::RangeType> {
        let start = *other.start();
        let end = self.end.min(*other.end());

        if start <= end {
            Some(start..=end)
        } else {
            None
        }
    }
}

pub trait RangeSplit<Idx: Copy + Ord> {
    type Output;
    /// Splits so that elements in result.0 satisfy `< at`
    fn split_lower(&self, at: Idx) -> (Option<Self::Output>, Option<Self::Output>);

    /// Splits so that elements in result.1 satisfy `> at`
    fn split_upper(&self, at: Idx) -> (Option<Self::Output>, Option<Self::Output>);
}

impl<Idx: Copy + Ord + num::Num> RangeSplit<Idx> for RangeInclusive<Idx> {
    type Output = Self;
    fn split_lower(&self, at: Idx) -> (Option<Self>, Option<Self>) {
        let left_half_end = at - Idx::one();
        let left_half = if *self.start() <= left_half_end {
            Some(*self.start()..=left_half_end.min(*self.end()))
        } else {
            None
        };
        let right_half = if at <= *self.end() {
            Some(at.max(*self.start())..=*self.end())
        } else {
            None
        };

        (left_half, right_half)
    }

    fn split_upper(&self, at: Idx) -> (Option<Self>, Option<Self>) {
        let right_half_start = at + Idx::one();
        let right_half = if right_half_start <= *self.end() {
            Some(right_half_start.max(*self.start())..=*self.end())
        } else {
            None
        };
        let left_half = if *self.start() <= at {
            Some(*self.start()..=at.min(*self.end()))
        } else {
            None
        };

        (right_half, left_half)
    }
}

pub trait RangeLen<Idx> {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

impl<Idx> RangeLen<Idx> for RangeInclusive<Idx>
where
    Idx: Sub<Idx, Output = usize> + Copy,
{
    fn len(&self) -> usize {
        *self.end() - *self.start() + 1
    }

    fn is_empty(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_len() {
        assert_eq!((0usize..=0).len(), 1);
        assert_eq!((0usize..=1).len(), 2);
        assert_eq!((0usize..=2).len(), 3);
    }

    #[test]
    fn test_intersect() {
        assert_eq!((0..3).intersect(&(1..3)), Some(1..3));
        assert_eq!((0..3).intersect(&(0..1)), Some(0..1));
        assert_eq!((0..5).intersect(&(2..10)), Some(2..5));
        assert_eq!((3..5).intersect(&(2..10)), Some(3..5));
        assert_eq!((3..5).intersect(&(2..10)), Some(3..5));

        assert_eq!((3..5).intersect(&(..10)), Some(3..5));
        assert_eq!((3..5).intersect(&(..4)), Some(3..4));
        assert_eq!((..5).intersect(&(2..5)), Some(2..5));
        assert_eq!((..3).intersect(&(2..5)), Some(2..3));
        assert_eq!((..2).intersect(&(2..5)), None);

        assert_eq!((3..=5).intersect(&(2..=6)), Some(3..=5));
        assert_eq!((0..3).intersect(&(3..5)), None);

        assert_eq!((3..=5).intersect(&(0..=1)), None);
        assert_eq!((3..=5).intersect(&(0..=2)), None);
        assert_eq!((3..=5).intersect(&(0..=3)), Some(3..=3));
        assert_eq!((3..=5).intersect(&(0..=4)), Some(3..=4));
        assert_eq!((3..=5).intersect(&(0..=5)), Some(3..=5));
        assert_eq!((3..=5).intersect(&(0..=6)), Some(3..=5));
        assert_eq!((3..=5).intersect(&(1..=6)), Some(3..=5));
        assert_eq!((3..=5).intersect(&(2..=6)), Some(3..=5));
        assert_eq!((3..=5).intersect(&(3..=6)), Some(3..=5));
        assert_eq!((3..=5).intersect(&(4..=6)), Some(4..=5));
        assert_eq!((3..=5).intersect(&(5..=6)), Some(5..=5));
        assert_eq!((3..=5).intersect(&(6..=6)), None);
    }

    #[test]
    fn test_split() {
        assert_eq!((0..=200).split_upper(100), (Some(101..=200), Some(0..=100)));
        assert_eq!((0..=200).split_lower(100), (Some(0..=99), Some(100..=200)));

        assert_eq!((2..=3).split_lower(0), (None, Some(2..=3)));
        assert_eq!((2..=3).split_lower(1), (None, Some(2..=3)));
        assert_eq!((2..=3).split_lower(2), (None, Some(2..=3)));
        assert_eq!((2..=3).split_lower(3), (Some(2..=2), Some(3..=3)));
        assert_eq!((2..=3).split_lower(4), (Some(2..=3), None));
        assert_eq!((2..=3).split_lower(5), (Some(2..=3), None));
        assert_eq!((2..=3).split_lower(6), (Some(2..=3), None));

        assert_eq!((0..=1).split_lower(1), (Some(0..=0), Some(1..=1)));
        assert_eq!((0..=1).split_lower(2), (Some(0..=1), None));
        assert_eq!((0..=1).split_lower(3), (Some(0..=1), None));

        assert_eq!((0..=1).split_upper(0), (Some(1..=1), Some(0..=0)));
        assert_eq!((0..=1).split_upper(1), (None, Some(0..=1)));
        assert_eq!((0..=1).split_upper(2), (None, Some(0..=1)));
        assert_eq!((0..=1).split_upper(3), (None, Some(0..=1)));

        assert_eq!((2..=3).split_upper(0), (Some(2..=3), None));
        assert_eq!((2..=3).split_upper(1), (Some(2..=3), None));
        assert_eq!((2..=3).split_upper(2), (Some(3..=3), Some(2..=2)));
        assert_eq!((2..=3).split_upper(3), (None, Some(2..=3),));
        assert_eq!((2..=3).split_upper(4), (None, Some(2..=3),));
        assert_eq!((2..=3).split_upper(5), (None, Some(2..=3),));
        assert_eq!((2..=3).split_upper(6), (None, Some(2..=3),));
    }
}
