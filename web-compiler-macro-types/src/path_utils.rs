/// Returns true if a link is an external URL.
///
/// # Example
///
/// ```rust
/// use your_crate::path_utils::is_external_url;
///
/// assert!(is_external_url("https://example.com"));
/// assert!(is_external_url("//cdn.example.com/lib.css"));
/// assert!(is_external_url("mailto:hi@example.com"));
/// assert!(!is_external_url("pages/page1.html"));
/// ```
pub fn is_external_url(href: &str) -> bool {
    let lowered = href.trim().to_ascii_lowercase();
    lowered.starts_with("http://")
        || lowered.starts_with("https://")
        || lowered.starts_with("//")
        || lowered.starts_with("mailto:")
        || lowered.starts_with("tel:")
        || lowered.starts_with("#")
}

