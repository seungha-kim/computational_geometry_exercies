use common::nannou::prelude::*;
use std::cmp::Ordering;

#[derive(PartialEq)]
pub struct DistinctPoint(pub Point2);

impl DistinctPoint {
    pub fn new(point: Point2) -> Self {
        assert!(point.x.is_finite());
        assert!(point.y.is_finite());
        Self(point)
    }
}

impl Eq for DistinctPoint {}

impl PartialOrd for DistinctPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.0.x.partial_cmp(&other.0.x) {
            Some(Ordering::Equal) => self.0.y.partial_cmp(&other.0.y),
            ord => ord,
        }
    }
}

impl Ord for DistinctPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
