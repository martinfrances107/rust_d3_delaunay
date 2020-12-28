#![allow(clippy::clippy::many_single_char_names)]

use delaunator::EMPTY;
use geo::CoordinateType;
use geo::Point;
use num_traits::{float::Float, AsPrimitive, FromPrimitive};
use std::collections::VecDeque;

use crate::delaunay::Delaunay;
use crate::polygon::Polygon;
use crate::RenderingContext2d;

pub struct Voronoi<T>
where
    T: CoordinateType + AsPrimitive<T> + Float,
{
    pub circumcenters: Vec<Point<T>>,
    delaunay: Delaunay<T>,
    pub vectors: VecDeque<Point<T>>,
    pub xmin: T,
    pub ymin: T,
    pub xmax: T,
    pub ymax: T,
}

impl<T> Default for Voronoi<T>
where
    T: CoordinateType + Float + FromPrimitive + AsPrimitive<T>,
{
    fn default() -> Voronoi<T> {
        return Voronoi {
            delaunay: Delaunay::default(),
            circumcenters: Vec::<Point<T>>::new(),
            vectors: VecDeque::new(),
            xmin: T::from_f64(0f64).unwrap(),
            xmax: T::from_f64(960.0f64).unwrap(),
            ymin: T::from_f64(0.0f64).unwrap(),
            ymax: T::from_f64(500f64).unwrap(),
        };
    }
}

