use std::collections::HashMap;

use geo_types::Coord;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::console_log;
use web_sys::js_sys::Math::random;
use web_sys::OffscreenCanvasRenderingContext2d;
use web_sys::Performance;
use web_sys::PerformanceMeasure;

use d3_delaunay_rs::delaunay::Delaunay;

#[wasm_bindgen]
pub struct Stippler {
    width: usize,
    height: usize,
    n: usize,
    data: Vec<f64>,
    delaunay: Delaunay<f64>,
    context: OffscreenCanvasRenderingContext2d,
}

#[wasm_bindgen]
impl Stippler {
    pub(crate) fn new(
        width: usize,
        height: usize,
        data: Vec<f64>, // gray scale - image data
        n: usize,
        context: &OffscreenCanvasRenderingContext2d,
        performance: &Performance,
    ) -> Result<Stippler, JsValue> {
        performance.mark("go")?;
        let mut h_points: HashMap<usize, Coord<f64>> =
            HashMap::with_capacity(n);
        console_log!("n {}", n);
        console_log!("data.len {}", data.len());
        // Initialize the points using rejection sampling.
        for i in 0..n {
            '_30Loop: for _ in 0..30 {
                let x = f64::floor(random() * (width as f64));
                let y = f64::floor(random() * (height as f64));
                let index = y as usize * width + x as usize;
                h_points.insert(i, Coord { x, y });

                if random() < data[index] {
                    break '_30Loop;
                }
            }
        }
        let points = h_points.into_values().collect::<Vec<_>>();

        performance.mark("rejection_sampling_complete")?;

        performance.measure_with_start_mark_and_end_mark(
            "rejection_sampling",
            "go",
            "rejection_sampling_complete",
        )?;

        let js_measure =
            performance.get_entries_by_name("rejection_sampling").get(0);
        let measure = PerformanceMeasure::from(js_measure);

        console_log!("rejection sampling {:#?} ms", measure.duration());

        let delaunay = Delaunay::new(&points);

        performance.mark("initial_voronoi_complete")?;

        performance.measure_with_start_mark_and_end_mark(
            "initial_voronoi",
            "rejection_sampling_complete",
            "initial_voronoi_complete",
        )?;
        let js_measure =
            performance.get_entries_by_name("initial_voronoi").get(0);
        let measure = PerformanceMeasure::from(js_measure);

        console_log!("initial veronoi {:#?} ms", measure.duration());

        let state = Stippler {
            width,
            height,
            n,
            data,
            delaunay,
            context: context.clone(),
        };
        Ok(state)
    }

    pub fn next(&mut self, k: usize) -> Result<(), JsValue> {
        // Compute the weighted centroid for each Voronoi cell.
        let mut c: Vec<Coord<f64>> = Vec::with_capacity(self.n);
        let mut s: Vec<f64> = Vec::with_capacity(self.n);
        for _i in 0..self.n {
            c.push(Coord { x: 0_f64, y: 0_f64 });
            s.push(0_f64);
        }

        let mut i = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let w = self.data[y * self.width + x];
                i = self.delaunay.find(
                    &Coord {
                        x: x as f64 + 0.5_f64,
                        y: y as f64 + 0.5_f64,
                    },
                    Some(i),
                );
                s[i] += w;
                c[i].x += w * (x as f64 + 0.5_f64);
                c[i].y += w * (y as f64 + 0.5_f64);
            }
        }

        // Relax the diagram by moving points to the weighted centroid.
        // Wiggle the points a little bit so they don’t get stuck.
        let w = (k as f64 + 1_f64).powf(-0.8) * 10_f64;
        for i in 0..self.n {
            let x0 = self.delaunay.points[i].x;
            let y0 = self.delaunay.points[i].y;
            let x1 = if s[i] == 0_f64 { x0 } else { c[i].x / s[i] };
            let y1 = if s[i] == 0_f64 { y0 } else { c[i].y / s[i] };

            self.delaunay.points[i].x =
                x0 + (x1 - x0) * 1.8 + (random() - 0.5) * w;
            self.delaunay.points[i].y =
                y0 + (y1 - y0) * 1.8 + (random() - 0.5) * w;
        }

        self.draw()?;

        // // TODO: doing a update() the hard way...
        // // What can I refactor here.
        self.delaunay = Delaunay::new(&self.delaunay.points);

        Ok(())
    }

    // Render to Canvas.
    pub fn draw(&self) -> Result<(), JsValue> {
        self.context.set_fill_style_str("#fff");
        self.context.fill_rect(
            0_f64,
            0_f64,
            self.width as f64,
            self.height as f64,
        );

        self.context.begin_path();
        for p in &self.delaunay.points {
            self.context.move_to(p.x + 1.5_f64, p.y);
            self.context.arc(
                p.x,
                p.y,
                1.5_f64,
                0_f64,
                core::f64::consts::TAU,
            )?;
        }
        self.context.set_fill_style_str("#000");
        self.context.fill();

        Ok(())
    }
}
