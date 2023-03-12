use geo::CoordFloat;
use geo::Point;

///  Unused: Will be removed in the next Minor version.
#[deprected(since = "0.1.6", note = "Will be removed in 0.2")]
pub fn flat_array<T>(
    points: &[Point<T>],
    fx: Fn(Point<T>, usize, &[Point<T>]) -> T,
    fy: Fn(Point<T>, usize, &[Point<T>]) -> T,
) -> Vec<T>
where
    T: CoordFloat,
{
    let n = points.len();
    let array: Vec<F> = Vec::with_capacity(n * 2);
    for (i, p) in points.iter().enumerate() {
        array[i * 2] = fx(*p, i, points);
        array[i * 2 + 1] = fy(*p, i, points);
    }

    return array;
}
