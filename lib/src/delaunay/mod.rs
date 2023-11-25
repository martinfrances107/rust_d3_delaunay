#![allow(clippy::many_single_char_names)]

mod colinear;
mod jitter;

use core::cmp::Ordering;
use core::fmt::Debug;
use core::fmt::Display;

use colinear::colinear;
use colinear::Tri;
use delaunator::triangulate;
use delaunator::Point as DPoint;
use delaunator::Triangulation;
use delaunator::EMPTY;
#[cfg(feature = "generator")]
use generator::done;
#[cfg(feature = "generator")]
use generator::Generator;
#[cfg(feature = "generator")]
use generator::Gn;
use geo::CoordFloat;
use geo_types::Coord;
use jitter::jitter;
use num_traits::float::FloatConst;
use num_traits::FromPrimitive;

use crate::path::Path;
use crate::polygon::Polygon;
use crate::voronoi::Bounds;
use crate::voronoi::Voronoi;
use crate::CanvasRenderingContext2d;

// type FnTransform<T> = Box<dyn Fn(Point<T>, usize, Vec<Point<T>>) -> T>;

/// Wrapper stores data associated with delaunator Triangulation.
///
/// `hull` and `half_edge` data.
pub struct Delaunay<T>
where
    T: CoordFloat,
{
    /// A Triangulation stores the computed results from a delaunay mesh.
    pub delaunator: Triangulation,
    /// The incoming halfedge indexes as a  [e0, e1, e2, …].
    /// For each point i, inedges\[i\] is the halfedge index e of an incoming halfedge.
    /// For coincident points, the halfedge index is EMPTY;
    /// for points on the convex hull, the incoming halfedge is on the convex hull;
    /// for other points, the choice of incoming halfedge is arbitrary.
    /// The inedges table can be used to traverse the Delaunay triangulation
    pub inedges: Vec<usize>,
    hull_index: Vec<usize>,
    /// The halfedge indexes as an [j0, j1, …]. For each index 0 ≤ i < halfedges.length,
    ///  there is a halfedge from triangle vertex j = halfedges\[i\] to triangle vertex i.
    ///  Equivalently, this means that triangle ⌊i / 3⌋ is adjacent to triangle ⌊j / 3⌋.
    // pub half_edges: Vec<usize>,

    /// The coordinates of a point as an vector.
    pub points: Vec<Coord<T>>,
    // pub fx: FnTransform<T>,
    // pub fy: FnTransform<T>,
}

impl<T> Debug for Delaunay<T>
where
    T: CoordFloat,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Centroid<T>")
            .field(&self.delaunator)
            .field(&self.inedges)
            .field(&self.hull_index)
            // .field(&self.delaunator.half_edges)
            // .field(&self.triangles)
            .field(&self.points)
            .finish()
    }
}

