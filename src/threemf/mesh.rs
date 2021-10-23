use crate::geometry::{Point, Triangle, Vector};
use crate::threemf::xml_parse::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Mesh {
    #[serde(rename = "vertices", with = "Vertices", default)]
    pub vertices: Vec<Point>,
    #[serde(rename = "triangles", with = "Triangles", default)]
    pub triangles: Vec<Triangle>,
}

pub struct NormalizeLocation {}

// These are all relative to the view looking at the origin of the object,
// from the X-Y-Z unit vector location
impl NormalizeLocation {
    #[allow(dead_code)]
    pub const LOWER_LEFT_BACK: Vector = Vector {
        x: -1.0,
        y: -1.0,
        z: -1.0,
    };
    #[allow(dead_code)]
    pub const LOWER_RIGHT_BACK: Vector = Vector {
        x: -1.0,
        y: 1.0,
        z: -1.0,
    };
    #[allow(dead_code)]
    pub const UPPER_LEFT_BACK: Vector = Vector {
        x: -1.0,
        y: -1.0,
        z: 1.0,
    };
    #[allow(dead_code)]
    pub const UPPER_RIGHT_BACK: Vector = Vector {
        x: -1.0,
        y: 1.0,
        z: 1.0,
    };
    #[allow(dead_code)]
    pub const LOWER_LEFT_FRONT: Vector = Vector {
        x: 1.0,
        y: -1.0,
        z: -1.0,
    };
    #[allow(dead_code)]
    pub const LOWER_RIGHT_FRONT: Vector = Vector {
        x: 1.0,
        y: 1.0,
        z: -1.0,
    };
    #[allow(dead_code)]
    pub const UPPER_LEFT_FRONT: Vector = Vector {
        x: 1.0,
        y: -1.0,
        z: 1.0,
    };
    #[allow(dead_code)]
    pub const UPPER_RIGHT_FRONT: Vector = Vector {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
}

impl Mesh {
    pub fn normalize(&mut self, loc: Vector) {
        let mut furthest_dist = 0.0;
        let mut furthest_point: &Point = &Point::zero();
        // Find the furthest point in the direction of loc
        for point in self.vertices.iter() {
            let dist = (point - Point::zero()).dot(loc);
            if dist >= furthest_dist {
                furthest_dist = dist;
                furthest_point = point;
            }
        }

        // Move all using the vector from (origin - furthest_point)
        let move_dir = Point::zero() - furthest_point;
        for point in self.vertices.iter_mut() {
            *point += move_dir;
        }
    }

    pub fn centroid(&self) -> Point {
        let mut center = Point::zero();

        for vert in self.vertices.iter() {
            center.add_elem_mut(*vert);
        }

        center / (self.vertices.len() as f64)
    }
}

#[test]
fn test_normalize() {
    use crate::load::load_model;
    let mut model = load_model("data/corner.3mf").unwrap();
    model.objects[0]
        .mesh
        .normalize(NormalizeLocation::LOWER_LEFT_BACK);
    // for point in model.objects[0].mesh.vertices.iter() {
    //     println!("{}", point);
    // }
    assert!(true)
}

#[test]
fn test_centroid() {
    use crate::load::load_model;

    let model = load_model("data/centered_cube_2x2x2.3mf").unwrap();
    let cent = model.objects[0].mesh.centroid();
    println!("Calculated Centroid: {}", cent);
    assert_eq!(cent, Point::zero())
}
