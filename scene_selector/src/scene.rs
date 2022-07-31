use common::*;
use nannou::prelude::*;

pub trait Scene {
    fn event(&mut self, app: &App, event: Event);
    fn view(&self, app: &App, frame: Frame);
}
