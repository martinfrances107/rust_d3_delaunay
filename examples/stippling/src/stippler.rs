use geo_types::Coord;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::console_log;
use web_sys::js_sys::Math::random;
use web_sys::OffscreenCanvasRenderingContext2d;
use web_sys::Performance;
use web_sys::PerformanceMeasure;

use d3_delaunay_rs::delaunay::Delaunay;
use d3_delaunay_rs::voronoi::Voronoi;

#[wasm_bindgen]
pub struct Stippler {
    width: usize,
    height: usize,
    n: usize,
    data: Vec<f64>,
    points: Vec<Coord<f64>>,
    voronoi: Voronoi<f64>,
    context: OffscreenCanvasRenderingContext2d,
    c: Vec<Coord<f64>>,
    s: Vec<f64>,
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
        let mut points = Vec::with_capacity(n);
        console_log!("n {}", n);
        console_log!("data.len {}", data.len());
        // Initialize the points using rejection sampling.
        for _i in 0..n {
            '_30Loop: for _ in 0..30 {
                let x = f64::floor(random() * (width as f64));
                let y = f64::floor(random() * (height as f64));
                let index = y as usize * width + x as usize;
                points.push(Coord { x, y });
                if random() < data[index] {
                    break '_30Loop;
                }
            }
        }

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
        let voronoi =
            delaunay.voronoi(Some((0_f64, 0_f64, width as f64, height as f64)));

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

        let mut c = Vec::with_capacity(n);
        let mut s = Vec::with_capacity(n);
        for _i in 0..n {
            c.push(Coord { x: 0_f64, y: 0_f64 });
            s.push(0_f64);
        }

        let state = Stippler {
            width,
            height,
            n,
            data,
            points,
            voronoi,
            context: context.clone(),
            s,
            c,
        };
        Ok(state)
    }

    pub fn next(&mut self, k: usize) -> Result<(), JsValue> {
        // Compute the weighted centroid for each Voronoi cell.
        for i in 0..self.n {
            self.c[i] = Coord { x: 0_f64, y: 0_f64 };
            self.s[i] = 0_f64;
        }

        // I javascript land i is null here.
        // find() treats None as zero.
        let mut i = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let w = self.data[y * self.width + x];
                i = self.voronoi.delaunay.find(
                    &Coord {
                        x: x as f64 + 0.5_f64,
                        y: y as f64 + 0.5_f64,
                    },
                    Some(i),
                );
                self.s[i] += w;
                self.c[i].x += w * (x as f64 + 0.5_f64);
                self.c[i].y += w * (y as f64 + 0.5_f64);
            }
        }

        // Relax the diagram by moving points to the weighted centroid.
        // Wiggle the points a little bit so they don’t get stuck.
        let w = (k as f64 + 1_f64).powf(-0.8) * 10_f64;
        // for (let i = 0; i < n; ++i) {
        for i in 0..self.n {
            let x0 = self.points[i].x;
            let y0 = self.points[i].y;
            // let x1 = s[i] ? c[i * 2] / s[i] : x0;
            let x1 = if self.s[i] == 0_f64 {
                x0
            } else {
                self.c[i].y / self.s[i]
            };
            // let y1 = s[i] ? c[i * 2 + 1] / s[i] : y0;
            let y1 = if self.s[i] == 0_f64 {
                y0
            } else {
                self.c[i].y / self.s[i]
            };
            self.points[i].x = x0 + (x1 - x0) * 1.8 + (random() - 0.5) * w;
            self.points[i].y = y0 + (y1 - y0) * 1.8 + (random() - 0.5) * w;
        }

        self.draw()?;

        // // TODO: doing a update() the hard way...
        // // What can I refactor here.
        let delaunay = Delaunay::new(&self.points);
        self.voronoi = delaunay.voronoi(Some((
            0_f64,
            0_f64,
            self.width as f64,
            self.height as f64,
        )));

        Ok(())
    }

    // Render to Canvas.
    pub fn draw(&self) -> Result<(), JsValue> {
        self.context.set_fill_style(&JsValue::from("#fff"));
        self.context.fill_rect(
            0_f64,
            0_f64,
            self.width as f64,
            self.height as f64,
        );
        self.context.begin_path();

        for p in &self.points {
            self.context.move_to(p.x + 1.5_f64, p.y);
            self.context.arc(
                p.x,
                p.y,
                1.5_f64,
                0_f64,
                core::f64::consts::TAU,
            )?;
        }
        self.context.set_fill_style(&JsValue::from("#000"));
        self.context.fill();

        Ok(())
    }
}
