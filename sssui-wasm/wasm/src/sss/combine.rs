use gloo_utils::format::JsValueSerdeExt;
use k256::Secp256k1;
use p256::NistP256;
use sssui_rs::point::Point256;
use sssui_rs::sss::combine;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn combine_ec(points: JsValue, t: u32, curve: String) -> Result<JsValue, JsValue> {
    // TODO @Hyeong-soo
    // curve = "secp256k1" | "secp256r1" 둘 중 하나가 되어야 하고 나머지는 그냥 에러 던지기

    unimplemented!()
    // let points: Vec<Point256> = points
    //     .into_serde()
    //     .map_err(|err| JsValue::from_str(&err.to_string()))?;

    // let out = combine::<Secp256k1>(points, t).map_err(|err| JsValue::from_str(&err.to_string()))?;

    // JsValue::from_serde(&out).map_err(|err| JsValue::from_str(&err.to_string()))
}
