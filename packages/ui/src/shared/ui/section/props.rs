use dioxus::prelude::*;

use super::variants::SectionTag;

#[derive(Clone, Props, PartialEq)]
/// Properties for composable page sections.
pub struct SectionProps {
    #[props(default = SectionTag::Section)]
    /// Semantic tag to render (defaults to `section`).
    pub r#as: SectionTag,
    #[props(optional)]
    /// Extra CSS classes applied alongside the base section styles.
    pub class: Option<String>,
    /// Nested nodes to render inside the section.
    pub children: Element,
}
