use crate::scenes::line_segment_intersection::logic::{LineSegment, LineSegmentId};
use std::collections::HashMap;

pub struct Input<'a> {
    pub segments: HashMap<LineSegmentId, &'a LineSegment>,
}
