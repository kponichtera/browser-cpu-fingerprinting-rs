use serde_json::{json, value::Value};

pub fn dummy_benchmark() -> (String, Value, Value) {
    (String::from("dummy"), json!(null), json!(null))
}
