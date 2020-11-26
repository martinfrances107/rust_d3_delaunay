use delaunator::Point;
use delaunator::Triangulation;

// A triangulation is collinear if all its triangles have a non-null area.
pub fn colinear(coords: &[Point], d: &Triangulation) -> bool {
    for i in (0..d.triangles.len()).step_by(3) {
        let a = d.triangles[i];
        let b = d.triangles[i + 1];
        let c = d.triangles[i + 2];
        let cross = (coords[c].x - coords[a].x) * (coords[b].y - coords[a].y)
            - (coords[b].x - coords[a].x) * (coords[c].y - coords[a].y);
        if cross > 1e-10 {
            return false;
        }
    }
    return true;
}
