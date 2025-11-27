use dioxus::prelude::*;

use crate::shared::ui::icon::{IconConfig, IconProps, icon};

#[component]
pub fn Instagram(props: IconProps) -> Element {
    icon(
        IconConfig {
            name: "instagram",
            view_box: "0 0 24 24",
            width: 24,
            height: 24,
        },
        props,
        rsx! {
            path {
                d: "M12 16C14.2091 16 16 14.2091 16 12C16 9.79086 14.2091 8 12 8C9.79086 8 8 9.79086 8 12C8 14.2091 9.79086 16 12 16Z",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M3 16V8C3 5.23858 5.23858 3 8 3H16C18.7614 3 21 5.23858 21 8V16C21 18.7614 18.7614 21 16 21H8C5.23858 21 3 18.7614 3 16Z",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.5",
            }
            path {
                d: "M17.5 6.51001L17.51 6.4989",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        },
    )
}
