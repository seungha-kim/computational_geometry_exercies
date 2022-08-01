mod convex_hull_2d;
mod orbiting_shape;
mod simple;
mod simple2;

use scene_selector::*;

pub fn all_scenes() -> SceneSelector {
    SceneSelector::new(vec![
        Box::new(convex_hull_2d::ConvexHull2D::new()),
        Box::new(simple::SimpleScene::new()),
        Box::new(simple2::Simple2::new()),
        Box::new(orbiting_shape::OrbitingShape::new()),
    ])
}
