use crate::scenes::line_segment_intersection::logic::sweep_line::input::Input;
use crate::scenes::line_segment_intersection::logic::{LineSegment, LineSegmentId};
use common::nannou::prelude::*;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct StatusItem {
    line_segment_id: LineSegmentId,
    downward_direction: Vec2,
}

impl StatusItem {
    pub fn new(line_segment_id: LineSegmentId, downward_direction: Vec2) -> Self {
        assert!(downward_direction.is_finite());
        Self {
            line_segment_id,
            downward_direction,
        }
    }
}

pub struct Status {
    items: Vec<StatusItem>,
}

impl Status {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push(&mut self, current_point: Point2, item: StatusItem, input: &Input) {
        self.items.push(item);
        // TODO: binary search
        let y = current_point.y;
        self.items.sort_by(|item1, item2| {
            let s1 = input.segments[&item1.line_segment_id];
            let s2 = input.segments[&item2.line_segment_id];
            let x1 = Self::find_x(s1, y);
            let x2 = Self::find_x(s2, y);
            match x1.partial_cmp(&x2).unwrap() {
                Ordering::Equal => {
                    let d1x = item1.downward_direction.x;
                    let d2x = item2.downward_direction.x;
                    d1x.partial_cmp(&d2x).unwrap()
                }
                ord => ord,
            }
        });
    }

    fn find_x(s: &LineSegment, y: f32) -> f32 {
        let LineSegment { p1, p2 } = s;
        assert_ne!(p2.y - p1.y, 0.0); // TODO
        let slope_inv = (p2.x - p1.x) / (p2.y - p1.y);
        (y - p1.y) * slope_inv + p1.x
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        let s1 = LineSegment {
            p1: pt2(1.0, 1.0),
            p2: pt2(-1.0, -1.0),
        };
        let s2 = LineSegment {
            p1: pt2(-1.0, 1.0),
            p2: pt2(1.0, -1.0),
        };
        let s3 = LineSegment {
            p1: pt2(2.0, 1.0),
            p2: pt2(-2.0, -1.0),
        };
        let mut input = Input {
            segments: HashMap::new(),
        };
        input.segments.insert(0, &s1);
        input.segments.insert(1, &s2);
        input.segments.insert(2, &s3);

        let zero = pt2(0.0, 0.0);
        let mut status = Status::new();
        status.push(
            zero,
            StatusItem {
                line_segment_id: 0,
                downward_direction: s1.downward_direction(),
            },
            &input,
        );
        status.push(
            zero,
            StatusItem {
                line_segment_id: 1,
                downward_direction: s2.downward_direction(),
            },
            &input,
        );
        status.push(
            zero,
            StatusItem {
                line_segment_id: 2,
                downward_direction: s3.downward_direction(),
            },
            &input,
        );

        assert_eq!(status.items.len(), 3);
        assert_eq!(status.items[0].line_segment_id, 2);
        assert_eq!(status.items[1].line_segment_id, 0);
        assert_eq!(status.items[2].line_segment_id, 1);
    }
}
