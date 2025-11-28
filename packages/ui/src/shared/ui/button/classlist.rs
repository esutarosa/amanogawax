use super::variants::{ButtonSize, ButtonVariant};

/// Compose CSS class string for a button instance.
pub fn button_class(
    variant: ButtonVariant,
    size: ButtonSize,
    stretch: bool,
    has_pill_before: bool,
    has_pill_after: bool,
) -> String {
    let mut class = String::from("button");

    push_class(&mut class, variant.class_name());
    push_class(&mut class, size.class_name());

    if stretch {
        push_class(&mut class, "button--stretch");
    }

    if has_pill_before {
        push_class(&mut class, "button--pill-before");
    }

    if has_pill_after {
        push_class(&mut class, "button--pill-after");
    }

    class
}

fn push_class(class: &mut String, value: &str) {
    class.push(' ');
    class.push_str(value);
}
