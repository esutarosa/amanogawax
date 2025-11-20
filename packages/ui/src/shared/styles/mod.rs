use dioxus::prelude::*;

pub fn reset_css() -> &'static str {
    include_str!("reset.css")
}

pub fn spacing_css() -> &'static str {
    include_str!("spacing.css")
}

pub fn utils_css() -> &'static str {
    include_str!("utils.css")
}

#[component]
pub fn GlobalStyles() -> Element {
    rsx! {
        style { "{reset_css()}" }
        style { "{spacing_css()}" }
        style { "{utils_css()}" }
    }
}
