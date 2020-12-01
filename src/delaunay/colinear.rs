use geo::CoordinateType;
use geo::Point;

use delaunator::Triangulation;
use num_traits::{float::Float, FromPrimitive};

// A triangulation is collinear if all its triangles have a non-null area.
pub fn colinear<T>(coords: &[Point<T>], d: &Triangulation) -> bool
where
    T: CoordinateType + Float + FromPrimitive,
{
    let t1e_minus_10 = T::from_f64(1e-10).unwrap();
    for i in (0..d.triangles.len()).step_by(3) {
        let a = d.triangles[i];
        let b = d.triangles[i + 1];
        let c = d.triangles[i + 2];
        let cross = (coords[c].x() - coords[a].x()) * (coords[b].y() - coords[a].y())
            - (coords[b].x() - coords[a].x()) * (coords[c].y() - coords[a].y());
        if cross > t1e_minus_10 {
            return false;
        }
    }
    return true;
}
