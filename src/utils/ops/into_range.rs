use std::ops::{Bound, Range, RangeBounds};

pub trait RangeIdx: Clone {
    fn next(&self) -> Self;
}

impl RangeIdx for usize {
    fn next(&self) -> Self {
        self + 1
    }
}

pub trait IntoRange<Idx: RangeIdx>: RangeBounds<Idx> + Sized {
    fn into_range(self, unbounded: Range<Idx>) -> Range<Idx> {
        let start = match self.start_bound() {
            Bound::Included(start) => start.clone(),
            Bound::Excluded(start) => start.next(),
            Bound::Unbounded => unbounded.start,
        };

        let end = match self.end_bound() {
            Bound::Included(end) => end.next(),
            Bound::Excluded(end) => end.clone(),
            Bound::Unbounded => unbounded.end,
        };

        Range { start, end }
    }
}

// Blanket Implementation
impl<Idx: RangeIdx, Rng: RangeBounds<Idx>> IntoRange<Idx> for Rng {}

#[cfg(test)]
mod tests {
    use super::*;

    fn range_eq<L, R>(left: L, right: R) -> bool
    where
        L: IntoRange<usize>,
        R: IntoRange<usize>,
    {
        left.into_range(0..10) == right.into_range(0..10)
    }

    #[test]
    fn into_range() {
        assert!(range_eq(1..5, 1..5), "1..5 is not equal to 1..5");
        assert!(range_eq(1..=5, 1..6), "1..=5 is not equal to 1..6");
        assert!(range_eq(1.., 1..10), "1.. is not equal to 1..10");
        assert!(range_eq(..5, 0..5), "..5 is not equal to 0..5");
        assert!(range_eq(.., 0..10), ".. is not equal to 0..10");
    }
}