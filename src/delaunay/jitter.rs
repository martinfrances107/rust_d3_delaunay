use geo::CoordinateType;
use geo::Point;
use num_traits::float::Float;

pub fn jitter<T>(p: &Point<T>, r: T) -> Point<T>
where
    T: CoordinateType + Float,
{
    return Point::new(
        p.x() + (p.x() + p.y()).sin() * r,
        p.y() + (p.x() - p.y()).cos() * r,
    );
}
