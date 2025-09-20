use gloo_utils::format::JsValueSerdeExt;
use sssui_mpc_rs::point::Point256;
use sssui_mpc_rs::sss_ed25519::sss_combine_ed25519;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn combine_ed25519(points: JsValue, t: u32) -> Result<JsValue, JsValue> {
    let points: Vec<Point256> = points
        .into_serde()
        .map_err(|err| JsValue::from_str(&err.to_string()))?;

    let out = sss_combine_ed25519(points, t).map_err(|err| JsValue::from_str(&err.to_string()))?;

    JsValue::from_serde(&out).map_err(|err| JsValue::from_str(&err.to_string()))
}
