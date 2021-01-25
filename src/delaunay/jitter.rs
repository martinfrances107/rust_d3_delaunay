use geo::{CoordFloat, Coordinate};

#[inline]
pub fn jitter<T>(p: &Coordinate<T>, r: T) -> Coordinate<T>
where
    T: CoordFloat,
{
    Coordinate {
        x: p.x + (p.x + p.y).sin() * r,
        y: p.y + (p.x - p.y).cos() * r,
    }
}
