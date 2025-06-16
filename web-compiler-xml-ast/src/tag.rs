use std::hash::Hash;

/// An owned, normalized tag. Like `PathBuf`.
#[derive(Clone)]
pub struct TagBuf {
    original: String,
    normalized: String,
}

impl std::fmt::Debug for TagBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_original().fmt(f)
    }
}

impl TagBuf {
    /// Constructs a new TagBuf from any string-like input.
    pub fn new(tag: impl Into<String>) -> Self {
        let original = tag.into();
        let normalized = original.to_lowercase();
        Self { original, normalized }
    }

    /// Returns the original form of the tag.
    pub fn as_original(&self) -> &str {
        &self.original
    }

    /// Returns the normalized form of the tag.
    pub fn as_normalized(&self) -> &str {
        &self.normalized
    }
    pub fn matches(&self, other: &Self) -> bool {
        self.as_normalized() == other.as_normalized()
    }
}

impl std::fmt::Display for TagBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_original())
    }
}

// Conversions

impl From<String> for TagBuf {
    fn from(s: String) -> Self {
        TagBuf::new(s)
    }
}

impl From<&str> for TagBuf {
    fn from(s: &str) -> Self {
        TagBuf::new(s)
    }
}

impl Hash for TagBuf {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_original().hash(state);
    }
}

