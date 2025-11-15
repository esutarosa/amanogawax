mod styles;

use dioxus::prelude::*;

#[component]
pub fn Button() -> Element {
    rsx! {
        style { "{styles::button_css()}" }
        button { class: "button" }
    }
}
