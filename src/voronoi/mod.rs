#![allow(clippy::clippy::many_single_char_names)]

use std::fmt::Display;
use std::collections::VecDeque;
use std::ops::AddAssign;

use delaunator::EMPTY;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::{AsPrimitive, Float, FloatConst, FromPrimitive};

use crate::delaunay::Delaunay;
use crate::path::Path;
use crate::polygon::Polygon;
use crate::RenderingContext2d;

pub struct Voronoi<T>
where
    T: AddAssign + AsPrimitive<T> + Default + CoordFloat + FloatConst ,
{
    pub circumcenters: Vec<Coordinate<T>>,
    delaunay: Delaunay<T>,
    pub vectors: VecDeque<Coordinate<T>>,
    pub xmin: T,
    pub ymin: T,
    pub xmax: T,
    pub ymax: T,
}

impl<T> Default for Voronoi<T>
where
    T: AddAssign + AsPrimitive<T> + Default + CoordFloat + FloatConst + FromPrimitive,
{
    #[inline]
    fn default() -> Voronoi<T> {
        Voronoi {
            delaunay: Delaunay::default(),
            circumcenters: Vec::<Coordinate<T>>::new(),
            vectors: VecDeque::new(),
            xmin: T::from_f64(0f64).unwrap(),
            xmax: T::from_f64(960.0f64).unwrap(),
            ymin: T::from_f64(0.0f64).unwrap(),
            ymax: T::from_f64(500f64).unwrap(),
        }
    }
}

impl<T> Voronoi<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst + FromPrimitive + AsPrimitive<T>,
{
    pub fn new(delaunay: Delaunay<T>, b_in: Option<(T, T, T, T)>) -> Self {
        let b;
        let mut v: Voronoi<T>;
        match b_in {
            Some(b_in) => {
                b = b_in;
            }
            None => {
                b = (
                    T::zero(),
                    T::zero(),
                    T::from_f64(960f64).unwrap(),
                    T::from_f64(500f64).unwrap(),
                );
            }
        }
        let xmin = b.0;
        let ymin = b.1;
        let xmax = b.2;
        let ymax = b.3;

        if xmax < xmin || ymax < ymin {
            panic!("Invalid bounds");
        }
        let len = delaunay.points.len() * 2;

        let mut circumcenters = Vec::with_capacity(len);
        let mut vectors = VecDeque::with_capacity(len);
        for _ in 0..len {
            circumcenters.push(Coordinate {
                x: T::zero(),
                y: T::zero(),
            });
            vectors.push_back(Coordinate {
                x: T::zero(),
                y: T::zero(),
            });
        }
        v = Self {
            delaunay,
            circumcenters,
            vectors,
            xmin,
            ymin,
            xmax,
            ymax,
        };

        v.init();
        return v;
    }

