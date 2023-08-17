use geo::CoordFloat;
use geo_types::Coord;

#[inline]
pub fn jitter<T>(p: &Coord<T>, r: T) -> Coord<T>
where
    T: CoordFloat,
{
    Coord {
        x: p.x + (p.x + p.y).sin() * r,
        y: p.y + (p.x - p.y).cos() * r,
    }
}
