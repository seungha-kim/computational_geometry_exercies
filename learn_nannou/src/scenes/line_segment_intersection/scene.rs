use super::logic::{
    LineSegment, LineSegmentIntersectionBuilder, LineSegmentIntersectionResult,
    LineSegmentIntersectionStrategy,
};
use crate::viewport::Viewport;
use common::nannou::prelude::*;
use scene_selector::*;
use std::time::SystemTime;

pub struct SceneState {
    segment_count_exp: u32,
    reset_time_taken: f32,
    strategy: LineSegmentIntersectionStrategy,
    segments: Vec<LineSegment>,
    result: LineSegmentIntersectionResult,
}

impl SceneState {
    pub fn new() -> Self {
        Self {
            segment_count_exp: 5,
            reset_time_taken: 0.0,
            strategy: LineSegmentIntersectionStrategy::SweepLine,
            segments: Vec::new(),
            result: LineSegmentIntersectionResult {
                intersections: Vec::new(),
            },
        }
    }

    fn generate_random_segments(segment_count: u32) -> Vec<LineSegment> {
        let mut segments = Vec::new();
        for _ in 0..segment_count {
            segments.push(LineSegment::new(
                pt2(random_range(-1.0, 1.0), random_range(-1.0, 1.0)),
                pt2(random_range(-1.0, 1.0), random_range(-1.0, 1.0)),
            ));
        }
        segments
    }

    fn more_segments(&mut self) {
        self.segment_count_exp += 1;
        self.recalc_result(true);
    }

    fn less_segments(&mut self) {
        if self.segment_count_exp > 0 {
            self.segment_count_exp -= 1;
            self.recalc_result(true);
        }
    }

    fn update_to_next_strategy(&mut self) {
        use LineSegmentIntersectionStrategy::*;
        self.strategy = match self.strategy {
            BruteForce => SweepLine,
            SweepLine => BruteForce,
            BruteForceParallel => unimplemented!(),
        };
        self.recalc_result(false);
    }

    fn recalc_result(&mut self, reset_segments: bool) {
        let start_time = SystemTime::now();
        if self.segments.len() == 0 || reset_segments {
            self.segments = Self::generate_random_segments(2u32.pow(self.segment_count_exp));
        }

        self.result = LineSegmentIntersectionBuilder::new()
            .strategy(self.strategy)
            .build_from_iter(self.segments.iter());
        let duration = SystemTime::now().duration_since(start_time).unwrap();
        self.reset_time_taken = duration.as_secs_f32();
    }
}

impl Scene for SceneState {
    fn loop_mode(&self) -> LoopMode {
        LoopMode::RefreshSync
    }
    fn window_event(&mut self, _app: &App, event: WindowEvent) {
        match event {
            KeyPressed(Key::R) => self.recalc_result(true),
            KeyPressed(Key::Up) => self.more_segments(),
            KeyPressed(Key::Down) => self.less_segments(),
            KeyPressed(Key::Tab) => self.update_to_next_strategy(),
            _ => {}
        };
    }

    fn start(&mut self, _app: &App) {
        self.recalc_result(true);
    }

    fn view(&self, app: &App, frame: Frame) {
        let draw = app.draw();
        let viewport = Viewport {
            rect: &app.window_rect().pad(50.0),
        };

        draw.background().color(LINEN);

        for LineSegment { p1, p2 } in &self.segments {
            draw.line()
                .start(viewport.rel_to_abs(*p1))
                .end(viewport.rel_to_abs(*p2))
                .weight(1.0)
                .color(GREEN);
        }

        let intersections = &self.result.intersections;
        if intersections.len() < 1000 {
            for intersection in intersections {
                draw.ellipse()
                    .radius(3.0)
                    .xy(viewport.rel_to_abs(*intersection))
                    .color(BLACK);
            }
        }

        let text_rect = Rect::from_w_h(viewport.rect.w(), 100.0).bottom_right_of(*viewport.rect);
        draw.text(&format!(
            "strategy: {:?}\nsegments: {}\nintersections: {}\nreset: {:.2}\nframe: {:.2}",
            self.strategy,
            self.segments.len(),
            self.result.intersections.len(),
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
