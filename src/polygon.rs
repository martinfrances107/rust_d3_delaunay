use crate::RenderingContext2d;
use geo::CoordFloat;
use geo::Coordinate;
#[derive(Clone, Debug)]
pub struct Polygon<T>
where
    T: CoordFloat,
{
    p: Vec<Coordinate<T>>,
}

impl<T> Default for Polygon<T>
where
    T: CoordFloat,
{
    fn default() -> Self {
        Polygon { p: Vec::new() }
    }
}

impl<T> ToString for Polygon<T>
where
    T: CoordFloat,
{
    fn to_string(&self) -> String {
        return String::from("");
    }
}

impl<T> RenderingContext2d<T> for Polygon<T>
where
    T: CoordFloat,
{
    fn arc(&mut self, _p: &Coordinate<T>, _r: T) {}

    fn move_to(&mut self, p: &Coordinate<T>) {
        self.p.push(*p);
    }

    fn close_path(&mut self) {
        self.p.push(self.p[0]);
    }

    fn line_to(&mut self, p: &Coordinate<T>) {
        self.p.push(*p);
    }

    fn rect(&mut self, _p: &Coordinate<T>, _w: T, _h: T) {}

    fn value(&self) -> Vec<Coordinate<T>> {
        self.p.clone()
    }
}
