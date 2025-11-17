use dioxus::prelude::*;

use crate::shared::utils::classlist::merge_class;

use super::props::IconProps;

#[derive(Clone, Copy)]
/// Static icon metadata used by the icon helper.
pub struct IconConfig {
    pub name: &'static str,
    pub view_box: &'static str,
    pub width: u32,
    pub height: u32,
}

/// Render an SVG icon with shared defaults and accessibility helpers.
pub fn icon(config: IconConfig, props: IconProps, body: Element) -> Element {
    let IconProps {
        size,
        width,
        height,
        color,
        class,
        title,
        title_id,
    } = props;

    let width = size.unwrap_or(width.unwrap_or(config.width));
    let height = size.unwrap_or(height.unwrap_or(config.height));
    let fill = color.unwrap_or_else(|| "currentColor".to_string());
    let class_name = merge_class("icon", class);
    let computed_title_id = title
        .as_ref()
        .map(|_| title_id.unwrap_or_else(|| format!("icon-{}-title", config.name)));

    rsx! {
        svg {
            class: "{class_name}",
            width: "{width}",
            height: "{height}",
            view_box: "{config.view_box}",
            fill: "{fill}",
            role: "img",
            if let (Some(title), Some(title_id)) = (title, computed_title_id) {
                title { id: "{title_id}", "{title}" }
            }
            {body}
        }
    }
}
