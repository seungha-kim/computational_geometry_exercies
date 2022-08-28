use super::{LineSegment, LineSegmentIntersectionResult};
use crate::scenes::line_segment_intersection::logic::sweep_line::executor::Executor;
use crate::scenes::line_segment_intersection::logic::sweep_line::input::Input;

mod distinct_point;
mod event_queue;
mod executor;
mod input;
mod intersection_map;
mod status;

pub fn calc_intersections<'a, I>(vals: I) -> LineSegmentIntersectionResult
where
    I: Iterator<Item = &'a LineSegment>,
{
    let segments = vals.enumerate().collect();
    let input = Input { segments };
    let executor = Executor::new(&input);
    executor.calc_result()
}
