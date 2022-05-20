extern crate regex;
use regex::Regex;
use perseus::{Html, Template, SsrNode, RenderFnResultWithCause};
use sycamore::{prelude::{View, view}, rt::Event};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::MessageEvent;

use crate::templates::ws::create_socket;

#[perseus::make_rx(IndexPageStateRx)]
pub struct IndexPageState {
  pub channel: String,
  pub messages: String,
  pub draft: String,
}

#[perseus::template_rx]
pub fn index_page(state: IndexPageStateRx) -> View<G> {
  let draft = state.draft.clone();
  let draft_reset = state.draft.clone();
  let messages = state.messages.clone();

  let ws = if G::IS_BROWSER {
    Some(create_socket())
  } else {
    messages.set("Loading...".to_string());
    None
  };

  let ws_clone = ws.clone();
  if let Some(Ok(ws)) = ws_clone {
    let cloned_messages = messages.clone();
    let cloned_channel = state.channel.clone();
    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
      if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
        if let Some(txt_string) = txt.as_string() {
          let re = Regex::new("^System: .+ joined (.+)$").unwrap();
          if let Some(x) = re.captures(&txt_string) {
            let channel_name = x.get(1).map_or("General", |m| m.as_str());
            cloned_channel.set(channel_name.to_string());
          }
          cloned_messages.set(format!("{}{}\n", cloned_messages.get(), txt_string));
        }
      }
    }) as Box<dyn FnMut(MessageEvent)>);
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();
    
    let cloned_ws = ws.clone();
    let onopen_callback = Closure::wrap(Box::new(move |_| {
      if cloned_ws.send_with_str("/help").is_err() {
        messages.set("Could not connect to server".to_string());
      }
    }) as Box<dyn FnMut(JsValue)>);
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();
  } else {
    messages.set("Could not connect to server".to_string());
  }

  let send = move |event: Event| {
    event.prevent_default();
    if let Some(Ok(ws)) = ws.clone() {
      let _ = ws.send_with_str(&draft_reset.get());
    }
    draft_reset.set("".to_string());
  };

  let channel = state.channel.clone();
  view! {
    div(class = "mx-auto", style = "max-width: 1200px;") {
      div(class = "border p-3 overflow-scroll", style = "height: 600px; max-height: 80vh;") {
        h2(class = "border-bottom") { (state.channel.get()) }
        pre { (state.messages.get()) }
      }
      form(on:submit=send) {
        div(class = "input-group mb-3") {
          input(bind:value = draft, type = "text", class = "form-control", placeholder = format!("Message {}", channel.get()))
          button(type = "submit", class = "btn btn-outline-secondary") { "Send" }
        }
      }
    }
  }
}

#[perseus::head]
pub fn head(_props: IndexPageState) -> View<SsrNode> {
  view! {
    title { "Index Page | Perseus Example - Basic" }
  }
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(
  _path: String,
  _locale: String,
) -> RenderFnResultWithCause<IndexPageState> {
  Ok(IndexPageState {
    channel: "General".to_string(),
    draft: "".to_string(),
    messages: "".to_string(),
  })
}

pub fn get_template<G: Html>() -> Template<G> {
  Template::new("index")
    .build_state_fn(get_build_state)
    .template(index_page)
    .head(head)
}