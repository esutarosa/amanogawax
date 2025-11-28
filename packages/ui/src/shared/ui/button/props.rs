use dioxus::prelude::*;

use super::variants::{AdornmentKind, ButtonSize, ButtonVariant};

#[derive(Clone, Props, PartialEq)]
/// Properties for the shared button component.
pub struct ButtonProps {
    #[props(default = ButtonVariant::Primary)]
    /// Visual styling preset.
    pub variant: ButtonVariant,
    #[props(default = ButtonSize::Md)]
    /// Size preset (`Icon` tightens spacing to a square wrapper).
    pub size: ButtonSize,
    #[props(optional)]
    /// Optional content rendered before the main label (e.g., icon).
    pub before: Option<Element>,
    #[props(optional)]
    /// Optional content rendered after the main label.
    pub after: Option<Element>,
    #[props(default = AdornmentKind::Plain)]
    /// Optional styling for the `before` slot (e.g., pill for icons).
    pub before_kind: AdornmentKind,
    #[props(default = AdornmentKind::Plain)]
    /// Optional styling for the `after` slot (e.g., pill for icons).
    pub after_kind: AdornmentKind,
    #[props(default = false)]
    /// Stretch to full width of the parent.
    pub stretch: bool,
    #[props(optional)]
    /// Accessible label for icon-only buttons.
    pub aria_label: Option<String>,
    #[props(optional)]
    /// Additional CSS classes.
    pub class: Option<String>,
    /// Main button content.
    pub children: Element,
}
