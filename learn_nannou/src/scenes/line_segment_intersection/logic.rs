use common::nannou::prelude::*;

type LineSegmentId = u32;

pub struct LineSegment {
    pub p1: Point2,
    pub p2: Point2,
}

impl LineSegment {
    pub fn new(p1: Point2, p2: Point2) -> Self {
        assert_ne!(p1, p2);
        Self { p1, p2 }
    }

    pub fn downward_direction(&self) -> Vec2 {
        let mut normalized = Vec2::new(self.p2.y - self.p1.y, self.p2.x - self.p1.x).normalize();
        if normalized.y > 0.0 {
            normalized = -normalized;
        }
        normalized
    }

    pub fn find_intersection(s1: &LineSegment, s2: &LineSegment) -> Option<Point2> {
        let LineSegment {
            p1: s1p1, p2: s1p2, ..
        } = s1;
        let LineSegment {
            p1: s2p1, p2: s2p2, ..
        } = s2;
        let v1 = *s1p2 - *s1p1;
        let v2 = *s2p2 - *s2p1;

        // Ax = b -> A: mat, b: vec
        let mat = Mat2::from_cols_array(&[v1.y, v2.y, -v1.x, -v2.x]);
        let vec = Vec2::from((v1.y * s1p1.x - v1.x * s1p1.y, v2.y * s2p1.x - v2.x * s2p1.y));
        if mat.determinant() == 0.0 {
            return None;
        }
        let candidate = mat.inverse() * vec;
        let t = if v1.x.abs() > v1.y.abs() {
            (candidate.x - s1p1.x) / (s1p2.x - s1p1.x)
        } else {
            (candidate.y - s1p1.y) / (s1p2.y - s1p1.y)
        };
        if t < 0.0 || t > 1.0 {
            return None;
        }
        let u = if v2.x.abs() > v2.y.abs() {
            (candidate.x - s2p1.x) / (s2p2.x - s2p1.x)
        } else {
            (candidate.y - s2p1.y) / (s2p2.y - s2p1.y)
        };
        if u < 0.0 || u > 1.0 {
            return None;
        }
        Some(candidate)
    }
}

#[derive(Copy, Clone)]
pub enum LineSegmentIntersectionStrategy {
    BruteForce,
    BruteForceParallel,
    EventQueue,
}

pub struct LineSegmentIntersectionBuilder {
    strategy: LineSegmentIntersectionStrategy,
    segments: Vec<LineSegment>,
}

impl LineSegmentIntersectionBuilder {
    pub fn new() -> Self {
        Self {
            strategy: LineSegmentIntersectionStrategy::BruteForce,
            segments: Vec::new(),
        }
    }

