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

    Ok(())
}
