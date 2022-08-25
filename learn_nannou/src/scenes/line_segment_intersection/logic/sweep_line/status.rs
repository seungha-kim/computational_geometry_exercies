use crate::scenes::line_segment_intersection::logic::sweep_line::input::Input;
use crate::scenes::line_segment_intersection::logic::{LineSegment, LineSegmentId};
use common::nannou::prelude::*;
use std::cmp::{max, Ordering};

#[derive(Clone)]
pub struct StatusItem {
    pub line_segment_id: LineSegmentId,
    normalized_downward_dir_x: f32,
}

impl StatusItem {
    pub fn new(line_segment_id: LineSegmentId, normalized_downward_dir_x: f32) -> Self {
        assert!(normalized_downward_dir_x > -1.0);
        assert!(normalized_downward_dir_x <= 1.0);
        Self {
            line_segment_id,
            normalized_downward_dir_x,
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

    pub fn push(&mut self, item: StatusItem) {
        self.items.push(item);
    }

    pub fn sort(&mut self, current_point: Point2, input: &Input) {
        // TODO: manual binary search?
        let y = current_point.y;
        self.items.sort_by(|item1, item2| {
            let s1 = input.segments[&item1.line_segment_id];
            let s2 = input.segments[&item2.line_segment_id];
            let x1 = Self::find_x(s1, y);
            let x2 = Self::find_x(s2, y);
            match x1.partial_cmp(&x2).unwrap() {
                Ordering::Equal => {
                    let d1x = item1.normalized_downward_dir_x;
                    let d2x = item2.normalized_downward_dir_x;
                    d1x.partial_cmp(&d2x).unwrap()
                }
                ord => ord,
            }
        });
    }

    pub fn find_left_and_right(
        &self,
        current_point: Point2,
        input: &Input,
    ) -> (Option<&StatusItem>, Option<&StatusItem>) {
        let index = self
            .items
            .binary_search_by(|item| {
                let segment = input.segments[&item.line_segment_id];
                Self::find_x(segment, current_point.y)
                    .partial_cmp(&current_point.x)
                    .unwrap()
            })
            .expect_err("must not have current point for this stage");
        let left_item = if index > 0 {
            Some(&self.items[index - 1])
        } else {
            None
        };
        let right_item = if index < self.items.len() {
            Some(&self.items[index])
        } else {
            None
        };
        (left_item, right_item)
    }

    pub fn remove(&mut self, segment_id: LineSegmentId) -> Option<StatusItem> {
        self.items
            .iter()
            .enumerate()
            .find(|(_, item)| item.line_segment_id == segment_id)
            .map(|(index, _)| index)
            .map(|index| self.items.remove(index))
    }

    fn find_x(s: &LineSegment, y: f32) -> f32 {
        let LineSegment { p1, p2 } = s;
        if p2.y - p1.y == 0.0 {
            // NOTE: 알고리즘 상 교점이 있을거라는 가정 + 큰 것 기준으로 정렬하겠다는 기준
            return p1.x.max(p2.x);
        }
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
        status.push(StatusItem {
            line_segment_id: 0,
            normalized_downward_dir_x: s1.downward_direction().x,
        });
        status.push(StatusItem {
            line_segment_id: 1,
            normalized_downward_dir_x: s2.downward_direction().x,
        });
        status.push(StatusItem {
            line_segment_id: 2,
            normalized_downward_dir_x: s3.downward_direction().x,
        });
        status.sort(zero, &input);

        assert_eq!(status.items.len(), 3);
        assert_eq!(status.items[0].line_segment_id, 2);
        assert_eq!(status.items[1].line_segment_id, 0);
        assert_eq!(status.items[2].line_segment_id, 1);
    }
}
