use super::viewport::Viewport;
use common::nannou::prelude::*;
use scene_selector::*;

pub struct LineSegmentIntersection {
    segment_count_exp: u32,
    segments: Vec<LineSegment>,
}

struct LineSegment(Point2, Point2);

impl LineSegment {
    fn new(p1: Point2, p2: Point2) -> Self {
        assert_ne!(p1, p2);
        Self(p1, p2)
    }
    fn find_intersection(s1: &LineSegment, s2: &LineSegment) -> Option<Point2> {
        let LineSegment(s1p1, s1p2) = s1;
        let LineSegment(s2p1, s2p2) = s2;
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
        self.segments.clear();
        for _ in 0..2u32.pow(self.segment_count_exp) {
            self.segments.push(LineSegment::new(
                pt2(random_range(-1.0, 1.0), random_range(-1.0, 1.0)),
                pt2(random_range(-1.0, 1.0), random_range(-1.0, 1.0)),
            ));
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

        for LineSegment(p1, p2) in &self.segments {
            draw.line()
                .start(viewport.rel_to_abs(*p1))
                .end(viewport.rel_to_abs(*p2))
                .weight(3.0)
                .color(GREEN);
        }

        // TODO: itertools combinations + rayon parallelism
        for i in 0..self.segments.len() - 1 {
            for j in i..self.segments.len() {
                let s1 = &self.segments[i];
                let s2 = &self.segments[j];
                if let Some(intersection) = LineSegment::find_intersection(s1, s2) {
                    draw.ellipse()
                        .radius(3.0)
                        .no_fill()
                        .stroke_weight(2.0)
                        .xy(viewport.rel_to_abs(intersection))
                        .color(BLACK);
                }
            }
        }

        draw.to_frame(app, &frame).unwrap();
    }
}
