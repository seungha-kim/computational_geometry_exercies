use common::*;
use nannou::prelude::*;
use scene_selector::Scene;

pub struct SimpleScene {
    started: bool,
}

impl SimpleScene {
    pub fn new() -> Self {
        Self { started: false }
    }
}

impl Scene for SimpleScene {
    fn view(&self, app: &App, frame: Frame) {
        if !self.started {
            panic!("not started");
        }

        let draw = app.draw();

        draw.rect().w_h(100.0, 100.0).color(RED);

        draw.background().color(SALMON);

        draw.to_frame(app, &frame).unwrap();
    }

    fn start(&mut self, _app: &App) {
        self.started = true;
        println!("Starting simple");
    }

    fn finish(&mut self, _app: &App) {
        println!("Finishing simple");
    }
}
