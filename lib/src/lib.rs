#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
//! A port of [d3/d3-delaunay](<https://github.com/d3/d3-delaunay>).
//!
//! Compute the Voronoi diagram of a set of two-dimensional points.
//!
//! <hr>
//!
//! Repository [`rust_d3_geo`](<https://github.com/martinfrances107/rust_d3_delaunay>)

pub mod delaunay;
pub mod path;
pub mod polygon;
pub mod voronoi;

// #[cfg(feature = "bevy")]
pub mod convex_polygon;

use geo::CoordFloat;
use geo_types::Coord;

/// Interface in web browser.
pub trait CanvasRenderingContext2d<T>
where
    T: CoordFloat,
{
    /// draws an arc.
    fn arc(&mut self, _p: &Coord<T>, _r: T, _start: T, _stop: T) {}
    /// signals path is closed.
    fn close_path(&mut self) {}
    /// draws line from current point to p specified.
    fn line_to(&mut self, _p: &Coord<T>) {}
    /// Sets the current point.
    fn move_to(&mut self, _p: &Coord<T>) {}
    /// draw rectangle.
    fn rect(&mut self, _p: &Coord<T>, _w: T, _h: T) {}
}
