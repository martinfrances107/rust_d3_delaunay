use crate::RenderingContext2d;
use geo::CoordFloat;
use geo::Coordinate;

/// A Vector of point which implements RenderingContext2d.
#[derive(Clone, Debug)]
pub struct Polygon<T>
where
    T: CoordFloat,
{
    pub p: Vec<Coordinate<T>>,
}

impl<T> Default for Polygon<T>
where
    T: CoordFloat,
{
    #[inline]
    fn default() -> Self {
        Polygon { p: Vec::new() }
    }
}

impl<T> ToString for Polygon<T>
where
    T: CoordFloat,
{
    #[inline]
    fn to_string(&self) -> String {
        todo!("Do I need this.");
    }
}

impl<T> Polygon<T>
where
    T: CoordFloat,
{
    #[inline]
    pub fn value(&self) -> Vec<Coordinate<T>> {
        self.p.clone()
    }
}

impl<T> RenderingContext2d<T> for Polygon<T>
where
    T: CoordFloat,
{
    fn arc(&mut self, _p: &Coordinate<T>, _r: T, _start: T, _stop: T) {
        todo!("must implement.");
    }

    #[inline]
    fn move_to(&mut self, p: &Coordinate<T>) {
        self.p.push(*p);
    }

    #[inline]
    fn close_path(&mut self) {
        self.p.push(self.p[0]);
    }

    #[inline]
    fn line_to(&mut self, p: &Coordinate<T>) {
        self.p.push(*p);
    }

    #[inline]
    fn rect(&mut self, _p: &Coordinate<T>, _w: T, _h: T) {}
}
