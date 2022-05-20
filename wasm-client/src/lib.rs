mod error_pages;
mod templates;

use perseus::{Html, PerseusApp, PerseusRoot};

#[perseus::main]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template)
        .error_pages(crate::error_pages::get_error_pages)
        .index_view(|| {
            sycamore::view! {
                head {
                    link(
                        rel = "stylesheet", 
                        href = "https://cdn.jsdelivr.net/npm/bootstrap@4.0.0/dist/css/bootstrap.min.css", 
                        integrity = "sha384-Gn5384xqQ1aoWXA+058RXPxPg6fy4IWvTNh0E263XmFcJlSAwiGgFAW/dAiS6JXm", 
                        crossorigin = "anonymous"
                    )
                }
                body {
                    PerseusRoot()
                    script(
                        src = "https://code.jquery.com/jquery-3.2.1.slim.min.js",
                        integrity = "sha384-KJ3o2DKtIkvYIK3UENzmM7KCkRr/rE9/Qpg6aAZGJwFDMVNA/GpGFF93hXpG5KkN",
                        crossorigin = "anonymous"
                    )
                    script(
                        src = "https://cdn.jsdelivr.net/npm/popper.js@1.12.9/dist/umd/popper.min.js",
                        integrity = "sha384-ApNbgh9B+Y1QKtv3Rn7W3mgPxhU9K/ScQsAP7hUibX39j7fakFPskvXusvfa0b4Q",
                        crossorigin = "anonymous"
                    )
                    script(
                        src = "https://cdn.jsdelivr.net/npm/bootstrap@4.0.0/dist/js/bootstrap.min.js",
                        integrity = "sha384-JZR6Spejh4U02d8jOt6vLEHfe/JQGiRRSQQxSfFWpi1MquVdAyjUar5+76PVCmYl",
                        crossorigin = "anonymous"
                    )
                }
            }
        })
}