use dioxus::prelude::*;

use super::variants::{
    TypographyAlign, TypographySize, TypographyTag, TypographyTransform, TypographyWeight,
};

#[derive(Clone, Props, PartialEq)]
pub struct TypographyProps {
    #[props(default = TypographyTag::P)]
    pub r#as: TypographyTag,
    #[props(default = TypographySize::Md)]
    pub size: TypographySize,
    #[props(default = TypographyWeight::Regular)]
    pub weight: TypographyWeight,
    #[props(optional)]
    pub align: Option<TypographyAlign>,
    #[props(optional)]
    pub transform: Option<TypographyTransform>,
    #[props(optional)]
    pub class: Option<String>,
    pub children: Element,
}
