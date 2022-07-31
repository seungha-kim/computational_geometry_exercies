use common::*;
use nannou::prelude::*;
use scene_selector::Scene;

pub struct SimpleScene {}

impl SimpleScene {
    pub fn new() -> Self {
        Self {}
    }
}

impl Scene for SimpleScene {
    fn event(&mut self, _app: &App, _event: Event) {}

    fn view(&self, app: &App, frame: Frame) {
        let draw = app.draw();

        draw.rect().w_h(100.0, 100.0).color(RED);

        draw.background().color(SALMON);

        draw.to_frame(app, &frame).unwrap();
    }
}
