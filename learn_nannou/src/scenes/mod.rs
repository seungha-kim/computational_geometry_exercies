mod convex_hull_2d;
mod line_segment_intersection;
mod orbiting_shape;
mod simple;
mod simple2;
mod text;
mod viewport;

use scene_selector::*;

pub fn all_scenes() -> SceneSelector {
    SceneSelector::new(vec![
        Box::new(line_segment_intersection::LineSegmentIntersection::new()),
        Box::new(convex_hull_2d::ConvexHull2D::new()),
        Box::new(simple::SimpleScene::new()),
        Box::new(simple2::Simple2::new()),
        Box::new(orbiting_shape::OrbitingShape::new()),
        Box::new(text::TextTestScene::new()),
    ])
}
