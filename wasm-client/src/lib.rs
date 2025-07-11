use wasm_bindgen::prelude::*;
use web_sys::{WebSocket, MessageEvent, ErrorEvent, CloseEvent};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {

    // Connect to the server
    let ws = WebSocket::new("ws://localhost:12345")?;

    // Set onopen callback — send a message when connection opens
    let onopen = Closure::wrap(Box::new({
        let ws = ws.clone();
        move || {
            ws.send_with_str("Hello from Rust + Wasm!").unwrap();
            web_sys::console::log_1(&"Message sent!".into());
        }
    }) as Box<dyn FnMut()>);
    ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
    onopen.forget(); // Prevent closure from being dropped

    // Optional: Log any message received
    let onmessage = Closure::wrap(Box::new(move |e: MessageEvent| {
        if let Some(txt) = e.data().as_string() {
            web_sys::console::log_1(&format!("Received: {txt}").into());
        }
    }) as Box<dyn FnMut(MessageEvent)>);
    ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();

    // Optional: Log errors
    let onerror = Closure::wrap(Box::new(move |e: ErrorEvent| {
        web_sys::console::error_1(&format!("Error: {:?}", e).into());
    }) as Box<dyn FnMut(ErrorEvent)>);
    ws.set_onerror(Some(onerror.as_ref().unchecked_ref()));
    onerror.forget();

    // Optional: Log when connection closes
    let onclose = Closure::wrap(Box::new(move |_: CloseEvent| {
        web_sys::console::log_1(&"Connection closed.".into());
    }) as Box<dyn FnMut(CloseEvent)>);
    ws.set_onclose(Some(onclose.as_ref().unchecked_ref()));
    onclose.forget();

    Ok(())
}
