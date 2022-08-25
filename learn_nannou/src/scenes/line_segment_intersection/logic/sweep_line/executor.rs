use super::super::{LineSegment, LineSegmentIntersectionResult};
use super::distinct_point::DistinctPoint;
use super::event_queue::{EventPoint, EventPointKey};
use super::status::{Status, StatusItem};
use super::Input;
use crate::scenes::line_segment_intersection::logic::LineSegmentId;
use common::nannou::prelude::*;
use std::collections::{BTreeMap, BTreeSet, Bound};

pub struct Executor<'a> {
    input: Input<'a>,
    event_queue: BTreeMap<EventPointKey, EventPoint>,
    status: Status,
    output: BTreeSet<DistinctPoint>,
}

impl<'a> Executor<'a> {
    pub fn new(input: Input<'a>) -> Self {
        Self {
            input,
            event_queue: BTreeMap::new(),
            status: Status::new(),
            output: BTreeSet::new(),
        }
    }

    pub fn calc_result(mut self) -> LineSegmentIntersectionResult {
        self.initialize_event_queue();
        while let Some(key) = self.event_queue.keys().cloned().next() {
            let event_point = self.event_queue.remove(&key).unwrap();
            self.handle_event_point(key.0, event_point);
        }
        LineSegmentIntersectionResult {
            intersections: self.output.into_iter().map(|item| item.0).collect(),
        }
        // // 다음 단계에서 이전 단계의 status item 을 찾아서 순서를 바꿔줘야 하는데... 어떻게 해야 할까
        // // -> 순서가 바깥 정보에 의존성이 있으니, 그냥 직접 binary tree 를 구현하자
        //
        // while let Some(key) = self.event_queue.keys().cloned().next() {
        //     let event = self.event_queue.remove(&key).unwrap();
        //     // LOWER -> INTERIOR -> UPPER ?
        //     // LOWER
        //
        //     for id in &event.as_lower_endpoint {
        //         let s = self.input.segments[&id];
        //         let item = StatusItem::new(*id, s.downward_direction());
        //         self.status.push(item.clone());
        //         // 좌우
        //         let self_line = self.input.segments.get(id).unwrap();
        //         if let Some(left) = self
        //             .status
        //             .range((Bound::Excluded(&item), Bound::Unbounded))
        //             .next()
        //         {
        //             let left_line = self.input.segments.get(&left.line_segment_id).unwrap();
        //             if let Some(intersection) = LineSegment::find_intersection(self_line, left_line)
        //             {
        //                 intersections.insert(DistinctPoint(intersection));
        //                 // TODO: 이미 존재할 수 있음
        //                 let mut new_event = EventPoint::new();
        //                 new_event.as_interior.push(*id);
        //                 new_event.as_interior.push(left_line.id);
        //                 self.event_queue
        //                     .insert(EventPointKey(intersection), new_event);
        //             }
        //         }
        //         if let Some(right) = status
        //             .range((Bound::Unbounded, Bound::Excluded(&item)))
        //             .next()
        //         {
        //             let right_line = self.input.segments.get(&right.line_segment_id).unwrap();
        //             if let Some(intersection) =
        //                 LineSegment::find_intersection(self_line, right_line)
        //             {
        //                 intersections.insert(DistinctPoint(intersection));
        //                 // TODO: 이미 존재할 수 있음
        //                 let mut new_event = EventPoint::new();
        //                 new_event.as_interior.push(*id);
        //                 new_event.as_interior.push(right_line.id);
        //                 self.event_queue
        //                     .insert(EventPointKey(intersection), new_event);
        //             }
        //         }
        //     }
        //     // UPPER
        //     for id in &event.as_upper_endpoint {
        //         let s = &self.input.segments[&id];
        //         let item = StatusItem::new(*id, s.downward_direction());
        //         self.status.push(item.clone());
        //         // 좌우
        //         let self_line = self.input.segments.get(id).unwrap();
        //         if let Some(left) = self
        //             .status
        //             .range((Bound::Excluded(&item), Bound::Unbounded))
        //             .next()
        //         {
        //             let left_line = self.segments.get(&left.line_segment_id).unwrap();
        //             if let Some(intersection) = LineSegment::find_intersection(self_line, left_line)
        //             {
        //                 intersections.push(intersection);
        //                 // TODO: 이미 존재할 수 있음
        //                 let mut new_event = EventPoint::new();
        //                 new_event.as_interior.push(*id);
        //                 new_event.as_interior.push(left_line.id);
        //                 self.event_queue
        //                     .insert(EventPointKey(intersection), new_event);
        //             }
        //         }
        //         if let Some(right) = status
        //             .range((Bound::Unbounded, Bound::Excluded(&item)))
        //             .next()
        //         {
        //             let right_line = self.segments.get(&right.line_segment_id).unwrap();
        //             if let Some(intersection) =
        //                 LineSegment::find_intersection(self_line, right_line)
        //             {
        //                 intersections.insert(DistinctPoint(intersection));
        //                 // TODO: 이미 존재할 수 있음
        //                 let mut new_event = EventPoint::new();
        //                 new_event.as_interior.push(*id);
        //                 new_event.as_interior.push(right_line.id);
        //                 self.event_queue
        //                     .insert(EventPointKey(intersection), new_event);
        //             }
        //         }
        //     }
        //     // INTERIOR
        // }
        //
        // LineSegmentIntersectionResult {
        //     intersections: intersections.into_iter().collect(),
        // }
    }