    fn init(&mut self) {
        // Compute circumcenters.
        let circumcenter_len = self.delaunay.triangles.len() / 3;
        // cannot use a slice cos need to be destermined at compile time.
        self.circumcenters = (&self.circumcenters[0..circumcenter_len]).to_vec();

        let triangles = &self.delaunay.triangles;
        let points = &self.delaunay.points;
        let hull = &self.delaunay.hull;

        let mut i = 0usize;
        let mut j = 0usize;
        let n = triangles.len();
        let mut x: T;
        let mut y: T;

        let t1e_minus_8 = T::from_f64(1e-8).unwrap();
        let t1e_plus_8 = T::from_f64(1e8).unwrap();
        let t2f64 = T::from_f64(2f64).unwrap();
        loop {
            let t1 = triangles[i];
            let t2 = triangles[i + 1];
            let t3 = triangles[i + 2];
            let x1 = points[t1].x;
            let y1 = points[t1].y;
            let x2 = points[t2].x;
            let y2 = points[t2].y;
            let x3 = points[t3].x;
            let y3 = points[t3].y;

            let dx = x2 - x1;
            let dy = y2 - y1;
            let ex = x3 - x1;
            let ey = y3 - y1;
            let bl = dx * dx + dy * dy;
            let cl = ex * ex + ey * ey;
            let ab = (dx * ey - dy * ex) * T::from_f64(2f64).unwrap();

            if ab.is_zero() {
                // degenerate case (collinear diagram)
                x = (x1 + x3) / t2f64 - t1e_plus_8 * ey;
                y = (y1 + y3) / t2f64 + t1e_plus_8 * ex;
            } else if ab.abs() < t1e_minus_8 {
                // almost equal points (degenerate triangle)
                x = (x1 + x3) / t2f64;
                y = (y1 + y3) / t2f64;
            } else {
                let d = T::one() / ab;
                x = x1 + (ey * bl - dy * cl) * d;
                y = y1 + (dx * cl - ex * bl) * d;
            }
            self.circumcenters[j] = Coordinate { x, y };
            i += 3;
            j += 1;
            if i >= n {
                break;
            }
        }

        // Compute exterior cell rays.
        let h = self.delaunay.hull[self.delaunay.hull.len() - 1];
        let mut p0: usize;
        let mut p1 = h * 2;
        let mut x0;
        let mut x1 = points[h].x;
        let mut y0;
        let mut y1 = points[h].y;

        let vectors_len = self.vectors.len();
        self.vectors.clear();
        for _ in 0..vectors_len {
            self.vectors.push_back(Coordinate {
                x: T::zero(),
                y: T::zero(),
            });
        }

        for h in hull {
            p0 = p1;
            x0 = x1;
            y0 = y1;
            p1 = h * 2;
            x1 = points[*h].x;
            y1 = points[*h].y;
            let xdiff = x1 - x0;
            let ydiff = y0 - y1;
            // clip infinte pushed to both the front and back of this queue.
            // remove() then insert() here is inefficient .. but will only be done
            // once during init().  clip_finite() is a common operation.
            self.vectors.remove(p0 + 1);
            self.vectors
                .insert(p0 + 1, Coordinate { x: ydiff, y: xdiff });
            self.vectors.remove(p1);
            self.vectors.insert(p1, Coordinate { x: ydiff, y: xdiff });
        }
    }

    // This function does not exits in javascript version.
    // It permits a simplification of render().
    pub fn render_to_string(&self) -> String
    where
        T: CoordFloat + Display,
    {
        let mut path = Path::<T>::default();
        self.render(&mut path);
        path.value_str()
    }

    pub fn render(&self, context: &mut impl RenderingContext2d<T>)
    where
        T: CoordFloat + Display,
    {
        // let circumcenters = self.circumcenters;
        for i in 0..self.delaunay.half_edges.len() {
            let j = self.delaunay.half_edges[i];
            if j < i {
                continue;
            }
            let ti = (i as f64 / 3.).floor() as usize;
            let tj = (j as f64 / 3.).floor() as usize;
            let pi = self.circumcenters[ti];
            let pj = self.circumcenters[tj];
            self.render_segment(pi, pj, context);
        }

        let mut h0;
        let mut h1 = *self.delaunay.hull.last().unwrap();
        for i in 0..self.delaunay.hull.len() {
            h0 = h1;
            h1 = self.delaunay.hull[i];
            let t = (self.delaunay.inedges[h1] as f64 / 3.).floor() as usize;
            let pi = self.circumcenters[t];
            let v = h0 * 2;
            let p = self.project(pi, self.vectors[v + 2].x, self.vectors[v + 2].y);
            if let Some(p) = p {
                self.render_segment(pi, p, context);
            }
        }
    }

    // TODO implement render_bounds()

