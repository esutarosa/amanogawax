use dioxus::prelude::*;

use crate::shared::{
    icons::Play,
    ui::{
        button::{AdornmentKind, Button},
        container::Container,
    },
    widgets::layout::Layout,
};

#[component]
pub fn HomePage() -> Element {
    rsx! {
        Layout {
            section { class: "hero",
                Container {
                    class: "page",
                    style { "
                        .hero {{
                            padding-block: var(--space-64);
                        }}

                        .hero .container {{
                            padding: var(--space-32);
                        }}

                        .button-demo__group {{
                            display: flex;
                            flex-direction: column;
                            gap: var(--space-16);
                        }}
                    " }
                    div { class: "button-demo__group",
                        Button {
                            after: rsx!(Play {}),
                            after_kind: AdornmentKind::Pill,
                            "Дивитися"
                        }
                    }
                }
            }
        }
    }
}
