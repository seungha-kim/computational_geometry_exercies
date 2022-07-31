use common::*;
use nannou::prelude::*;

pub trait Scene {
    fn window_event(&mut self, _app: &App, _event: WindowEvent) {}
    fn update(&mut self, _app: &App, _update: Update) {}
    fn view(&self, app: &App, frame: Frame);
    fn start(&mut self, _app: &App) {}
    fn finish(&mut self, _app: &App) {}
}
