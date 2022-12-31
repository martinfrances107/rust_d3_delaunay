use std::fmt::Display;
use std::fmt::Write;
use std::string::ToString;

use d3_geo_rs::math::EPSILON;
use geo::CoordFloat;
use geo_types::Coord;

use super::CanvasRenderingContext2d;

#[derive(Clone, Debug)]
/// Produces a string into response to `RendingContext2d` API calls.
pub struct Path<T>
where
    T: CoordFloat,
{
    p0: Coord<T>,
    p1: Option<Coord<T>>,
    s: String,
    epsilon: T,
}

impl<T> Default for Path<T>
where
    T: CoordFloat,
{
    #[inline]
    fn default() -> Self {
        Self {
            p0: Coord {
                x: T::zero(),
                y: T::zero(),
            },
            p1: None,
            s: String::new(),
            epsilon: T::from(EPSILON).unwrap(),
        }
    }
}

impl<T> ToString for Path<T>
where
    T: CoordFloat,
{
    fn to_string(&self) -> String {
        if self.s.is_empty() {
            String::new()
        } else {
            self.s.clone()
        }
    }
}

impl<T> CanvasRenderingContext2d<T> for Path<T>
where
    T: CoordFloat + Display,
{
    fn move_to(&mut self, p: &Coord<T>) {
        self.p0 = *p;
        self.p1 = Some(*p);
        write!(self.s, "M{},{}", p.x, p.y).expect("cannot apppend to buffer");
    }

    fn close_path(&mut self) {
        if self.p1.is_some() {
            self.p1 = Some(self.p0);
            write!(self.s, "Z").expect("cannot apppend to buffer");
        }
    }

    fn line_to(&mut self, p: &Coord<T>) {
        self.p1 = Some(*p);
        write!(self.s, "L{},{}", p.x, p.y).expect("cannot apppend to buffer");
    }

    fn arc(&mut self, p: &Coord<T>, r: T, _start: T, _stop: T) {
        let x0 = p.x + r;
        let y0 = p.y;

        debug_assert!(r >= T::zero(), "negative radius");

        match &self.p1 {
            Some(p1) => {
                if (p1.x - x0).abs() > self.epsilon || (p1.y - y0).abs() > self.epsilon {
                    write!(self.s, "L{x0},{y0}").expect("cannot apppend to buffer");
                }
                if r == T::zero() {
                    return;
                }
                self.p1 = Some(*p1);
                write!(
                    self.s,
                    "A{},{},0,1,1,{},{}A{},{},0,1,1,{},{}",
                    r,
                    r,
                    p.x - r,
                    p.y,
                    r,
                    r,
                    self.p0.x,
                    self.p0.y
                )
                .expect("cannot apppend to buffer");
            }
            _ => {
                write!(self.s, "M{x0},{y0}").expect("cannot apppend to buffer");
            }
        }
    }

    fn rect(&mut self, p: &Coord<T>, w: T, h: T) {
        self.p0 = *p;
        self.p1 = Some(*p);
        write!(self.s, "M{},{},{w}h{h}v{h}h{}Z", p.x, p.y, -w).expect("cannot apppend to buffer");
    }
}
