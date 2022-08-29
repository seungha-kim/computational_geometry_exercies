use super::input::Input;
use crate::scenes::line_segment_intersection::logic::LineSegmentId;
use common::nannou::prelude::*;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};

/**

# Event Queue

- Event 를 요소로 갖는 *우선순위 큐*
- insert_event, dequeue 메소드를 가짐
- insert_event 는 특정 점에
- dequeue 호출 시 sweep line 탐색 순서대로 반환되어야 함
- dequeue 는 point 단위로 뱉음

# Sweep line 탐색 순서 (nannou 기준)

- 위에서부터 아래로 (큰 y 부터 작은 y로)
- 좌측에서 우측으로 (작은 x 부터 큰 x로)

# Event

- 특정 점과 연관된 선분에는 어떤 것들이 있는지를 표현하는 자료구조
- 특정 점을 upper endpoint 로 갖는 선분의 목록
- 특정 점을 lower endpoint 로 갖는 선분의 목록
- 특정 점을 내부 교점으로 갖는 선분의 목록
- (?) upper endpoint 면서 동시에 다른 선분과의 교점이면?

*/

pub enum EventKind {
    AsUpperEndpoint,
    AsLowerEndpoint,
    AsIntersection,
}

pub struct EventQueue<'a> {
    map: BTreeMap<EventPoint, EventData>,
    input: &'a Input<'a>,
}

impl<'a> EventQueue<'a> {
    pub fn new(input: &'a Input) -> Self {
        Self {
            map: BTreeMap::new(),
            input,
        }
    }

    fn ensure_existence(&mut self, ep: &EventPoint) {
        if !self.map.contains_key(ep) {
            self.map.insert(ep.clone(), EventData::new());
        }
    }

    pub fn insert_segment(&mut self, id: LineSegmentId) {
        let s = self.input.segments[&id];
        let ep1 = EventPoint(s.p1);
        let ep2 = EventPoint(s.p2);
        let (upper, lower) = if ep1 < ep2 { (ep1, ep2) } else { (ep2, ep1) };

        self.ensure_existence(&upper);
        self.map
            .get_mut(&upper)
            .unwrap()
            .as_upper_endpoint
            .insert(id);

        self.ensure_existence(&lower);
        self.map
            .get_mut(&lower)
            .unwrap()
            .as_lower_endpoint
            .insert(id);
    }

    pub fn insert_intersection(&mut self, p: Point2, id1: LineSegmentId, id2: LineSegmentId) {
        let ep = EventPoint(p);
        self.ensure_existence(&ep);
        // TODO: 정말로 interior 인지 검사.
        // TODO: 혹은 밖에서 interior intersection 만 보고하도록 변경!
        self.map.get_mut(&ep).unwrap().as_interior.insert(id1);
        self.map.get_mut(&ep).unwrap().as_interior.insert(id2);
    }

    pub fn dequeue_point(&mut self) -> Option<(Point2, EventData)> {
        self.map
            .keys()
            .cloned()
            .next()
            .map(|ep| (ep.0, self.map.remove(&ep).unwrap()))
    }
}

#[derive(Clone, Copy, PartialEq)]
struct EventPoint(pub Point2);

impl Eq for EventPoint {}

impl PartialOrd for EventPoint {
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

impl Ord for EventPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct EventData {
    pub as_upper_endpoint: HashSet<LineSegmentId>,
    pub as_lower_endpoint: HashSet<LineSegmentId>,
    pub as_interior: HashSet<LineSegmentId>,
}

impl EventData {
    pub fn new() -> Self {
        Self {
            as_upper_endpoint: HashSet::new(),
            as_lower_endpoint: HashSet::new(),
            as_interior: HashSet::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenes::line_segment_intersection::logic::LineSegment;
    use common::nannou::prelude::*;
    use std::collections::HashMap;

    #[test]
    fn it_works_1() {
        let p1 = EventPoint(pt2(0.0, 0.0));
        let p2 = EventPoint(pt2(0.0, -1.0));
        assert_eq!(p1.cmp(&p2), Ordering::Less);
    }

    #[test]
    fn it_works_2() {
        let p1 = EventPoint(pt2(0.0, 0.0));
        let p2 = EventPoint(pt2(1.0, 0.0));
        assert_eq!(p1.cmp(&p2), Ordering::Less);
    }

    #[test]
    fn it_works_3() {
        let p1 = EventPoint(pt2(1.0, 0.0));
        let p2 = EventPoint(pt2(0.0, -1.0));
        assert_eq!(p1.cmp(&p2), Ordering::Less);
    }

    #[test]
    fn it_works_4() {
        let p1 = EventPoint(pt2(0.0, 0.0));
        let p2 = EventPoint(pt2(0.0, 0.0));
        assert_eq!(p1.cmp(&p2), Ordering::Equal);
    }

    #[test]
    fn event_queue_dequeue_oreder_test_1() {
        let s1 = LineSegment {
            p1: pt2(0.0, 0.0),
            p2: pt2(1.0, 1.0),
        };
        let s2 = LineSegment {
            p1: pt2(0.0, 1.0),
            p2: pt2(1.0, 0.0),
        };
        let mut segments = HashMap::new();
        segments.insert(0, &s1);
        segments.insert(1, &s2);
        let input = Input { segments };
        let mut eq = EventQueue::new(&input);
        eq.insert_segment(0);
        eq.insert_segment(1);
        eq.insert_intersection(pt2(0.5, 0.5), 0, 1);

        let (point, data) = eq.dequeue_point().unwrap();
        assert_eq!(point, pt2(0.0, 1.0));
        assert_eq!(data.as_upper_endpoint.len(), 1);
        assert_eq!(data.as_lower_endpoint.len(), 0);
        assert_eq!(data.as_interior.len(), 0);

        let (point, data) = eq.dequeue_point().unwrap();
        assert_eq!(point, pt2(1.0, 1.0));
        assert_eq!(data.as_upper_endpoint.len(), 1);
        assert_eq!(data.as_lower_endpoint.len(), 0);
        assert_eq!(data.as_interior.len(), 0);

        let (point, data) = eq.dequeue_point().unwrap();
        assert_eq!(point, pt2(0.5, 0.5));
        assert_eq!(data.as_upper_endpoint.len(), 0);
        assert_eq!(data.as_lower_endpoint.len(), 0);
        assert_eq!(data.as_interior.len(), 2);

        let (point, data) = eq.dequeue_point().unwrap();
        assert_eq!(point, pt2(0.0, 0.0));
        assert_eq!(data.as_upper_endpoint.len(), 0);
        assert_eq!(data.as_lower_endpoint.len(), 1);
        assert_eq!(data.as_interior.len(), 0);

        let (point, data) = eq.dequeue_point().unwrap();
        assert_eq!(point, pt2(1.0, 0.0));
        assert_eq!(data.as_upper_endpoint.len(), 0);
        assert_eq!(data.as_lower_endpoint.len(), 1);
        assert_eq!(data.as_interior.len(), 0);
    }
}
