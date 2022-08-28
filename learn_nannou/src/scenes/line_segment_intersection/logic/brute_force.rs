use super::common::*;

pub fn calc_intersections_brute_force<'a, I>(vals: I) -> LineSegmentIntersectionResult
where
    I: Iterator<Item = &'a LineSegment>,
{
    // TODO: itertools combinations + rayon parallelism
    let segments: Vec<&LineSegment> = vals.collect();
    let mut intersections = Vec::new();
    for i in 0..segments.len() - 1 {
        for j in i..segments.len() {
            let s1 = segments[i];
            let s2 = segments[j];
            if let Some(intersection) = LineSegment::find_interior_intersection(s1, s2) {
                intersections.push(intersection);
            }
        }
    }
    LineSegmentIntersectionResult { intersections }
}
