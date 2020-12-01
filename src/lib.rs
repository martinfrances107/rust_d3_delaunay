#![allow(clippy::needless_return)]
pub mod delaunay;
pub mod path;
pub mod polygon;
pub mod voronoi;

use geo::CoordinateType;
use geo::Point;

pub trait RenderingContext2d<T>
where
    T: CoordinateType,
{
    fn new() -> Self;
    fn arc(&mut self, p: &Point<T>, r: T);
    fn close_path(&mut self);
    fn line_to(&mut self, p: &Point<T>);
    fn move_to(&mut self, p: &Point<T>);
    fn rect(&mut self, p: &Point<T>, w: T, h: T);
    fn value(&self) -> Vec<Point<T>>;
    fn value_str(&self) -> String;
}
