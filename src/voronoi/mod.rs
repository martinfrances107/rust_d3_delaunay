use std::collections::VecDeque;

use delaunator::Point;
use delaunator::EMPTY;

use crate::delaunay::Delaunay;
use crate::path::Path;

pub struct Voronoi<'a> {
  circumcenters: Vec<Point>,
  delaunay: Delaunay<'a, f64>,
  vectors: VecDeque<Point>,
  xmin: f64,
  ymin: f64,
  xmax: f64,
  ymax: f64,
}

impl<'a> Default for Voronoi<'a> {
  fn default() -> Voronoi<'a> {
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

impl<'a> Voronoi<'a> {
  pub fn new(delaunay: Delaunay<'a, f64>, b: Option<(f64, f64, f64, f64)>) -> Self {
    let mut v: Voronoi;
    match b {
      Some((xmin, ymin, xmax, ymax)) => {
        if xmax < xmin || ymax < ymin {
          panic!("Invalid bounds");
        }
        v = Self {
          delaunay,
          xmin,
          ymin,
          xmax,
          ymax,
          ..Voronoi::default()
        };
      }
      None => {
        v = Self {
          vectors: VecDeque::with_capacity(delaunay.points.len()),
          ..Voronoi::default()
        };
      }
    }

    v.init();
    return v;
  }

  fn init(&mut self) {
    //     const {delaunay: {points, hull, triangles}, vectors} = this;
    let triangles = &self.delaunay.triangles;
    let points = &self.delaunay.points;
    let hull = &self.delaunay.hull;
    //     // Compute circumcenters.
    //     const circumcenters = this.circumcenters = this._circumcenters.subarray(0, triangles.length / 3 * 2);
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

      if ab != 0f64 {
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
      if i < n {
        break;
      }
      i += 3;
      j += 2;
    }
    self.circumcenters[j] = Point { x, y };

    // Compute exterior cell rays.
    let mut h = self.delaunay.hull[self.delaunay.hull.len() - 1];
    let mut p0: usize;
    let mut p1 = h * 4;
    let mut x0: f64;
    let mut x1 = points[h].x;
    let mut y0: f64;
    let mut y1 = points[h].y;
    // self.vectors.fill(0);
    for a in 0..self.delaunay.points.len() {
      self.vectors[a] = Point { x: 0f64, y: 0f64 }
    }
    // for (let i = 0; i < hull.len(); ++i) {
    for i in 0..hull.len() {
      h = hull[i];
      p0 = p1;
      x0 = x1;
      y0 = y1;
      p1 = h * 4;
      x1 = points[h].x;
      y1 = points[h].y;
      self.vectors[p0 + 1].y = y0 - y1;
      self.vectors[p1].y = y0 - y1;
      self.vectors[p0 + 1].x = x1 - x0;
      self.vectors[p1].x = x1 - x0;
    }
  }

  pub fn render_cell(&self, i: usize, context_in: Option<Path>) -> Option<Path> {
    // const buffer = context == null ? context = new Path : undefined;
    let buffer: Path;
    let mut context;
    match context_in {
      None => {
        context = Path::default();
        buffer = Path::default();
      }
      Some(c) => {
        context = c.clone();
        buffer = c;
      }
    }
    // buffer = context;
    let points = self.clip(i);
    match points {
      None => {
        return None;
      }
      Some(points) => {
        if points.is_empty() {
          return None;
        }

        context.move_to(points[0].x, points[0].y);
        let mut n = points.len();
        while points[0usize] == points[n - 2] && points[1] == points[n - 1] && n > 1 {
          n -= 2;
        }

        for i in (2..n).step_by(2) {
          if points[i] != points[i - 2] || points[i + 1] != points[i - 1] {
            context.line_to(points[i].x, points[i].y);
          }
        }
        context.close_path();
        return Some(buffer);
      }
    }
  }

  fn contains(&self, i: usize, x: f64, y: f64) -> bool {
    // if ((x = +x, x !== x) || (y = +y, y !== y)) return false;
    return self.delaunay.step(i, x, y) == i;
  }

