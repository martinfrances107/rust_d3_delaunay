use super::RenderingContext2d;
use geo::CoordinateType;
use geo::Point;
use num_traits::float::Float;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Path<T>
where
    T: CoordinateType,
{
    p0: Point<T>,
    p1: Option<Point<T>>,
    s: String,
}

impl<T> RenderingContext2d<T> for Path<T>
where
    T: CoordinateType + Float + Display,
{
    fn new() -> Self {
        return Self {
            p0: Point::new(T::zero(), T::zero()),
            p1: None,
            s: "".to_owned(),
        };
    }

    fn move_to(&mut self, p: &Point<T>) {
        self.p0 = *p;
        self.p1 = Some(*p);
        self.s.push_str(&format!("M{},{}", p.x(), p.y()));
    }

    fn close_path(&mut self) {
        if self.p1.is_some() {
            self.p1 = Some(self.p0);
            self.s += "Z";
        }
    }

    fn line_to(&mut self, p: &Point<T>) {
        self.p1 = Some(*p);
        self.s.push_str(&format!("L{},{}", p.x(), p.y()));
    }

    fn arc(&mut self, p: &Point<T>, r: T) {
        let x0 = p.x() + r;
        let y0 = p.y();
        if r < T::zero() {
            panic!("negative radius");
        }
        match &self.p1 {
            Some(p1) => {
                if (p1.x() - x0).abs() > T::epsilon() || (p1.y() - y0).abs() > T::epsilon() {
                    self.s.push_str(&format!("L{},{}", x0, y0));
                }
                if r == T::zero() {
                    return;
                }
                self.p1 = Some(*p1);
                self.s.push_str(&format!(
                    "AS{},{},0,1,1,{},{}AS{},{},0,1,1{},{}",
                    r,
                    r,
                    p.x() - r,
                    p.y(),
                    r,
                    r,
                    self.p0.x(),
                    self.p0.y()
                ));
            }
            _ => {
                self.s.push_str(&format!("M{},{}", x0, y0));
            }
        }
    }

    fn rect(&mut self, p: &Point<T>, w: T, h: T) {
        self.p0 = *p;
        self.p1 = Some(*p);
        self.s
            .push_str(&format!("M{},{},{}h{}v{}h{}Z", p.x(), p.y(), w, h, h, -w));
    }

    fn value_str(&self) -> String {
        if self.s.is_empty() {
            return "".to_string();
        } else {
            return self.s.clone();
        }
    }

    fn value(&self) -> Vec<Point<T>> {
        return Vec::new();
    }
}
