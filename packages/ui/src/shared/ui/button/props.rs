use dioxus::prelude::*;

use super::variants::{ButtonSize, ButtonVariant};

#[derive(Clone, Props, PartialEq)]
/// Properties for the shared button component.
pub struct ButtonProps {
    #[props(default = ButtonVariant::Default)]
    /// Visual styling preset.
    pub variant: ButtonVariant,
    #[props(default = ButtonSize::Default)]
    /// Size preset (`Icon` removes padding around content).
    pub size: ButtonSize,
    #[props(optional)]
    /// Optional content rendered before the main label (e.g., icon).
    pub before: Option<Element>,
    #[props(optional)]
    /// Optional content rendered after the main label.
    pub after: Option<Element>,
    #[props(default = false)]
    /// Stretch to full width of the parent.
    pub stretch: bool,
    #[props(optional)]
    /// Additional CSS classes.
    pub class: Option<String>,
    /// Main button content.
    pub children: Element,
}
