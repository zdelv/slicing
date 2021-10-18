use crate::error::Error;
use crate::geometry::{Plane, Point, Vector};
use crate::threemf::Model;
use itertools::Itertools;

pub mod common;
pub mod error;
pub mod geometry;
pub mod load;
pub mod threemf;

// Check if the cuts are possible
fn verify_cut(cut: &Plane) -> bool {
    // Check if the plane is vertical
    if cut.normal.dot(Vector::Z).abs() < f64::EPSILON {
        return false;
    }

    // Check if the plane is horizontal and on z=0
    if cut.normal.dot(Vector::X).abs() < f64::EPSILON && cut.point.z.abs() < f64::EPSILON {
        return false;
    }

    true
}

// TODO: This is non-relative. It *requires* different similarities for different scales of floats
fn check_equal_normals(v1: Vector, v2: Vector, similarity: f64) -> bool {
    // Relative version (but allows inf and NaN (Not good))
    // let diff = (v1.sub_elem(v2)).div_elem(v2).abs();

    let diff = v1.sub_elem(v2).abs();
    (diff.x < similarity) && (diff.y < similarity) && (diff.z < similarity)
}

pub fn slice_model(model: Model) -> Result<Vec<Plane>, Error> {
    let num_objects = model.num_objects();
    if num_objects > 1 {
        return Err(Error::TooManyModels);
    }
    let object = &model.objects[0];

    let mut cutting_planes: Vec<Plane> = Vec::new();

    // For every triangle
    for triangle in object.mesh.triangles.iter() {
        // Get vertices of triangle
        let v1 = object.mesh.vertices[triangle.v1];
        let v2 = object.mesh.vertices[triangle.v2];
        let v3 = object.mesh.vertices[triangle.v3];

        // Form the "plane" from the triangle's verts
        let cut = Plane::from_points(
            Point::new(v1.x, v1.y, v1.z),
            Point::new(v2.x, v2.y, v2.z),
            Point::new(v3.x, v3.y, v3.z),
        );

        // If triangle fits characteristics
        if verify_cut(&cut) {
            // Form cutting plane from triangle.
            // Push cutting plane into cutting_planes vec
            cutting_planes.push(cut);
        }
    }

    // Iterate through the cuts and determine if any of them overlap
    // An overlapping cut has:
    //  * Same normal direction
    //  * At least one axis of the point is the same (one of x, y, or z must be the same)
    //    TODO: This second bullet point might not be true
    let mut marked_indices: Vec<usize> = Vec::new();
    for i in 0..cutting_planes.len() {
        for j in (i + 1)..cutting_planes.len() {
            let cut1 = &cutting_planes[i];
            let cut2 = &cutting_planes[j];
            if check_equal_normals(cut1.normal, cut2.normal, 0.001) {
                // let p1 = &cut1.point;
                // let p2 = &cut2.point;
                // if p1.x == p2.x || p1.y == p2.y || p1.z == p2.z {
                //     marked_indices.push(j)
                // }
                marked_indices.push(j)
            }
        }
    }

    // println!(
    //     "{:?}",
    //     marked_indices
    //         .clone()
    //         .into_iter()
    //         .unique()
    //         .collect::<Vec<usize>>()
    // );

    // We have the indices we want to remove
    // There may be more than one similar (e.g. 3 planes similar), meaning some indices could
    // be duplicated. Use unique to solve this, then reverse it to remove back to front.
    for i in marked_indices.into_iter().unique().rev() {
        cutting_planes.swap_remove(i);
    }

    Ok(cutting_planes)
}

#[cfg(test)]
mod tests {
    use crate::load::load_model;
    use crate::slice_model;
    use crate::threemf::NormalizeLocation;

    #[test]
    fn test_slice_model() {
        let mut model = load_model("data/corner3.3mf").unwrap();
        println!("{:?}", model.unit);
        model.objects[0]
            .mesh
            .normalize(NormalizeLocation::LOWER_LEFT_BACK);
        let ret = slice_model(model).unwrap();
        for plane in ret.iter() {
            println!("{}", plane);
        }
    }

    #[test]
    fn test_serialize_slice_model() {
        let model = load_model("data/corner3.3mf").unwrap();
        let ret = slice_model(model).unwrap();
        let json = serde_json::to_string(&ret).unwrap();
        println!("{}", json)
    }
}
