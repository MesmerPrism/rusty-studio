pub(super) fn joined_list_or_none(values: &[String], separator: &str) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(separator)
    }
}
