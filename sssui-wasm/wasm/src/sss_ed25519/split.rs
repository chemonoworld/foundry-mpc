use gloo_utils::format::JsValueSerdeExt;
use sssui_mpc_rs::sss_ed25519::sss_split_ed25519;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn split_ed25519(secret: JsValue, point_xs: JsValue, t: u32) -> Result<JsValue, JsValue> {
    let secret: [u8; 32] = secret
        .into_serde()
        .map_err(|err| JsValue::from_str(&err.to_string()))?;
    let point_xs: Vec<[u8; 32]> = point_xs
        .into_serde()
        .map_err(|err| JsValue::from_str(&err.to_string()))?;

    let share_points = sss_split_ed25519(secret, point_xs, t)
        .map_err(|err| JsValue::from_str(&err.to_string()))?;

    JsValue::from_serde(&share_points).map_err(|err| JsValue::from_str(&err.to_string()))
}
