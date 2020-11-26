#![allow(clippy::clippy::many_single_char_names)]
use delaunator::Point;
use delaunator::EMPTY;
use std::collections::VecDeque;

use rust_d3_geo::math::EPSILON;

use crate::delaunay::Delaunay;
use crate::polygon::Polygon;
use crate::RenderingContext2d;

pub struct Voronoi {
    pub circumcenters: Vec<Point>,
    delaunay: Delaunay,
    pub vectors: VecDeque<Point>,
    pub xmin: f64,
    pub ymin: f64,
    pub xmax: f64,
    pub ymax: f64,
}

impl Default for Voronoi {
    fn default() -> Voronoi {
        return Voronoi {
            delaunay: Delaunay::default(),
            circumcenters: Vec::<Point>::new(),
            vectors: VecDeque::new(),
            xmin: 0f64,
            xmax: 960f64,
            ymin: 0f64,
            ymax: 500f64,
        };
    }
}

impl Voronoi {
    pub fn new(delaunay: Delaunay, b_in: Option<(f64, f64, f64, f64)>) -> Self {
        let b;
        let mut v: Voronoi;
        match b_in {
            Some(b_in) => {
                b = b_in;
            }
            None => {
                b = (0f64, 0f64, 960f64, 500f64);
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
            circumcenters.push(Point { x: 0f64, y: 0f64 });
            vectors.push_back(Point { x: 0f64, y: 0f64 });
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
        let mut x: f64;
        let mut y: f64;

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
            let ab = (dx * ey - dy * ex) * 2f64;

            if ab == 0f64 {
                // degenerate case (collinear diagram)
                x = (x1 + x3) / 2f64 - 1e8 * ey;
                y = (y1 + y3) / 2f64 + 1e8 * ex;
            } else if ab.abs() < 1e-8 {
                // almost equal points (degenerate triangle)
                x = (x1 + x3) / 2f64;
                y = (y1 + y3) / 2f64;
            } else {
                let d = 1f64 / ab;
                x = x1 + (ey * bl - dy * cl) * d;
                y = y1 + (dx * cl - ex * bl) * d;
            }
            self.circumcenters[j] = Point { x, y };
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
        let mut x0: f64;
        let mut x1 = points[h].x;
        let mut y0: f64;
        let mut y1 = points[h].y;

        // self.vectors.fill(0);
        let vectors_len = self.vectors.len();
        self.vectors.clear();
        for _ in 0..vectors_len {
            self.vectors.push_back(Point { x: 0f64, y: 0f64 });
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
            self.vectors.insert(p0 + 1, Point { x: ydiff, y: xdiff });
            self.vectors.remove(p1);
            self.vectors.insert(p1, Point { x: ydiff, y: xdiff });
        }
    }

    // TODO implement render()

    // TODO implement render_bounds()

    pub fn render_cell<T: RenderingContext2d>(&self, i: usize, context: &mut T) -> String {
        let points = self.clip(i);
        return match points {
            None => {
                return "".to_string();
            }
            Some(points) => {
                if points.is_empty() {
                    return "".to_string();
                }

                context.move_to(points[0].clone());
                let mut n = points.len();
                while (points[0usize].x - points[n - 1].x).abs() < EPSILON
                    && (points[0].y - points[n - 1].y).abs() < EPSILON
                    && n > 1
                {
                    n -= 1;
                }

                for i in 1..n {
                    if (points[i].x - points[i - 1].x).abs() >= EPSILON
                        || (points[i].y - points[i - 1].y).abs() >= EPSILON
                    {
                        context.line_to(points[i].clone());
                    }
                }
                context.close_path();

                context.value_str()
            }
        };
    }

    // TODO implement cellPolgons*()

    pub fn cell_polygon(&self, i: usize) -> Vec<Point> {
        let mut polygon = Polygon::new();
        self.render_cell(i, &mut polygon);
        return polygon.value();
    }

    // TODO implement renderSegment()

    pub fn contains(&self, i: usize, x: f64, y: f64) -> bool {
        // if ((x = +x, x !== x) || (y = +y, y !== y)) return false;
        return self.delaunay.step(i, x, y) == i;
    }

    // TODO place neighbours* here()

    fn cell(&self, i: usize) -> Option<VecDeque<Point>> {
        let e0 = self.delaunay.inedges[i];
        if e0 == EMPTY {
            // Coincident point.
            return None;
        }
        let mut points: VecDeque<Point> = VecDeque::new();
        let mut e = e0;
        loop {
            let t = (e as f64 / 3f64).floor() as usize;
            points.push_back(self.circumcenters[t].clone());
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

    fn clip(&self, i: usize) -> Option<VecDeque<Point>> {
        // degenerate case (1 valid point: return the box)
        if i == 0 && self.delaunay.hull.len() == 1 {
            return Some(
                vec![
                    Point {
                        x: self.xmax,
                        y: self.ymin,
                    },
                    Point {
                        x: self.xmax,
                        y: self.ymax,
                    },
                    Point {
                        x: self.xmin,
                        y: self.ymax,
                    },
                    Point {
                        x: self.xmin,
                        y: self.ymin,
                    },
                ]
                .into_iter()
                .collect(),
            );
        }
        // if (points == null) return null;
        match self.cell(i) {
            None => {
                return None;
            }
            Some(points) => {
                #[allow(non_snake_case)]
                let V = &self.vectors;
                let v = i * 2;
                if V[v].x != 0f64 || V[v].y != 0f64 {
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

    fn clip_finite(&self, i: usize, points: &VecDeque<Point>) -> VecDeque<Point> {
        let n = points.len();
        #[allow(non_snake_case)]
        let mut P = VecDeque::new();
        let mut x0;
        let mut y0;
        let mut x1 = points[n - 1].x;
        let mut y1 = points[n - 1].y;
        let mut c0;
        let mut c1 = self.regioncode(x1, y1);
        let mut e0;
        // There is a bug/inconsitencey in the javascript implementation.
        // e1 must be given a reasonable default value.
        let mut e1 = 0;
        // for (let j = 0; j < n; j += 2) {
        for point in points {
            x0 = x1;
            y0 = y1;
            x1 = point.x;
            y1 = point.y;
            c0 = c1;
            c1 = self.regioncode(x1, y1);
            if c0 == 0 && c1 == 0 {
                // e0 = e1;
                e1 = 0;
                if !P.is_empty() {
                    P.push_back(Point { x: x1, y: y1 });
                } else {
                    P = vec![Point { x: x1, y: y1 }].into_iter().collect();
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
                            sx1 = s[1].x;
                            sy1 = s[1].y;
                        }
                    }
                } else {
                    S = self.clip_segment(x1, y1, x0, y0, c1, c0);
                    match S {
                        None => {
                            continue;
                        }
                        Some(s) => {
                            sx1 = s[0].x;
                            sy1 = s[0].y;
                            sx0 = s[1].x;
                            sy0 = s[1].y;
                            e0 = e1;
                            e1 = self.edgecode(sx0, sy0);
                            if e0 != 0u8 && e1 != 0u8 {
                                let len = P.len();
                                self.edge(i, e0, e1, &mut P, len);
                            }
                            if !P.is_empty() {
                                P.push_back(Point { x: sx0, y: sy0 });
                            } else {
                                P = vec![Point { x: sx0, y: sy0 }].into_iter().collect();
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
                    P.push_back(Point { x: sx1, y: sy1 });
                } else {
                    P = vec![Point { x: sx1, y: sy1 }].into_iter().collect();
                };
            }
        }
        if !P.is_empty() {
            e0 = e1;
            e1 = self.edgecode(P[0].x, P[0].y);
            if e0 != 0u8 && e1 != 0u8 {
                let len = P.len();
                self.edge(i, e0, e1, &mut P, len);
            }
        } else if self.contains(
            i,
            (self.xmin + self.xmax) / 2f64,
            (self.ymin + self.ymax) / 2f64,
        ) {
            return vec![
                Point {
                    x: self.xmax,
                    y: self.ymin,
                },
                Point {
                    x: self.xmax,
                    y: self.ymax,
                },
                Point {
                    x: self.xmin,
                    y: self.ymax,
                },
                Point {
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
        x0_in: f64,
        y0_in: f64,
        x1_in: f64,
        y1_in: f64,
        c0_in: u8,
        c1_in: u8,
    ) -> Option<Vec<Point>> {
        let mut x0 = x0_in;
        let mut x1 = x1_in;
        let mut y0 = y0_in;
        let mut y1 = y1_in;
        let mut c0 = c0_in;
        let mut c1 = c1_in;
        loop {
            if c0 == 0 && c1 == 0 {
                return Some(vec![Point { x: x0, y: y0 }, Point { x: x1, y: y1 }]);
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
        points: &VecDeque<Point>,
        vx0: f64,
        vy0: f64,
        vxn: f64,
        vyn: f64,
    ) -> VecDeque<Point> {
        #[allow(non_snake_case)]
        let mut P: VecDeque<Point> = VecDeque::into(points.clone());
        if let Some(p1) = self.project(P[0].x, P[0].y, vx0, vy0) {
            P.push_front(p1);
        }

        if let Some(p2) = self.project(P[P.len() - 1].x, P[P.len() - 1].y, vxn, vyn) {
            P.push_back(p2);
        }

        P = self.clip_finite(i, &P);
        if !P.is_empty() {
            let mut n = P.len();
            let mut c0;
            let mut c1 = self.edgecode(P[n - 1].x, P[n - 1].y);
            let mut j = 0;
            loop {
                c0 = c1;
                c1 = self.edgecode(P[j].x, P[j].y);
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
            (self.xmin + self.xmax) / 2f64,
            (self.ymin + self.ymax) / 2f64,
        ) {
            P = vec![
                Point {
                    x: self.xmin,
                    y: self.ymin,
                },
                Point {
                    x: self.xmax,
                    y: self.ymin,
                },
                Point {
                    x: self.xmax,
                    y: self.ymax,
                },
                Point {
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
    fn edge(&self, i: usize, e0_in: u8, e1: u8, P: &mut VecDeque<Point>, j_in: usize) -> usize {
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

            if ((P[j].x - x).abs() >= EPSILON || (P[j].y - y).abs() >= EPSILON)
                && self.contains(i, x, y)
            {
                P.insert(j, Point { x, y });
                j += 1;
            }
        }
        if P.len() > 2 {
            let mut i = 0;
            loop {
                let j = (i + 1) % P.len();
                let k = (i + 2) % P.len();
                if (P[i].x - P[j].x).abs() < EPSILON && (P[j].x - P[k].x).abs() < EPSILON
                    || (P[i].y - P[j].y).abs() < EPSILON && (P[j].y - P[k].y).abs() < EPSILON
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

    fn project(&self, x0: f64, y0: f64, vx: f64, vy: f64) -> Option<Point> {
        let mut t = f64::INFINITY;
        let mut c;
        // The is a mistake in the javascript implementation
        // if vy and vx == 0 then x, y are undefined.
        let mut x = 0f64;
        let mut y = 0f64;
        if vy < 0f64 {
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
        } else if vy > 0f64 {
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

        if vx > 0f64 {
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
        } else if vx < 0f64 {
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
        return Some(Point { x, y });
    }

    fn edgecode(&self, x: f64, y: f64) -> u8 {
        // Lower and upper nibbles.
        let lower: u8;
        let upper: u8;

        if (x - self.xmin).abs() < EPSILON {
            lower = 0b0001;
        } else if (x - self.xmax).abs() < EPSILON {
            lower = 0b0010;
        } else {
            lower = 0b0000;
        }

        if (y - self.ymin).abs() < EPSILON {
            upper = 0b0100;
        } else if (y - self.ymax).abs() < EPSILON {
            upper = 0b1000;
        } else {
            upper = 0b0000;
        }

        return lower | upper;
    }

    fn regioncode(&self, x: f64, y: f64) -> u8 {
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
