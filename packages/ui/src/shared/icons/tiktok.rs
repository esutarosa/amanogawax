use dioxus::prelude::*;

use crate::shared::ui::icon::{IconConfig, IconProps, icon};

#[component]
pub fn TikTok(props: IconProps) -> Element {
    icon(
        IconConfig {
            name: "tiktok",
            view_box: "0 0 24 24",
            width: 24,
            height: 24,
        },
        props,
        rsx! {
            path {
                d: "M21 8V16C21 18.7614 18.7614 21 16 21H8C5.23858 21 3 18.7614 3 16V8C3 5.23858 5.23858 3 8 3H16C18.7614 3 21 5.23858 21 8Z",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            },
            path {
                d: "M10 12C8.34315 12 7 13.3431 7 15C7 16.6569 8.34315 18 10 18C11.6569 18 13 16.6569 13 15V6C13.3333 7 14.6 9 17 9",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }

        },
    )
}