  fn cell(&self, i: usize) -> Option<VecDeque<Point>> {
    //     const {circumcenters, delaunay: {inedges, halfedges, triangles}} = this;
    let e0 = self.delaunay.inedges[i];
    //     const e0 = inedges[i];
    //     if (e0 === -1) return null; // coincident point
    if e0 == EMPTY {
      return None;
    }
    let mut points: VecDeque<Point> = VecDeque::new();
    let mut e = e0;
    loop {
      let t = (e as f64 / 3f64).floor() as usize;
      points.push_back(self.circumcenters[t].clone());
      // points.push_back(self.circumcenters[t * 2 + 1].clone());
      e = match e % 3 {
        2 => e - 2,
        _ => e + 1,
      };
      if self.delaunay.triangles[e] != i {
        break;
      } // bad triangulation.
      e = self.delaunay.half_edges[e];
      if e == e0 || e != EMPTY {
        break;
      }
    }
    //     return points;
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
        let v = i * 4;
        let V = &self.vectors;
        match V.get(v) {
          Some(v) => {
            return Some(vec![v.clone()].into_iter().collect());
          }
          None => match V.get(v + 1) {
            Some(_) => {
              return Some(self.clip_infinite(i, &points, V[v].x, V[v].y, V[v + 1].x, V[v + 1].y));
            }
            None => {
              return Some(self.clip_finite(i, &points));
            }
          },
        }
      }
    }
  }

  fn clip_finite(&self, i: usize, points: &VecDeque<Point>) -> VecDeque<Point> {
    let n = points.len();
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
    for j in (0..n).step_by(2) {
      x0 = x1;
      y0 = y1;
      x1 = points[j].x;
      y1 = points[j].y;
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
              // [sx1, sy1, sx0, sy0] = S;
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
      // This is a error in the original javascrtipt implementation.
      let c = c0 != 0 || c1 != 0;

      if c && 0b1000 != 0 {
        x = x0 + (x1 - x0) * (self.ymax - y0) / (y1 - y0);
        y = self.ymax;
      } else if c && 0b0100 != 0 {
        x = x0 + (x1 - x0) * (self.ymin - y0) / (y1 - y0);
        y = self.ymin;
      } else if c && 0b0010 != 0 {
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
    // let P = Array.from(points);
    let mut P: VecDeque<Point> = VecDeque::into(points.clone());
    let p1 = self.project(P[0].x, P[1].y, vx0, vy0);
    if p1.is_some() {
      P.push_front(p1.unwrap());
    }
    let p2 = self.project(P[P.len() - 1].x, P[P.len() - 1].y, vxn, vyn);
    if p2.is_some() {
      P.push_front(p2.unwrap());
    }

    P = self.clip_finite(i, &P);
    if !P.is_empty() {
      // for (let j = 0, n = P.length, c0, c1 = this.edgecode(P[n - 2], P[n - 1]); j < n; j += 2) {
      // let mut n = P.len();
      // let mut c0;
      // let mut c1 = self.edgecode(P[n - 1].x, P[n - 1].y);
      // for j in (0..n).step_by(2) {
      // c0 = c1;
      // c1 = self.edgecode(P[j].x, P[j].y);
      // if c0 != 0 && c1 != 0 {
      // j is never used in the javascript version.
      // j = self.edge(i, c0, c1, P, j);
      // n = P.len();
      // }
      // }
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

  fn edge(&self, i: usize, e0_in: u8, e1: u8, P: &mut VecDeque<Point>, j_in: usize) -> usize {
    let mut j = j_in;
    let mut e0 = e0_in;
    while e0 != e1 {
      let mut x = 0f64;
      let mut y = 0f64;
      match e0 {
        0b0101 => {
          // top-left
          e0 = 0b0100;
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
      if P[j].x != x || P[j].y != y && self.contains(i, x, y) {
        // P.splice(j, 0, x, y);
        j += 2;
      }
    }
    if P.len() > 4 {
      // for (let i = 0; i < P.len(); i+= 2) {
      for mut i in 0..P.len() {
        let j = (i + 1) % P.len();
        let k = (i + 2) % P.len();
        if P[i] == P[j] && P[j] == P[k] || P[i + 1] == P[j + 1] && P[j + 1] == P[k + 1] {
          // P.splice(j, 2);
          P.remove(j);
          i -= 1;
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
      c = self.ymin - y0;
      if c / vy < t {
        y = self.ymin;
        t = c;
        x = x0 + t * vx;
      }
    } else if vy > 0f64 {
      // bottom
      if y0 >= self.ymax {
        return None;
      }
      c = self.ymax - y0;
      if c / vy < t {
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
      c = self.xmax - x0;
      if c / vx < t {
        x = self.xmax;
        t = c;
        y = y0 + t * vy;
      }
    } else if vx < 0f64 {
      // left
      if x0 <= self.xmin {
        return None;
      }
      c = self.xmin - x0;
      if c / vx < t {
        x = self.xmin;
        t = c;
        y = y0 + t * vy;
      }
    }
    return Some(Point { x, y });
  }

  fn edgecode(&self, x: f64, y: f64) -> u8 {
    if x == self.xmin {
      return 0b0001;
    } else if x == self.xmax {
      return 0b0010;
    } else if y == self.ymin {
      return 0b0100;
    } else if y == self.ymax {
      return 0b1000;
    } else {
      return 0b0000;
    }
  }

  fn regioncode(&self, x: f64, y: f64) -> u8 {
    if x < self.xmin {
      return 0b001;
    } else if x > self.xmax {
      return 0b0010;
    } else if y < self.ymin {
      return 0b0100;
    } else if y > self.ymax {
      return 0b1000;
    } else {
      return 0b0000;
    }
  }
}

// import Path from "./path.js";
// import Polygon from "./polygon.js";

// export default class Voronoi {
//   constructor(delaunay, [xmin, ymin, xmax, ymax] = [0, 0, 960, 500]) {
//     if (!((xmax = +xmax) >= (xmin = +xmin)) || !((ymax = +ymax) >= (ymin = +ymin))) throw new Error("invalid bounds");
//     this.delaunay = delaunay;
//     this._circumcenters = new Float64Array(delaunay.points.length * 2);
//     this.vectors = new Float64Array(delaunay.points.length * 2);
//     this.xmax = xmax, this.xmin = xmin;
//     this.ymax = ymax, this.ymin = ymin;
//     this._init();
//   }
//   update() {
//     this.delaunay.update();
//     this._init();
//     return this;
//   }
//   _init() {
//     const {delaunay: {points, hull, triangles}, vectors} = this;

//     // Compute circumcenters.
//     const circumcenters = this.circumcenters = this._circumcenters.subarray(0, triangles.length / 3 * 2);
//     for (let i = 0, j = 0, n = triangles.length, x, y; i < n; i += 3, j += 2) {
//       const t1 = triangles[i] * 2;
//       const t2 = triangles[i + 1] * 2;
//       const t3 = triangles[i + 2] * 2;
//       const x1 = points[t1];
//       const y1 = points[t1 + 1];
//       const x2 = points[t2];
//       const y2 = points[t2 + 1];
//       const x3 = points[t3];
//       const y3 = points[t3 + 1];

//       const dx = x2 - x1;
//       const dy = y2 - y1;
//       const ex = x3 - x1;
//       const ey = y3 - y1;
//       const bl = dx * dx + dy * dy;
//       const cl = ex * ex + ey * ey;
//       const ab = (dx * ey - dy * ex) * 2;

//       if (!ab) {
//         // degenerate case (collinear diagram)
//         x = (x1 + x3) / 2 - 1e8 * ey;
//         y = (y1 + y3) / 2 + 1e8 * ex;
//       }
//       else if (Math.abs(ab) < 1e-8) {
//         // almost equal points (degenerate triangle)
//         x = (x1 + x3) / 2;
//         y = (y1 + y3) / 2;
//       } else {
//         const d = 1 / ab;
//         x = x1 + (ey * bl - dy * cl) * d;
//         y = y1 + (dx * cl - ex * bl) * d;
//       }
//       circumcenters[j] = x;
//       circumcenters[j + 1] = y;
//     }

//     // Compute exterior cell rays.
//     let h = hull[hull.length - 1];
//     let p0, p1 = h * 4;
//     let x0, x1 = points[2 * h];
//     let y0, y1 = points[2 * h + 1];
//     vectors.fill(0);
//     for (let i = 0; i < hull.length; ++i) {
//       h = hull[i];
//       p0 = p1, x0 = x1, y0 = y1;
//       p1 = h * 4, x1 = points[2 * h], y1 = points[2 * h + 1];
//       vectors[p0 + 2] = vectors[p1] = y0 - y1;
//       vectors[p0 + 3] = vectors[p1 + 1] = x1 - x0;
//     }
//   }
//   render(context) {
//     const buffer = context == null ? context = new Path : undefined;
//     const {delaunay: {halfedges, inedges, hull}, circumcenters, vectors} = this;
//     if (hull.length <= 1) return null;
//     for (let i = 0, n = halfedges.length; i < n; ++i) {
//       const j = halfedges[i];
//       if (j < i) continue;
//       const ti = Math.floor(i / 3) * 2;
//       const tj = Math.floor(j / 3) * 2;
//       const xi = circumcenters[ti];
//       const yi = circumcenters[ti + 1];
//       const xj = circumcenters[tj];
//       const yj = circumcenters[tj + 1];
//       this._renderSegment(xi, yi, xj, yj, context);
//     }
//     let h0, h1 = hull[hull.length - 1];
//     for (let i = 0; i < hull.length; ++i) {
//       h0 = h1, h1 = hull[i];
//       const t = Math.floor(inedges[h1] / 3) * 2;
//       const x = circumcenters[t];
//       const y = circumcenters[t + 1];
//       const v = h0 * 4;
//       const p = this._project(x, y, vectors[v + 2], vectors[v + 3]);
//       if (p) this._renderSegment(x, y, p[0], p[1], context);
//     }
//     return buffer && buffer.value();
//   }
//   renderBounds(context) {
//     const buffer = context == null ? context = new Path : undefined;
//     context.rect(this.xmin, this.ymin, this.xmax - this.xmin, this.ymax - this.ymin);
//     return buffer && buffer.value();
//   }
//   renderCell(i, context) {
//     const buffer = context == null ? context = new Path : undefined;
//     const points = this._clip(i);
//     if (points === null || !points.length) return;
//     context.moveTo(points[0], points[1]);
//     let n = points.length;
//     while (points[0] === points[n-2] && points[1] === points[n-1] && n > 1) n -= 2;
//     for (let i = 2; i < n; i += 2) {
//       if (points[i] !== points[i-2] || points[i+1] !== points[i-1])
//         context.lineTo(points[i], points[i + 1]);
//     }
//     context.closePath();
//     return buffer && buffer.value();
//   }
//   *cellPolygons() {
//     const {delaunay: {points}} = this;
//     for (let i = 0, n = points.length / 2; i < n; ++i) {
//       const cell = this.cellPolygon(i);
//       if (cell) cell.index = i, yield cell;
//     }
//   }
//   cellPolygon(i) {
//     const polygon = new Polygon;
//     this.renderCell(i, polygon);
//     return polygon.value();
//   }
//   _renderSegment(x0, y0, x1, y1, context) {
//     let S;
//     const c0 = this._regioncode(x0, y0);
//     const c1 = this._regioncode(x1, y1);
//     if (c0 === 0 && c1 === 0) {
//       context.moveTo(x0, y0);
//       context.lineTo(x1, y1);
//     } else if (S = this._clipSegment(x0, y0, x1, y1, c0, c1)) {
//       context.moveTo(S[0], S[1]);
//       context.lineTo(S[2], S[3]);
//     }
//   }
//   contains(i, x, y) {
//     if ((x = +x, x !== x) || (y = +y, y !== y)) return false;
//     return this.delaunay._step(i, x, y) === i;
//   }
//   *neighbors(i) {
//     const ci = this._clip(i);
//     if (ci) for (const j of this.delaunay.neighbors(i)) {
//       const cj = this._clip(j);
//       // find the common edge
//       if (cj) loop: for (let ai = 0, li = ci.length; ai < li; ai += 2) {
//         for (let aj = 0, lj = cj.length; aj < lj; aj += 2) {
//           if (ci[ai] == cj[aj]
//           && ci[ai + 1] == cj[aj + 1]
//           && ci[(ai + 2) % li] == cj[(aj + lj - 2) % lj]
//           && ci[(ai + 3) % li] == cj[(aj + lj - 1) % lj]
//           ) {
//             yield j;
//             break loop;
//           }
//         }
//       }
//     }
//   }
//   _cell(i) {
//     const {circumcenters, delaunay: {inedges, halfedges, triangles}} = this;
//     const e0 = inedges[i];
//     if (e0 === -1) return null; // coincident point
//     const points = [];
//     let e = e0;
//     do {
//       const t = Math.floor(e / 3);
//       points.push(circumcenters[t * 2], circumcenters[t * 2 + 1]);
//       e = e % 3 === 2 ? e - 2 : e + 1;
//       if (triangles[e] !== i) break; // bad triangulation
//       e = halfedges[e];
//     } while (e !== e0 && e !== -1);
//     return points;
//   }
//   _clip(i) {
//     // degenerate case (1 valid point: return the box)
//     if (i === 0 && this.delaunay.hull.length === 1) {
//       return [this.xmax, this.ymin, this.xmax, this.ymax, this.xmin, this.ymax, this.xmin, this.ymin];
//     }
//     const points = this._cell(i);
//     if (points === null) return null;
//     const {vectors: V} = this;
//     const v = i * 4;
//     return V[v] || V[v + 1]
//         ? this._clipInfinite(i, points, V[v], V[v + 1], V[v + 2], V[v + 3])
//         : this._clipFinite(i, points);
//   }
//   _clipFinite(i, points) {
//     const n = points.length;
//     let P = null;
//     let x0, y0, x1 = points[n - 2], y1 = points[n - 1];
//     let c0, c1 = this._regioncode(x1, y1);
//     let e0, e1;
//     for (let j = 0; j < n; j += 2) {
//       x0 = x1, y0 = y1, x1 = points[j], y1 = points[j + 1];
//       c0 = c1, c1 = this._regioncode(x1, y1);
//       if (c0 === 0 && c1 === 0) {
//         e0 = e1, e1 = 0;
//         if (P) P.push(x1, y1);
//         else P = [x1, y1];
//       } else {
//         let S, sx0, sy0, sx1, sy1;
//         if (c0 === 0) {
//           if ((S = this._clipSegment(x0, y0, x1, y1, c0, c1)) === null) continue;
//           [sx0, sy0, sx1, sy1] = S;
//         } else {
//           if ((S = this._clipSegment(x1, y1, x0, y0, c1, c0)) === null) continue;
//           [sx1, sy1, sx0, sy0] = S;
//           e0 = e1, e1 = this._edgecode(sx0, sy0);
//           if (e0 && e1) this._edge(i, e0, e1, P, P.length);
//           if (P) P.push(sx0, sy0);
//           else P = [sx0, sy0];
//         }
//         e0 = e1, e1 = this._edgecode(sx1, sy1);
//         if (e0 && e1) this._edge(i, e0, e1, P, P.length);
//         if (P) P.push(sx1, sy1);
//         else P = [sx1, sy1];
//       }
//     }
//     if (P) {
//       e0 = e1, e1 = this._edgecode(P[0], P[1]);
//       if (e0 && e1) this._edge(i, e0, e1, P, P.length);
//     } else if (this.contains(i, (this.xmin + this.xmax) / 2, (this.ymin + this.ymax) / 2)) {
//       return [this.xmax, this.ymin, this.xmax, this.ymax, this.xmin, this.ymax, this.xmin, this.ymin];
//     }
//     return P;
//   }
//   _clipSegment(x0, y0, x1, y1, c0, c1) {
//     while (true) {
//       if (c0 === 0 && c1 === 0) return [x0, y0, x1, y1];
//       if (c0 & c1) return null;
//       let x, y, c = c0 || c1;
//       if (c & 0b1000) x = x0 + (x1 - x0) * (this.ymax - y0) / (y1 - y0), y = this.ymax;
//       else if (c & 0b0100) x = x0 + (x1 - x0) * (this.ymin - y0) / (y1 - y0), y = this.ymin;
//       else if (c & 0b0010) y = y0 + (y1 - y0) * (this.xmax - x0) / (x1 - x0), x = this.xmax;
//       else y = y0 + (y1 - y0) * (this.xmin - x0) / (x1 - x0), x = this.xmin;
//       if (c0) x0 = x, y0 = y, c0 = this._regioncode(x0, y0);
//       else x1 = x, y1 = y, c1 = this._regioncode(x1, y1);
//     }
//   }
//   _clipInfinite(i, points, vx0, vy0, vxn, vyn) {
//     let P = Array.from(points), p;
//     if (p = this._project(P[0], P[1], vx0, vy0)) P.unshift(p[0], p[1]);
//     if (p = this._project(P[P.length - 2], P[P.length - 1], vxn, vyn)) P.push(p[0], p[1]);
//     if (P = this._clipFinite(i, P)) {
//       for (let j = 0, n = P.length, c0, c1 = this._edgecode(P[n - 2], P[n - 1]); j < n; j += 2) {
//         c0 = c1, c1 = this._edgecode(P[j], P[j + 1]);
//         if (c0 && c1) j = this._edge(i, c0, c1, P, j), n = P.length;
//       }
//     } else if (this.contains(i, (this.xmin + this.xmax) / 2, (this.ymin + this.ymax) / 2)) {
//       P = [this.xmin, this.ymin, this.xmax, this.ymin, this.xmax, this.ymax, this.xmin, this.ymax];
//     }
//     return P;
//   }
//   _edge(i, e0, e1, P, j) {
//     while (e0 !== e1) {
//       let x, y;
//       switch (e0) {
//         case 0b0101: e0 = 0b0100; continue; // top-left
//         case 0b0100: e0 = 0b0110, x = this.xmax, y = this.ymin; break; // top
//         case 0b0110: e0 = 0b0010; continue; // top-right
//         case 0b0010: e0 = 0b1010, x = this.xmax, y = this.ymax; break; // right
//         case 0b1010: e0 = 0b1000; continue; // bottom-right
//         case 0b1000: e0 = 0b1001, x = this.xmin, y = this.ymax; break; // bottom
//         case 0b1001: e0 = 0b0001; continue; // bottom-left
//         case 0b0001: e0 = 0b0101, x = this.xmin, y = this.ymin; break; // left
//       }
//       if ((P[j] !== x || P[j + 1] !== y) && this.contains(i, x, y)) {
//         P.splice(j, 0, x, y), j += 2;
//       }
//     }
//     if (P.length > 4) {
//       for (let i = 0; i < P.length; i+= 2) {
//         const j = (i + 2) % P.length, k = (i + 4) % P.length;
//         if (P[i] === P[j] && P[j] === P[k]
//         || P[i + 1] === P[j + 1] && P[j + 1] === P[k + 1])
//           P.splice(j, 2), i -= 2;
//       }
//     }
//     return j;
//   }
//   _project(x0, y0, vx, vy) {
//     let t = Infinity, c, x, y;
//     if (vy < 0) { // top
//       if (y0 <= this.ymin) return null;
//       if ((c = (this.ymin - y0) / vy) < t) y = this.ymin, x = x0 + (t = c) * vx;
//     } else if (vy > 0) { // bottom
//       if (y0 >= this.ymax) return null;
//       if ((c = (this.ymax - y0) / vy) < t) y = this.ymax, x = x0 + (t = c) * vx;
//     }
//     if (vx > 0) { // right
//       if (x0 >= this.xmax) return null;
//       if ((c = (this.xmax - x0) / vx) < t) x = this.xmax, y = y0 + (t = c) * vy;
//     } else if (vx < 0) { // left
//       if (x0 <= this.xmin) return null;
//       if ((c = (this.xmin - x0) / vx) < t) x = this.xmin, y = y0 + (t = c) * vy;
//     }
//     return [x, y];
//   }
//   _edgecode(x, y) {
//     return (x === this.xmin ? 0b0001
//         : x === this.xmax ? 0b0010 : 0b0000)
//         | (y === this.ymin ? 0b0100
//         : y === this.ymax ? 0b1000 : 0b0000);
//   }
//   _regioncode(x, y) {
//     return (x < this.xmin ? 0b0001
//         : x > this.xmax ? 0b0010 : 0b0000)
//         | (y < this.ymin ? 0b0100
//         : y > this.ymax ? 0b1000 : 0b0000);
//   }
// }
