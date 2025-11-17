/// Append optional space-delimited class string to a base class.
pub fn merge_class(base: &str, extra: Option<String>) -> String {
    match extra {
        Some(extra) => {
            let trimmed = extra.trim();
            if trimmed.is_empty() {
                base.to_string()
            } else {
                format!("{base} {trimmed}")
            }
        }
        None => base.to_string(),
    }
}
