use dioxus::prelude::*;

use crate::shared::ui::icon::{IconConfig, IconProps, icon};

#[component]
pub fn Telegram(props: IconProps) -> Element {
    icon(
        IconConfig {
            name: "telegram",
            view_box: "0 0 24 24",
            width: 24,
            height: 24,
        },
        props,
        rsx! {
            path {
                d: "M21 5L2 12.5L9 13.5M21 5L18.5 20L9 13.5M21 5L9 13.5M9 13.5V19L12.2488 15.7229",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        },
    )
}
