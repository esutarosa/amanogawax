use dioxus::prelude::*;

#[derive(Clone, Props, PartialEq)]
pub struct IconProps {
    #[props(optional)]
    pub size: Option<u32>,
    #[props(optional)]
    pub width: Option<u32>,
    #[props(optional)]
    pub height: Option<u32>,
    #[props(optional)]
    pub color: Option<String>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub title: Option<String>,
    #[props(optional)]
    pub title_id: Option<String>,
}
