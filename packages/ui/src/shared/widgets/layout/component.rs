use dioxus::prelude::*;

use crate::shared::{
    icons::Logo, ui::container::Container, utils::classlist::merge_class, widgets::navbar::Navbar,
};

use super::{props::LayoutProps, styles};

#[component]
/// Top-level scaffold that injects global navigation and wraps page content.
pub fn Layout(props: LayoutProps) -> Element {
    let LayoutProps { class, children } = props;
    let class_name = merge_class("layout", class);

    rsx! {
        style { "{styles::layout_css()}" }
        div {
            class: "{class_name}",
            Navbar {
                Container {
                    div {
                        Logo {}
                    }
                }
            }
            main {
                class: "layout__main",
                {children}
            }
        }
    }
}
