mod stippler;

extern crate d3_delaunay_rs;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::console_log;
use web_sys::window;
use web_sys::Document;
use web_sys::ImageData;
use web_sys::OffscreenCanvas;
use web_sys::PerformanceMeasure;

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
pub fn main(
    original_canvas: OffscreenCanvas,
    strippled_canvas: OffscreenCanvas,
) -> Result<Stippler, JsValue> {
    // load image.

    let window =
        web_sys::window().expect("should have a window in this context");

    let performance = window
        .performance()
        .expect("performance should be available");

    performance.mark("start")?;

    let document = document()?;

    let eye_img = document
        .get_element_by_id("eye_img")
        .unwrap()
        .dyn_into::<web_sys::HtmlImageElement>()?;

    let width = eye_img.width() as usize;
    let height = eye_img.height() as usize;

    let eye_context = original_canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::OffscreenCanvasRenderingContext2d>(
    )?;

    let context_raw = strippled_canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::OffscreenCanvasRenderingContext2d>(
    )?;
    performance.mark("init_complete")?;

    performance.measure_with_start_mark_and_end_mark(
        "init",
        "start",
        "init_complete",
    )?;

    let js_measure = performance.get_entries_by_name("init").get(0);
    let measure = PerformanceMeasure::from(js_measure);

    console_log!("intitialisation {:#?} ms", measure.duration());

    eye_context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
      &eye_img, 0_f64, 0_f64, width as f64, height as f64, 0_f64, 0_f64, width as f64, height as f64
    )?;
    performance.mark("draw_complete")?;

    performance.measure_with_start_mark_and_end_mark(
        "draw_to_canvas",
        "init_complete",
        "draw_complete",
    )?;

    let js_measure = performance.get_entries_by_name("draw_to_canvas").get(0);
    let measure = PerformanceMeasure::from(js_measure);

    console_log!("draw to the canvas {:#?} ms", measure.duration());

    let image_data: ImageData = eye_context.get_image_data(
        0_f64,
        0_f64,
        width as f64,
        height as f64,
    )?;

    performance.mark("get_data_from_canvas_complete")?;

    performance.measure_with_start_mark_and_end_mark(
        "get_image_data_from_canvas",
        "draw_complete",
        "get_data_from_canvas_complete",
    )?;

    let js_measure = performance
        .get_entries_by_name("get_image_data_from_canvas")
        .get(0);
    let measure = PerformanceMeasure::from(js_measure);

    console_log!("get image data from canvas {:#?} ms", measure.duration());

    let data: Vec<f64> = image_data
        .data()
        .drain(..)
        .step_by(4)
        .map(|d| f64::max(0_f64, (1 - d / 254).into()))
        .collect();

    performance.mark("data_transform_complete")?;

    performance.measure_with_start_mark_and_end_mark(
        "data_transform",
        "get_data_from_canvas_complete",
        "data_transform_complete",
    )?;

    let js_measure = performance.get_entries_by_name("data_transform").get(0);
    let measure = PerformanceMeasure::from(js_measure);

    console_log!("data transform {:#?} ms", measure.duration());

    let stippler = Stippler::new(
        width,
        height,
        data,
        width * height,
        &context_raw,
        &performance,
    )?;

    Ok(stippler)
}
