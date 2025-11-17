use dioxus::prelude::*;

use super::{classlist::typography_class, props::TypographyProps, styles, variants::TypographyTag};

#[component]
pub fn Typography(props: TypographyProps) -> Element {
    let class = typography_class(&props);
    let TypographyProps { r#as, children, .. } = props;

    let node = render_by_tag(r#as, class, children);

    rsx! {
        style { "{styles::typography_css()}" }
        {node}
    }
}

fn render_by_tag(tag: TypographyTag, class: String, children: Element) -> Element {
    match tag {
        TypographyTag::P => rsx! { p { class: "{class}", {children} } },
        TypographyTag::Span => rsx! { span { class: "{class}", {children} } },
        TypographyTag::Div => rsx! { div { class: "{class}", {children} } },
        TypographyTag::H1 => rsx! { h1 { class: "{class}", {children} } },
        TypographyTag::H2 => rsx! { h2 { class: "{class}", {children} } },
        TypographyTag::H3 => rsx! { h3 { class: "{class}", {children} } },
        TypographyTag::H4 => rsx! { h4 { class: "{class}", {children} } },
        TypographyTag::H5 => rsx! { h5 { class: "{class}", {children} } },
        TypographyTag::H6 => rsx! { h6 { class: "{class}", {children} } },
    }
}
