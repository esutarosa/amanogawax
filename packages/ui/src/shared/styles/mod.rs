use dioxus::prelude::*;

pub fn reset_css() -> &'static str {
    include_str!("reset.css")
}

pub fn spacing_css() -> &'static str {
    include_str!("spacing.css")
}

#[component]
pub fn GlobalStyles() -> Element {
    rsx! {
        style { "{reset_css()}" }
        style { "{spacing_css()}" }
    }
}
