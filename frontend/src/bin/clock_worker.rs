use gloo_console::info;
use js_sys::{Array, Atomics, BigUint64Array, SharedArrayBuffer};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{DedicatedWorkerGlobalScope, MessageEvent};

fn main() {
    console_error_panic_hook::set_once();
    info!("Clock worker starting");

    let scope = DedicatedWorkerGlobalScope::from(JsValue::from(js_sys::global()));

    let onmessage = Closure::wrap(Box::new(move |msg: MessageEvent| {
        info!("Clock worker received shared array buffer");
        let buffer = SharedArrayBuffer::from(msg.data());
        let data = BigUint64Array::new(&buffer);
        Atomics::store_bigint(&data, 0, 0)
            .expect("clock should initialize");
        loop {
            let value = Atomics::add_bigint(&data, 0, 1)
                .expect("clock should increment");
            if value % 10000 == 0 {
                info!("Clock value: ", value)
            }
        }
    }) as Box<dyn Fn(MessageEvent)>);

    scope.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();

    // The worker must send a message to indicate that it's ready to receive messages.
    info!("Clock worker ready - send message to invoker");
    scope
        .post_message(&Array::new().into())
        .expect("posting ready message succeeds");
}
