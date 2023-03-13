use serde_json::Value;

pub trait Benchmark {

    fn get_name(&self) -> &'static str;

    fn run(&self) -> (Value, Value);

}