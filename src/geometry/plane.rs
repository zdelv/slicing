use euclid::{Point3D, Vector3D};

// A plane with generic backing type and unit
pub struct Plane<T, U> {
    pub normal: Vector3D<T, U>,
    pub point: Point3D<T, U>,
}

impl<T, U> Plane<T, U> {
    pub fn new(normal: Vector3D<T, U>, point: Point3D<T, U>) -> Self {
        Plane { normal, point }
    }
}
