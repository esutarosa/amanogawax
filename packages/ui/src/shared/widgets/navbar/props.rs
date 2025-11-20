use dioxus::prelude::*;

#[derive(Clone, Props, PartialEq)]
/// Minimal props for the Navbar scaffold; extend when implementing.
pub struct NavbarProps {
    #[props(default = false)]
    /// Hide the navbar when `true`.
    pub hide: bool,
    #[props(optional)]
    /// Additional CSS classes for custom styling.
    pub class: Option<String>,
    /// Slot for navigation content.
    pub children: Element,
}
