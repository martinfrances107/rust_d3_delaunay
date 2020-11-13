//

use std::cmp::Ordering;

// use delaunator::Point;

mod colinear;
mod jitter;

/// Delaunay triangulation
use std::collections::HashMap;
use std::fmt;

// use delaunator::Point;

use rust_d3_geo::projection::projection_mutator::ProjectionMutator;

use crate::voronoi::Voronoi;
use colinear::colinear;
use rust_d3_geo::Transform;
use rust_d3_geo::TransformIdentity;

use delaunator::{triangulate, Point, Triangulation, EMPTY};

use jitter::jitter;

pub struct Delaunay {
    pub colinear: Vec<usize>,
    delaunator: Option<Triangulation>,
    pub inedges: Vec<usize>,
    pub hull_index: Vec<usize>,
    pub half_edges: Vec<usize>,
    pub hull: Vec<usize>,
    pub triangles: Vec<usize>,
    pub points: Vec<Point>,
    pub projection: Box<dyn Transform>,
    fx: Box<dyn Fn(Point, usize, Vec<Point>) -> f64>,
    fy: Box<dyn Fn(Point, usize, Vec<Point>) -> f64>,
}

impl<'a> Default for Delaunay {
    fn default() -> Self {
        // let points = Vec::new();
        return Self {
            colinear: Vec::new(),
            delaunator: None,
            inedges: Vec::new(),
            half_edges: Vec::new(),
            hull: Vec::new(),
            hull_index: Vec::new(),
            points: Vec::new(),
            projection: Box::new(TransformIdentity {}),
            fx: Box::new(|p: Point, _i: usize, _points: Vec<Point>| return p.x),
            fy: Box::new(|p: Point, _i: usize, _points: Vec<Point>| return p.y),
            triangles: Vec::new(),
        };
    }
}

