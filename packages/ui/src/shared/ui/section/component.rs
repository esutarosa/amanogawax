use dioxus::prelude::*;

use crate::shared::utils::classlist::merge_class;

use super::{props::SectionProps, styles, variants::SectionTag};

#[component]
/// Structural wrapper for page/grouped content.
pub fn Section(props: SectionProps) -> Element {
    let SectionProps {
        r#as,
        flush,
        class,
        children,
    } = props;

    let base_class = if flush {
        "section section--flush"
    } else {
        "section"
    };

    let class_name = merge_class(base_class, class);
    let node = render_by_tag(r#as, class_name, children);

    rsx! {
        style { "{styles::section_css()}" }
        {node}
    }
}

fn render_by_tag(tag: SectionTag, class: String, children: Element) -> Element {
    match tag {
        SectionTag::Section => rsx! { section { class: "{class}", {children} } },
        SectionTag::Article => rsx! { article { class: "{class}", {children} } },
        SectionTag::Div => rsx! { div { class: "{class}", {children} } },
        SectionTag::Main => rsx! { main { class: "{class}", {children} } },
        SectionTag::Header => rsx! { header { class: "{class}", {children} } },
        SectionTag::Footer => rsx! { footer { class: "{class}", {children} } },
    }
}
