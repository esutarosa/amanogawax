use dioxus::prelude::*;

use crate::shared::ui::button::Button;

#[component]
pub fn HomePage() -> Element {
    rsx! {
        main {
            class: "page",
            h1 { "amanogawax" }
            Button {}
        }
    }
}
