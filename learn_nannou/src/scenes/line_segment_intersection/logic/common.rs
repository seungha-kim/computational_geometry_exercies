pub use ::common::nannou::prelude::*;

pub type LineSegmentId = usize;

pub struct LineSegment {
    pub p1: Point2,
    pub p2: Point2,
}

impl LineSegment {
    pub fn new(p1: Point2, p2: Point2) -> Self {
        assert_ne!(p1, p2);
        Self { p1, p2 }
    }

    pub fn downward_direction(&self) -> Vec2 {
        let mut normalized =
            Vec2::new(self.p2.x - self.p1.x, self.p2.y - self.p1.y).normalize_or_zero();
        assert_ne!(normalized, Vec2::ZERO);
        if normalized.y > 0.0 {
            normalized = -normalized;
        }
        if normalized.x <= -1.0 {
            normalized.x = 1.0;
        }
        normalized
    }

    pub fn find_intersection(s1: &LineSegment, s2: &LineSegment) -> Option<Point2> {
        let LineSegment {
            p1: s1p1, p2: s1p2, ..
        } = s1;
        let LineSegment {
            p1: s2p1, p2: s2p2, ..
        } = s2;
        let v1 = *s1p2 - *s1p1;
        let v2 = *s2p2 - *s2p1;

        // Ax = b -> A: mat, b: vec
        let mat = Mat2::from_cols_array(&[v1.y, v2.y, -v1.x, -v2.x]);
        let vec = Vec2::from((v1.y * s1p1.x - v1.x * s1p1.y, v2.y * s2p1.x - v2.x * s2p1.y));
        if mat.determinant() == 0.0 {
            return None;
        }
        let candidate = mat.inverse() * vec;
        let t = if v1.x.abs() > v1.y.abs() {
            (candidate.x - s1p1.x) / (s1p2.x - s1p1.x)
        } else {
            (candidate.y - s1p1.y) / (s1p2.y - s1p1.y)
        };
        if t < 0.0 || t > 1.0 {
            return None;
        }
        let u = if v2.x.abs() > v2.y.abs() {
            (candidate.x - s2p1.x) / (s2p2.x - s2p1.x)
        } else {
            (candidate.y - s2p1.y) / (s2p2.y - s2p1.y)
        };
        if u < 0.0 || u > 1.0 {
            return None;
        }
        Some(candidate)
    }
}

pub struct LineSegmentIntersectionResult {
    pub intersections: Vec<Point2>,
}
