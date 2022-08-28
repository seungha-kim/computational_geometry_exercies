use crate::scenes::line_segment_intersection::logic::LineSegmentId;
use common::nannou::geom::Point2;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};

/**

# Event Queue

- Event 를 요소로 갖는 *우선순위 큐*
- insert_event, dequeue 메소드를 가짐
- insert_event 는 특정 점에
- dequeue 호출 시 sweep line 탐색 순서대로 반환되어야 함
- (?) dequeue 는 point 를 반환해야 하는가? 그냥 모든 개별 Event 처리 순서를 여기에서 결정하는 것도 좋지 않을까?

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

pub struct EventQueue {
    map: BTreeMap<EventPoint, EventDataPerPoint>,
}

impl EventQueue {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    fn ensure_existence(&mut self, ep: &EventPoint) {
        if !self.map.contains_key(ep) {
            self.map[ep] = EventDataPerPoint::new();
        }
    }

    pub fn insert_event(&mut self, event_kind: EventKind, p: Point2, id: LineSegmentId) {
        let ep = EventPoint(p);
        self.ensure_existence(&ep);
        match event_kind {
            EventKind::AsUpperEndpoint => self.map[&ep].as_upper_endpoint.insert(id),
            EventKind::AsLowerEndpoint => self.map[&ep].as_lower_endpoint.insert(id),
            EventKind::AsIntersection => self.map[&ep].as_intersection.insert(id),
        };
    }

    pub fn insert_lower_segment(&mut self, p: Point2, id: LineSegmentId) {
        let ep = EventPoint(p);
        self.ensure_existence(&ep);
        self.map[&ep].as_lower_endpoint.insert(id);
    }

    pub fn dequeue(&mut self) -> Option<(Point2, EventDataPerPoint)> {
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

pub struct EventDataPerPoint {
    pub as_upper_endpoint: HashSet<LineSegmentId>,
    pub as_lower_endpoint: HashSet<LineSegmentId>,
    pub as_intersection: HashSet<LineSegmentId>,
}

impl EventDataPerPoint {
    pub fn new() -> Self {
        Self {
            as_upper_endpoint: HashSet::new(),
            as_lower_endpoint: HashSet::new(),
            as_intersection: HashSet::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::nannou::prelude::*;
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
}
