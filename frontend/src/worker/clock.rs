use gloo_console::info;
use js_sys::{Array, Atomics, BigInt64Array, BigUint64Array, Int32Array, SharedArrayBuffer};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use web_sys::{Blob, BlobPropertyBag, MessageEvent, Url, Worker};
use web_sys::console::info;

pub fn start_clock_worker() -> Result<(Worker, BigUint64Array), JsValue> {
    // let origin = window()
    //     .expect("window to be available")
    //     .location()
    //     .origin()
    //     .expect("origin to be available");

    // TODO: Figure out how to pass the origin (equivalent of http://127.0.0.1:9000),
    //  probably by sending message to the invoking benchmark worker
    let script = Array::new();
    script.push(
        &format!(r#"importScripts("http://127.0.0.1:9000/clock_worker.js");wasm_bindgen("http://127.0.0.1:9000/clock_worker_bg.wasm");"#)
            .into()
    );

    let blob = Blob::new_with_str_sequence_and_options(
        &script,
        BlobPropertyBag::new().type_("text/javascript"),
    )?;
    let url = Url::create_object_url_with_blob(&blob)?;

    // Instantiate the worker
    info!("Instantiating worker");
    let worker = Worker::new(&url)?;
    let shared_buffer = SharedArrayBuffer::new(std::mem::size_of::<u64>() as u32);
    let buffer_data = BigUint64Array::new(&shared_buffer);

    // Handle the message from the worker, which means that it was successfully loaded
    // and send the shared array buffer to it
    let shared_buffer_clone = shared_buffer.clone();
    let worker_clone = worker.clone();
    let onmessage = Closure::wrap(Box::new(move |msg: MessageEvent| {
        let shared_buffer_clone = shared_buffer_clone.clone();
        let worker_clone = worker_clone.clone();
        info!("Sending buffer to the clock worker");
        worker_clone.post_message(&JsValue::from(shared_buffer_clone))
            .expect("worker is having shared array buffer sent to it.");
    }) as Box<dyn Fn(MessageEvent)>);
    worker.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();

    // (Busy) wait for the shared buffer to become non-zero,
    // which means that clock worker started to do its job
    // info!("Waiting for clock to start...");
    // loop {
    //     let clock_state = Atomics::add_bigint(&buffer_data, 0, 0)?;
    //     if clock_state != 0 {
    //         break;
    //     }
    // }
    // info!("Clock is working");

    Ok((worker, buffer_data))
}