impl<T> Voronoi<T>
where
    T: CoordinateType + Float + FromPrimitive + AsPrimitive<T>,
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
            circumcenters.push(Point::new(T::zero(), T::zero()));
            vectors.push_back(Point::new(T::zero(), T::zero()));
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
            self.circumcenters[j] = Point::new(x, y);
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
            self.vectors.push_back(Point::new(T::zero(), T::zero()));
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
            self.vectors.insert(p0 + 1, Point::new(ydiff, xdiff));
            self.vectors.remove(p1);
            self.vectors.insert(p1, Point::new(ydiff, xdiff));
        }
    }

    // TODO implement render()

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
                while (points[0usize].x() - points[n - 1].x()).abs() < T::epsilon()
                    && (points[0].y() - points[n - 1].y()).abs() < T::epsilon()
                    && n > 1
                {
                    n -= 1;
                }

                for i in 1..n {
                    if (points[i].x() - points[i - 1].x()).abs() >= T::epsilon()
                        || (points[i].y() - points[i - 1].y()).abs() >= T::epsilon()
                    {
                        context.line_to(&points[i].clone());
                    }
                }
                context.close_path();

                context.value_str()
            }
        };
    }

    // TODO implement cellPolgons*()

    pub fn cell_polygon(&self, i: usize) -> Vec<Point<T>> {
        let mut polygon = Polygon::new();
        self.render_cell(i, &mut polygon);
        return polygon.value();
    }

    // TODO implement renderSegment()

    pub fn contains(&self, i: usize, x: T, y: T) -> bool {
        return self.delaunay.step(i, x, y) == i;
    }

    // TODO place neighbours* here()

    fn cell(&self, i: usize) -> Option<VecDeque<Point<T>>> {
        let e0 = self.delaunay.inedges[i];
        if e0 == EMPTY {
            // Coincident point.
            return None;
        }
        let mut points: VecDeque<Point<T>> = VecDeque::new();
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

    fn clip(&self, i: usize) -> Option<VecDeque<Point<T>>> {
        // degenerate case (1 valid point: return the box)
        if i == 0 && self.delaunay.hull.len() == 1 {
            return Some(
                vec![
                    Point::new(self.xmax, self.ymin),
                    Point::new(self.xmax, self.ymax),
                    Point::new(self.xmin, self.ymax),
                    Point::new(self.xmin, self.ymin),
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
                if V[v].x() != T::zero() || V[v].y() != T::zero() {
                    return Some(self.clip_infinite(
                        i,
                        &points,
                        V[v].x(),
                        V[v].y(),
                        V[v + 1].x(),
                        V[v + 1].y(),
                    ));
                } else {
                    return Some(self.clip_finite(i, &points));
                }
            }
        }
    }

    fn clip_finite(&self, i: usize, points: &VecDeque<Point<T>>) -> VecDeque<Point<T>> {
        let n = points.len();
        #[allow(non_snake_case)]
        let mut P = VecDeque::new();
        let mut x0;
        let mut y0;
        let mut x1 = points[n - 1].x();
        let mut y1 = points[n - 1].y();
        let mut c0;
        let mut c1 = self.regioncode(x1, y1);
        let mut e0;
        // There is a bug/inconsitencey in the javascript implementation.
        // e1 must be given a reasonable default value.
        let mut e1 = 0;
        let t2 = T::from_f64(2f64).unwrap();
        for point in points {
            x0 = x1;
            y0 = y1;
            x1 = point.x();
            y1 = point.y();
            c0 = c1;
            c1 = self.regioncode(x1, y1);
            if c0 == 0 && c1 == 0 {
                // e0 = e1;
                e1 = 0;
                if !P.is_empty() {
                    P.push_back(Point::new(x1, y1));
                } else {
                    P = vec![Point::new(x1, y1)].into_iter().collect();
                }
            } else {
                #[allow(non_snake_case)]
                let S;
                let sx0;
                let sy0;
                let sx1;
                let sy1;
                if c0 == 0 {
                    S = self.clip_segment(x0, y0, x1, y1, c0, c1);
                    match S {
                        None => {
                            continue;
                        }
                        Some(s) => {
                            // sx0 = s[0].x;
                            // sy0 = s[0].y;
                            sx1 = s[1].x();
                            sy1 = s[1].y();
                        }
                    }
                } else {
                    S = self.clip_segment(x1, y1, x0, y0, c1, c0);
                    match S {
                        None => {
                            continue;
                        }
                        Some(s) => {
                            sx1 = s[0].x();
                            sy1 = s[0].y();
                            sx0 = s[1].x();
                            sy0 = s[1].y();
                            e0 = e1;
                            e1 = self.edgecode(sx0, sy0);
                            if e0 != 0u8 && e1 != 0u8 {
                                let len = P.len();
                                self.edge(i, e0, e1, &mut P, len);
                            }
                            if !P.is_empty() {
                                P.push_back(Point::new(sx0, sy0));
                            } else {
                                P = vec![Point::new(sx0, sy0)].into_iter().collect();
                            }
                        }
                    }
                }
                e0 = e1;
                e1 = self.edgecode(sx1, sy1);
                if e0 != 0u8 && e1 != 0u8 {
                    let len = P.len();
                    self.edge(i, e0, e1, &mut P, len);
                }
                if !P.is_empty() {
                    P.push_back(Point::new(sx1, sy1));
                } else {
                    P = vec![Point::new(sx1, sy1)].into_iter().collect();
                };
            }
        }
        if !P.is_empty() {
            e0 = e1;
            e1 = self.edgecode(P[0].x(), P[0].y());
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
                Point::new(self.xmax, self.ymin),
                Point::new(self.xmax, self.ymax),
                Point::new(self.xmin, self.ymax),
                Point::new(self.xmin, self.ymin),
            ]
            .into_iter()
            .collect();
        }
        return P;
    }

    fn clip_segment(
        &self,
        x0_in: T,
        y0_in: T,
        x1_in: T,
        y1_in: T,
        c0_in: u8,
        c1_in: u8,
    ) -> Option<Vec<Point<T>>> {
        let mut x0 = x0_in;
        let mut x1 = x1_in;
        let mut y0 = y0_in;
        let mut y1 = y1_in;
        let mut c0 = c0_in;
        let mut c1 = c1_in;
        loop {
            if c0 == 0 && c1 == 0 {
                return Some(vec![Point::new(x0, y0), Point::new(x1, y1)]);
            }
            if c0 & c1 != 0 {
                return None;
            }
            let x;
            let y;
            let c = if c0 != 0 { c0 } else { c1 };

            if c & 0b1000 != 0 {
                x = x0 + (x1 - x0) * (self.ymax - y0) / (y1 - y0);
                y = self.ymax;
            } else if c & 0b0100 != 0 {
                x = x0 + (x1 - x0) * (self.ymin - y0) / (y1 - y0);
                y = self.ymin;
            } else if c & 0b0010 != 0 {
                y = y0 + (y1 - y0) * (self.xmax - x0) / (x1 - x0);
                x = self.xmax;
            } else {
                y = y0 + (y1 - y0) * (self.xmin - x0) / (x1 - x0);
                x = self.xmin;
            }
            if c0 != 0 {
                x0 = x;
                y0 = y;
                c0 = self.regioncode(x0, y0);
            } else {
                x1 = x;
                y1 = y;
                c1 = self.regioncode(x1, y1);
            }
        }
    }

    fn clip_infinite(
        &self,
        i: usize,
        points: &VecDeque<Point<T>>,
        vx0: T,
        vy0: T,
        vxn: T,
        vyn: T,
    ) -> VecDeque<Point<T>> {
        #[allow(non_snake_case)]
        let mut P: VecDeque<Point<T>> = VecDeque::into(points.clone());
        if let Some(p1) = self.project(P[0].x(), P[0].y(), vx0, vy0) {
            P.push_front(p1);
        }

        if let Some(p2) = self.project(P[P.len() - 1].x(), P[P.len() - 1].y(), vxn, vyn) {
            P.push_back(p2);
        }

        P = self.clip_finite(i, &P);
        let t2 = T::from_f64(2f64).unwrap();
        if !P.is_empty() {
            let mut n = P.len();
            let mut c0;
            let mut c1 = self.edgecode(P[n - 1].x(), P[n - 1].y());
            let mut j = 0;
            loop {
                c0 = c1;
                c1 = self.edgecode(P[j].x(), P[j].y());
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
                Point::new(self.xmin, self.ymin),
                Point::new(self.xmax, self.ymin),
                Point::new(self.xmax, self.ymax),
                Point::new(self.xmin, self.ymax),
            ]
            .into_iter()
            .collect();
        }
        return P;
    }

    #[allow(non_snake_case)]
    fn edge(&self, i: usize, e0_in: u8, e1: u8, P: &mut VecDeque<Point<T>>, j_in: usize) -> usize {
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

            if ((P[j].x() - x).abs() >= T::epsilon() || (P[j].y() - y).abs() >= T::epsilon())
                && self.contains(i, x, y)
            {
                P.insert(j, Point::new(x, y));
                j += 1;
            }
        }
        if P.len() > 2 {
            let mut i = 0;
            loop {
                let j = (i + 1) % P.len();
                let k = (i + 2) % P.len();
                if (P[i].x() - P[j].x()).abs() < T::epsilon()
                    && (P[j].x() - P[k].x()).abs() < T::epsilon()
                    || (P[i].y() - P[j].y()).abs() < T::epsilon()
                        && (P[j].y() - P[k].y()).abs() < T::epsilon()
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

    fn project(&self, x0: T, y0: T, vx: T, vy: T) -> Option<Point<T>> {
        let mut t = Float::infinity();
        let mut c;
        // The is a mistake in the javascript implementation
        // if vy and vx == 0 then x, y are undefined.
        let mut x = T::zero();
        let mut y = T::zero();
        if vy < T::zero() {
            // top
            if y0 <= self.ymin {
                return None;
            }
            c = (self.ymin - y0) / vy;
            if c < t {
                y = self.ymin;
                t = c;
                x = x0 + t * vx;
            }
        } else if vy > T::zero() {
            // bottom
            if y0 >= self.ymax {
                return None;
            }
            c = (self.ymax - y0) / vy;
            if c < t {
                y = self.ymax;
                t = c;
                x = x0 + t * vx;
            }
        }

        if vx > T::zero() {
            // right
            if x0 >= self.xmax {
                return None;
            }
            c = (self.xmax - x0) / vx;
            if c < t {
                x = self.xmax;
                t = c;
                y = y0 + t * vy;
            }
        } else if vx < T::zero() {
            // left
            if x0 <= self.xmin {
                return None;
            }
            c = (self.xmin - x0) / vx;
            if c < t {
                x = self.xmin;
                t = c;
                y = y0 + t * vy;
            }
        }
        return Some(Point::new(x, y));
    }

    fn edgecode(&self, x: T, y: T) -> u8 {
        // Lower and upper nibbles.
        let lower: u8;
        let upper: u8;

        if (x - self.xmin).abs() < T::epsilon() {
            lower = 0b0001;
        } else if (x - self.xmax).abs() < T::epsilon() {
            lower = 0b0010;
        } else {
            lower = 0b0000;
        }

        if (y - self.ymin).abs() < T::epsilon() {
            upper = 0b0100;
        } else if (y - self.ymax).abs() < T::epsilon() {
            upper = 0b1000;
        } else {
            upper = 0b0000;
        }

        return lower | upper;
    }

    fn regioncode(&self, x: T, y: T) -> u8 {
        // Lower and upper nibbles.
        let lower: u8;
        let upper: u8;

        if x < self.xmin {
            lower = 0b001;
        } else if x > self.xmax {
            lower = 0b0010;
        } else {
            lower = 0b0000;
        }

        if y < self.ymin {
            upper = 0b0100;
        } else if y > self.ymax {
            upper = 0b1000;
        } else {
            upper = 0b0000;
        }

        return lower | upper;
    }
}
