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
        let segment_count_exp = 5;
        let strategy = LineSegmentIntersectionStrategy::SweepLine;
        let (segments, result, reset_time_taken) = Self::calc_result(strategy, segment_count_exp);
        Self {
            segment_count_exp,
            reset_time_taken,
            strategy,
            segments,
            result,
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
        self.recalc_result();
    }

    fn less_segments(&mut self) {
        if self.segment_count_exp > 0 {
            self.segment_count_exp -= 1;
            self.recalc_result();
        }
    }

    fn recalc_result(&mut self) {
        let (segments, result, reset_time_taken) =
            Self::calc_result(self.strategy, self.segment_count_exp);
        self.segments = segments;
        self.result = result;
        self.reset_time_taken = reset_time_taken;
    }

    fn calc_result(
        strategy: LineSegmentIntersectionStrategy,
        segment_count_exp: u32,
    ) -> (Vec<LineSegment>, LineSegmentIntersectionResult, f32) {
        let start_time = SystemTime::now();
        let segments = Self::generate_random_segments(2u32.pow(segment_count_exp));
        let result = LineSegmentIntersectionBuilder::new()
            .strategy(strategy)
            .build_from_iter(segments.iter());
        let duration = SystemTime::now().duration_since(start_time).unwrap();
        let reset_time_taken = duration.as_secs_f32();
        (segments, result, reset_time_taken)
    }
}

impl Scene for SceneState {
    fn loop_mode(&self) -> LoopMode {
        LoopMode::RefreshSync
    }
    fn window_event(&mut self, _app: &App, event: WindowEvent) {
        match event {
            KeyPressed(Key::R) => self.recalc_result(),
            KeyPressed(Key::Up) => self.more_segments(),
            KeyPressed(Key::Down) => self.less_segments(),
            _ => {}
        };
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
