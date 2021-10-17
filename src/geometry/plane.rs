use crate::geometry::{Point, Vector};
use serde::Serialize;
use std::fmt::Formatter;

// A plane with generic backing type and unit
#[derive(Debug, Clone, Serialize)]
pub struct Plane {
    pub normal: Vector,
    pub point: Point,
}

impl Plane {
    #[allow(dead_code)]
    pub fn new(mut normal: Vector, point: Point) -> Self {
        // If the normal isn't a unit vector, make it such
        if (normal.len() - 1.0).abs() > f64::EPSILON {
            normal = normal.unit();
        }
        Plane { normal, point }
    }

    #[allow(dead_code)]
    pub fn from_points(p1: Point, p2: Point, p3: Point) -> Self {
        let normal = (p2 - p1).cross(p3 - p1);
        let point = p3;
        Plane::new(normal, point)
    }
}

// For some reason #[derive(PartialEq)] doesn't work.
impl std::cmp::PartialEq for Plane {
    fn eq(&self, other: &Self) -> bool {
        self.normal == other.normal && self.point == other.point
    }
}

impl std::fmt::Display for Plane {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Normal: {}, Point: {}", self.normal, self.point)
    }
}

#[test]
fn test_from_points() {
    assert_eq!(
        Plane::new(Vector::new(0.0, 0.0, 1.0), Point::new(0.0, 0.0, 0.0)),
        Plane::from_points(
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
            Point::new(0.0, 0.0, 0.0)
        )
    )
}
