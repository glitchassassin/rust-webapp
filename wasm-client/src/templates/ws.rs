use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebSocket, ErrorEvent};

macro_rules! console_log {
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn create_socket() -> Result<WebSocket, JsValue> {
  let ws = WebSocket::new("ws://127.0.0.1:8000/ws")?;
  
  let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
    console_log!("error event: {:?}", e);
  }) as Box<dyn FnMut(ErrorEvent)>);
  ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
  onerror_callback.forget();
  
  Ok(ws)
}
