const EPSILON: f64 = 1e-6;

#[derive(Clone, Debug)]
pub struct Path {
  x0: f64,
  y0: f64,
  x1: Option<f64>,
  y1: Option<f64>,
  s: String,
}

impl Default for Path {
  fn default() -> Self {
    return Self {
      x0: 0f64,
      y0: 0f64,
      x1: None,
      y1: None,
      s: "".to_owned(),
    };
  }
}

impl Path {
  pub fn move_to(&mut self, x: f64, y: f64) {
    self.x0 = x;
    self.x1 = Some(x);
    self.y0 = y;
    self.y1 = Some(y);
    self.s.push_str(&format!("M{},{}", x, y));
  }

  pub fn close_path(&mut self) {
    match self.x1 {
      Some(_) => {
        self.x1 = Some(self.x0);
        self.y1 = Some(self.y0);
        self.s += "Z";
      }
      None => {}
    }
  }

  pub fn line_to(&mut self, x: f64, y: f64) {
    self.x1 = Some(x);
    self.y1 = Some(y);
    self.s.push_str(&format!("L{},{}", x, y));
  }

  pub fn arc(&mut self, x: f64, y: f64, r: f64) {
    let x0 = x + r;
    let y0 = y;
    if r < 0f64 {
      panic!("negative radius");
    }
    match (self.x1, self.y1) {
      (Some(x1), Some(y1)) => {
        if (x1 - x0).abs() > EPSILON || (y1 - y0).abs() > EPSILON {
          self.s.push_str(&format!("L{},{}", x0, y0));
        }
        if r == 0f64 {
          return;
        }
        self.x1 = Some(x0);
        self.y1 = Some(y0);
        self.s.push_str(&format!(
          "AS{},{},0,1,1,{},{}AS{},{},0,1,1{},{}",
          r,
          r,
          x - r,
          y,
          r,
          r,
          self.x0,
          self.y0
        ));
      }
      _ => {
        self.s.push_str(&format!("M{},{}", x0, y0));
      }
    }
  }

  pub fn rect(&mut self, x: f64, y: f64, w: f64, h: f64) {
    self.x0 = x;
    self.x1 = Some(x);
    self.y0 = y;
    self.y1 = Some(y);
    // `M${this._x0 = this._x1 = +x},${this._y0 = this._y1 = +y}h${+w}v${+h}h${-w}Z`;
    self
      .s
      .push_str(&format!("M{},{},{}h{}v{}h{}Z", x, y, w, h, h, -w));
  }

  pub fn value(&self) -> Option<String> {
    if self.s.is_empty() {
      return None;
    } else {
      return Some(self.s.clone());
    }
  }
}

//   value() {
//     return this._ || null;
//   }
// }
// const epsilon = 1e-6;

// export default class Path {
//   constructor() {
//     this._x0 = this._y0 = // start of current subpath
//     this._x1 = this._y1 = null; // end of current subpath
//     this._ = "";
//   }
//   moveTo(x, y) {
//     this._ += `M${this._x0 = this._x1 = +x},${this._y0 = this._y1 = +y}`;
//   }
//   closePath() {
//     if (this._x1 !== null) {
//       this._x1 = this._x0, this._y1 = this._y0;
//       this._ += "Z";
//     }
//   }
//   lineTo(x, y) {
//     this._ += `L${this._x1 = +x},${this._y1 = +y}`;
//   }
//   arc(x, y, r) {
//     x = +x, y = +y, r = +r;
//     const x0 = x + r;
//     const y0 = y;
//     if (r < 0) throw new Error("negative radius");
//     if (this._x1 === null) this._ += `M${x0},${y0}`;
//     else if (Math.abs(this._x1 - x0) > epsilon || Math.abs(this._y1 - y0) > epsilon) this._ += "L" + x0 + "," + y0;
//     if (!r) return;
//     this._ += `A${r},${r},0,1,1,${x - r},${y}A${r},${r},0,1,1,${this._x1 = x0},${this._y1 = y0}`;
//   }
//   rect(x, y, w, h) {
//     this._ += `M${this._x0 = this._x1 = +x},${this._y0 = this._y1 = +y}h${+w}v${+h}h${-w}Z`;
//   }
//   value() {
//     return this._ || null;
//   }
// }
