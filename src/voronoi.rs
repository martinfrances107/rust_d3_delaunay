#![allow(clippy::many_single_char_names)]

use core::fmt::Display;
use std::collections::VecDeque;

use delaunator::EMPTY;
use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;
use num_traits::FromPrimitive;
use num_traits::Zero;

use super::delaunay::Delaunay;
use super::path::Path;
use super::polygon::Polygon;
use super::CanvasRenderingContext2d;

// xmin, ymin, xmax, ymax.
pub(super) type Bounds<T> = (T, T, T, T);

/// Data stores for a voronoi mesh.
#[derive(Debug)]
pub struct Voronoi<PROJECTOR, T>
where
    T: CoordFloat,
{
    /// The circumcenters of the Delaunay triangles as a Vec<c0, c1, …>.
    /// Each contiguous pair of coordinates c0.x, c0.y is the circumcenter for the corresponding triangle.
    /// These circumcenters form the coordinates of the Voronoi cell polygons.
    pub circumcenters: Vec<Coord<T>>,
    /// The delaunay triangulaiton.
    pub delaunay: Delaunay<PROJECTOR, T>,
    /// A Vec<v0, v0, w0, w0, …> where each non-zero quadruple describes an open (infinite) cell on the outer hull,
    ///  giving the directions of two open half-lines.
    pub vectors: Vec<Coord<T>>,
    /// Bounds component.
    pub xmin: T,
    /// Bounds component.
    pub ymin: T,
    /// Bounds component.
    pub xmax: T,
    /// Bounds component.
    pub ymax: T,
}

