use wasm_bindgen::prelude::*;
use serde_json::Value;

#[wasm_bindgen]
pub fn _parse(input: &str) -> JsValue {
    let v: Value = serde_json_wasm::from_str(input).expect("aa");
    JsValue::from_serde(&v).expect("cannot cerate from serde")
}

#[wasm_bindgen]
pub fn parse(input: &str) -> JsValue {
    let v: Value = serde_json::from_str(&input).expect("wrong input");
    JsValue::from_serde(&v).expect("cannot cerate from serde")
}
