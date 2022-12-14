use common::*;
use nannou::prelude::*;
use scene_selector::Scene;

pub struct Simple2 {}

impl Simple2 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Scene for Simple2 {
    fn view(&self, app: &App, frame: Frame) {
        let draw = app.draw();

        draw.rect().w_h(100.0, 100.0).color(BLUE);

        draw.background().color(RED);

        draw.to_frame(app, &frame).unwrap();
    }

    fn start(&mut self, _app: &App) {
        println!("Starting simple2");
    }

    fn finish(&mut self, _app: &App) {
        println!("Finishing simple2");
    }
}
