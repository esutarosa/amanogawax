use super::variants::{ButtonSize, ButtonVariant};

pub fn button_class(variant: ButtonVariant, size: ButtonSize, stretch: bool) -> String {
    let mut class = String::from("button");

    push_class(&mut class, variant.class_name());
    push_class(&mut class, size.class_name());

    if stretch {
        push_class(&mut class, "button--stretch");
    }

    class
}

fn push_class(class: &mut String, value: &str) {
    class.push(' ');
    class.push_str(value);
}
