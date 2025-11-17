use dioxus::prelude::*;

use crate::shared::utils::classlist::merge_class;

use super::{props::ContainerProps, styles, variants::ContainerTag};

#[component]
pub fn Container(props: ContainerProps) -> Element {
    let ContainerProps {
        r#as,
        class,
        children,
    } = props;

    let class_name = merge_class("container", class);
    let node = render_by_tag(r#as, class_name, children);

    rsx! {
        style { "{styles::container_css()}" }
        {node}
    }
}

fn render_by_tag(tag: ContainerTag, class: String, children: Element) -> Element {
    match tag {
        ContainerTag::Div => rsx! { div { class: "{class}", {children} } },
        ContainerTag::Section => rsx! { section { class: "{class}", {children} } },
        ContainerTag::Main => rsx! { main { class: "{class}", {children} } },
        ContainerTag::Article => rsx! { article { class: "{class}", {children} } },
        ContainerTag::Header => rsx! { header { class: "{class}", {children} } },
        ContainerTag::Footer => rsx! { footer { class: "{class}", {children} } },
    }
}
