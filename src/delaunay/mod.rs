#![allow(clippy::many_single_char_names)]

mod colinear;
mod jitter;

use crate::path::Path;
use crate::voronoi::Bounds;
use crate::voronoi::Voronoi;
use crate::RenderingContext2d;
use rust_d3_geo::Transform;

use std::cmp::Ordering;
use std::fmt::Display;
// use std::marker::Sync;

// use generator::done;
// use generator::Gn;

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
use rust_d3_geo::projection::projector::Projector;

use rust_d3_geo::stream::Stream;

type FnTransform<T> = Box<dyn Fn(Point<T>, usize, Vec<Point<T>>) -> T>;
/// Wrapper stores data associated with delaunator Triangulation.
///
/// hull and hald_edge data.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Delaunay<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    I: Clone,
    LB: Clone,
    LC: Clone,
    LU: Clone,
    PCNC: Clone,
    PCNU: Clone,
    PR: Transform<T = T>,
    PV: Clone,
    RC: Clone,
    RU: Clone,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
    // StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    // StreamNode<DRAIN, LINE, ResampleNode<DRAIN, PR, PostClipNode<DRAIN, DRAIN, T>, T>, T>:
    // Stream<EP = DRAIN, T = T>,
{
    // colinear: Vec<usize>,
    #[derivative(Debug = "ignore")]
    /// A Triangulation stores the computed results from a delaunay mesh.
    pub delaunator: Triangulation,
    /// The incoming halfedge indexes as a  [e0, e1, e2, …].
    /// For each point i, inedges[i] is the halfedge index e of an incoming halfedge.
    /// For coincident points, the halfedge index is EMPTY;
    /// for points on the convex hull, the incoming halfedge is on the convex hull;
    /// for other points, the choice of incoming halfedge is arbitrary.
    /// The inedges table can be used to traverse the Delaunay triangulation
    pub inedges: Vec<usize>,
    hull_index: Vec<usize>,
    /// The halfedge indexes as an [j0, j1, …]. For each index 0 ≤ i < halfedges.length,
    ///  there is a halfedge from triangle vertex j = halfedges[i] to triangle vertex i.
    ///  Equivalently, this means that triangle ⌊i / 3⌋ is adjacent to triangle ⌊j / 3⌋.
    pub half_edges: Vec<usize>,
    /// The triangle vertex indexes as an Vec<usize> [i0, j0, k0, i1, j1, k1, …].
    ///  Each contiguous triplet of indexes i, j, k forms a counterclockwise triangle.
    pub triangles: Vec<usize>,
    /// The coordinates of a point as an vector.
    pub points: Vec<Coordinate<T>>,
    //Projector<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
    pub projection: Option<Projector<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>>,
    #[derivative(Debug = "ignore")]
    pub fx: FnTransform<T>,
    #[derivative(Debug = "ignore")]
    pub fy: FnTransform<T>,
}

impl<'a, DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
    Delaunay<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    I: Clone,
    LB: Clone,
    LC: Clone,
    LU: Clone,
    PCNC: Clone,
    PCNU: Clone,
    PR: Transform<T = T>,
    PV: PointVisible<T = T>,
    RC: Clone,
    RU: Clone,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst + FromPrimitive,
    // StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    // StreamNode<DRAIN, LINE, ResampleNode<DRAIN, PR, PostClipNode<DRAIN, DRAIN, T>, T>, T>:
    //     Stream<EP = DRAIN, T = T>,
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
    pub fn voronoi(
        self,
        bounds: Option<Bounds<T>>,
    ) -> Voronoi<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> {
        Voronoi::new(self, bounds)
    }

    fn init(&mut self) {
        // Check for colinear.
        if self.delaunator.hull.len() > 2usize && colinear(&self.points, &self.delaunator) {
            let mut colinear_vec: Vec<usize> = (0..self.points.len()).collect();
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
            let e = colinear_vec[0];
            let f = colinear_vec[colinear_vec.len() - 1];
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
            // colinear_vec.clear();
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

    // fn neighbours(&self, i: usize) {
    //     let g = Gn::new_scoped(move |s| {
    //         // degenerate case with several collinear points
    //         if self.collinear.is_empty() {
    //             let l = self.collinear[i];
    //             if l > 0 {
    //                 s.yield_(Some(self.collinear[l - 1]));
    //             }
    //             if l < self.collinear.len() - 1 {
    //                 s.yield_(Some(self.collinear[l + 1]));
    //             }
    //             done!();
    //         }

    //         let e0 = self.inedges[i];
    //         if e0 == EMPTY {
    //             done!()
    //         }; // coincident point
    //         let e = e0;
    //         let p0 = EMPTY;
    //         loop {
    //             p0 = self.triangles[e];
    //             s.yield_(Some(p0));
    //             // e = e % 3 == 2 ? e - 2 : e + 1;
    //             let e = if e % 3 == 2 { e - 2 } else { e + 1 };
    //             if self.triangles[e] != i {
    //                 done!()
    //             }; // bad triangulation
    //             e = self.half_edges[e];
    //             if e == EMPTY {
    //                 let p = self.hull[(self._hull_index[i] + 1) % self.hull.len()];
    //                 if p != p0 {
    //                     s.yield_(Some(p));
    //                 }
    //                 done!();
    //             }
    //             if e == e0 {
    //                 break None;
    //             }
    //         }
    //     });
    // }

    /// Returns the index of the input point that is closest to the specified point p.
    /// The search is started at the specified point i. If i is not specified, it defaults to zero.
    pub fn find(&self, p: &Coordinate<T>, i: Option<usize>) -> usize {
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
