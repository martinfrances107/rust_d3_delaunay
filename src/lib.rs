#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
//! A port of [d3/d3-delaunay](<https://github.com/d3/d3-delaunay>).
//!
//! Compute the Voronoi diagram of a set of two-dimensional points.
//!
//! <hr>
//!
//! Repository [`rust_d3_geo`](<https://github.com/martinfrances107/rust_d3_delaunay>)

/// Holds helper functions and a wrapper struct that hold data associated with a delaunay triangulation.
pub mod delaunay;
/// A struct that handles `RendingContext2d` API calls for points and line segments.
pub mod path;
/// A wrapper struct for a polygon, so that `RenderingAPI` call can be made.
pub mod polygon;
/// Storage a helpers for a  voronoi mesh.
pub mod voronoi;

use geo::CoordFloat;
use geo_types::Coord;

/// Interface in web browser.
pub trait CanvasRenderingContext2d<T>
where
    T: CoordFloat,
{
    /// draws an arc.
    fn arc(&mut self, p: &Coord<T>, r: T, start: T, stop: T);
    /// signals path is closed.
    fn close_path(&mut self);
    /// draws line from current point to p specified.
    fn line_to(&mut self, p: &Coord<T>);
    /// Sets the current point.
    fn move_to(&mut self, p: &Coord<T>);
    /// draw rectangle.
    fn rect(&mut self, p: &Coord<T>, w: T, h: T);
}