impl<T> Delaunay<T>
where
    T: CoordFloat + FloatConst + FromPrimitive,
{
    /// # Panics
    /// unwrap() is used here but a panic will never happen as T will always be converted into f64.
    ///
    /// Computes a delaunay triangulation and stores the results.
    pub fn new(points: &[Coord<T>]) -> Self {
        // conversion into delaunay point!!!
        let d_point_in: Vec<DPoint> = points
            .iter()
            .map(|p| DPoint {
                x: p.x.to_f64().unwrap(),
                y: p.y.to_f64().unwrap(),
            })
            .collect();

        // TODO breaking API change if all points are colinear
        // now returning a special triangulation where
        // all point are on the hull... I am not sure about the
        // implications of this yet.?????
        let delaunator = triangulate(&d_point_in);

        let mut out = Self {
            delaunator,
            inedges: Vec::with_capacity(points.len() / 2),
            hull_index: Vec::with_capacity(points.len() / 2),
            points: points.to_vec(),
            // half_edges: Vec::with_capacity(points.len()),
            // fx: Box::new(|p: Point<T>, _i: usize, _points: Vec<Point<T>>| p.x()),
            // fy: Box::new(|p: Point<T>, _i: usize, _points: Vec<Point<T>>| p.y()),
            // triangles: Vec::new(),
        };

        out.init();

        out
    }

    #[inline]
    /// Use the stored delaunay mesh data to compute the associated voronoi mesh.
    pub fn voronoi(self, bounds: Option<Bounds<T>>) -> Voronoi<T> {
        Voronoi::new(self, bounds)
    }

    fn init(&mut self) {
        // Check for colinear.
        if self.delaunator.hull.len() > 2usize
            && colinear(&self.points, &self.delaunator) == Tri::Collinear
        {
            let mut colinear_vec: Vec<usize> = (0..self.points.len()).collect();
            colinear_vec.sort_by(|i, j| {
                let x_diff = self.points[*i].x - self.points[*j].x;
                if x_diff == T::zero() {
                    let y_diff = self.points[*i].y - self.points[*j].y;
                    if y_diff.is_zero() {
                        Ordering::Equal
                    } else if y_diff.is_sign_positive() {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                } else if x_diff.is_sign_positive() {
                    Ordering::Greater
                } else {
                    Ordering::Less
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
            // now returning a special triangulation where
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
        }
        // self.delaunator.half_edges = self.delaunator.halfedges.clone();

        // self.triangles = self.delaunator.triangles.clone();
        let len = self.points.len();
        self.inedges = Vec::with_capacity(len);
        self.hull_index = Vec::with_capacity(len);
        for _i in 0..len {
            self.inedges.push(EMPTY);
            self.hull_index.push(EMPTY);
        }

        // Compute an index from each point to an (arbitrary) incoming halfedge
        // Used to give the first neighbor of each point; for this reason,
        // on the hull we give priority to exterior halfedges
        for (e, he) in self.delaunator.halfedges.iter().enumerate() {
            let p = if e % 3 == 2 {
                self.delaunator.triangles[e - 2]
            } else {
                self.delaunator.triangles[e + 1]
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
            self.delaunator.triangles = vec![EMPTY, EMPTY, EMPTY];
            self.delaunator.halfedges = vec![EMPTY, EMPTY, EMPTY];
            self.delaunator.triangles[0] = self.delaunator.hull[0];
            self.inedges[self.delaunator.hull[0]] = 1;
            if self.delaunator.hull.len() == 2 {
                self.inedges[self.delaunator.hull[1]] = 0;
                self.delaunator.triangles[1] = self.delaunator.hull[1];
                self.delaunator.triangles[2] = self.delaunator.hull[1];
            }
        }
    }

    // Returns a generator that returns the neighbors for a given point
    // specified at the time of generation.
    // fn neighbors_generator(&self, i: usize) -> Generator<'_, (), Polygon<T>> {
    //     Gn::new_scoped(move |s| {
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
    //                 let p = self.delaunator.hull[(self.hull_index[i] + 1) % self.hull.len()];
    //                 if p != p0 {
    //                     s.yield_(Some(p));
    //                 }
    //                 done!();
    //             }
    //             if e == e0 {
    //                 break None;
    //             }
    //         }
    //     })
    // }

    /// Returns the index of the point that is closest to the specified point p.
    /// The search is started at the specified point i. If i is not specified, it defaults to zero.
    pub fn find(&self, p: &Coord<T>, i: Option<usize>) -> usize {
        // Skip return early if p is invalid.
        let mut i: usize = i.unwrap_or(0usize);
        let i0 = i;
        let mut c = self.step(i, p);

        while c != EMPTY && c != i && c != i0 {
            i = c;
            c = self.step(i, p);
        }
        c
    }

    /// Step through the triangulation, starting at i, return the index
    /// of the point closets to point p.
    pub fn step(&self, i: usize, p: &Coord<T>) -> usize {
        if self.inedges[i] == EMPTY || self.points.is_empty() {
            return (i + 1) % (self.points.len() >> 1);
        };
        let mut c = i;
        let mut dc = (p.x - self.points[i].x).powi(2) + (p.y - self.points[i].y).powi(2);
        let e0 = self.inedges[i];
        let mut e = e0;
        loop {
            let t = self.delaunator.triangles[e];
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

            if self.delaunator.triangles[e] != i {
                // bad triangulation
                break;
            }
            e = self.delaunator.halfedges[e];
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

    /// Returns the delaunay mesh as a string.
    #[must_use]
    pub fn render_to_string(&self) -> String
    where
        T: CoordFloat + Display,
    {
        let mut path = Path::<T>::default();
        self.render(&mut path);
        path.to_string()
    }

    /// Dumps the delaunay mesh to the [`CanvasRenderingContext2d`].
    pub fn render(&self, context: &mut impl CanvasRenderingContext2d<T>) {
        for i in 0..self.delaunator.halfedges.len() {
            let j = self.delaunator.halfedges[i];
            if j < i || j == EMPTY {
                continue;
            };
            let ti = self.delaunator.triangles[i];
            let tj = self.delaunator.triangles[j];

            context.move_to(&self.points[ti]);
            context.line_to(&self.points[tj]);
        }
        self.render_hull(context);
    }

    /// Output the hull to a string.
    ///
    /// Wrapper function - a departure from the javascript version.
    /// render() has been spit into two functions.
    /// rust expects variable type to be determined statically.
    /// 'context' cannot be either a Path type of a [`CanvasRenderingContext2d`].
    pub fn render_points_to_string(&self, r: Option<T>) -> String
    where
        T: CoordFloat + Display,
    {
        let mut path = Path::<T>::default();
        self.render_points(&mut path, r);
        path.to_string()
    }

    /// Given a context render the points of the triangulation.
    ///
    /// # Panics
    ///  Will never happen as '2' will always be converted into T.
    pub fn render_points(&self, context: &mut impl CanvasRenderingContext2d<T>, r: Option<T>) {
        // if (r === undefined && (!context || typeof context.moveTo !== "function")) r = context, context = null;
        // r = r == undefined ? 2 : +r;

        let tau = T::from(2_f64).unwrap() * T::PI();

        let r = r.map_or_else(|| T::from(2.0).unwrap(), |r| r);

        for p in &self.points {
            context.move_to(&Coord { x: p.x + r, y: p.y });
            context.arc(p, r, T::zero(), tau);
        }
    }

    /// Output the hull to a string.
    ///
    /// Wrapper function - a departure from the javascript version.
    /// render() has been spit into two functions.
    /// rust expects variable type to be determined statically.
    /// 'context' cannot be either a Path type of a [`CanvasRenderingContext2d`].
    #[must_use]
    pub fn render_hull_to_string(&self) -> String
    where
        T: CoordFloat + Display,
    {
        let mut path = Path::<T>::default();
        self.render_hull(&mut path);
        path.to_string()
    }

    /// Dumps the hull to the [`CanvasRenderingContext2d`].
    pub fn render_hull(&self, context: &mut impl CanvasRenderingContext2d<T>) {
        let h = self.delaunator.hull[0];
        let n = self.delaunator.hull.len();
        context.move_to(&self.points[h]);
        for i in 1..n {
            let h = self.delaunator.hull[i];
            context.line_to(&self.points[h]);
        }
        context.close_path();
    }

    /// Returns the hull as series of [`Coord`]'s
    #[must_use]
    pub fn hull_polygon(&self) -> Vec<Coord<T>> {
        let mut polygon = Polygon::default();
        self.render_hull(&mut polygon);
        polygon.0
    }
}

/// Generator and helper.
impl<T> Delaunay<T>
where
    T: CoordFloat + Display + FloatConst + FromPrimitive + Send + Sync,
{
    /// Renders selected triangle into [`CanvasRenderingContext2d`]
    pub fn render_triangle(&self, mut i: usize, context: &mut impl CanvasRenderingContext2d<T>) {
        i *= 3;
        let t0 = self.delaunator.triangles[i];
        let t1 = self.delaunator.triangles[i + 1];
        let t2 = self.delaunator.triangles[i + 2];
        context.move_to(&self.points[t0]);
        context.move_to(&self.points[t1]);
        context.move_to(&self.points[t2]);
        context.close_path();
    }

    /// Deviation from javascript
    /// `render_triangle` is split into two
    /// idiomatic rust avoid multi-value context
    #[must_use]
    pub fn render_triangle_to_string(&self, i: usize) -> String
    where
        T: Display,
    {
        let mut path = Path::default();
        self.render_triangle(i, &mut path);
        path.to_string()
    }

    /// Returns a polygon representing the selected triangle.
    #[must_use]
    pub fn triangle_polygon(&self, i: usize) -> Polygon<T> {
        let mut polygon = Polygon::default();
        self.render_triangle(i, &mut polygon);
        polygon
    }

    /// Returns a [`Generator`] that can be use to successively yield triangles.
    #[must_use]
    #[cfg(feature = "generator")]
    pub fn triangle_polygons_generator(&self) -> Generator<'_, (), Polygon<T>> {
        Gn::new_scoped(move |mut s| {
            for i in 0..self.triangles.len() / 3 {
                s.yield_with(self.triangle_polygon(i));
            }
            done!();
        })
    }
}