    pub fn strategy(mut self, strategy: LineSegmentIntersectionStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    pub fn build_from_iter<'a, I>(&self, vals: I) -> LineSegmentIntersectionResult
    where
        I: Iterator<Item = &'a LineSegment>,
    {
        self.calc_by_strategy(vals)
    }

    fn calc_by_strategy<'a, I>(&self, vals: I) -> LineSegmentIntersectionResult
    where
        I: Iterator<Item = &'a LineSegment>,
    {
        match self.strategy {
            LineSegmentIntersectionStrategy::BruteForce => {
                self.calc_intersections_brute_force(vals)
            }
            LineSegmentIntersectionStrategy::BruteForceParallel => todo!(),
            LineSegmentIntersectionStrategy::EventQueue => todo!(),
        }
    }

    fn calc_intersections_brute_force<'a, I>(&self, vals: I) -> LineSegmentIntersectionResult
    where
        I: Iterator<Item = &'a LineSegment>,
    {
        // TODO: itertools combinations + rayon parallelism
        let segments: Vec<&LineSegment> = vals.collect();
        let mut intersections = Vec::new();
        for i in 0..segments.len() - 1 {
            for j in i..segments.len() {
                let s1 = segments[i];
                let s2 = segments[j];
                if let Some(intersection) = LineSegment::find_intersection(s1, s2) {
                    intersections.push(intersection);
                }
            }
        }
        LineSegmentIntersectionResult { intersections }
    }
}

pub struct LineSegmentIntersectionResult {
    pub intersections: Vec<Point2>,
}

//
// type EventQueue = BTreeMap<EventPointKey, EventPoint>;
// type Holder = HashMap<LineSegmentId, LineSegment>;
//
//
// impl LineSegmentIntersectionResult {}
//
// #[derive(PartialEq, Clone)]
// struct StatusItem {
//     line_segment_id: LineSegmentId,
//     point: EventPointKey,
//     downward_direction: Vec2,
// }
//
// impl Eq for StatusItem {}
//
// impl PartialOrd for StatusItem {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         match self.point.0.x.partial_cmp(&other.point.0.x) {
//             Some(Ordering::Equal) => self
//                 .downward_direction
//                 .x
//                 .partial_cmp(&other.downward_direction.x),
//             ord => ord,
//         }
//     }
// }
//
// impl Ord for StatusItem {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.partial_cmp(other).unwrap()
//     }
// }
//
// impl StatusItem {
//     fn new(line_segment_id: LineSegmentId, point: EventPointKey, downward_direction: Vec2) -> Self {
//         assert!(downward_direction.is_finite());
//         Self {
//             line_segment_id,
//             point,
//             downward_direction,
//         }
//     }
// }
//
// impl Ord for EventPointKey {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.partial_cmp(other).unwrap()
//     }
// }
//
// static mut ID_SOURCE: u32 = 0;

// #[derive(Clone, Copy, PartialEq)]
// struct EventPointKey(Point2);
//
// impl Eq for EventPointKey {}
//
// impl PartialOrd for EventPointKey {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         let x_cmp = self.0.x.partial_cmp(&other.0.x);
//         let y_cmp = self.0.y.partial_cmp(&other.0.y);
//         match (y_cmp, x_cmp) {
//             (Some(Ordering::Equal), Some(ord)) => Some(ord),
//             (Some(ord), _) => Some(ord.reverse()),
//             _ => panic!("Cannot determine order"),
//         }
//     }
// }
//
// struct EventPoint {
//     as_upper_endpoint: Vec<LineSegmentId>,
//     as_lower_endpoint: Vec<LineSegmentId>,
//     as_interior: Vec<LineSegmentId>,
// }
//
// impl EventPoint {
//     fn new() -> Self {
//         Self {
//             as_upper_endpoint: Vec::new(),
//             as_lower_endpoint: Vec::new(),
//             as_interior: Vec::new(),
//         }
//     }
// }
// fn reset(&mut self) {
//     let start_time = SystemTime::now();
//     self.segments.clear();
//     self.intersections.clear();
//
//     for (id, s) in &self.segments {
//         let k1 = EventPointKey(s.p1);
//         let k2 = EventPointKey(s.p2);
//         let (upper_endpoint, lower_endpoint) = if k1 < k2 { (k1, k2) } else { (k2, k1) };
//
//         {
//             if !self.event_queue.contains_key(&upper_endpoint) {
//                 let event_point = EventPoint::new();
//                 self.event_queue.insert(upper_endpoint.clone(), event_point);
//             }
//             let mut ep = self.event_queue.get_mut(&upper_endpoint).unwrap();
//             ep.as_upper_endpoint.push(s.id);
//         }
//         {
//             if !self.event_queue.contains_key(&lower_endpoint) {
//                 let event_point = EventPoint::new();
//                 self.event_queue.insert(lower_endpoint, event_point);
//             }
//             let mut ep = self.event_queue.get_mut(&lower_endpoint).unwrap();
//             ep.as_lower_endpoint.push(s.id);
//         }
//     }
//
//     // 다음 단계에서 이전 단계의 status item 을 찾아서 순서를 바꿔줘야 하는데... 어떻게 해야 할까
//     let mut status: BTreeSet<StatusItem> = BTreeSet::new();
//
//     while let Some(key) = self.event_queue.keys().cloned().next() {
//         let event = self.event_queue.remove(&key).unwrap();
//         // LOWER -> INTERIOR -> UPPER ?
//         // LOWER
//         for id in &event.as_lower_endpoint {
//             let s = &self.segments[&id];
//             let item = StatusItem::new(*id, key, s.downward_direction());
//             status.insert(item.clone());
//             // 좌우
//             let self_line = self.segments.get(id).unwrap();
//             if let Some(left) = status
//                 .range((Bound::Excluded(&item), Bound::Unbounded))
//                 .next()
//             {
//                 let left_line = self.segments.get(&left.line_segment_id).unwrap();
//                 if let Some(intersection) = LineSegment::find_intersection(self_line, left_line) {
//                     self.intersections.push(intersection);
//                     // TODO: 이미 존재할 수 있음
//                     let mut new_event = EventPoint::new();
//                     new_event.as_interior.push(*id);
//                     new_event.as_interior.push(left_line.id);
//                     self.event_queue
//                         .insert(EventPointKey(intersection), new_event);
//                 }
//             }
//             if let Some(right) = status
//                 .range((Bound::Unbounded, Bound::Excluded(&item)))
//                 .next()
//             {
//                 let right_line = self.segments.get(&right.line_segment_id).unwrap();
//                 if let Some(intersection) = LineSegment::find_intersection(self_line, right_line) {
//                     self.intersections.push(intersection);
//                     // TODO: 이미 존재할 수 있음
//                     let mut new_event = EventPoint::new();
//                     new_event.as_interior.push(*id);
//                     new_event.as_interior.push(right_line.id);
//                     self.event_queue
//                         .insert(EventPointKey(intersection), new_event);
//                 }
//             }
//         }
//         // UPPER
//         for id in &event.as_upper_endpoint {
//             let s = &self.segments[&id];
//             let item = StatusItem::new(*id, key, s.downward_direction());
//             status.insert(item.clone());
//             // 좌우
//             let self_line = self.segments.get(id).unwrap();
//             if let Some(left) = status
//                 .range((Bound::Excluded(&item), Bound::Unbounded))
//                 .next()
//             {
//                 let left_line = self.segments.get(&left.line_segment_id).unwrap();
//                 if let Some(intersection) = LineSegment::find_intersection(self_line, left_line) {
//                     self.intersections.push(intersection);
//                     // TODO: 이미 존재할 수 있음
//                     let mut new_event = EventPoint::new();
//                     new_event.as_interior.push(*id);
//                     new_event.as_interior.push(left_line.id);
//                     self.event_queue
//                         .insert(EventPointKey(intersection), new_event);
//                 }
//             }
//             if let Some(right) = status
//                 .range((Bound::Unbounded, Bound::Excluded(&item)))
//                 .next()
//             {
//                 let right_line = self.segments.get(&right.line_segment_id).unwrap();
//                 if let Some(intersection) = LineSegment::find_intersection(self_line, right_line) {
//                     self.intersections.push(intersection);
//                     // TODO: 이미 존재할 수 있음
//                     let mut new_event = EventPoint::new();
//                     new_event.as_interior.push(*id);
//                     new_event.as_interior.push(right_line.id);
//                     self.event_queue
//                         .insert(EventPointKey(intersection), new_event);
//                 }
//             }
//         }
//         // INTERIOR
//     }
//
//     let duration = SystemTime::now().duration_since(start_time).unwrap();
//     self.reset_time_taken = duration.as_secs_f32();
// }
