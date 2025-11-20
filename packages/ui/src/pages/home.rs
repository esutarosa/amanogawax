use dioxus::prelude::*;

use crate::shared::{
    ui::{
        button::Button,
        container::Container,
        typography::{
            Typography, TypographySize, TypographyTag, TypographyTransform, TypographyWeight,
        },
    },
    widgets::layout::Layout,
};

#[component]
pub fn HomePage() -> Element {
    rsx! {
        Layout {
            Container {
                class: "page",
                div {
                    Typography {
                        r#as: TypographyTag::H1,
                        size: TypographySize::Xl5,
                        weight: TypographyWeight::Regular,
                        transform: TypographyTransform::Capitalize,
                        "AmanogawaX"
                    }
                }
                div {
                    Typography {
                        r#as: TypographyTag::P,
                        "UwU"
                    }
                    div {
                        Button {
                            "Click me"
                        }
                    }
                }
            }
        }
    }
}
