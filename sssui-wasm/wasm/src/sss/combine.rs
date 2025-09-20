use gloo_utils::format::JsValueSerdeExt;
use k256::Secp256k1;
use sssui_rs::point::Point256;
use sssui_rs::sss::combine;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sss_combine(points: JsValue, t: u32) -> Result<JsValue, JsValue> {
    let points: Vec<Point256> = points
        .into_serde()
        .map_err(|err| JsValue::from_str(&err.to_string()))?;

    let out = combine::<Secp256k1>(points, t).map_err(|err| JsValue::from_str(&err.to_string()))?;

    JsValue::from_serde(&out).map_err(|err| JsValue::from_str(&err.to_string()))
}
