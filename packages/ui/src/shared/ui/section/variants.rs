#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Semantic tags supported by the `Section` component.
pub enum SectionTag {
    Section,
    Article,
    Div,
    Main,
    Header,
    Footer,
}
