mod stippler;

extern crate d3_delaunay_rs;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::window;
use web_sys::Document;
use web_sys::ImageData;

use stippler::Stippler;

fn document() -> Result<Document, JsValue> {
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            Ok(document)
        } else {
            Err(JsValue::from_str("unable to get document"))
        }
    } else {
        Err(JsValue::from_str("Unable to get window."))
    }
}
#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    // load image.

    let eye_img = document()?
        .get_element_by_id("eye_img")
        .unwrap()
        .dyn_into::<web_sys::HtmlImageElement>()?;

    let width = eye_img.width() as usize;
    let height = eye_img.height() as usize;

    // Must write img to hidden canvas *ONLY* so that
    // data can be inspected.
    let eye_canvas = document()?
        .get_element_by_id("eye_canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let eye_context = eye_canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    eye_context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
      &eye_img, 0_f64, 0_f64, width as f64, height as f64, 0_f64, 0_f64, width as f64, height as f64
    )?;

    let image_data: ImageData = eye_context.get_image_data(
        0_f64,
        0_f64,
        width as f64,
        height as f64,
    )?;

    let rgba: Clamped<Vec<u8>> = image_data.data();

    let mut data: Vec<f64> = rgba
        .iter()
        .step_by(4)
        .map(|d| f64::max(0_f64, (1 - d / 254).into()))
        .collect();

    let canvas = document()?
        .get_element_by_id("c")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context_raw = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    Stippler::go(width, height, &mut data, width * height, context_raw)?;
    // let _ = stippler.draw()?;

    // // const delaunay = new d3.Delaunay(points);
    // let delaunay = Delaunay::new(&stippler.points);
    // let voronoi =
    //     delaunay.voronoi(Some((0_f64, 0_f64, width as f64, height as f64)));

    // // const c = new Float64Array(n * 2);
    // // const s = new Float64Array(n);
    // let c = Vec::<Coord<f64>>::with_capacity(n * 2);
    // let s = Vec::<f64>::with_capacity(n);

    // // for k in 0..80 {
    // let k = 0;
    // let mut c = iter::repeat(0)
    //     .take(n)
    //     .map(|i| Coord { x: 0_f64, y: 0_f64 })
    //     .collect::<Vec<Coord<f64>>>();
    // let mut s = iter::repeat(0f64).take(n).collect::<Vec<f64>>();
    // for y in 0..height as usize {
    //     for x in 0..width as usize {
    //         let w = data[y * width + x];
    //         let i = 0;
    //         let i = voronoi.delaunay.find(
    //             &Coord {
    //                 x: x as f64 + 0.5_f64,
    //                 y: y as f64 + 0.5_f64,
    //             },
    //             None,
    //         );
    //         s[i] += w;
    //         c[i].x += w * (x as f64 + 0.5_f64);
    //         c[i].y += w * (y as f64 + 0.5_f64);
    //     }
    // }

    // // Relax the diagram by moving points to the weighted centroid.
    // // Wiggle the points a little bit so they don’t get stuck.
    // let w = f64::powf(k as f64 + 1_f64, -0.8_f64) * 10_f64;
    // for i in 0..n {
    //     let x0 = stippler.points[i].x;
    //     let y0 = stippler.points[1].y;
    //     let (x1, y1) = if s[i] != 0_f64 {
    //         (c[i].x / s[i], c[i].y / s[i])
    //     } else {
    //         (x0, y0)
    //     };
    //     stippler.points[i].x =
    //         x0 + (x1 - x0) * 1.8_f64 + (random::<f64>() - 0.5_f64) * w;
    //     stippler.points[i].y =
    //         y0 + (y1 - y0) * 1.8_f64 + (random::<f64>() - 0.5_f64) * w;
    // }
    // _ stippler.draw()?;

    // }

    // for (let k = 0; k < 80; ++k) {

    //   // Compute the weighted centroid for each Voronoi cell.
    //   c.fill(0);
    //   s.fill(0);
    //   for (let y = 0, i = 0; y < height; ++y) {
    //     for (let x = 0; x < width; ++x) {
    //       const w = data[y * width + x];
    //       i = delaunay.find(x + 0.5, y + 0.5, i);
    //       s[i] += w;
    //       c[i * 2] += w * (x + 0.5);
    //       c[i * 2 + 1] += w * (y + 0.5);
    //     }
    //   }

    //   // Relax the diagram by moving points to the weighted centroid.
    //   // Wiggle the points a little bit so they don’t get stuck.
    //   const w = Math.pow(k + 1, -0.8) * 10;
    //   for (let i = 0; i < n; ++i) {
    //     const x0 = points[i * 2], y0 = points[i * 2 + 1];
    //     const x1 = s[i] ? c[i * 2] / s[i] : x0, y1 = s[i] ? c[i * 2 + 1] / s[i] : y0;
    //     points[i * 2] = x0 + (x1 - x0) * 1.8 + (Math.random() - 0.5) * w;
    //     points[i * 2 + 1] = y0 + (y1 - y0) * 1.8 + (Math.random() - 0.5) * w;
    //   }

    //   postMessage(points);
    //   voronoi.update();
    // }
    // }

    Ok(())
}
