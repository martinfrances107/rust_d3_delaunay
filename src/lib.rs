#![allow(clippy::needless_return)]
pub mod delaunay;
pub mod path;
pub mod polygon;
pub mod voronoi;

use delaunator::Point;

pub trait RenderingContext2d {
    fn new() -> Self;
    fn arc(&mut self, p: Point, r: f64);
    fn close_path(&mut self);
    fn line_to(&mut self, p: Point);
    fn move_to(&mut self, p: Point);
    fn rect(&mut self, p: Point, w: f64, h: f64);
    fn value(&self) -> Vec<Point>;
    fn value_str(&self) -> String;
}
