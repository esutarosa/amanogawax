mod styles;

use dioxus::prelude::*;

#[derive(Clone, Props, PartialEq)]
pub struct ButtonProps {
    pub children: Element,
}

#[component]
pub fn Button(ButtonProps { children }: ButtonProps) -> Element {
    rsx! {
        style { "{styles::button_css()}" }
        button { class: "button", {children} }
    }
}
