use foundry_mpc_rs::sss::split;
use gloo_utils::format::JsValueSerdeExt;
use k256::Secp256k1;
use p256::NistP256;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn split_ec(
    secret: JsValue,
    point_xs: JsValue,
    t: u32,
    curve: String,
) -> Result<JsValue, JsValue> {
    let secret: [u8; 32] = secret
        .into_serde()
        .map_err(|err| JsValue::from_str(&err.to_string()))?;
    let point_xs: Vec<[u8; 32]> = point_xs
        .into_serde()
        .map_err(|err| JsValue::from_str(&err.to_string()))?;

    let split_fn = match curve.as_str() {
        "secp256k1" => split::<Secp256k1>,
        "secp256r1" => split::<NistP256>,
        other => return Err(JsValue::from_str(&format!("Unsupported curve: {}", other))),
    };

    let share_points = split_fn(secret, point_xs, t).map_err(|err| JsValue::from_str(&err))?;

    JsValue::from_serde(&share_points).map_err(|err| JsValue::from_str(&err.to_string()))
}
