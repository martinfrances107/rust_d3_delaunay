use geo_types::Coord;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::console_log;
use web_sys::js_sys::Math::random;
use web_sys::CanvasRenderingContext2d;

use d3_delaunay_rs::delaunay::Delaunay;

pub(crate) struct Stippler {
    width: usize,
    height: usize,
    n: usize,
    context: CanvasRenderingContext2d,
}

impl Stippler {
    pub(crate) fn go(
        width: usize,
        height: usize,
        data: &mut [f64], // gray scale - image data
        n: usize,
        context: CanvasRenderingContext2d,
    ) -> Result<(), JsValue> {
        let state = Stippler {
            width,
            height,
            n,
            context,
        };

        let mut points = Vec::with_capacity(n);
        console_log!("n {}", n);
        console_log!("data.len {}", data.len());
        // Initialize the points using rejection sampling.
        for i in 0..n {
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

        let mut delaunay = Delaunay::new(&points);
        let mut voronoi =
            delaunay.voronoi(Some((0_f64, 0_f64, width as f64, height as f64)));

        for k in 0..80 {
            // Compute the weighted centroid for each Voronoi cell.
            let mut c = Vec::with_capacity(n);
            let mut s = Vec::with_capacity(n);
            for i in 0..n {
                c.push(Coord { x: 0_f64, y: 0_f64 });
                s.push(0_f64);
            }

            //     // I javascript land i is null here.
            //     // find() treats None as zero.
            //     let mut i = 0;
            //     for y in 0..height {
            //         // for (let x = 0; x < width; ++x) {
            //         for x in 0..width {
            //             let w = data[y * width + x];
            //             let i = voronoi.delaunay.find(
            //                 &Coord {
            //                     x: x as f64 + 0.5_f64,
            //                     y: y as f64 + 0.5_f64,
            //                 },
            //                 Some(i),
            //             );
            //             s[i] += w;
            //             // c[i * 2] += w * (x + 0.5);
            //             // c[i * 2 + 1] += w * (y + 0.5);
            //             c[i].x += w * (x as f64 + 0.5_f64);
            //             c[i].y += w * (y as f64 + 0.5_f64);
            //         }
            //     }

            //     // Relax the diagram by moving points to the weighted centroid.
            //     // Wiggle the points a little bit so they don’t get stuck.
            //     let w = (k as f64 + 1_f64).powf(-0.8) * 10_f64;
            //     // for (let i = 0; i < n; ++i) {
            //     for i in 0..n {
            //         let x0 = points[i].x;
            //         let y0 = points[i].y;
            //         // let x1 = s[i] ? c[i * 2] / s[i] : x0;
            //         let x1 = if s[i] == 0_f64 { x0 } else { c[i].y / s[i] };
            //         // let y1 = s[i] ? c[i * 2 + 1] / s[i] : y0;
            //         let y1 = if s[i] == 0_f64 { y0 } else { c[i].y / s[i] };
            //         points[i].x = x0 + (x1 - x0) * 1.8 + (random() - 0.5) * w;
            //         points[i].y = y0 + (y1 - y0) * 1.8 + (random() - 0.5) * w;
            //     }

            state.draw(&points)?;
            // // TODO: doing a update() the hard way...
            // // What can I refactor here.
            delaunay = Delaunay::new(&points);
            voronoi = delaunay.voronoi(Some((
                0_f64,
                0_f64,
                width as f64,
                height as f64,
            )));
        }

        Ok(())
    }

    // Render to Canvas.
    fn draw(&self, points: &[Coord<f64>]) -> Result<(), JsValue> {
        self.context.set_fill_style(&JsValue::from("#fff"));
        self.context.fill_rect(
            0_f64,
            0_f64,
            self.width as f64,
            self.height as f64,
        );
        self.context.begin_path();

        for p in points {
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
