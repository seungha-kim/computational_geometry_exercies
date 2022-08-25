use crate::scenes::line_segment_intersection::logic::LineSegmentId;
use common::nannou::geom::Point2;
use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq)]
pub struct EventPointKey(pub Point2);

impl Eq for EventPointKey {}

impl PartialOrd for EventPointKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let x_cmp = self.0.x.partial_cmp(&other.0.x);
        let y_cmp = self.0.y.partial_cmp(&other.0.y);
        match (y_cmp, x_cmp) {
            (Some(Ordering::Equal), Some(ord)) => Some(ord),
            (Some(ord), _) => Some(ord.reverse()),
            _ => panic!("Cannot determine order"),
        }
    }
}

impl Ord for EventPointKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct EventPoint {
    pub as_upper_endpoint: HashSet<LineSegmentId>,
    pub as_lower_endpoint: HashSet<LineSegmentId>,
    pub as_interior: HashSet<LineSegmentId>,
}

impl EventPoint {
    pub fn new() -> Self {
        Self {
            as_upper_endpoint: HashSet::new(),
            as_lower_endpoint: HashSet::new(),
            as_interior: HashSet::new(),
        }
    }
}
