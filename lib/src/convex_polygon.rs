//! Generates a collection of cells that can be rendered by the bevy games engine.
//!
use bevy_math::prelude::ConvexPolygon;
use bevy_math::prelude::Primitive2d;
use bevy_math::prelude::Primitive3d;
use bevy_math::primitives::ConvexPolygonError;
use bevy_math::Vec2;

use crate::CanvasRenderingContext2d;

/// Wrapped - So I can output to a bevy friendly `ConvexPolygon`.
#[derive(Debug)]
pub struct CP(Vec<Vec<Vec2>>);

impl Default for CP {
    fn default() -> Self {
        Self(vec![])
    }
}

impl Primitive3d for CP {}
impl Primitive2d for CP {}

impl CP {
    /// Self consuming conversion
    ///
    /// #Panics
    pub fn build(self) -> Result<Vec<ConvexPolygon>, ConvexPolygonError> {
        let mut out = Vec::with_capacity(self.0.len());
        for v in self.0 {
            let cp = ConvexPolygon::new(v)?;
            out.push(cp);
        }
        Ok(out)
    }
}
impl CanvasRenderingContext2d<f32> for CP {
    fn arc(&mut self, _p: &geo::Coord<f32>, _r: f32, _start: f32, _stop: f32) {
        unimplemented!()
    }

    fn close_path(&mut self) {
        if let Some(last_vec) = self.0.last_mut() {
            // When the cell is empty, fail by silently doing nothing
            if let Some(first_point) = last_vec.last() {
                last_vec.push(*first_point);
            }
        };
    }

    fn line_to(&mut self, p: &geo::Coord<f32>) {
        let last = self.0.last_mut().expect("Cell must have been started");
        last.push(Vec2::new(p.x, p.y))
    }

    fn move_to(&mut self, p: &geo::Coord<f32>) {
        // Start a new vector
        self.0.push(vec![Vec2::new(p.x, p.y)]);
    }

    fn rect(&mut self, _p: &geo::Coord<f32>, _w: f32, _h: f32) {
        unimplemented!()
    }
}
