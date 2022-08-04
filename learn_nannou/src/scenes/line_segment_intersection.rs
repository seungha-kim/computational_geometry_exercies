use super::viewport::Viewport;
use common::nannou::prelude::*;
use scene_selector::*;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::time::SystemTime;

pub struct LineSegmentIntersection {
    segment_count_exp: u32,
    segments: Vec<LineSegment>,
    intersections: Vec<Point2>,
    reset_time_taken: f32,
    event_queue: BTreeMap<EventPointKey, EventPoint>,
}

struct LineSegment {
    id: LineSegmentId,
    p1: Point2,
    p2: Point2,
}

impl LineSegment {
    fn new(p1: Point2, p2: Point2) -> Self {
        assert_ne!(p1, p2);
        let id = unsafe {
            ID_SOURCE += 1;
            ID_SOURCE
        };
        Self { id, p1, p2 }
    }

    fn find_intersection(s1: &LineSegment, s2: &LineSegment) -> Option<Point2> {
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

impl LineSegmentIntersection {
    pub fn new() -> Self {
        let mut result = Self {
            segment_count_exp: 5,
            segments: Vec::new(),
            intersections: Vec::new(),
            reset_time_taken: 0.0,
            event_queue: BTreeMap::new(),
        };
        result.reset();
        result
    }

    fn more_segments(&mut self) {
        self.segment_count_exp += 1;
        self.reset();
    }

    fn less_segments(&mut self) {
        if self.segment_count_exp > 0 {
            self.segment_count_exp -= 1;
            self.reset();
        }
    }

    fn reset(&mut self) {
        let start_time = SystemTime::now();
        self.segments.clear();
        self.intersections.clear();
        for _ in 0..2u32.pow(self.segment_count_exp) {
            self.segments.push(LineSegment::new(
                pt2(random_range(-1.0, 1.0), random_range(-1.0, 1.0)),
                pt2(random_range(-1.0, 1.0), random_range(-1.0, 1.0)),
            ));
        }

        self.find_intersections_brute_force();

        for s in &self.segments {
            let k1 = EventPointKey(s.p1);
            let k2 = EventPointKey(s.p2);
            let (upper_endpoint, lower_endpoint) = if k1 < k2 { (k1, k2) } else { (k2, k1) };

            {
                if !self.event_queue.contains_key(&upper_endpoint) {
                    let event_point = EventPoint::new();
                    self.event_queue.insert(upper_endpoint.clone(), event_point);
                }
                let mut ep = self.event_queue.get_mut(&upper_endpoint).unwrap();
                ep.as_upper_endpoint.push(s.id);
            }
            {
                if !self.event_queue.contains_key(&lower_endpoint) {
                    let event_point = EventPoint::new();
                    self.event_queue.insert(lower_endpoint, event_point);
                }
                let mut ep = self.event_queue.get_mut(&lower_endpoint).unwrap();
                ep.as_lower_endpoint.push(s.id);
            }
        }

        let duration = SystemTime::now().duration_since(start_time).unwrap();
        self.reset_time_taken = duration.as_secs_f32();
    }

    fn find_intersections_brute_force(&mut self) {
        // TODO: itertools combinations + rayon parallelism
        for i in 0..self.segments.len() - 1 {
            for j in i..self.segments.len() {
                let s1 = &self.segments[i];
                let s2 = &self.segments[j];
                if let Some(intersection) = LineSegment::find_intersection(s1, s2) {
                    self.intersections.push(intersection);
                }
            }
        }
    }
}

impl Scene for LineSegmentIntersection {
    fn window_event(&mut self, _app: &App, event: WindowEvent) {
        match event {
            KeyPressed(Key::R) => self.reset(),
            KeyPressed(Key::Up) => self.more_segments(),
            KeyPressed(Key::Down) => self.less_segments(),
            _ => {}
        }
    }
    fn view(&self, app: &App, frame: Frame) {
        let draw = app.draw();
        let viewport = Viewport {
            rect: &app.window_rect().pad(50.0),
        };

        draw.background().color(LINEN);

        for LineSegment { p1, p2, .. } in &self.segments {
            draw.line()
                .start(viewport.rel_to_abs(*p1))
                .end(viewport.rel_to_abs(*p2))
                .weight(1.0)
                .color(GREEN);
        }

        if self.intersections.len() < 1000 {
            for intersection in &self.intersections {
                draw.ellipse()
                    .radius(3.0)
                    .xy(viewport.rel_to_abs(*intersection))
                    .color(BLACK);
            }
        }

        for (ek, _) in &self.event_queue {
            draw.ellipse()
                .radius(5.0)
                .xy(viewport.rel_to_abs((*ek).0))
                .color(BLUE);
        }

        let text_rect = Rect::from_w_h(viewport.rect.w(), 100.0).bottom_right_of(*viewport.rect);
        draw.text(&format!(
            "segments: {}\nintersections: {}\nreset: {:.2}\nframe: {:.2}",
            self.segments.len(),
            self.intersections.len(),
            self.reset_time_taken,
            app.duration.since_prev_update.as_secs_f32(),
        ))
        .xy(text_rect.xy())
        .wh(text_rect.wh())
        .color(BLACK)
        .align_text_bottom()
        .right_justify();

        draw.to_frame(app, &frame).unwrap();
    }
}

// ---

struct EventPoint {
    as_upper_endpoint: Vec<LineSegmentId>,
    as_lower_endpoint: Vec<LineSegmentId>,
    as_interior: Vec<LineSegmentId>,
}

impl EventPoint {
    fn new() -> Self {
        Self {
            as_upper_endpoint: Vec::new(),
            as_lower_endpoint: Vec::new(),
            as_interior: Vec::new(),
        }
    }
}

static mut ID_SOURCE: u32 = 0;

type LineSegmentId = u32;

#[derive(Clone, Copy, PartialEq)]
struct EventPointKey(Point2);

impl Eq for EventPointKey {}

impl PartialOrd for EventPointKey {
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

impl Ord for EventPointKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
