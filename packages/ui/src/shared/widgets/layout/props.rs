use dioxus::prelude::*;

#[derive(Clone, Props, PartialEq)]
/// Properties for the `Layout` wrapper.
pub struct LayoutProps {
    #[props(optional)]
    /// Additional classes for the root wrapper.
    pub class: Option<String>,
    /// Slot for the page-specific content rendered inside `<main>`.
    pub children: Element,
}
