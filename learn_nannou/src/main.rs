mod scenes;

use self::scenes::*;
use common::*;
use nannou::prelude::*;
use scene_selector::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

fn model(_app: &App) -> SceneSelector {
    SceneSelector::new(vec![
        Box::new(SimpleScene::new()),
        Box::new(OrbitingShape::new()),
    ])
}

fn update(app: &App, model: &mut SceneSelector, update: Update) {
    model.update(app, update);
}

fn view(app: &App, model: &SceneSelector, frame: Frame) {
    model.view(app, frame);
}
