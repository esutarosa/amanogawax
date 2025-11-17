use dioxus::prelude::*;

use super::variants::ContainerTag;

#[derive(Clone, Props, PartialEq)]
/// Properties for semantic layout containers.
pub struct ContainerProps {
    #[props(default = ContainerTag::Div)]
    /// Tag to render (defaults to `div`).
    pub r#as: ContainerTag,
    #[props(optional)]
    /// Additional CSS classes.
    pub class: Option<String>,
    /// Child nodes to render inside the container.
    pub children: Element,
}
