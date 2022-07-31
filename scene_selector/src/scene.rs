use common::*;
use nannou::prelude::*;

pub trait Scene {
    fn update(&mut self, app: &App, update: Update);
    fn view(&self, app: &App, frame: Frame);
}
