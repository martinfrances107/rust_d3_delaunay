#![allow(clippy::needless_return)]
pub mod delaunay;
pub mod path;
pub mod polygon;
pub mod voronoi;

use geo::Coordinate;
use geo::CoordinateType;

// use rust_d3_geo::projection::projection_mutator::ProjectionMutator;
pub trait RenderingContext2d<T>
where
    T: CoordinateType,
{
    fn arc(&mut self, p: &Coordinate<T>, r: T);
    fn close_path(&mut self);
    fn line_to(&mut self, p: &Coordinate<T>);
    fn move_to(&mut self, p: &Coordinate<T>);
    fn rect(&mut self, p: &Coordinate<T>, w: T, h: T);
    fn value(&self) -> Vec<Coordinate<T>>;
    fn value_str(&self) -> String;
}
