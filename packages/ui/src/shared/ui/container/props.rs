use dioxus::prelude::*;

use super::variants::ContainerTag;

#[derive(Clone, Props, PartialEq)]
pub struct ContainerProps {
    #[props(default = ContainerTag::Div)]
    pub r#as: ContainerTag,
    #[props(optional)]
    pub class: Option<String>,
    pub children: Element,
}
