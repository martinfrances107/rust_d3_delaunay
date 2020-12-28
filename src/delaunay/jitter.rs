use geo::{Coordinate, CoordinateType};
use num_traits::float::Float;

pub fn jitter<T>(p: &Coordinate<T>, r: T) -> Coordinate<T>
where
    T: CoordinateType + Float,
{
    Coordinate{
        x: p.x + (p.x + p.y).sin() * r,
        y: p.y + (p.x - p.y).cos() * r,
    }
}
