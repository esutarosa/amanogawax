use super::props::TypographyProps;

pub fn typography_class(props: &TypographyProps) -> String {
    let mut class = String::from("typo");
    push_class(&mut class, props.size.class_name());
    push_class(&mut class, props.weight.class_name());

    if let Some(align) = props.align {
        push_class(&mut class, align.class_name());
    }

    if let Some(transform) = props.transform {
        push_class(&mut class, transform.class_name());
    }

    if let Some(additional) = props.class.as_deref() {
        push_class(&mut class, additional);
    }

    class
}

fn push_class(class: &mut String, value: &str) {
    class.push(' ');
    class.push_str(value);
}
