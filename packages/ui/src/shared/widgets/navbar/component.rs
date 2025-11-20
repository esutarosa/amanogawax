use dioxus::prelude::*;

use crate::shared::utils::classlist::merge_class;

use super::{props::NavbarProps, styles};

#[component]
pub fn Navbar(props: NavbarProps) -> Element {
    let NavbarProps {
        hide,
        class,
        children,
    } = props;

    let mut class_name = merge_class("navbar", class);
    if hide {
        class_name.push_str(" hidden");
    }
    let aria_hidden = hide.then_some("true");

    rsx! {
        style { "{styles::navbar_css()}" }
        nav {
            class: "{class_name}",
            aria_hidden: "{aria_hidden.unwrap_or(\"false\")}",
            {children}
        }
    }
}
