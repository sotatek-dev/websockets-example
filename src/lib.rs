use wasm_bindgen::prelude::*;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn register(endpoint: &str, message: &str, callback: &js_sys::Function) -> Result<(), JsValue> {
    // Connect to an echo server
    let ws = WebSocket::new(endpoint)?;
    // For small binary messages, like CBOR, Arraybuffer is more efficient than Blob handling
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

    // create callback
    let cloned_ws = ws.clone();
    let callback = callback.clone();

    let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
            let array = js_sys::Uint8Array::new(&abuf);
            let buf = array.to_vec();
            let decoded_message: &str = rmp_serde::from_slice(&buf).unwrap();

            callback
                .call2(
                    &JsValue::null(),
                    &JsValue::null(), // error-first
                    &JsValue::from_str(decoded_message),
                )
                .unwrap();
        } else {
            console_log!("message event, received Unknown: {:?}", e.data());
        }

        // take only first message
        cloned_ws.close_with_code(1000).unwrap();
    });

    // set message event handler on WebSocket
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    // forget the callback to keep it alive
    onmessage_callback.forget();

    let onerror_callback = Closure::<dyn FnMut(_)>::new(move |e: ErrorEvent| {
        console_log!("error event: {:?}", e);
    });
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    let buf = rmp_serde::to_vec(&message).unwrap();
    let cloned_ws = ws.clone();
    let onopen_callback = Closure::<dyn FnMut()>::new(move || {
        // console_log!("socket opened");
        match cloned_ws.send_with_u8_array(&buf) {
            Ok(_) => {
                // console_log!("message successfully sent")
            }
            Err(err) => console_log!("error sending message: {:?}", err),
        }
    });
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    Ok(())
}
