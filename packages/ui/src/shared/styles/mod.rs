use dioxus::prelude::*;

pub fn reset_css() -> &'static str {
    include_str!("reset.css")
}

#[component]
pub fn GlobalStyles() -> Element {
    rsx! {
        style { "{reset_css()}" }
    }
}
