pub(super) fn optional_dotted_id(value: Option<&str>) -> bool {
    match value {
        Some(value) => is_dotted_id(value),
        None => true,
    }
}

pub(super) fn all_dotted_ids(values: &[String]) -> bool {
    values.iter().all(|value| is_dotted_id(value))
}

pub fn is_dotted_id(value: &str) -> bool {
    let mut chars = value.chars().peekable();
    let mut previous_dot = true;
    let mut saw_segment_char = false;
    while let Some(ch) = chars.next() {
        let is_segment_char =
            ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_' || ch == '-';
        if ch == '.' {
            if previous_dot || !saw_segment_char || chars.peek().is_none() {
                return false;
            }
            previous_dot = true;
            saw_segment_char = false;
            continue;
        }
        if !is_segment_char {
            return false;
        }
        previous_dot = false;
        saw_segment_char = true;
    }
    !previous_dot && saw_segment_char
}
