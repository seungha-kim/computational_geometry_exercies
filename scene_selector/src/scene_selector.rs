use super::scene::Scene;
use common::*;
use nannou::prelude::*;

pub struct SceneSelector {
    scenes: Vec<Box<dyn Scene>>,
    current_scene: usize,
}

impl SceneSelector {
    pub fn new(scenes: Vec<Box<dyn Scene>>) -> Self {
        SceneSelector {
            scenes,
            current_scene: 0,
        }
    }
}

impl Scene for SceneSelector {
    fn update(&mut self, app: &App, update: Update) {
        if app.keys.down.contains(&Key::LBracket) && self.current_scene > 0 {
            self.current_scene -= 1;
        } else if app.keys.down.contains(&Key::RBracket)
            && self.current_scene < self.scenes.len() - 1
        {
            self.current_scene += 1;
        }

        self.scenes[self.current_scene].update(app, update);
    }

    fn view(&self, app: &App, frame: Frame) {
        self.scenes[self.current_scene].view(app, frame);
    }
}
