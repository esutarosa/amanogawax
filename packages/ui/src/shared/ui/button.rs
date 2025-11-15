use dioxus::prelude::*;

#[component]
pub fn Button() -> Element {
    rsx! {
        button { class: "button" }
    }
}
