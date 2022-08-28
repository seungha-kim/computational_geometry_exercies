use super::super::{LineSegment, LineSegmentIntersectionResult};
use super::distinct_point::DistinctPoint;
use super::event_queue::{EventData, EventQueue};
use super::intersection_map::IntersectionCache;
use super::status::{Status, StatusItem};
use super::Input;
use crate::scenes::line_segment_intersection::logic::LineSegmentId;
use common::nannou::prelude::*;
use std::collections::{BTreeMap, BTreeSet, Bound};

pub struct Executor<'a> {
    input: &'a Input<'a>,
    event_queue: EventQueue<'a>,
    status: Status,
    intersection_cache: IntersectionCache,
    output: BTreeSet<DistinctPoint>,
}

impl<'a> Executor<'a> {
    pub fn new(input: &'a Input<'a>) -> Self {
        Self {
            input,
            event_queue: EventQueue::new(input),
            status: Status::new(),
            intersection_cache: IntersectionCache::new(),
            output: BTreeSet::new(),
        }
    }

    pub fn calc_result(mut self) -> LineSegmentIntersectionResult {
        self.initialize_event_queue();
        while let Some((point, data)) = self.event_queue.dequeue_point() {
            self.handle_event_point(point, data);
        }
        LineSegmentIntersectionResult {
            intersections: self.output.iter().map(|item| item.0).collect(),
        }
    }

    fn initialize_event_queue(&mut self) {
        for id in self.input.segments.keys() {
            self.event_queue.insert_segment(*id);
        }
    }

    fn handle_event_point(&mut self, point: Point2, event_point: EventData) {
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
        let intersection_opt = {
            if let Some(p) = self.intersection_cache.lookup(s1_id, s2_id) {
                Some(*p)
            } else {
                if let Some(intersection) = LineSegment::find_interior_intersection(s1, s2) {
                    self.intersection_cache.insert(s1_id, s2_id, intersection);
                    self.output.insert(DistinctPoint(intersection));
                    Some(intersection)
                } else {
                    None
                }
            }
        };
        if let Some(intersection) = intersection_opt {
            if intersection.y < current_point.y
                || (intersection.y == current_point.y && intersection.x > current_point.x)
            {
                self.event_queue
                    .insert_intersection(intersection, s1_id, s2_id);
            }
        }
    }
}
