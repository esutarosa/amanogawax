use dioxus::prelude::*;

use crate::shared::utils::classlist::merge_class;

use super::{classlist::button_class, props::ButtonProps, styles};

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let ButtonProps {
        variant,
        size,
        before,
        after,
        stretch,
        class,
        children,
    } = props;

    let class_name = merge_class(&button_class(variant, size, stretch), class);

    rsx! {
        style { "{styles::button_css()}" }
        button { class: "{class_name}",
            if let Some(before) = before {
                span { class: "button__before", {before} }
            }
            span { class: "button__content", {children} }
            if let Some(after) = after {
                span { class: "button__after", {after} }
            }
        }
    }
}
