use dioxus::prelude::*;

use crate::shared::utils::classlist::merge_class;

use super::{classlist::button_class, props::ButtonProps, styles, variants::AdornmentKind};

#[component]
/// Pressable control with optional before/after slots and visual variants.
pub fn Button(props: ButtonProps) -> Element {
    let ButtonProps {
        variant,
        size,
        before,
        after,
        before_kind,
        after_kind,
        stretch,
        aria_label,
        class,
        children,
    } = props;

    let aria_label = aria_label.unwrap_or_default();

    let class_name = merge_class(
        &button_class(
            variant,
            size,
            stretch,
            matches!(before_kind, AdornmentKind::Pill),
            matches!(after_kind, AdornmentKind::Pill),
        ),
        class,
    );
    let before_class = match before_kind {
        AdornmentKind::Plain => "button__before",
        AdornmentKind::Pill => "button__before button__adornment--pill",
    };
    let after_class = match after_kind {
        AdornmentKind::Plain => "button__after",
        AdornmentKind::Pill => "button__after button__adornment--pill",
    };

    rsx! {
        style { "{styles::button_css()}" }
        button { class: "{class_name}",
            aria_label: "{aria_label}",
            if let Some(before) = before {
                span { class: "{before_class}", {before} }
            }
            span { class: "button__content", {children} }
            if let Some(after) = after {
                span { class: "{after_class}", {after} }
            }
        }
    }
}
