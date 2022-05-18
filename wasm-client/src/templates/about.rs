use perseus::Template;
use sycamore::prelude::{view, SsrNode, Html, View};

#[perseus::template_rx]
pub fn about_page() -> View<G> {
  view! {
    p { "About." }
    a(href = "/", id = "index-link") { "Go Home!" }
  }
}

#[perseus::head]
pub fn head() -> View<SsrNode> {
  view! {
    title { "About Page | Perseus Example - Basic" }
  }
}

pub fn get_template<G: Html>() -> Template<G> {
  Template::new("about").template(about_page).head(head)
}