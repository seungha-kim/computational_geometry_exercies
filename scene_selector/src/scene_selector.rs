use super::scene::Scene;
use common::*;
use nannou::prelude::*;

pub struct SceneSelector {
    scenes: Vec<Box<dyn Scene>>,
    current_scene_index: usize,
    needs_start_scene: bool,
}

impl SceneSelector {
    pub fn new(scenes: Vec<Box<dyn Scene>>) -> Self {
        SceneSelector {
            scenes,
            current_scene_index: 0,
            needs_start_scene: true,
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

    #[inline]
    fn current_scene(&mut self) -> &mut Box<dyn Scene> {
        &mut self.scenes[self.current_scene_index]
    }

    fn ensure_current_scene_started(&mut self, app: &App) {
        if self.needs_start_scene {
            self.current_scene().start(app);
            app.set_loop_mode(self.current_scene().loop_mode());
            self.needs_start_scene = false;
        }
    }
}

impl Scene for SceneSelector {
    fn window_event(&mut self, app: &App, event: WindowEvent) {
        self.ensure_current_scene_started(app);

        if let KeyPressed(key) = event {
            match key {
                Key::LBracket if self.current_scene_index > 0 => {
                    self.current_scene().finish(app);
                    self.current_scene_index -= 1;
                    self.needs_start_scene = true;
                }
                Key::RBracket if self.current_scene_index < self.scenes.len() - 1 => {
                    self.current_scene().finish(app);
                    self.current_scene_index += 1;
                    self.needs_start_scene = true;
                }
                _ => {}
            }
        }

        self.current_scene().window_event(app, event);
    }

    fn update(&mut self, app: &App, update: Update) {
        self.ensure_current_scene_started(app);

        self.current_scene().update(app, update);
    }

    fn view(&self, app: &App, frame: Frame) {
        self.scenes[self.current_scene_index].view(app, frame);
    }
}
