use euclid::{Point3D, Vector3D};
use std::ops::{Mul, Sub};

// A plane with generic backing type and unit
#[derive(Debug)]
pub struct Plane<T, U> {
    pub normal: Vector3D<T, U>,
    pub point: Point3D<T, U>,
}

impl<T: Clone + Copy, U> Plane<T, U> {
    #[allow(dead_code)]
    pub fn new(normal: Vector3D<T, U>, point: Point3D<T, U>) -> Self {
        Plane { normal, point }
    }
}

impl<T, U> Plane<T, U>
where
    T: Copy + Sub<Output = T> + Mul<Output = T>,
    <T as Sub>::Output: Copy,
{
    #[allow(dead_code)]
    pub fn from_points(p1: Point3D<T, U>, p2: Point3D<T, U>, p3: Point3D<T, U>) -> Self
    where
        <T as Sub>::Output: Sub,
    {
        let normal = (p2 - p1).cross(p3 - p1);
        let point = p3;
        Plane::new(normal, point)
    }
}

// For some reason #[derive(PartialEq)] doesn't work.
impl<T, U> std::cmp::PartialEq for Plane<T, U>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.normal == other.normal && self.point == other.point
    }
}

#[test]
fn test_from_points() {
    use crate::geometry::units::*;
    assert_eq!(
        Plane::new(Vector3D::<i32, Meter>::new(0, 0, 1), Point3D::new(0, 0, 0)),
        Plane::from_points(
            Point3D::<i32, Meter>::new(1, 0, 0),
            Point3D::new(0, 1, 0),
            Point3D::new(0, 0, 0)
        )
    )
}
