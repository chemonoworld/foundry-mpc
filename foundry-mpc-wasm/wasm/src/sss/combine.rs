use gloo_utils::format::JsValueSerdeExt;
use k256::Secp256k1;
use p256::NistP256;
use sssui_mpc_rs::point::Point256;
use sssui_mpc_rs::sss::combine;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn combine_ec(points: JsValue, t: u32, curve: String) -> Result<JsValue, JsValue> {
    let points: Vec<Point256> = points
        .into_serde()
        .map_err(|err| JsValue::from_str(&err.to_string()))?;

    let combine_fn = match curve.as_str() {
        "secp256k1" => combine::<Secp256k1>,
        "secp256r1" => combine::<NistP256>,
        other => return Err(JsValue::from_str(&format!("Unsupported curve: {}", other))),
    };

    let secret = combine_fn(points, t).map_err(|err| JsValue::from_str(&err))?;

    JsValue::from_serde(&secret).map_err(|err| JsValue::from_str(&err.to_string()))
}
