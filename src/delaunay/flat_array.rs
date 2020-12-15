use geo::CoordinateType;
use geo::Point;

pub fn flat_array<T>(points: &[Point<T>], fx: Box<dyn Fn(Point<T>, usize, Vec<Point<T>>)-> T>, fy: Box<dyn Fn(Point<T>, usize, Vec<Point<T>>)-> T>) -> Vec<T>
where F: CoordinateType {
  let n = points.len();
  let array: Vec<F> = Vec::with_capacity(n*2);
  for (i, p) in points.iter().enumerate() {
    array[i*2] = fx(*p, i, points);
    array[i*2 + 1] = fy(*p, i, points);
  }

  return array;
}