    pub fn render_cell<C: RenderingContext2d<T>>(&self, i: usize, context: &mut C) -> String {
        let points = self.clip(i);
        return match points {
            None => {
                return "".to_string();
            }
            Some(points) => {
                if points.is_empty() {
                    return "".to_string();
                }

                context.move_to(&points[0].clone());
                let mut n = points.len();
                while (points[0usize].x - points[n - 1].x).abs() < T::epsilon()
                    && (points[0].y - points[n - 1].y).abs() < T::epsilon()
                    && n > 1
                {
                    n -= 1;
                }

                for i in 1..n {
                    if (points[i].x - points[i - 1].x).abs() >= T::epsilon()
                        || (points[i].y - points[i - 1].y).abs() >= T::epsilon()
                    {
                        context.line_to(&points[i].clone());
                    }
                }
                context.close_path();

                context.value_str()
            }
        };
    }

    //  cellPolgons*() is a generator which rustlang does not support.
    // in tests this is implemented as a for loop using cell_polygon().

    pub fn cell_polygon(&self, i: usize) -> Vec<Coordinate<T>> {
        let mut polygon = Polygon::default();
        self.render_cell(i, &mut polygon);
        return polygon.value();
    }

    fn render_segment(
        &self,
        p0: Coordinate<T>,
        p1: Coordinate<T>,
        context: &mut impl RenderingContext2d<T>,
    ) {
        let s;
        let c0 = self.regioncode(p0);
        let c1 = self.regioncode(p1);
        if c0 == 0 && c1 == 0 {
            context.move_to(&p0);
            context.move_to(&p1);
        } else {
            s = self.clip_segment(p0, p1, c0, c1);

            if let Some(s) = s {
                context.move_to(&s[0]);
                context.move_to(&s[2]);
            }
        }
    }

    pub fn contains(&self, i: usize, x: T, y: T) -> bool {
        return self.delaunay.step(i, x, y) == i;
    }

    // TODO place neighbours* here() rustlang does not yet support generator functions.

