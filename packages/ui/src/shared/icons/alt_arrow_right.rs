use dioxus::prelude::*;

use crate::shared::ui::icon::{IconConfig, IconProps, icon};

#[component]
pub fn AltArrowRight(props: IconProps) -> Element {
    icon(
        IconConfig {
            name: "alt_arrow_right",
            view_box: "0 0 24 24",
            width: 24,
            height: 24,
        },
        props,
        rsx! {
            path {
                d: "M9 5L15 12L9 19",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        },
    )
}
