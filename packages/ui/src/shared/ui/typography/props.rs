use dioxus::prelude::*;

use super::variants::{
    TypographyAlign, TypographySize, TypographyTag, TypographyTransform, TypographyWeight,
};

#[derive(Clone, Props, PartialEq)]
/// Properties for the Typography component.
pub struct TypographyProps {
    #[props(default = TypographyTag::P)]
    /// HTML tag to render.
    pub r#as: TypographyTag,
    #[props(default = TypographySize::Md)]
    /// Type scale value.
    pub size: TypographySize,
    #[props(default = TypographyWeight::Regular)]
    /// Font weight preset.
    pub weight: TypographyWeight,
    #[props(optional)]
    /// Optional text alignment class.
    pub align: Option<TypographyAlign>,
    #[props(optional)]
    /// Optional text transform class.
    pub transform: Option<TypographyTransform>,
    #[props(optional)]
    /// Additional CSS classes.
    pub class: Option<String>,
    /// Text content.
    pub children: Element,
}