    fn cell(&self, i: usize) -> Option<VecDeque<Coordinate<T>>> {
        let e0 = self.delaunay.inedges[i];
        if e0 == EMPTY {
            // Coincident point.
            return None;
        }
        let mut points: VecDeque<Coordinate<T>> = VecDeque::new();
        let mut e = e0;
        loop {
            let t = (e as f64 / 3f64).floor() as usize;
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
        return Some(points);
    }

    fn clip(&self, i: usize) -> Option<VecDeque<Coordinate<T>>> {
        // degenerate case (1 valid point: return the box)
        if i == 0 && self.delaunay.hull.len() == 1 {
            return Some(
                vec![
                    Coordinate {
                        x: self.xmax,
                        y: self.ymin,
                    },
                    Coordinate {
                        x: self.xmax,
                        y: self.ymax,
                    },
                    Coordinate {
                        x: self.xmin,
                        y: self.ymax,
                    },
                    Coordinate {
                        x: self.xmin,
                        y: self.ymin,
                    },
                ]
                .into_iter()
                .collect(),
            );
        }
        match self.cell(i) {
            None => {
                return None;
            }
            Some(points) => {
                #[allow(non_snake_case)]
                let V = &self.vectors;
                let v = i * 2;
                if V[v].x != T::zero() || V[v].y != T::zero() {
                    return Some(self.clip_infinite(
                        i,
                        &points,
                        V[v].x,
                        V[v].y,
                        V[v + 1].x,
                        V[v + 1].y,
                    ));
                } else {
                    return Some(self.clip_finite(i, &points));
                }
            }
        }
    }

    fn clip_finite(&self, i: usize, points: &VecDeque<Coordinate<T>>) -> VecDeque<Coordinate<T>> {
        let n = points.len();
        #[allow(non_snake_case)]
        let mut P = VecDeque::new();
        let mut p0: Coordinate<T>;
        let mut p1 = points[n - 1];
        let mut c0;
        let mut c1 = self.regioncode(p1);
        let mut e0;
        // There is a bug/inconsitencey in the javascript implementation.
        // e1 must be given a reasonable default value.
        let mut e1 = 0;
        let t2 = T::from_f64(2f64).unwrap();
        for point in points {
            p0 = p1;
            p1 = *point;
            c0 = c1;
            c1 = self.regioncode(p1);
            if c0 == 0 && c1 == 0 {
                // e0 = e1;
                e1 = 0;
                if !P.is_empty() {
                    P.push_back(p1);
                } else {
                    P = vec![p1].into_iter().collect();
                }
            } else {
                #[allow(non_snake_case)]
                let S;
                let s0: Coordinate<T>;
                let s1: Coordinate<T>;
                if c0 == 0 {
                    S = self.clip_segment(p0, p1, c0, c1);
                    match S {
                        None => {
                            continue;
                        }
                        Some(s) => {
                            // sx0 = s[0].x;
                            // sy0 = s[0].y;
                            s1 = s[1];
                        }
                    }
                } else {
                    S = self.clip_segment(p1, p0, c1, c0);
                    match S {
                        None => {
                            continue;
                        }
                        Some(s) => {
                            s1 = s[0];
                            s0 = s[1];
                            e0 = e1;
                            e1 = self.edgecode(s0);
                            if e0 != 0u8 && e1 != 0u8 {
                                let len = P.len();
                                self.edge(i, e0, e1, &mut P, len);
                            }
                            if !P.is_empty() {
                                P.push_back(s0);
                            } else {
                                P = vec![s0].into_iter().collect();
                            }
                        }
                    }
                }
                e0 = e1;
                e1 = self.edgecode(s1);
                if e0 != 0u8 && e1 != 0u8 {
                    let len = P.len();
                    self.edge(i, e0, e1, &mut P, len);
                }
                if !P.is_empty() {
                    P.push_back(s1);
                } else {
                    P = vec![s1].into_iter().collect();
                };
            }
        }
        if !P.is_empty() {
            e0 = e1;
            e1 = self.edgecode(P[0]);
            if e0 != 0u8 && e1 != 0u8 {
                let len = P.len();
                self.edge(i, e0, e1, &mut P, len);
            }
        } else if self.contains(
            i,
            (self.xmin + self.xmax) / t2,
            (self.ymin + self.ymax) / T::from_f64(2f64).unwrap(),
        ) {
            return vec![
                Coordinate {
                    x: self.xmax,
                    y: self.ymin,
                },
                Coordinate {
                    x: self.xmax,
                    y: self.ymax,
                },
                Coordinate {
                    x: self.xmin,
                    y: self.ymax,
                },
                Coordinate {
                    x: self.xmin,
                    y: self.ymin,
                },
            ]
            .into_iter()
            .collect();
        }
        return P;
    }

    fn clip_segment(
        &self,
        p0_in: Coordinate<T>,
        p1_in: Coordinate<T>,
        c0_in: u8,
        c1_in: u8,
    ) -> Option<Vec<Coordinate<T>>> {
        let mut p0 = p0_in;
        let mut p1 = p1_in;
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
            let c = if c0 != 0 { c0 } else { c1 };

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
            if c0 != 0 {
                p0 = Coordinate { x, y };
                c0 = self.regioncode(p0);
            } else {
                p1 = Coordinate { x, y };
                c1 = self.regioncode(p1);
            }
        }
    }

