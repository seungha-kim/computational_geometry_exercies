use crate::scenes::line_segment_intersection::logic::sweep_line::input::Input;
use crate::scenes::line_segment_intersection::logic::{LineSegment, LineSegmentId};
use common::nannou::prelude::*;
use std::cmp::{max, Ordering};

/**

# Status

- Sweep line 의 특정 위치에서 선분을 왼쪽부터 오른쪽으로 정렬해놓은 상태를 표현하는 자료구조

# 정렬 순서

- Sweep line 과의 교점의 x 좌표가 작은 것부터
- x 좌표가 같아면, 기울기가 작은 것부터
- 모두 같다면?

*/

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

pub struct Status<'a> {
    items: Vec<StatusItem>,
    input: &'a Input<'a>,
}

impl<'a> Status<'a> {
    pub fn new(input: &'a Input<'a>) -> Self {
        Self {
            items: Vec::new(),
            input,
        }
    }

    // returns neighbors
    pub fn insert(
        &mut self,
        item: StatusItem,
        current_point: Point2,
    ) -> (Option<&StatusItem>, Option<&StatusItem>) {
        let y = current_point.y;
        let result = self.items.binary_search_by(|target| {
            let s1 = self.input.segments[&target.line_segment_id];
            let s2 = self.input.segments[&item.line_segment_id];
            let x1 = Self::find_x(s1, y);
            let x2 = Self::find_x(s2, y);
            match x1.partial_cmp(&x2).unwrap() {
                Ordering::Equal => {
                    let d1x = target.normalized_downward_dir_x;
                    let d2x = item.normalized_downward_dir_x;
                    d1x.partial_cmp(&d2x).unwrap()
                }
                ord => ord,
            }
        });
        let index = if result.is_ok() {
            result.unwrap()
        } else {
            result.unwrap_err()
        };
        self.items.insert(index, item);
        let left = if index > 0 {
            Some(&self.items[index - 1])
        } else {
            None
        };
        let right = if index < self.items.len() - 1 {
            Some(&self.items[index + 1])
        } else {
            None
        };
        (left, right)
    }

    // pub fn find_left_and_right(
    //     &self,
    //     current_point: Point2,
    //     input: &Input,
    // ) -> (Option<&StatusItem>, Option<&StatusItem>) {
    //     let index = self
    //         .items
    //         .binary_search_by(|item| {
    //             let segment = input.segments[&item.line_segment_id];
    //             Self::find_x(segment, current_point.y)
    //                 .partial_cmp(&current_point.x)
    //                 .unwrap()
    //         })
    //         .expect_err("must not have current point for this stage");
    //     let left_item = if index > 0 {
    //         Some(&self.items[index - 1])
    //     } else {
    //         None
    //     };
    //     let right_item = if index < self.items.len() {
    //         Some(&self.items[index])
    //     } else {
    //         None
    //     };
    //     (left_item, right_item)
    // }

    pub fn find_index(&self, id: LineSegmentId) -> usize {
        self.items
            .iter()
            .enumerate()
            .find(|(_, item)| item.line_segment_id == id)
            .map(|(index, _)| index)
            .unwrap()
    }

    /// returns (original_left_item, original_right_item, removed_item)
    pub fn remove(
        &mut self,
        segment_id: LineSegmentId,
    ) -> (Option<&StatusItem>, Option<&StatusItem>, StatusItem) {
        self.items
            .iter()
            .enumerate()
            .find(|(_, item)| item.line_segment_id == segment_id)
            .map(|(index, _)| index)
            .map(|index| {
                let removed = self.items.remove(index);
                let left = if index > 0 {
                    Some(&self.items[index - 1])
                } else {
                    None
                };
                let right = if index < self.items.len() {
                    Some(&self.items[index])
                } else {
                    None
                };
                (left, right, removed)
            })
            .unwrap()
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
        let mut input = Input {
            segments: HashMap::new(),
        };
        input.segments.insert(0, &s1);
        input.segments.insert(1, &s2);

        let zero = pt2(0.0, 0.0);
        let mut status = Status::new(&input);
        status.insert(
            StatusItem {
                line_segment_id: 0,
                normalized_downward_dir_x: s1.downward_direction().x,
            },
            pt2(-1.0, 1.0),
        );
        status.insert(
            StatusItem {
                line_segment_id: 1,
                normalized_downward_dir_x: s2.downward_direction().x,
            },
            pt2(1.0, 1.0),
        );

        assert_eq!(status.items.len(), 2);
        assert_eq!(status.items[0].line_segment_id, 1);
        assert_eq!(status.items[1].line_segment_id, 0);
    }
}
