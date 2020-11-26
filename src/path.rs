const EPSILON: f64 = 1e-6;

use super::RenderingContext2d;
use delaunator::Point;

#[derive(Clone, Debug)]
pub struct Path {
    // x0: f64,
    // y0: f64,
    p0: Point,
    p1: Option<Point>,
    // x1: Option<f64>,
    // y1: Option<f64>,
    s: String,
}

impl RenderingContext2d for Path {
    fn new() -> Self {
        return Self {
            p0: Point { x: 0f64, y: 0f64 },
            p1: None,
            s: "".to_owned(),
        };
    }

    fn move_to(&mut self, p: Point) {
        self.p0 = p.clone();
        self.p1 = Some(p.clone());
        self.s.push_str(&format!("M{},{}", p.x, p.y));
    }

    fn close_path(&mut self) {
        if self.p1.is_some() {
            self.p1 = Some(self.p0.clone());
            self.s += "Z";
        }
    }

    fn line_to(&mut self, p: Point) {
        self.p1 = Some(p.clone());
        self.s.push_str(&format!("L{},{}", p.x, p.y));
    }

    fn arc(&mut self, p: Point, r: f64) {
        let x0 = p.x + r;
        let y0 = p.y;
        if r < 0f64 {
            panic!("negative radius");
        }
        match &self.p1 {
            Some(p1) => {
                if (p1.x - x0).abs() > EPSILON || (p1.y - y0).abs() > EPSILON {
                    self.s.push_str(&format!("L{},{}", x0, y0));
                }
                if r == 0f64 {
                    return;
                }
                self.p1 = Some(p1.clone());
                self.s.push_str(&format!(
                    "AS{},{},0,1,1,{},{}AS{},{},0,1,1{},{}",
                    r,
                    r,
                    p.x - r,
                    p.y,
                    r,
                    r,
                    self.p0.x,
                    self.p0.y
                ));
            }
            _ => {
                self.s.push_str(&format!("M{},{}", x0, y0));
            }
        }
    }

    fn rect(&mut self, p: Point, w: f64, h: f64) {
        self.p0 = p.clone();
        self.p1 = Some(p.clone());
        self.s
            .push_str(&format!("M{},{},{}h{}v{}h{}Z", p.x, p.y, w, h, h, -w));
    }

    fn value_str(&self) -> String {
        if self.s.is_empty() {
            return "".to_string();
        } else {
            return self.s.clone();
        }
    }

    fn value(&self) -> Vec<Point> {
        return Vec::new();
    }
}