    fn initialize_event_queue(&mut self) {
        for (id, s) in &self.input.segments {
            let k1 = EventPointKey(s.p1);
            let k2 = EventPointKey(s.p2);
            let (upper_endpoint, lower_endpoint) = if k1 < k2 { (k1, k2) } else { (k2, k1) };

            {
                if !self.event_queue.contains_key(&upper_endpoint) {
                    let event_point = EventPoint::new();
                    self.event_queue.insert(upper_endpoint.clone(), event_point);
                }
                let mut ep = self.event_queue.get_mut(&upper_endpoint).unwrap();
                ep.as_upper_endpoint.insert(*id);
            }
            {
                if !self.event_queue.contains_key(&lower_endpoint) {
                    let event_point = EventPoint::new();
                    self.event_queue.insert(lower_endpoint, event_point);
                }
                let mut ep = self.event_queue.get_mut(&lower_endpoint).unwrap();
                ep.as_lower_endpoint.insert(*id);
            }
        }
    }

    fn handle_event_point(&mut self, point: Point2, event_point: EventPoint) {
        let upper_count = event_point.as_upper_endpoint.len();
        let lower_count = event_point.as_lower_endpoint.len();
        let interior_count = event_point.as_interior.len();
        if upper_count + lower_count + interior_count > 1 {
            self.output.insert(DistinctPoint(point.clone()));
        }
        for id in &event_point.as_lower_endpoint {
            self.status.remove(*id);
        }
        for id in &event_point.as_interior {
            let item = self.status.remove(*id).unwrap();
            self.status.push(item);
        }
        for id in event_point.as_upper_endpoint {
            let dir_x = self.input.segments[&id].downward_direction().x;
            let item = StatusItem::new(id, dir_x);
            self.status.push(item);
        }
        self.status.sort(point, &self.input);
        if upper_count + interior_count == 0 {
            if let (Some(left_item), Some(right_item)) =
                self.status.find_left_and_right(point, &self.input)
            {
                self.find_new_event(left_item.line_segment_id, right_item.line_segment_id, point);
            } else {
                // TODO: ...
            }
        }
    }

    fn find_new_event(
        &mut self,
        s1_id: LineSegmentId,
        s2_id: LineSegmentId,
        current_point: Point2,
    ) {
        let s1 = self.input.segments[&s1_id];
        let s2 = self.input.segments[&s2_id];
        if let Some(intersection) = LineSegment::find_intersection(s1, s2) {
            if intersection.y < current_point.y
                || (intersection.y == current_point.y && intersection.x > current_point.x)
            {
                let key = EventPointKey(current_point);
                if !self.event_queue.contains_key(&key) {
                    self.event_queue.insert(key, EventPoint::new());
                }
                {
                    let event_point = self.event_queue.get_mut(&key).unwrap();
                    event_point.as_interior.insert(s1_id);
                    event_point.as_interior.insert(s2_id);
                }
            }
        }
    }
}
