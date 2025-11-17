use dioxus::prelude::*;

use super::variants::{ButtonSize, ButtonVariant};

#[derive(Clone, Props, PartialEq)]
pub struct ButtonProps {
    #[props(default = ButtonVariant::Default)]
    pub variant: ButtonVariant,
    #[props(default = ButtonSize::Default)]
    pub size: ButtonSize,
    #[props(optional)]
    pub before: Option<Element>,
    #[props(optional)]
    pub after: Option<Element>,
    #[props(default = false)]
    pub stretch: bool,
    #[props(optional)]
    pub class: Option<String>,
    pub children: Element,
}
