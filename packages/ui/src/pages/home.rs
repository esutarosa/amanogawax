use dioxus::prelude::*;

use crate::shared::ui::{
    button::Button,
    typography::{
        Typography, TypographySize, TypographyTag, TypographyTransform, TypographyWeight,
    },
};

#[component]
pub fn HomePage() -> Element {
    rsx! {
        main {
            class: "page",
            Typography {
                r#as: TypographyTag::H1,
                size: TypographySize::Xl5,
                weight: TypographyWeight::Regular,
                transform: TypographyTransform::Capitalize,
                "amanogawax"
            }
            Button {}
        }
    }
}