impl<'a> Delaunay {
    // pub fn from(points: Vec<Point>, fx:Option<Box<dyn Fn(Point, usize, Vec<Point>) -> F>>, fy:Option<Box<dyn Fn(Point, usize, Vec<Point>) -> F>>) -> Self
    // {
    //   let  default = Delaunay::<F>::default();
    //   match (fx, fy) {
    //     (Some(fx), Some(fy)) => {return Self::new(flat_array(points, fx, fy));}
    //     (Some(fx), None) => {return Self::new(flat_array(points, fx, default.fy));}
    //     (None, Some(fy)) => {return Self::new(flat_array(points, default.fx, fy));}
    //     (None, None) => {return Self::new(flat_array(points, default.fx, default.fy));}

    //   }

    // }
    //   static from(points, fx = pointX, fy = pointY, that) {
    //     return new Delaunay("length" in points
    //         ? flatArray(points, fx, fy, that)
    //         : Float64Array.from(flatIterable(points, fx, fy, that)));
    //   }
    //   constructor(points) {
    //     this._delaunator = new Delaunator(points);
    //     this.inedges = new Int32Array(points.length / 2);
    //     this._hullIndex = new Int32Array(points.length / 2);
    //     this.points = this._delaunator.coords;
    //     this._init();
    //   }
    pub fn new(points: Vec<Point>) -> Self {
        let half = points.len() / 2;
        let delaunator = triangulate(&points);
        let mut out = Self {
            delaunator,
            inedges: Vec::with_capacity(half),
            hull_index: Vec::with_capacity(half),
            points,
            ..Delaunay::default()
        };
        {
            out.init();
        }
        return out;
    }

    fn init(&mut self) {
        let d = &self.delaunator;
        // let points = self.points;

        // check for collinear
        // if d.hull && d.hull.len() > 2usize && colinear(&points, &d) {
        match d {
            None => {}
            Some(d) => {
                if d.hull.len() > 2usize && colinear(&self.points, &d) {
                    // this.collinear = Int32Array.from({length: points.length/2}, (_,i) => i)
                    //   .sort((i, j) => points[2 * i] - points[2 * j] || points[2 * i + 1] - points[2 * j + 1]); // for exact neighbors
                    let len = self.points.len() as u32 / 2;
                    // self.colinear = (0..len).collect::u32()
                    let mut colinear_vec: Vec<usize> = (0..len)
                        .map(|i| {
                            return i as usize;
                        })
                        .collect();
                    // .sort_by(|i, j| points[2 * i] - points[2 * j] || points[2 * i + 1] - points[2 * j + 1]);
                    colinear_vec.sort_by(|i, j| {
                        let x_diff = self.points[*i].x - self.points[*j].x;
                        if x_diff != 0f64 {
                            if x_diff.is_sign_positive() {
                                return Ordering::Greater;
                            } else {
                                return Ordering::Less;
                            }
                        } else {
                            let y_diff = self.points[*i].y - self.points[*j].y;
                            if y_diff == 0f64 {
                                return Ordering::Equal;
                            } else if y_diff.is_sign_positive() {
                                return Ordering::Greater;
                            } else {
                                return Ordering::Less;
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
                    let r = 1e-8f64 * (bounds[3] - bounds[1]).hypot(bounds[3] - bounds[0]);
                    // for (let i = 0, n = points.length / 2; i < n; ++i) {
                    for i in 0..self.points.len() / 2 {
                        let p = jitter(&self.points[i], r);
                        self.points[i].x = p.x;
                        self.points[i].y = p.y;
                    }
                    self.delaunator = triangulate(&self.points);
                } else {
                    // delete self.colinear;
                    self.colinear = Vec::new();
                }
            }
        }
        // self.delaunator.expect("expetced a valid return from delaunator");
        let hull: Vec<usize>;
        match &self.delaunator {
            Some(d) => {
                self.half_edges = d.halfedges.clone();
                self.hull = d.hull.clone();
                hull = self.hull.clone();
                self.triangles = d.triangles.clone();
            }
            None => {
                panic!("expected a delaunator.");
            }
        }

        // todo work out a appropiate work arround to not being
        // able to use -1 as a invalid state.
        self.inedges = Vec::new();
        self.hull_index = Vec::new();
        let len = self.points.len();
        for _i in 0..len {
            self.inedges.push(EMPTY);
            self.hull_index.push(EMPTY);
        }

        // let inedges = &self.inedges;
        // let hull_index = &self.hull_index;

        // Compute an index from each point to an (arbitrary) incoming halfedge
        // Used to give the first neighbor of each point; for this reason,
        // on the hull we give priority to exterior halfedges
        for (e, he) in self.half_edges.iter().enumerate() {
            let p: usize;
            if e % 3 == 2 {
                p = self.triangles[e - 2];
            } else {
                p = self.triangles[e + 1];
            }
            if *he == EMPTY || self.inedges[p] == EMPTY {
                self.inedges[p] = e;
            }
        }

        for (i, h) in hull.iter().enumerate() {
            self.hull_index[*h] = i;
        }

        // degenerate case: 1 or 2 (distinct) points
        let hull_len: u32 = hull.len() as u32;
        if hull_len <= 2u32 && !hull.is_empty() {
            // TODO work out the implications of not setting an invalid
            // value here rust has a usize, javascript  allows -1 as invalid.
            self.triangles = vec![EMPTY, EMPTY, EMPTY];
            self.half_edges = vec![EMPTY, EMPTY, EMPTY];
            self.triangles[0] = hull[0];
            self.triangles[1] = hull[1];
            self.triangles[2] = hull[1];
            self.inedges[hull[0]] = 1;
            let hull_len: u32 = hull.len() as u32;
            if hull_len == 2 {
                self.inedges[hull[1]] = 0;
            }
        }
    }
    pub fn step(&self, i: usize, x: f64, y: f64) -> usize {
        if self.inedges[i] == EMPTY || self.points.is_empty() {
            return (i + 1) % (self.points.len() >> 1);
        };
        let mut c = i;
        let mut dc = (x - self.points[i].x).powi(2) + (y - self.points[i].y).powi(2);
        let e0 = self.inedges[i];
        let mut e = e0;
        loop {
            let t = self.triangles[e];
            let dt = (x - self.points[t].x).powi(2) + (y - self.points[t].y).powi(2);
            if dt < dc {
                dc = dt;
                c = t;
            }

            if e % 3 == 2 {
                e = e - 2;
            } else {
                e = e + 1;
            }

            if self.triangles[e] != i {
                // bad triangulation
                break;
            }
            e = self.half_edges[e];
            if e == EMPTY {
                e = self.hull[(self.hull_index[i] + 1) % self.hull.len()];
                if e != t {
                    if (x - self.points[e].x).powi(2) + (y - self.points[e].y).powi(2) < dc {
                        return e;
                    };
                }
                break;
            }
            if e == e0 {
                break;
            }
        }

        return c;
    }
}

// import Delaunator from "delaunator";
// import Path from "./path.js";
// import Polygon from "./polygon.js";
// import Voronoi from "./voronoi.js";

// const tau = 2 * Math.PI, pow = Math.pow;

// function pointX(p) {
//   return p[0];
// }

// function pointY(p) {
//   return p[1];
// }

// // A triangulation is collinear if all its triangles have a non-null area
// function collinear(d) {
//   const {triangles, coords} = d;
//   for (let i = 0; i < triangles.length; i += 3) {
//     const a = 2 * triangles[i],
//           b = 2 * triangles[i + 1],
//           c = 2 * triangles[i + 2],
//           cross = (coords[c] - coords[a]) * (coords[b + 1] - coords[a + 1])
//                 - (coords[b] - coords[a]) * (coords[c + 1] - coords[a + 1]);
//     if (cross > 1e-10) return false;
//   }
//   return true;
// }

// function jitter(x, y, r) {
//   return [x + Math.sin(x + y) * r, y + Math.cos(x - y) * r];
// }

// export default class Delaunay {
//   static from(points, fx = pointX, fy = pointY, that) {
//     return new Delaunay("length" in points
//         ? flatArray(points, fx, fy, that)
//         : Float64Array.from(flatIterable(points, fx, fy, that)));
//   }
//   constructor(points) {
//     this._delaunator = new Delaunator(points);
//     this.inedges = new Int32Array(points.length / 2);
//     this._hullIndex = new Int32Array(points.length / 2);
//     this.points = this._delaunator.coords;
//     this._init();
//   }
//   update() {
//     this._delaunator.update();
//     this._init();
//     return this;
//   }
//   _init() {
//     const d = this._delaunator, points = this.points;

//     // check for collinear
//     if (d.hull && d.hull.length > 2 && collinear(d)) {
//       this.collinear = Int32Array.from({length: points.length/2}, (_,i) => i)
//         .sort((i, j) => points[2 * i] - points[2 * j] || points[2 * i + 1] - points[2 * j + 1]); // for exact neighbors
//       const e = this.collinear[0], f = this.collinear[this.collinear.length - 1],
//         bounds = [ points[2 * e], points[2 * e + 1], points[2 * f], points[2 * f + 1] ],
//         r = 1e-8 * Math.hypot(bounds[3] - bounds[1], bounds[2] - bounds[0]);
//       for (let i = 0, n = points.length / 2; i < n; ++i) {
//         const p = jitter(points[2 * i], points[2 * i + 1], r);
//         points[2 * i] = p[0];
//         points[2 * i + 1] = p[1];
//       }
//       this._delaunator = new Delaunator(points);
//     } else {
//       delete this.collinear;
//     }

//     const halfedges = this.halfedges = this._delaunator.halfedges;
//     const hull = this.hull = this._delaunator.hull;
//     const triangles = this.triangles = this._delaunator.triangles;
//     const inedges = this.inedges.fill(-1);
//     const hullIndex = this._hullIndex.fill(-1);

//     // Compute an index from each point to an (arbitrary) incoming halfedge
//     // Used to give the first neighbor of each point; for this reason,
//     // on the hull we give priority to exterior halfedges
//     for (let e = 0, n = halfedges.length; e < n; ++e) {
//       const p = triangles[e % 3 === 2 ? e - 2 : e + 1];
//       if (halfedges[e] === -1 || inedges[p] === -1) inedges[p] = e;
//     }
//     for (let i = 0, n = hull.length; i < n; ++i) {
//       hullIndex[hull[i]] = i;
//     }

//     // degenerate case: 1 or 2 (distinct) points
//     if (hull.length <= 2 && hull.length > 0) {
//       this.triangles = new Int32Array(3).fill(-1);
//       this.halfedges = new Int32Array(3).fill(-1);
//       this.triangles[0] = hull[0];
//       this.triangles[1] = hull[1];
//       this.triangles[2] = hull[1];
//       inedges[hull[0]] = 1;
//       if (hull.length === 2) inedges[hull[1]] = 0;
//     }
//   }
//   voronoi(bounds) {
//     return new Voronoi(this, bounds);
//   }
//   *neighbors(i) {
//     const {inedges, hull, _hullIndex, halfedges, triangles, collinear} = this;

//     // degenerate case with several collinear points
//     if (collinear) {
//       const l = collinear.indexOf(i);
//       if (l > 0) yield collinear[l - 1];
//       if (l < collinear.length - 1) yield collinear[l + 1];
//       return;
//     }

//     const e0 = inedges[i];
//     if (e0 === -1) return; // coincident point
//     let e = e0, p0 = -1;
//     do {
//       yield p0 = triangles[e];
//       e = e % 3 === 2 ? e - 2 : e + 1;
//       if (triangles[e] !== i) return; // bad triangulation
//       e = halfedges[e];
//       if (e === -1) {
//         const p = hull[(_hullIndex[i] + 1) % hull.length];
//         if (p !== p0) yield p;
//         return;
//       }
//     } while (e !== e0);
//   }
//   find(x, y, i = 0) {
//     if ((x = +x, x !== x) || (y = +y, y !== y)) return -1;
//     const i0 = i;
//     let c;
//     while ((c = this._step(i, x, y)) >= 0 && c !== i && c !== i0) i = c;
//     return c;
//   }
//   _step(i, x, y) {
//     const {inedges, hull, _hullIndex, halfedges, triangles, points} = this;
//     if (inedges[i] === -1 || !points.length) return (i + 1) % (points.length >> 1);
//     let c = i;
//     let dc = pow(x - points[i * 2], 2) + pow(y - points[i * 2 + 1], 2);
//     const e0 = inedges[i];
//     let e = e0;
//     do {
//       let t = triangles[e];
//       const dt = pow(x - points[t * 2], 2) + pow(y - points[t * 2 + 1], 2);
//       if (dt < dc) dc = dt, c = t;
//       e = e % 3 === 2 ? e - 2 : e + 1;
//       if (triangles[e] !== i) break; // bad triangulation
//       e = halfedges[e];
//       if (e === -1) {
//         e = hull[(_hullIndex[i] + 1) % hull.length];
//         if (e !== t) {
//           if (pow(x - points[e * 2], 2) + pow(y - points[e * 2 + 1], 2) < dc) return e;
//         }
//         break;
//       }
//     } while (e !== e0);
//     return c;
//   }
//   render(context) {
//     const buffer = context == null ? context = new Path : undefined;
//     const {points, halfedges, triangles} = this;
//     for (let i = 0, n = halfedges.length; i < n; ++i) {
//       const j = halfedges[i];
//       if (j < i) continue;
//       const ti = triangles[i] * 2;
//       const tj = triangles[j] * 2;
//       context.moveTo(points[ti], points[ti + 1]);
//       context.lineTo(points[tj], points[tj + 1]);
//     }
//     this.renderHull(context);
//     return buffer && buffer.value();
//   }
//   renderPoints(context, r = 2) {
//     const buffer = context == null ? context = new Path : undefined;
//     const {points} = this;
//     for (let i = 0, n = points.length; i < n; i += 2) {
//       const x = points[i], y = points[i + 1];
//       context.moveTo(x + r, y);
//       context.arc(x, y, r, 0, tau);
//     }
//     return buffer && buffer.value();
//   }
//   renderHull(context) {
//     const buffer = context == null ? context = new Path : undefined;
//     const {hull, points} = this;
//     const h = hull[0] * 2, n = hull.length;
//     context.moveTo(points[h], points[h + 1]);
//     for (let i = 1; i < n; ++i) {
//       const h = 2 * hull[i];
//       context.lineTo(points[h], points[h + 1]);
//     }
//     context.closePath();
//     return buffer && buffer.value();
//   }
//   hullPolygon() {
//     const polygon = new Polygon;
//     this.renderHull(polygon);
//     return polygon.value();
//   }
//   renderTriangle(i, context) {
//     const buffer = context == null ? context = new Path : undefined;
//     const {points, triangles} = this;
//     const t0 = triangles[i *= 3] * 2;
//     const t1 = triangles[i + 1] * 2;
//     const t2 = triangles[i + 2] * 2;
//     context.moveTo(points[t0], points[t0 + 1]);
//     context.lineTo(points[t1], points[t1 + 1]);
//     context.lineTo(points[t2], points[t2 + 1]);
//     context.closePath();
//     return buffer && buffer.value();
//   }
//   *trianglePolygons() {
//     const {triangles} = this;
//     for (let i = 0, n = triangles.length / 3; i < n; ++i) {
//       yield this.trianglePolygon(i);
//     }
//   }
//   trianglePolygon(i) {
//     const polygon = new Polygon;
//     this.renderTriangle(i, polygon);
//     return polygon.value();
//   }
// }

// function flatArray(points, fx, fy, that) {
//   const n = points.length;
//   const array = new Float64Array(n * 2);
//   for (let i = 0; i < n; ++i) {
//     const p = points[i];
//     array[i * 2] = fx.call(that, p, i, points);
//     array[i * 2 + 1] = fy.call(that, p, i, points);
//   }
//   return array;
// }

// function* flatIterable(points, fx, fy, that) {
//   let i = 0;
//   for (const p of points) {
//     yield fx.call(that, p, i, points);
//     yield fy.call(that, p, i, points);
//     ++i;
//   }
// }
