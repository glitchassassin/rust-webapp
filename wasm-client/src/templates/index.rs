use perseus::{Html, Template, SsrNode, RenderFnResultWithCause};
use sycamore::prelude::{View, view};

#[perseus::make_rx(IndexPageStateRx)]
pub struct IndexPageState {
  pub greeting: String,
}

#[perseus::template_rx]
pub fn index_page(state: IndexPageStateRx) -> View<G> {
  view! {
    p { (state.greeting.get()) }
    a(href = "about", id = "about-link") { "About!" }
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
    greeting: "Hello World!".to_string(),
  })
}

pub fn get_template<G: Html>() -> Template<G> {
  Template::new("index")
    .build_state_fn(get_build_state)
    .template(index_page)
    .head(head)
}