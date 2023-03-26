use js_sys::Object;
use web_sys::{DedicatedWorkerGlobalScope, Performance};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

pub mod page_size;

fn get_performance() -> Performance {
    // js_sys::global().dyn_into::<Performance>().unwrap()
    Performance::from(JsValue::from(js_sys::global()))
}
