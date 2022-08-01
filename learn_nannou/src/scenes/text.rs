use common::nannou::prelude::*;
use scene_selector::*;

pub struct TextTestScene {}

impl TextTestScene {
    pub fn new() -> Self {
        Self {}
    }
}

impl Scene for TextTestScene {
    fn view(&self, app: &App, frame: Frame) {
        let viewport = app.window_rect();
        let draw = app.draw();

        draw.background().color(PLUM);

        let r = Rect::from_w_h(viewport.w(), 60.0)
            .bottom_left_of(viewport)
            .pad(10.0);
        draw.text("Hello Text!").xy(r.xy()).wh(r.wh()).font_size(16);
        draw.rect()
            .xy(r.xy())
            .wh(r.wh())
            .no_fill()
            .stroke(WHITE)
            .stroke_weight(1.0);

        draw.to_frame(app, &frame).unwrap();
    }
}
