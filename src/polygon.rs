use crate::RenderingContext2d;
use geo::CoordinateType;
use geo::Point;
#[derive(Clone, Debug)]
pub struct Polygon<T>
where
    T: CoordinateType,
{
    p: Vec<Point<T>>,
}

impl<T> RenderingContext2d<T> for Polygon<T>
where
    T: CoordinateType,
{
    fn new() -> Self {
        return Self { p: Vec::new() };
    }
    fn arc(&mut self, _p: &Point<T>, _r: T) {}

    fn move_to(&mut self, p: &Point<T>) {
        self.p.push(*p);
    }

    fn close_path(&mut self) {
        self.p.push(self.p[0]);
    }

    fn line_to(&mut self, p: &Point<T>) {
        self.p.push(*p);
    }

    fn rect(&mut self, _p: &Point<T>, _w: T, _h: T) {}

    fn value_str(&self) -> String {
        return String::from("");
    }
    fn value(&self) -> Vec<Point<T>> {
        self.p.clone()
    }
}
