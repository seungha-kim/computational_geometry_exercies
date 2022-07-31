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
    fn event(&mut self, app: &App, event: Event) {
        if let Event::WindowEvent {
            simple: Some(KeyPressed(key)),
            ..
        } = event
        {
            match key {
                Key::LBracket if self.current_scene > 0 => self.current_scene -= 1,
                Key::RBracket if self.current_scene < self.scenes.len() - 1 => {
                    self.current_scene += 1
                }
                _ => {}
            }
        }

        self.scenes[self.current_scene].event(app, event);
    }

    fn view(&self, app: &App, frame: Frame) {
        self.scenes[self.current_scene].view(app, frame);
    }
}
