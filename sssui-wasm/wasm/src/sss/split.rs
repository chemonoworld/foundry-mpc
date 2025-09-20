use gloo_utils::format::JsValueSerdeExt;
use k256::Secp256k1;
use p256::NistP256;
use sssui_rs::sss::split;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn split_ec(
    secret: JsValue,
    // ks_node_hashes: JsValue, -> point_xs: JsValue
    point_xs: JsValue,
    t: u32,
    curve: String,
) -> Result<JsValue, JsValue> {
    // TODO @Hyeong-soo
    // curve = "secp256k1" | "secp256r1" 둘 중 하나가 되어야 하고 나머지는 그냥 에러 던지기

    unimplemented!()
    // let secret: [u8; 32] = secret
    //     .into_serde()
    //     .map_err(|err| JsValue::from_str(&err.to_string()))?;
    // let ks_node_hashes: Vec<[u8; 32]> = ks_node_hashes
    //     .into_serde()
    //     .map_err(|err| JsValue::from_str(&err.to_string()))?;

    // let out = split::<Secp256k1>(secret, ks_node_hashes, t)
    //     .map_err(|err| JsValue::from_str(&err.to_string()))?;

    // JsValue::from_serde(&out).map_err(|err| JsValue::from_str(&err.to_string()))
}
