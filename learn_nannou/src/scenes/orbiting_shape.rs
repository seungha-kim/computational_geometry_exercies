use common::*;
use nannou::prelude::*;
use scene_selector::Scene;

pub struct OrbitingShape {
    position: Point2,
}

impl OrbitingShape {
    pub fn new() -> Self {
        Self {
            position: Point2::new(0.0, 0.0),
        }
    }
}

impl Scene for OrbitingShape {
    fn update(&mut self, app: &App, _update: Update) {
        let win_rect = app.window_rect();
        let time = app.time * 3.0;
        let x = map_range(time.cos(), -1.0, 1.0, win_rect.left(), win_rect.right());
        let y = map_range(time.sin(), -1.0, 1.0, win_rect.bottom(), win_rect.top());
        let pos = app
            .mouse
            .buttons
            .left()
            .if_down()
            .unwrap_or(Point2::new(x, y));
        self.position = pos;
    }

    fn view(&self, app: &App, frame: Frame) {
        let pos = self.position;

        // get canvas to draw on
        let draw = app.draw();

        // set background to blue
        draw.background().color(BLUE);

        // let pos = Point2::new(app.mouse.x, app.mouse.y);
        let r = Rect::from_x_y_w_h(pos.x, pos.y, 100.0, 100.0);
        draw.rect()
            .xy(r.xy())
            .wh(r.wh())
            // .z_degrees(45.0)
            .color(PLUM);

        let c = r.below(r).shift_y(-10.0);
        draw.ellipse().xy(c.xy()).wh(c.wh()).color(SALMON);

        // put everything on the frame
        draw.to_frame(app, &frame).unwrap();
    }
}
