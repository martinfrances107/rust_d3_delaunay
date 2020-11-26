use crate::RenderingContext2d;
use delaunator::Point;
#[derive(Clone, Debug)]
pub struct Polygon {
    p: Vec<Point>,
}

impl RenderingContext2d for Polygon {
    fn new() -> Self {
        return Self { p: Vec::new() };
    }
    fn arc(&mut self, _p: Point, _r: f64) {}

    fn move_to(&mut self, p: Point) {
        self.p.push(p);
    }

    fn close_path(&mut self) {
        self.p.push(self.p[0].clone());
    }

    fn line_to(&mut self, p: Point) {
        self.p.push(p);
    }

    fn rect(&mut self, _p: Point, _w: f64, _h: f64) {}

    fn value_str(&self) -> String {
        return String::from("");
    }
    fn value(&self) -> Vec<Point> {
        self.p.clone()
    }
}
