#![allow(clippy::pedantic)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
//! A port of [d3/d3-delaunay](<https://github.com/d3/d3-delaunay>).
//!
//! Compute the Voronoi diagram of a set of two-dimensional points.
//!
//! <hr>
//!
//! Repository [rust_d3_geo](<https://github.com/martinfrances107/rust_d3_delaunay>)

extern crate derivative;

pub mod delaunay;
pub mod path;
pub mod polygon;
pub mod voronoi;

use geo::CoordFloat;
use geo::Coordinate;

pub trait RenderingContext2d<T>: ToString
where
    T: CoordFloat,
{
    fn arc(&mut self, p: &Coordinate<T>, r: T, start: T, stop: T);
    fn close_path(&mut self);
    fn line_to(&mut self, p: &Coordinate<T>);
    fn move_to(&mut self, p: &Coordinate<T>);
    fn rect(&mut self, p: &Coordinate<T>, w: T, h: T);
}
