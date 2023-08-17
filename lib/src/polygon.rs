use crate::CanvasRenderingContext2d;
use geo::CoordFloat;
use geo_types::Coord;

/// A Vector of point which implements [`CanvasRenderingContext2d`].
#[derive(Clone, Debug)]
pub struct Polygon<T>(pub Vec<Coord<T>>)
where
    T: CoordFloat;

impl<T> Default for Polygon<T>
where
    T: CoordFloat,
{
    #[inline]
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<T> CanvasRenderingContext2d<T> for Polygon<T>
where
    T: CoordFloat,
{
    fn arc(&mut self, _p: &Coord<T>, _r: T, _start: T, _stop: T) {
        todo!("must implement.");
    }

    #[inline]
    fn move_to(&mut self, p: &Coord<T>) {
        self.0.push(*p);
    }

    #[inline]
    fn close_path(&mut self) {
        self.0.push(self.0[0]);
    }

    #[inline]
    fn line_to(&mut self, p: &Coord<T>) {
        self.0.push(*p);
    }

    #[inline]
    fn rect(&mut self, _p: &Coord<T>, _w: T, _h: T) {}
}
