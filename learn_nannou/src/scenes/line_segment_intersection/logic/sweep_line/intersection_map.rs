use crate::scenes::line_segment_intersection::logic::LineSegmentId;
use common::nannou::prelude::*;
use std::collections::HashMap;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct SegmentPair(LineSegmentId, LineSegmentId);

impl SegmentPair {
    fn new(id1: LineSegmentId, id2: LineSegmentId) -> Self {
        if id1 == id2 {
            panic!("two id must be different");
        }
        let (small, big) = if id1 < id2 { (id1, id2) } else { (id2, id1) };
        Self(id1, id2)
    }
}

pub struct IntersectionMap {
    map: HashMap<SegmentPair, Point2>,
}

impl IntersectionMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn lookup(&self, id1: LineSegmentId, id2: LineSegmentId) -> Option<&Point2> {
        self.map.get(&SegmentPair::new(id1, id2))
    }
    pub fn insert(&mut self, id1: LineSegmentId, id2: LineSegmentId, point: Point2) {
        self.map.insert(SegmentPair::new(id1, id2), point);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut map = IntersectionMap::new();
        map.insert(0, 1, pt2(0.0, 0.0));
        assert_eq!(map.lookup(0, 1), Some(&pt2(0.0, 0.0)));
        assert!(map.lookup(0, 2).is_none());
    }
}