    fn clip_infinite(
        &self,
        i: usize,
        points: &VecDeque<Coordinate<T>>,
        vx0: T,
        vy0: T,
        vxn: T,
        vyn: T,
    ) -> VecDeque<Coordinate<T>> {
        #[allow(non_snake_case)]
        let mut P: VecDeque<Coordinate<T>> = VecDeque::into(points.clone());
        if let Some(p1) = self.project(P[0], vx0, vy0) {
            P.push_front(p1);
        }

        if let Some(p2) = self.project(P[P.len() - 1], vxn, vyn) {
            P.push_back(p2);
        }

        P = self.clip_finite(i, &P);
        let t2 = T::from_f64(2f64).unwrap();
        if !P.is_empty() {
            let mut n = P.len();
            let mut c0;
            let mut c1 = self.edgecode(P[n - 1]);
            let mut j = 0;
            loop {
                c0 = c1;
                c1 = self.edgecode(P[j]);
                if c0 != 0 && c1 != 0 {
                    j = self.edge(i, c0, c1, &mut P, j);
                    n = P.len();
                }
                j += 2;
                if j >= n {
                    break;
                }
            }
        } else if self.contains(
            i,
            (self.xmin + self.xmax) / t2,
            (self.ymin + self.ymax) / t2,
        ) {
            P = vec![
                Coordinate {
                    x: self.xmin,
                    y: self.ymin,
                },
                Coordinate {
                    x: self.xmax,
                    y: self.ymin,
                },
                Coordinate {
                    x: self.xmax,
                    y: self.ymax,
                },
                Coordinate {
                    x: self.xmin,
                    y: self.ymax,
                },
            ]
            .into_iter()
            .collect();
        }
        return P;
    }

    #[allow(non_snake_case)]
    fn edge(
        &self,
        i: usize,
        e0_in: u8,
        e1: u8,
        P: &mut VecDeque<Coordinate<T>>,
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

            if ((P[j].x - x).abs() >= T::epsilon() || (P[j].y - y).abs() >= T::epsilon())
                && self.contains(i, x, y)
            {
                P.insert(j, Coordinate { x, y });
                j += 1;
            }
        }
        if P.len() > 2 {
            let mut i = 0;
            loop {
                let j = (i + 1) % P.len();
                let k = (i + 2) % P.len();
                if (P[i].x - P[j].x).abs() < T::epsilon() && (P[j].x - P[k].x).abs() < T::epsilon()
                    || (P[i].y - P[j].y).abs() < T::epsilon()
                        && (P[j].y - P[k].y).abs() < T::epsilon()
                {
                    // P.splice(j, 2);
                    P.remove(j);
                    i -= 1;
                }
                i += 1;
                if i >= P.len() {
                    break;
                }
            }
        }
        return j;
    }

    fn project(&self, p0: Coordinate<T>, vx: T, vy: T) -> Option<Coordinate<T>> {
        let mut t = Float::infinity();
        let mut c;
        // The is a mistake in the javascript implementation
        // if vy and vx == 0 then x, y are undefined.
        let mut x = T::zero();
        let mut y = T::zero();
        if vy < T::zero() {
            // top
            if p0.y <= self.ymin {
                return None;
            }
            c = (self.ymin - p0.y) / vy;
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
            c = (self.ymax - p0.y) / vy;
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
            c = (self.xmax - p0.x) / vx;
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
            c = (self.xmin - p0.x) / vx;
            if c < t {
                x = self.xmin;
                t = c;
                y = p0.x + t * vy;
            }
        }
        return Some(Coordinate { x, y });
    }

    fn edgecode(&self, p: Coordinate<T>) -> u8 {
        // Lower and upper nibbles.
        let lower: u8;
        let upper: u8;

        if (p.x - self.xmin).abs() < T::epsilon() {
            lower = 0b0001;
        } else if (p.x - self.xmax).abs() < T::epsilon() {
            lower = 0b0010;
        } else {
            lower = 0b0000;
        }

        if (p.y - self.ymin).abs() < T::epsilon() {
            upper = 0b0100;
        } else if (p.y - self.ymax).abs() < T::epsilon() {
            upper = 0b1000;
        } else {
            upper = 0b0000;
        }

        return lower | upper;
    }

    fn regioncode(&self, p: Coordinate<T>) -> u8 {
        // Lower and upper nibbles.
        let lower: u8;
        let upper: u8;

        if p.x < self.xmin {
            lower = 0b001;
        } else if p.x > self.xmax {
            lower = 0b0010;
        } else {
            lower = 0b0000;
        }

        if p.y < self.ymin {
            upper = 0b0100;
        } else if p.y > self.ymax {
            upper = 0b1000;
        } else {
            upper = 0b0000;
        }

        return lower | upper;
    }
}
