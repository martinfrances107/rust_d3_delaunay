#![allow(clippy::many_single_char_names)]

mod colinear;
mod jitter;

use crate::path::Path;
use crate::voronoi::Bounds;
use crate::voronoi::Voronoi;
use crate::RenderingContext2d;

use std::cmp::Ordering;
use std::fmt::Display;

use approx::AbsDiffEq;
use colinear::colinear;
use delaunator::{triangulate, Point as DPoint, Triangulation, EMPTY};
use derivative::*;
use geo::Point;
use geo::{CoordFloat, Coordinate};
use jitter::jitter;
use num_traits::float::FloatConst;
use num_traits::FromPrimitive;

use rust_d3_geo::clip::PointVisible;
use rust_d3_geo::projection::projection::Projection;
use rust_d3_geo::projection::Raw as ProjectionRaw;
use rust_d3_geo::stream::Stream;

/// Wrapper stores data associated with delaunator Triangulation.
///
/// hull and hald_edge data.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Delaunay<DRAIN, PR, PV, T>
where
    DRAIN: Stream<T = T>,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    colinear: Vec<usize>,
    #[derivative(Debug = "ignore")]
    /// A Triangulation stores the computed results from a delaunay mesh.
    pub delaunator: Triangulation,
    pub inedges: Vec<usize>,
    hull_index: Vec<usize>,
    pub half_edges: Vec<usize>,
    pub triangles: Vec<usize>,
    pub points: Vec<Coordinate<T>>,
    pub projection: Option<Projection<DRAIN, PR, PV, T>>,
    #[derivative(Debug = "ignore")]
    pub fx: Box<dyn Fn(Point<T>, usize, Vec<Point<T>>) -> T>,
    #[derivative(Debug = "ignore")]
    pub fy: Box<dyn Fn(Point<T>, usize, Vec<Point<T>>) -> T>,
}

