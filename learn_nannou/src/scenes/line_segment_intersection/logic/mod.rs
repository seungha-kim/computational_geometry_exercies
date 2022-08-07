mod brute_force;
mod common;
mod sweep_line;

pub use self::common::*;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum LineSegmentIntersectionStrategy {
    BruteForce,
    BruteForceParallel,
    SweepLine,
}

pub struct LineSegmentIntersectionBuilder {
    strategy: LineSegmentIntersectionStrategy,
}

impl LineSegmentIntersectionBuilder {
    pub fn new() -> Self {
        Self {
            strategy: LineSegmentIntersectionStrategy::BruteForce,
        }
    }

    pub fn strategy(mut self, strategy: LineSegmentIntersectionStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    pub fn build_from_iter<'a, I>(&self, vals: I) -> LineSegmentIntersectionResult
    where
        I: Iterator<Item = &'a LineSegment>,
    {
        self.calc_by_strategy(vals)
    }

    fn calc_by_strategy<'a, I>(&self, vals: I) -> LineSegmentIntersectionResult
    where
        I: Iterator<Item = &'a LineSegment>,
    {
        match self.strategy {
            LineSegmentIntersectionStrategy::BruteForce => {
                brute_force::calc_intersections_brute_force(vals)
            }
            LineSegmentIntersectionStrategy::BruteForceParallel => todo!(),
            LineSegmentIntersectionStrategy::SweepLine => todo!(),
        }
    }
}
