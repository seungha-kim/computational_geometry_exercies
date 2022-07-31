mod scenes;

use self::scenes::*;
use common::*;
use nannou::prelude::*;
use scene_selector::*;

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(_app: &App) -> SceneSelector {
    SceneSelector::new(vec![
        Box::new(SimpleScene::new()),
        Box::new(Simple2::new()),
        Box::new(OrbitingShape::new()),
    ])
}

fn event(app: &App, model: &mut SceneSelector, event: Event) {
    match event {
        Event::WindowEvent {
            simple: Some(window_event),
            ..
        } => model.window_event(app, window_event),
        Event::Update(update) => model.update(app, update),
        _ => {}
    }
}

fn view(app: &App, model: &SceneSelector, frame: Frame) {
    model.view(app, frame);
}
