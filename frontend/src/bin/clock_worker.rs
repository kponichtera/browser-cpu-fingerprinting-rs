use gloo_console::info;
use js_sys::{Array, Atomics, BigUint64Array, JsString, SharedArrayBuffer};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{DedicatedWorkerGlobalScope, MessageEvent};
use frontend::clock::{Clock, CLOCK_MESSAGE_READY, CLOCK_MESSAGE_STARTED};

fn main() {
    console_error_panic_hook::set_once();
    info!("Clock worker starting");

    let scope = DedicatedWorkerGlobalScope::from(JsValue::from(js_sys::global()));

    let scope_clone = scope.clone();
    let onmessage = Closure::wrap(Box::new(move |msg: MessageEvent| {
        info!("Clock worker received shared array buffer");
        let buffer = SharedArrayBuffer::from(msg.data());
        let clock = Clock::from(buffer);

        scope_clone
            .post_message(&JsString::from(CLOCK_MESSAGE_STARTED))
            .expect("posting started message succeeds");

        loop {
            let value = clock.increment().unwrap();
            // if value % 100000 == 0 {
            //     info!("Clock value: ", value)
            // }
        }
    }) as Box<dyn Fn(MessageEvent)>);

    scope.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();

    // The worker must send a message to indicate that it's ready to receive messages.
    info!("Clock worker ready - send message to invoker");
    scope
        .post_message(&JsString::from(CLOCK_MESSAGE_READY))
        .expect("posting ready message succeeds");
}