impl<PROJECTOR, T> Voronoi<PROJECTOR, T>
where
    T: CoordFloat + FloatConst + FromPrimitive,
{
    /// Given a delaunay object and a bounds construct a Voronoi object.
    ///
    /// # Panics
    ///  Will never happen as constants will always be converted into T.
    #[allow(clippy::similar_names)]
    pub fn new(delaunay: Delaunay<PROJECTOR, T>, bounds: Option<Bounds<T>>) -> Self {
        let (xmin, ymin, xmax, ymax) = bounds.map_or_else(
            || {
                (
                    T::zero(),
                    T::zero(),
                    T::from_f64(960f64).unwrap(),
                    T::from_f64(500f64).unwrap(),
                )
            },
            |bounds| bounds,
        );

        assert!(!(xmax < xmin || ymax < ymin), "Invalid bounds");
        let len = delaunay.points.len() * 2;

        let mut circumcenters = Vec::with_capacity(len);
        let mut vectors = Vec::with_capacity(len);
        let p_zero = Coord {
            x: T::zero(),
            y: T::zero(),
        };
        for _ in 0..len {
            circumcenters.push(p_zero);
            vectors.push(p_zero);
        }
        let mut v = Self {
            circumcenters,
            delaunay,
            vectors,
            xmin,
            ymin,
            xmax,
            ymax,
        };

        v.init();
        v
    }

    #[allow(clippy::similar_names)]
    #[allow(clippy::too_many_lines)]
    fn init(&mut self) {
        // Compute circumcenters.
        let circumcenter_len = self.delaunay.triangles.len() / 3;
        // Cannot use a slice cos need to be destermined at compile time.
        // self.circumcenters = (self.circumcenters[0..circumcenter_len]).to_vec();
        self.circumcenters.truncate(circumcenter_len);
        let triangles = &self.delaunay.triangles;
        let points = &self.delaunay.points;
        let hull = &self.delaunay.delaunator.hull;

        let mut i = 0usize;
        let mut j = 0usize;
        let n = triangles.len();
        if !n.is_zero() {
            let t1e_minus_8 = T::from_f64(1e-8).unwrap();
            let two = T::from_f64(2f64).unwrap();
            loop {
                let (x1, y1) = match triangles.get(i) {
                    Some(&EMPTY) | None => (None, None),
                    Some(t1) => (Some(points[*t1].x), Some(points[*t1].y)),
                };

                let (x2, y2) = match triangles.get(i + 1) {
                    Some(&EMPTY) | None => (None, None),
                    Some(t2) => (Some(points[*t2].x), Some(points[*t2].y)),
                };

                let (x3, y3) = match triangles.get(i + 2) {
                    Some(&EMPTY) | None => (None, None),
                    Some(t3) => (Some(points[*t3].x), Some(points[*t3].y)),
                };

                let dx = match (x1, x2) {
                    (Some(x1), Some(x2)) => x2 - x1,
                    _ => T::nan(),
                };

                let dy = match (y1, y2) {
                    (Some(y1), Some(y2)) => y2 - y1,
                    _ => T::nan(),
                };

                let ex = match (x1, x3) {
                    (Some(x1), Some(x3)) => x3 - x1,
                    _ => T::nan(),
                };

                let ey = match (y1, y3) {
                    (Some(y1), Some(y3)) => y3 - y1,
                    _ => T::nan(),
                };

                let ab = (dx * ey - dy * ex) * two;
                // Out of bound checking is x and y type values are bound of bounds
                // following the js closely dx and ex become nan
                // JS is wierd !NAN === true

                let (x, y) = if ab.is_zero() || ab.is_nan() {
                    // degenerate case (collinear diagram)
                    // almost equal points (degenerate triangle)
                    // the circumcenter is at the infinity, in a
                    // direction that is:
                    // 1. orthogonal to the halfedge.
                    let mut a = T::from(1e9).unwrap();
                    // 2. points away from the center; since the list of triangles starts
                    // in the center, the first point of the first triangle
                    // will be our reference
                    let r = triangles[0];
                    // In the JS original Math.sign() is used here
                    // Math.sign(0) return 0... not +/-1
                    // rust takes a different line.
                    //     +0.signum() is 1.
                    //     -0.signum() is -1.
                    // so I must special case -0 and +0 here.
                    let delta = (points[r].x - x1.unwrap()) * ey - (points[r].y - y1.unwrap()) * ex;
                    if delta.is_zero() {
                        a = T::zero();
                    } else {
                        a = a
                            * ((points[r].x - x1.unwrap()) * ey - (points[r].y - y1.unwrap()) * ex)
                                .signum();
                    }
                    match (x1, y1, x3, y3) {
                        (Some(x1), Some(y1), Some(x3), Some(y3)) => {
                            ((x1 + x3) / two - a * ey, (y1 + y3) / two + a * ex)
                        }
                        _ => (T::nan(), T::nan()),
                    }
                } else {
                    //NB if ab is not NAN then x1,y1 must be numbers.
                    let x1 = x1.unwrap();
                    let y1 = y1.unwrap();
                    if ab.abs() < t1e_minus_8 {
                        // almost equal points (degenerate triangle)
                        // NB if ab is not NAN then x3,y3 must be numbers.
                        let x3 = x3.unwrap();
                        let y3 = y3.unwrap();
                        ((x1 + x3) / two, (y1 + y3) / two)
                    } else {
                        let d = T::one() / ab;

                        let bl = dx * dx + dy * dy;
                        let cl = ex * ex + ey * ey;
                        (
                            (x1 + (ey * bl - dy * cl) * d),
                            (y1 + (dx * cl - ex * bl) * d),
                        )
                    }
                };
                self.circumcenters[j] = Coord { x, y };
                i += 3;

                j += 1;
                if i >= n {
                    break;
                }
            }
        }

        // JS used fill() here
        // VecDeque has no fill() unlike Vec
        // This is because VecDecque unlike Vec is not a
        // contiguous memory element.
        // .. is this going to be slow!!!
        let p_zero = Coord {
            x: T::zero(),
            y: T::zero(),
        };
        for v in &mut self.vectors {
            *v = p_zero;
        }
        // deviation from JS ... resolves index out of bounds issues
        // indexing using a negative value in JS returns undefined.
        // causes panic in rust.
        if !hull.is_empty() {
            // Compute exterior cell rays.
            let h = self.delaunay.delaunator.hull[self.delaunay.delaunator.hull.len() - 1];
            let mut p1 = h * 2;
            let mut x1 = points[h].x;
            let mut y1 = points[h].y;

            for h in hull {
                let p0 = p1;
                let x0 = x1;
                let y0 = y1;
                p1 = h * 2;
                x1 = points[*h].x;
                y1 = points[*h].y;
                let xdiff = x1 - x0;
                let ydiff = y0 - y1;
                // clip infinte pushed to both the front and back of this queue.
                // remove() then insert() here is inefficient .. but will only be done
                // once during init(). clip_finite() is a common operation.
                self.vectors.remove(p0 + 1);
                self.vectors.insert(p0 + 1, Coord { x: ydiff, y: xdiff });
                self.vectors.remove(p1);
                self.vectors.insert(p1, Coord { x: ydiff, y: xdiff });
            }
        }
    }

    /// Wrapper function - a departure from the javascript version.
    /// render() has been spit into two functions.
    /// rust expects variable type to be determined statically
    /// 'context' cannot be either a Path type of a `RenderingContext2d`.
    pub fn render_to_string(&self) -> String
    where
        T: CoordFloat + Display,
    {
        let mut path = Path::<T>::default();
        self.render(&mut path);
        path.to_string()
    }

    /// Render all segments.
    pub fn render(&self, context: &mut impl CanvasRenderingContext2d<T>)
    where
        T: CoordFloat + Display,
    {
        if self.delaunay.delaunator.hull.len() <= 1 {
            return;
        }

        for i in 0..self.delaunay.half_edges.len() {
            let j = self.delaunay.half_edges[i];
            if j < i || j == EMPTY {
                continue;
            }
            let ti = i / 3;
            let tj = j / 3;
            let pi = self.circumcenters[ti];
            let pj = self.circumcenters[tj];
            self.render_segment(&pi, &pj, context);
        }

        if let Some(mut h1) = self.delaunay.delaunator.hull.last() {
            for i in 0..self.delaunay.delaunator.hull.len() {
                let h0 = h1;
                h1 = &self.delaunay.delaunator.hull[i];
                let t = self.delaunay.inedges[*h1] / 3;
                let pi = self.circumcenters[t];
                let v = h0 * 2;
                let p = self.project(&pi, self.vectors[v + 1].x, self.vectors[v + 1].y);
                if let Some(p) = p {
                    self.render_segment(&pi, &p, context);
                }
            }
        }
    }

    // TODO implement render_bounds()

    /// Wrapper function - a departure from the javascript version.
    /// `render_cell()` has been spit into two functions.
    /// rust expects variable type to be determined statically
    /// 'context' cannot be either a Path type of a `RenderingContext2d`.
    pub fn render_cell_to_string(&self, i: usize) -> String
    where
        T: CoordFloat + Display + FloatConst,
    {
        let mut path = Path::default();
        self.render_cell(i, &mut path);
        path.to_string()
    }

    /// Renders cells of the voronoi mesh to a context.
    pub fn render_cell(&self, i: usize, context: &mut impl CanvasRenderingContext2d<T>)
    where
        T: CoordFloat + Display,
    {
        let points = self.clip(i);
        match points {
            None => {}
            Some(points) => {
                if points.is_empty() {
                    return;
                }

                context.move_to(&points[0]);

                let mut n = points.len();
                while points[0usize].x == points[n - 1].x && points[0].y == points[n - 1].y && n > 1
                {
                    n -= 1;
                }

                for i in 1..n {
                    if points[i].x != points[i - 1].x || points[i].y != points[i - 1].y {
                        context.line_to(&points[i].clone());
                    }
                }

                context.close_path();
            }
        }
    }
    //  cellPolgons*() is a generator which rustlang does not support.
    // in tests this is implemented as a for loop using cell_polygon().

    /// Returns a vec points that for a voronoi cell.
    pub fn cell_polygon(&self, i: usize) -> Vec<Coord<T>>
    where
        T: CoordFloat + Display,
    {
        let mut polygon = Polygon::default();
        self.render_cell(i, &mut polygon);
        polygon.0
    }

    fn render_segment(
        &self,
        p0: &Coord<T>,
        p1: &Coord<T>,
        context: &mut impl CanvasRenderingContext2d<T>,
    ) {
        let s;
        let c0 = self.regioncode(p0);
        let c1 = self.regioncode(p1);
        if c0 == 0 && c1 == 0 {
            context.move_to(p0);
            context.line_to(p1);
        } else {
            s = self.clip_segment(p0, p1, c0, c1);

            if let Some(s) = s {
                context.move_to(&s[0]);
                context.line_to(&s[2]);
            }
        }
    }

    /// Returns true if the cell with the specified index i contains the specified point p.
    #[inline]
    pub fn contains(&self, i: usize, p: &Coord<T>) -> bool {
        self.delaunay.step(i, p) == i
    }

    // TODO place neighbours* here() rustlang does not yet support generator functions.

    fn cell(&self, i: usize) -> Option<VecDeque<Coord<T>>> {
        let e0 = self.delaunay.inedges[i];
        if e0 == EMPTY {
            // Coincident point.
            return None;
        }
        let mut points: VecDeque<Coord<T>> = VecDeque::new();
        let mut e = e0;
        loop {
            let t = e / 3;
            points.push_back(self.circumcenters[t]);
            e = match e % 3 {
                2 => e - 2,
                _ => e + 1,
            };
            if self.delaunay.triangles[e] != i {
                break;
            } // bad triangulation.
            e = self.delaunay.half_edges[e];
            if e == e0 || e == EMPTY {
                break;
            }
        }
        Some(points)
    }

    fn clip(&self, i: usize) -> Option<VecDeque<Coord<T>>> {
        // degenerate case (1 valid point: return the box)
        if i == 0 && self.delaunay.delaunator.hull.len() == 1 {
            return Some(VecDeque::from(vec![
                Coord {
                    x: self.xmax,
                    y: self.ymin,
                },
                Coord {
                    x: self.xmax,
                    y: self.ymax,
                },
                Coord {
                    x: self.xmin,
                    y: self.ymax,
                },
                Coord {
                    x: self.xmin,
                    y: self.ymin,
                },
            ]));
        }
        self.cell(i).map(|points| {
            #[allow(non_snake_case)]
            let V = &self.vectors;
            let v = i * 2;
            if V[v].x != T::zero() || V[v].y != T::zero() {
                self.clip_infinite(i, &points, V[v].x, V[v].y, V[v + 1].x, V[v + 1].y)
            } else {
                self.clip_finite(i, &points)
            }
        })
    }

    #[allow(non_snake_case)]
    fn clip_finite(&self, i: usize, points: &VecDeque<Coord<T>>) -> VecDeque<Coord<T>> {
        let mut P = VecDeque::new();
        let mut p1 = points[points.len() - 1];
        let mut c1 = self.regioncode(&p1);
        let mut e1 = 0;
        let two = T::from_f64(2f64).unwrap();
        for point in points {
            let p0 = p1;
            p1 = *point;
            let c0 = c1;
            c1 = self.regioncode(&p1);
            if c0 == 0 && c1 == 0 {
                e1 = 0;
                if P.is_empty() {
                    P = VecDeque::from(vec![p1]);
                } else {
                    P.push_back(p1);
                }
            } else {
                let s0: Coord<T>;
                let s1: Coord<T>;
                if c0 == 0 {
                    match self.clip_segment(&p0, &p1, c0, c1) {
                        None => {
                            continue;
                        }
                        Some(s) => {
                            s1 = s[1];
                        }
                    }
                } else {
                    match self.clip_segment(&p1, &p0, c1, c0) {
                        None => {
                            continue;
                        }
                        Some(s) => {
                            s1 = s[0];
                            s0 = s[1];
                            let e0 = e1;
                            e1 = self.edgecode(&s0);
                            if e0 != 0u8 && e1 != 0u8 {
                                let len = P.len();
                                self.edge(i, e0, e1, &mut P, len);
                            }
                            if P.is_empty() {
                                P = VecDeque::from(vec![s0]);
                            } else {
                                P.push_back(s0);
                            }
                        }
                    }
                }
                let e0 = e1;
                e1 = self.edgecode(&s1);
                if e0 != 0u8 && e1 != 0u8 {
                    let len = P.len();
                    self.edge(i, e0, e1, &mut P, len);
                }
                if P.is_empty() {
                    P = VecDeque::from(vec![s1]);
                } else {
                    P.push_back(s1);
                };
            }
        }
        if !P.is_empty() {
            let e0 = e1;
            e1 = self.edgecode(&P[0]);
            if e0 != 0u8 && e1 != 0u8 {
                let len = P.len();
                self.edge(i, e0, e1, &mut P, len);
            }
        } else if self.contains(
            i,
            &Coord {
                x: (self.xmin + self.xmax) / two,
                y: (self.ymin + self.ymax) / two,
            },
        ) {
            return VecDeque::from(vec![
                Coord {
                    x: self.xmax,
                    y: self.ymin,
                },
                Coord {
                    x: self.xmax,
                    y: self.ymax,
                },
                Coord {
                    x: self.xmin,
                    y: self.ymax,
                },
                Coord {
                    x: self.xmin,
                    y: self.ymin,
                },
            ]);
        }
        P
    }

    #[allow(clippy::similar_names)]
    fn clip_segment(
        &self,
        p0_in: &Coord<T>,
        p1_in: &Coord<T>,
        c0_in: u8,
        c1_in: u8,
    ) -> Option<Vec<Coord<T>>> {
        let mut p0 = *p0_in;
        let mut p1 = *p1_in;
        let mut c0 = c0_in;
        let mut c1 = c1_in;
        loop {
            if c0 == 0 && c1 == 0 {
                return Some(vec![p0, p1]);
            }
            if c0 & c1 != 0 {
                return None;
            }
            let x;
            let y;
            let c = if c0 == 0 { c1 } else { c0 };

            if c & 0b1000 != 0 {
                x = p0.x + (p1.x - p0.x) * (self.ymax - p0.y) / (p1.y - p0.y);
                y = self.ymax;
            } else if c & 0b0100 != 0 {
                x = p0.x + (p1.x - p0.x) * (self.ymin - p0.y) / (p1.y - p0.y);
                y = self.ymin;
            } else if c & 0b0010 != 0 {
                y = p0.y + (p1.y - p0.y) * (self.xmax - p0.x) / (p1.x - p0.x);
                x = self.xmax;
            } else {
                y = p0.y + (p1.y - p0.y) * (self.xmin - p0.x) / (p1.x - p0.x);
                x = self.xmin;
            }
            if c0 == 0 {
                p1 = Coord { x, y };
                c1 = self.regioncode(&p1);
            } else {
                p0 = Coord { x, y };
                c0 = self.regioncode(&p0);
            }
        }
    }

    #[allow(clippy::similar_names)]
    fn clip_infinite(
        &self,
        i: usize,
        points: &VecDeque<Coord<T>>,
        vx0: T,
        vy0: T,
        vxn: T,
        vyn: T,
    ) -> VecDeque<Coord<T>> {
        #[allow(non_snake_case)]
        let mut P: VecDeque<Coord<T>> = points.clone();
        if let Some(p1) = self.project(&P[0], vx0, vy0) {
            P.push_front(p1);
        }

        if let Some(p2) = self.project(&P[P.len() - 1], vxn, vyn) {
            P.push_back(p2);
        }

        P = self.clip_finite(i, &P);
        let t2 = T::from_f64(2f64).unwrap();
        if !P.is_empty() {
            let mut n = P.len();
            let mut c0;
            let mut c1 = self.edgecode(&P[n - 1]);
            let mut j = 0;
            loop {
                c0 = c1;
                c1 = self.edgecode(&P[j]);
                if c0 != 0 && c1 != 0 {
                    j = self.edge(i, c0, c1, &mut P, j);
                    n = P.len();
                }
                j += 1;
                if j >= n {
                    break;
                }
            }
        } else if self.contains(
            i,
            &Coord {
                x: (self.xmin + self.xmax) / t2,
                y: (self.ymin + self.ymax) / t2,
            },
        ) {
            P = VecDeque::from([
                Coord {
                    x: self.xmin,
                    y: self.ymin,
                },
                Coord {
                    x: self.xmax,
                    y: self.ymin,
                },
                Coord {
                    x: self.xmax,
                    y: self.ymax,
                },
                Coord {
                    x: self.xmin,
                    y: self.ymax,
                },
            ]);
        }
        P
    }

    #[allow(non_snake_case)]
    fn edge(
        &self,
        i_in: usize,
        e0_in: u8,
        e1: u8,
        P: &mut VecDeque<Coord<T>>,
        j_in: usize,
    ) -> usize {
        let mut j = j_in;
        let mut e0 = e0_in;
        while e0 != e1 {
            let x;
            let y;
            match e0 {
                0b0101 => {
                    // top-left
                    e0 = 0b0100;
                    continue;
                }
                0b0100 => {
                    // top
                    e0 = 0b0110;
                    x = self.xmax;
                    y = self.ymin;
                }

                0b0110 => {
                    // top-right
                    e0 = 0b0010;
                    continue;
                }
                0b0010 => {
                    // right
                    e0 = 0b1010;
                    x = self.xmax;
                    y = self.ymax;
                }
                0b1010 => {
                    // bottom-right
                    e0 = 0b1000;
                    continue;
                }
                0b1000 => {
                    // bottom
                    e0 = 0b1001;
                    x = self.xmin;
                    y = self.ymax;
                }
                0b1001 => {
                    // bottom-left
                    e0 = 0b0001;
                    continue;
                }
                0b0001 => {
                    // left
                    e0 = 0b0101;
                    x = self.xmin;
                    y = self.ymin;
                }
                _ => {
                    panic!("unexpected code");
                }
            }

            // The JS version has subtle handling of out of bounds checks.
            let out_of_bounds = j >= P.len();
            if (out_of_bounds || P[j].x != x || P[j].y != y) && self.contains(i_in, &Coord { x, y })
            {
                P.insert(j, Coord { x, y });
                j += 1;
            }
        }

        if P.len() > 2 {
            let mut i = 0;
            loop {
                let j = (i + 1) % P.len();
                let k = (i + 2) % P.len();
                if P[i].x == P[j].x && P[j].x == P[k].x || P[i].y == P[j].y && P[j].y == P[k].y {
                    P.remove(j);
                    // Skip increment
                } else {
                    i += 1;
                }
                if i >= P.len() {
                    break;
                }
            }
        }
        j
    }

    fn project(&self, p0: &Coord<T>, vx: T, vy: T) -> Option<Coord<T>> {
        let mut t = T::infinity();
        // There is a mistake in the javascript implementation
        // if vy and vx == 0 then x, y are undefined.
        let mut x = T::zero();
        let mut y = T::zero();
        if vy < T::zero() {
            // top
            if p0.y <= self.ymin {
                return None;
            }
            let c = (self.ymin - p0.y) / vy;
            if c < t {
                y = self.ymin;
                t = c;
                x = p0.x + t * vx;
            }
        } else if vy > T::zero() {
            // bottom
            if p0.y >= self.ymax {
                return None;
            }
            let c = (self.ymax - p0.y) / vy;
            if c < t {
                y = self.ymax;
                t = c;
                x = p0.x + t * vx;
            }
        }

        if vx > T::zero() {
            // right
            if p0.x >= self.xmax {
                return None;
            }
            let c = (self.xmax - p0.x) / vx;
            if c < t {
                x = self.xmax;
                t = c;
                y = p0.y + t * vy;
            }
        } else if vx < T::zero() {
            // left
            if p0.x <= self.xmin {
                return None;
            }
            let c = (self.xmin - p0.x) / vx;
            if c < t {
                x = self.xmin;
                t = c;
                y = p0.x + t * vy;
            }
        }
        Some(Coord { x, y })
    }

    fn edgecode(&self, p: &Coord<T>) -> u8 {
        // Lower and upper nibbles.
        let lower = if p.x == self.xmin {
            0b0001
        } else if p.x == self.xmax {
            0b0010
        } else {
            0b0000
        };

        let upper = if p.y == self.ymin {
            0b0100
        } else if p.y == self.ymax {
            0b1000
        } else {
            0b0000
        };

        lower | upper
    }

    fn regioncode(&self, p: &Coord<T>) -> u8 {
        // Lower and upper nibbles.
        let lower = if p.x < self.xmin {
            0b001
        } else if p.x > self.xmax {
            0b0010
        } else {
            0b0000
        };

        let upper = if p.y < self.ymin {
            0b0100
        } else if p.y > self.ymax {
            0b1000
        } else {
            0b0000
        };

        lower | upper
    }
}
