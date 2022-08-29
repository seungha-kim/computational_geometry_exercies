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
    status: Status<'a>,
    intersection_cache: IntersectionCache,
    output: BTreeSet<DistinctPoint>,
}

impl<'a> Executor<'a> {
    pub fn new(input: &'a Input<'a>) -> Self {
        Self {
            input,
            event_queue: EventQueue::new(input),
            status: Status::new(input),
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
            let (left, right, _) = self.status.remove(*id);
            left.zip(right)
                .map(|(left, right)| (left.line_segment_id, right.line_segment_id))
                .map(|(left_id, right_id)| {
                    self.find_new_event(left_id, right_id, point);
                });
        }
        for id in &event_point.as_interior {
            let (left, right) = {
                let (_, _, item) = self.status.remove(*id);
                let (left, right) = self.status.insert(item, point);
                (
                    left.map(|item| item.line_segment_id),
                    right.map(|item| item.line_segment_id),
                )
            };
            left.map(|left_id| self.find_new_event(*id, left_id, point));
            right.map(|right_id| self.find_new_event(*id, right_id, point));
        }
        for id in &event_point.as_upper_endpoint {
            let dir_x = self.input.segments[&id].downward_direction().x;
            let item = StatusItem::new(*id, dir_x);
            let (left, right) = {
                let (left, right) = self.status.insert(item, point);
                (
                    left.map(|i| i.line_segment_id),
                    right.map(|i| i.line_segment_id),
                )
            };
            left.map(|left_id| self.find_new_event(*id, left_id, point));
            right.map(|right_id| self.find_new_event(*id, right_id, point));
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
        self.intersection_cache
            .lookup(s1_id, s2_id)
            .cloned()
            .or_else(|| {
                LineSegment::find_interior_intersection(s1, s2).map(|intersection| {
                    self.intersection_cache.insert(s1_id, s2_id, intersection);
                    self.output.insert(DistinctPoint(intersection));
                    intersection
                })
            })
            .map(|intersection| {
                if intersection.y < current_point.y
                    || (intersection.y == current_point.y && intersection.x > current_point.x)
                {
                    self.event_queue
                        .insert_intersection(intersection, s1_id, s2_id);
                };
            });
    }
}
