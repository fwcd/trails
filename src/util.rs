/// Some if the string is non-empty, otherwise None.
pub fn none_if_empty(s: &str) -> Option<&str> {
    Some(s).filter(|s| !s.is_empty())
}
