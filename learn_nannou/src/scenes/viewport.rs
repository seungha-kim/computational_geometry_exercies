use common::nannou::prelude::*;

pub struct Viewport<'a> {
    pub rect: &'a Rect,
}

impl<'a> Viewport<'a> {
    pub fn rel_to_abs(&self, point: Point2) -> Point2 {
        self.rect.wh() * point * 0.5
    }
}
