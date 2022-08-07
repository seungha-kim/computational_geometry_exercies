use crate::viewport::Viewport;
use common::nannou::prelude::*;
use scene_selector::Scene;
use std::cmp::Ordering;

const GRID_COUNT: f32 = 30.0;

pub struct ConvexHull2D {
    point_count_exp: u32,
    points: Vec<Point2>,
    upper_hull: Vec<Point2>,
    lower_hull: Vec<Point2>,
}

fn is_right_turn(p1: Point2, p2: Point2, p3: Point2) -> bool {
    if p1 == p2 || p2 == p3 || p3 == p1 {
        return false;
    }
    (p2 - p1).angle_between(p3 - p2) <= 0.0
}

fn is_left_turn(p1: Point2, p2: Point2, p3: Point2) -> bool {
    if p1 == p2 || p2 == p3 || p3 == p1 {
        return false;
    }
    (p2 - p1).angle_between(p3 - p2) >= 0.0
}

impl ConvexHull2D {
    pub fn new() -> Self {
        let mut result = Self {
            point_count_exp: 8,
            points: Vec::new(),
            upper_hull: Vec::new(),
            lower_hull: Vec::new(),
        };
        result.reset();
        result
    }

    fn reset(&mut self) {
        self.points.clear();
        self.upper_hull.clear();
        self.lower_hull.clear();
        for _ in 0..(2i32.pow(self.point_count_exp)) {
            let random_radian = random_range(0.0, 2.0 * PI);
            let x = random_radian.cos();
            let y = random_radian.sin();
            let random_radius = random_range(0.0, 1.0).sqrt();
            let result = (GRID_COUNT * random_radius * pt2(x, y)).round() / GRID_COUNT;
            self.points.push(result);
        }
        self.points
            .sort_by(|p1, p2| match p1.x.partial_cmp(&p2.x).unwrap() {
                Ordering::Equal => p1.y.partial_cmp(&p2.y).unwrap(),
                ord => ord,
            });

        for i in 0..2 {
            if i < self.points.len() {
                self.upper_hull.push(self.points[i]);
                self.lower_hull.push(self.points[i]);
            }
        }

        for i in 2..self.points.len() {
            let p = self.points[i];
            while self.upper_hull.len() >= 2
                && !is_right_turn(
                    self.upper_hull[self.upper_hull.len() - 2],
                    self.upper_hull[self.upper_hull.len() - 1],
                    p,
                )
            {
                self.upper_hull.pop();
            }
            self.upper_hull.push(p);
        }

        for i in 2..self.points.len() {
            let p = self.points[i];
            while self.lower_hull.len() >= 2
                && !is_left_turn(
                    self.lower_hull[self.lower_hull.len() - 2],
                    self.lower_hull[self.lower_hull.len() - 1],
                    p,
                )
            {
                self.lower_hull.pop();
            }
            self.lower_hull.push(p);
        }
    }

    fn increase_points(&mut self) {
        self.point_count_exp += 1;
        self.reset();
    }

    fn decrease_points(&mut self) {
        if self.point_count_exp > 0 {
            self.point_count_exp -= 1;
            self.reset();
        }
    }

    fn draw_points(&self, draw: &Draw, viewport: &Viewport) {
        for p in &self.points {
            draw.ellipse()
                .xy(viewport.rel_to_abs(*p))
                .radius(5.0)
                .color(STEELBLUE);
        }
    }

    #[allow(dead_code)]
    fn draw_line_strip(&self, draw: &Draw, viewport: &Viewport) {
        for (p1, p2) in self.points.iter().skip(1).zip(self.points.iter()) {
            draw.line()
                .start(viewport.rel_to_abs(*p1))
                .end(viewport.rel_to_abs(*p2))
                .weight(3.0)
                .color(CYAN);
        }
    }

    fn draw_upper_hull(&self, draw: &Draw, viewport: &Viewport) {
        for (p1, p2) in self.upper_hull.iter().skip(1).zip(self.upper_hull.iter()) {
            draw.line()
                .start(viewport.rel_to_abs(*p1))
                .end(viewport.rel_to_abs(*p2))
                .weight(3.0)
                .color(RED);
        }
    }

    fn draw_lower_hull(&self, draw: &Draw, viewport: &Viewport) {
        for (p1, p2) in self.lower_hull.iter().skip(1).zip(self.lower_hull.iter()) {
            draw.line()
                .start(viewport.rel_to_abs(*p1))
                .end(viewport.rel_to_abs(*p2))
                .weight(3.0)
                .color(BLUE);
        }
    }
}

impl Scene for ConvexHull2D {
    fn window_event(&mut self, _app: &App, event: WindowEvent) {
        match event {
            KeyPressed(Key::R) => self.reset(),
            KeyPressed(Key::Up) => self.increase_points(),
            KeyPressed(Key::Down) => self.decrease_points(),
            _ => {}
        }
    }

    fn update(&mut self, _app: &App, _update: Update) {}

    fn view(&self, app: &App, frame: Frame) {
        let draw = app.draw();
        let padded = app.window_rect().pad(50.0);
        let viewport = Viewport { rect: &padded };

        draw.background().color(PLUM);

        self.draw_points(&draw, &viewport);
        // self.draw_line_strip(&draw, &viewport);
        self.draw_upper_hull(&draw, &viewport);
        self.draw_lower_hull(&draw, &viewport);
        draw.to_frame(app, &frame).unwrap();
    }
}
