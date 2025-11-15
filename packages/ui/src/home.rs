use dioxus::prelude::*;

#[component]
pub fn HomePage() -> Element {
    rsx! {
        main {
            class: "page",
            h1 { "amanogawax" }
        }
    }
}
