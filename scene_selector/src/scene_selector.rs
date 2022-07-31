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

    pub fn event(&mut self, app: &App, event: Event) {
        match event {
            Event::WindowEvent {
                simple: Some(window_event),
                ..
            } => self.window_event(app, window_event),
            Event::Update(update) => self.update(app, update),
            _ => {}
        }
    }
}

impl Scene for SceneSelector {
    fn window_event(&mut self, app: &App, event: WindowEvent) {
        if let KeyPressed(key) = event {
            match key {
                Key::LBracket if self.current_scene > 0 => self.current_scene -= 1,
                Key::RBracket if self.current_scene < self.scenes.len() - 1 => {
                    self.current_scene += 1
                }
                _ => {}
            }
        }

        self.scenes[self.current_scene].window_event(app, event);
    }

    fn update(&mut self, app: &App, update: Update) {
        self.scenes[self.current_scene].update(app, update);
    }

    fn view(&self, app: &App, frame: Frame) {
        self.scenes[self.current_scene].view(app, frame);
    }
}
