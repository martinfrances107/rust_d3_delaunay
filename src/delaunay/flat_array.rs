use delaunator::Point;

pub fn flat_array<F>(points: &[Point], fx: Box<dyn Fn(Point, usize, Vec<Point>)-> F>, fy: Box<dyn Fn(Point, usize, Vec<Point>)-> F>) -> Vec<F>
where F: Float {
  let n = points.len();
  let array: Vec<F> = Vec::with_capacity(n*2);
  for (i, p) in points.iter().enumerate() {
    array[i*2] = fx(*p, i, points);
    array[i*2 + 1] = fy(*p, i, points);
  }

  return array;
}
// function flatArray(points, fx, fy, that) {
//   const n = points.length;
//   const array = new Float64Array(n * 2);
//   for (let i = 0; i < n; ++i) {
//     const p = points[i];
//     array[i * 2] = fx.call(that, p, i, points);
//     array[i * 2 + 1] = fy.call(that, p, i, points);
//   }
//   return array;
// }