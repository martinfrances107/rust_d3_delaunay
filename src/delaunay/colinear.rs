use delaunator::Point;
use delaunator::Triangulation;

// struct triangle_data {
//   triangle: Vec<F>,
//   coords: Vec<F>,
// }

// A triangulation is collinear if all its triangles have a non-null area.
pub fn colinear(coords: &[Point], d: &Triangulation) -> bool {
  // let triangles = d.triangles;
  // for (let i = 0; i < triangles.length; i += 3) {
  for i in (0..d.triangles.len()).step_by(3) {
    let a =  d.triangles[i];
    let b =  d.triangles[i + 1];
    let c =  d.triangles[i + 2];
    let cross = (coords[c].x - coords[a].x) * (coords[b].y - coords[a].y)
      - (coords[b].x - coords[a].x) * (coords[c].y - coords[a].y);
    if cross > 1e-10 {
      return false;
    }

  }
  return true;
}

// // A triangulation is collinear if all its triangles have a non-null area
// function collinear(d) {
//   const {triangles, coords} = d;
//   for (let i = 0; i < triangles.length; i += 3) {
//     const a = 2 * triangles[i],
//           b = 2 * triangles[i + 1],
//           c = 2 * triangles[i + 2],
//           cross = (coords[c] - coords[a]) * (coords[b + 1] - coords[a + 1])
//                 - (coords[b] - coords[a]) * (coords[c + 1] - coords[a + 1]);
//     if (cross > 1e-10) return false;
//   }
//   return true;
// }
