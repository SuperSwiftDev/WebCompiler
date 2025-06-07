//! Path utility helpers: MIME type inference, external link checks, and glob resolution.
//!
//! Includes:
//! - `is_external_url()` for skipping absolute/remote links
//! - `MimeType` and `PathExt` for determining content type and file behavior
//! - `resolve_file_path_patterns()` to expand user-supplied globs
//! - `common_ancestor()` for computing shared parent directories
//!
//! Also includes platform-agnostic path resolution helpers (e.g. normalize, clean).

use std::path::{Path, PathBuf};

/// Returns true if a link is an external URL and should not be rewritten.
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

pub fn resolve_file_path_paterns(patterns: &[String]) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    fn resolve_entry_as_glob(pattern: &str) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        let mut results = Vec::<PathBuf>::new();
        for pattern in glob::glob(pattern)? {
            match pattern {
                Ok(path) => {
                    results.push(path);
                    continue;
                }
                Err(error) => return Err(Box::new(error)),
            }
        }
        Ok(results)
    }
    fn resolve_entry(pattern: &str) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        if let Ok(results) = resolve_entry_as_glob(pattern) {
            return Ok(results)
        }
        let path = PathBuf::from(pattern);
        return Ok(vec![path])
    }
    let mut results = Vec::<PathBuf>::new();
    for pattern in patterns {
        match resolve_entry(&pattern) {
            Ok(paths) => {
                results.extend(paths);
            }
            Err(error) => {
                return Err(error)
            }
        }
    }
    Ok(results)
}