impl<'a, DRAIN, PR, PV, T> Delaunay<DRAIN, PR, PV, T>
where
    DRAIN: Stream<T = T>,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst + FromPrimitive,
{
    /// Computes a delanay triangularization and stores the results.
    pub fn new(points: &[Coordinate<T>]) -> Self {
        // conversion into delaunay point!!!
        let d_point_in: Vec<DPoint> = points
            .iter()
            .map(|p| DPoint {
                x: p.x.to_f64().unwrap(),
                y: p.y.to_f64().unwrap(),
            })
            .collect();

        // TODO breaking API change if all points are collinear
        // now returning a special triangulaization where
        // all point are on the hull... I am not sure about the
        // implications of this yet.?????
        let delaunator = triangulate(&d_point_in);
        // let delaunator = match triangulate(&d_point_in) {
        //     Some(d) => d,
        //     None => {
        //         // When triangulation fails the javascript response
        //         // is mostly empty but hull has an value.
        //         Triangulation {
        //             triangles: Vec::new(),
        //             halfedges: Vec::new(),
        //             hull: (0..d_point_in.len()).collect(),
        //         }
        //     }
        // };

        let mut out = Self {
            delaunator,
            inedges: Vec::with_capacity(points.len() / 2),
            hull_index: Vec::with_capacity(points.len() / 2),
            points: points.to_vec(),
            colinear: Vec::new(),
            half_edges: Vec::new(),
            projection: None,
            fx: Box::new(|p: Point<T>, _i: usize, _points: Vec<Point<T>>| p.x()),
            fy: Box::new(|p: Point<T>, _i: usize, _points: Vec<Point<T>>| p.y()),
            triangles: Vec::new(),
        };

        out.init();

        out
    }

    #[inline]
    /// Use the stored delaunay mesh data to compute the assoicated voronoi mesh.
    pub fn voronoi(self, bounds: Option<Bounds<T>>) -> Voronoi<DRAIN, PR, PV, T> {
        Voronoi::new(self, bounds)
    }

    fn init(&mut self) {
        // Check for colinear.
        if self.delaunator.hull.len() > 2usize && colinear(&self.points, &self.delaunator) {
            let len = self.points.len() as u32 / 2;
            let mut colinear_vec: Vec<usize> = (0..len).map(|i| i as usize).collect();
            colinear_vec.sort_by(|i, j| {
                let x_diff = self.points[*i].x - self.points[*j].x;
                if x_diff != T::zero() {
                    if x_diff.is_sign_positive() {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                } else {
                    let y_diff = self.points[*i].y - self.points[*j].y;
                    if y_diff.is_zero() {
                        Ordering::Equal
                    } else if y_diff.is_sign_positive() {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                }
            });
            let e = self.colinear[0];
            let f = self.colinear[self.colinear.len() - 1];
            let bounds = [
                self.points[e].x,
                self.points[e].y,
                self.points[f].x,
                self.points[f].y,
            ];
            let r =
                T::from_f64(1e-8).unwrap() * (bounds[3] - bounds[1]).hypot(bounds[3] - bounds[0]);
            for i in 0..self.points.len() {
                let p = jitter(&self.points[i], r);
                self.points[i].x = p.x;
                self.points[i].y = p.y;
            }
            let d_point_in: Vec<DPoint> = self
                .points
                .iter()
                .map(|p| DPoint {
                    x: p.x.to_f64().unwrap(),
                    y: p.y.to_f64().unwrap(),
                })
                .collect();

            // TODO breaking API change if all points are collinear
            // now returning a special triangulaization where
            // all point are on the hull... I am not sure about the
            // implications of this yet.?????
            self.delaunator = triangulate(&d_point_in);
            // self.delaunator = match triangulate(&d_point_in) {
            //     Some(d) => d,
            //     None => {
            //         // When triangulation fails the javascript response
            //         // is mostly empty but hull has an value.
            //         Triangulation {
            //             triangles: Vec::new(),
            //             halfedges: Vec::new(),
            //             hull: (0..d_point_in.len()).collect(),
            //         }
            //     }
            // };
        } else {
            self.colinear.clear();
        }
        self.half_edges = self.delaunator.halfedges.clone();

        self.triangles = self.delaunator.triangles.clone();
        self.inedges = Vec::new();
        self.hull_index = Vec::new();
        let len = self.points.len();
        for _i in 0..len {
            self.inedges.push(EMPTY);
            self.hull_index.push(EMPTY);
        }

        // Compute an index from each point to an (arbitrary) incoming halfedge
        // Used to give the first neighbor of each point; for this reason,
        // on the hull we give priority to exterior halfedges
        for (e, he) in self.half_edges.iter().enumerate() {
            let p = if e % 3 == 2 {
                self.triangles[e - 2]
            } else {
                self.triangles[e + 1]
            };
            if *he == EMPTY || self.inedges[p] == EMPTY {
                self.inedges[p] = e;
            }
        }

        for (i, h) in self.delaunator.hull.iter().enumerate() {
            self.hull_index[*h] = i;
        }

        // degenerate case: 1 or 2 (distinct) points
        if self.delaunator.hull.len() <= 2 && !self.delaunator.hull.is_empty() {
            self.triangles = vec![EMPTY, EMPTY, EMPTY];
            self.half_edges = vec![EMPTY, EMPTY, EMPTY];
            self.triangles[0] = self.delaunator.hull[0];
            self.inedges[self.delaunator.hull[0]] = 1;
            if self.delaunator.hull.len() == 2 {
                self.triangles[1] = self.delaunator.hull[1];
                self.triangles[2] = self.delaunator.hull[1];
                self.inedges[self.delaunator.hull[1]] = 0;
            }
        }
    }

    pub fn find(self, p: &Coordinate<T>, i: Option<usize>) -> usize {
        // Skip return early if p is invalid.
        let mut i: usize = i.unwrap_or(0usize);
        let i0 = i;
        let mut c = self.step(i, p);

        while c != EMPTY && c != i && c != i0 {
            i = c;
            c = self.step(i, p)
        }
        c
    }

    pub fn step(&self, i: usize, p: &Coordinate<T>) -> usize {
        if self.inedges[i] == EMPTY || self.points.is_empty() {
            return (i + 1) % (self.points.len() >> 1);
        };
        let mut c = i;
        let mut dc = (p.x - self.points[i].x).powi(2) + (p.y - self.points[i].y).powi(2);
        let e0 = self.inedges[i];
        let mut e = e0;
        loop {
            let t = self.triangles[e];
            let dt = (p.x - self.points[t].x).powi(2) + (p.y - self.points[t].y).powi(2);
            if dt < dc {
                dc = dt;
                c = t;
            }

            if e % 3 == 2 {
                e -= 2;
            } else {
                e += 1;
            }

            if self.triangles[e] != i {
                // bad triangulation
                break;
            }
            e = self.half_edges[e];
            if e == EMPTY {
                e = self.delaunator.hull[(self.hull_index[i] + 1) % self.delaunator.hull.len()];
                if e != t
                    && (p.x - self.points[e].x).powi(2) + (p.y - self.points[e].y).powi(2) < dc
                {
                    return e;
                }
                break;
            }
            if e == e0 {
                break;
            }
        }

        c
    }

    // TODO Mising functions :-
    // fn render()
    // render(&self, context: &mut impl RenderingContext2d<T>)

    /// Output the hull to a string.
    ///
    /// Wrapper function - a departure from the javascript version.
    /// render() has been spit into two functions.
    /// rust expects variable type to be determined statically.
    /// 'context' cannot be either a Path type of a RenderingContext2d.
    pub fn render_points_to_string(&self, r: Option<T>) -> String
    where
        T: CoordFloat + Display,
    {
        let mut path = Path::<T>::default();
        self.render_points(&mut path, r);
        path.to_string()
    }

    // fn rednerPoints
    pub fn render_points(&self, context: &mut impl RenderingContext2d<T>, r: Option<T>) {
        // if (r === undefined && (!context || typeof context.moveTo !== "function")) r = context, context = null;
        // r = r == undefined ? 2 : +r;

        let tau = T::from(2_f64).unwrap() * T::PI();

        let r = match r {
            Some(r) => r,
            None => T::from(2.0).unwrap(),
        };
        // const buffer = context == null ? context = new Path : undefined;
        // const {points} = this;
        // for (let i = 0, n = self.points.length; i < n; i += 2) {
        for p in &self.points {
            //   let x = points[i], y = points[i + 1];
            context.move_to(&Coordinate { x: p.x + r, y: p.y });
            context.arc(p, r, T::zero(), tau);
        }
        // return buffer && buffer.value();
    }

    /// Output the hull to a string.
    ///
    /// Wrapper function - a departure from the javascript version.
    /// render() has been spit into two functions.
    /// rust expects variable type to be determined statically.
    /// 'context' cannot be either a Path type of a RenderingContext2d.
    pub fn render_hull_to_string(&self) -> String
    where
        T: CoordFloat + Display,
    {
        let mut path = Path::<T>::default();
        self.render_hull(&mut path);
        path.to_string()
    }

    /// Dumps the hull to the render context.
    pub fn render_hull(&self, context: &mut impl RenderingContext2d<T>) {
        let h = self.delaunator.hull[0];
        let n = self.delaunator.hull.len();
        context.move_to(&self.points[h]);
        for i in 1..n {
            let h = self.delaunator.hull[i];
            context.line_to(&self.points[h]);
        }
        context.close_path();
    }

    // TODO hullPolygon
    // TODO renderTriangle
    // TODO trianglePolygon
}