/// Returns the common ancestor (shared prefix) of two paths, if any.
pub fn common_ancestor(p1: impl AsRef<std::path::Path>, p2: impl AsRef<std::path::Path>) -> Option<PathBuf> {
    use std::path::Component;
    let p1 = p1.as_ref();
    let p2 = p2.as_ref();
    /// Converts a `Component` to a string slice
    fn component_as_str<'a>(component: &'a Component) -> &'a std::ffi::OsStr {
        component.as_os_str()
    }
    let mut result = PathBuf::new();
    let mut p1_components = p1.components();
    let mut p2_components = p2.components();

    loop {
        match (p1_components.next(), p2_components.next()) {
            (Some(c1), Some(c2)) if c1 == c2 => result.push(component_as_str(&c1)),
            _ => break,
        }
    }

    if result.as_os_str().is_empty() {
        None
    } else {
        Some(result)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MimeType {
    // ───── Textual Content ─────
    /// `text/html`
    Html,
    /// `text/css`
    Css,
    /// `application/javascript`
    JavaScript,
    /// `application/json`
    Json,
    /// `application/xml`
    Xml,
    /// `text/plain`
    Text,

    // ───── Vector & Document Formats ─────
    /// `image/svg+xml`
    Svg,
    /// `application/pdf`
    Pdf,

    // ───── Raster Images ─────
    /// `image/png`
    Png,
    /// `image/jpeg`
    Jpeg,
    /// `image/gif`
    Gif,
    /// `image/webp`
    Webp,
    /// `image/avif`
    Avif,

    // ───── Video ─────
    /// `video/mp4`
    Mp4,
    /// `video/webm`
    Webm,

    // ───── Fonts ─────
    /// `font/woff`
    Woff,
    /// `font/woff2`
    Woff2,
    /// `font/ttf`
    Ttf,
    /// `font/otf`
    Otf,

    // ───── Binary or Unknown ─────
    /// `application/octet-stream`
    Binary,
    /// Custom or unknown MIME type, stored as raw string
    Unknown(String),
}

impl MimeType {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "html" | "htm" => MimeType::Html,
            "css" => MimeType::Css,
            "js" => MimeType::JavaScript,
            "json" => MimeType::Json,
            "xml" => MimeType::Xml,
            "svg" => MimeType::Svg,
            "png" => MimeType::Png,
            "jpg" | "jpeg" => MimeType::Jpeg,
            "gif" => MimeType::Gif,
            "webp" => MimeType::Webp,
            "avif" => MimeType::Avif,
            "mp4" => MimeType::Mp4,
            "webm" => MimeType::Webm,
            "woff" => MimeType::Woff,
            "woff2" => MimeType::Woff2,
            "ttf" => MimeType::Ttf,
            "otf" => MimeType::Otf,
            "pdf" => MimeType::Pdf,
            "txt" => MimeType::Text,
            "bin" => MimeType::Binary,
            other => MimeType::Unknown(other.to_string()),
        }
    }

    pub fn to_content_type(&self) -> &'static str {
        match self {
            MimeType::Html => "text/html",
            MimeType::Css => "text/css",
            MimeType::JavaScript => "application/javascript",
            MimeType::Json => "application/json",
            MimeType::Xml => "application/xml",
            MimeType::Svg => "image/svg+xml",
            MimeType::Png => "image/png",
            MimeType::Jpeg => "image/jpeg",
            MimeType::Gif => "image/gif",
            MimeType::Webp => "image/webp",
            MimeType::Avif => "image/avif",
            MimeType::Mp4 => "video/mp4",
            MimeType::Webm => "video/webm",
            MimeType::Woff => "font/woff",
            MimeType::Woff2 => "font/woff2",
            MimeType::Ttf => "font/ttf",
            MimeType::Otf => "font/otf",
            MimeType::Pdf => "application/pdf",
            MimeType::Text => "text/plain",
            MimeType::Binary => "application/octet-stream",
            MimeType::Unknown(_) => "application/octet-stream",
        }
    }

    pub fn from_path(path: &Path) -> Self {
        match path.extension().and_then(|s| s.to_str()) {
            Some(ext) => MimeType::from_extension(ext),
            None => MimeType::Binary,
        }
    }

    // ───── Utility Methods by Category ─────
    pub fn is_text(&self) -> bool {
        matches!(
            self,
            MimeType::Html
                | MimeType::Css
                | MimeType::JavaScript
                | MimeType::Json
                | MimeType::Xml
                | MimeType::Svg
                | MimeType::Text
        )
    }

    pub fn is_image(&self) -> bool {
        matches!(
            self,
            MimeType::Png | MimeType::Jpeg | MimeType::Gif | MimeType::Webp | MimeType::Avif | MimeType::Svg
        )
    }

    pub fn is_video(&self) -> bool {
        matches!(self, MimeType::Mp4 | MimeType::Webm)
    }

    pub fn is_font(&self) -> bool {
        matches!(self, MimeType::Woff | MimeType::Woff2 | MimeType::Ttf | MimeType::Otf)
    }

    pub fn is_binary(&self) -> bool {
        matches!(self, MimeType::Pdf | MimeType::Binary | MimeType::Unknown(_))
    }

    pub fn is_known(&self) -> bool {
        !matches!(self, MimeType::Unknown(_))
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum PathType {
    Dir,
    SymlinkDir,
    File,
    SymlinkFile,
}

impl PathType {
    pub fn from_path(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();
        path
            .symlink_metadata()
                .map(|meta| {
                    let is_symlink = meta.file_type().is_symlink();
                    let is_dir = path.is_dir();
                    match (is_symlink, is_dir) {
                        (true, true) => PathType::SymlinkDir,
                        (false, true) => PathType::Dir,
                        (true, false) => PathType::SymlinkFile,
                        (false, false) => PathType::File,
                    }
                })
                .unwrap_or(PathType::File)
    }
}

pub trait PathExt {
    /// Get a filename `&str` from a path.
    fn filename_str(&self) -> Option<&str>;
    /// Guess the MIME type from a path.
    fn mime_type(&self) -> MimeType;
    /// Determine whether the given path is a normal file/directory or a symlink.
    fn path_type(&self) -> PathType;
}

impl PathExt for Path {
    /// Get a filename `&str` from a path.
    fn filename_str(&self) -> Option<&str> {
        self.file_name().and_then(|s| s.to_str())
    }

    /// Guess MIME type from a path.
    fn mime_type(&self) -> MimeType {
        MimeType::from_path(self)
    }

    /// Determine given path is a normal file/directory or a symlink.
    fn path_type(&self) -> PathType {
        PathType::from_path(self)
    }
}

impl PathExt for PathBuf {
    /// Get a filename `&str` from a path.
    fn filename_str(&self) -> Option<&str> {
        self.file_name().and_then(|s| s.to_str())
    }

    /// Guess MIME type from a path.
    fn mime_type(&self) -> MimeType {
        MimeType::from_path(self)
    }

    /// Determine given path is a normal file/directory or a symlink.
    fn path_type(&self) -> PathType {
        PathType::from_path(self)
    }
}

/// Converts a filesystem path into a browser-safe relative URL.
/// Handles Windows `\` separators and cleans the path.
pub fn to_url_path(path: &Path) -> String {
    path_clean::clean(path)
        .to_string_lossy()
        .replace('\\', "/") // Required on Windows to prevent backslash pollution in HTML/CSS/JS
}

