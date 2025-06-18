use serde::{Deserialize, Serialize};
use serde_json::Value;
use xml_ast::{AttributeMap, Fragment, Node, TagBuf};

pub fn compile_report(node: &Node) -> Option<HeadMetadata> {
    let head_tag = TagBuf::from("head");
    let head = node.find_first(&head_tag)?;
    let head = head.as_element()?;
    let head_metadata = HeadMetadata::from_head_element_children(&head.children);
    Some(head_metadata)
}

// ————————————————————————————————————————————————————————————————————————————
// DATA MODEL - BASICS
// ————————————————————————————————————————————————————————————————————————————

/// Represents all metadata extracted from the <head> of an HTML document.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HeadMetadata {
    /// The content of the `<title>` tag, used by browsers and search engines.
    ///
    /// Example: `<title>ContractorName | Roofing & Siding Experts</title>`
    pub title: Option<String>,

    /// `<base href="...">` for resolving relative URLs
    ///
    /// Example: <base href="https://example.com/docs/">
    pub base_url: Option<String>,

    /// Grouped `<meta>` tags
    /// 
    /// A structured collection of `<meta>` tag contents, grouped by function.
    pub meta: MetaTags,

    /// Metadata-oriented `<link>` tags
    /// 
    /// Relevant `<link>` tags that describe metadata (e.g. canonical, icons, preconnect).
    pub links: Vec<LinkTag>,

    /// `<script type="application/ld+json">` blocks
    /// 
    /// All `<script type="application/ld+json">` blocks, preserving both raw and parsed content.
    pub json_ld: Vec<JsonLdBlock>,
}

/// Structured groupings of <meta> tag contents based on semantic purpose.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MetaTags {
    /// SEO-related metadata: description, robots, canonical, etc.
    pub seo: SeoMetadata,

    /// Metadata related to mobile UX and responsiveness.
    pub mobile: MobileMetadata,

    /// Metadata used by social media platforms (Open Graph, Twitter Cards).
    pub social: SocialMetadata,

    /// Miscellaneous or rarely-used metadata fields.
    pub misc: MiscMetadata,
}

/// Standard metadata for search engine optimization.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SeoMetadata {
    /// `<meta name="description">` — the page's search snippet description.
    ///
    /// Example: `<meta name="description" content="Utah's best siding and roofing contractor.">`
    pub description: Option<String>,

    /// `<meta name="robots">` — instructions to search engine crawlers (e.g., "index, follow").
    ///
    /// Example: `<meta name="robots" content="noindex, nofollow">`
    pub robots: Option<String>,

    /// `<meta name="keywords">` — comma-separated keywords (mostly obsolete).
    ///
    /// Example: `<meta name="keywords" content="roofing, siding, rain gutters, Utah">`
    pub keywords: Option<String>,

    /// `<meta name="author">` — identifies the content author.
    ///
    /// Example: `<meta name="author" content="Colbyn Web Studio">`
    pub author: Option<String>,

    /// `<meta name="generator">` — identifies the tool used to generate the page.
    ///
    /// Example: `<meta name="generator" content="WebCompiler 0.6.0">`
    pub generator: Option<String>,

    /// `<link rel="canonical">` — preferred URL to avoid duplicate content issues.
    ///
    /// Example: `<link rel="canonical" href="https://example.com/services/roofing">`
    pub canonical_url: Option<String>,
}

/// Metadata related to mobile devices and progressive web apps.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MobileMetadata {
    /// `<meta name="viewport">` — configures viewport scaling and dimensions.
    ///
    /// Example: `<meta name="viewport" content="width=device-width, initial-scale=1">`
    pub viewport: Option<String>,

    /// `<meta name="theme-color">` — sets the browser toolbar color on mobile.
    ///
    /// Example: `<meta name="theme-color" content="#004aad">`
    pub theme_color: Option<String>,

    /// `<meta name="mobile-web-app-capable">` — signals if a site can run as a full-screen app (Android).
    ///
    /// Example: `<meta name="mobile-web-app-capable" content="yes">`
    pub mobile_web_app_capable: Option<bool>,

    /// `<meta name="apple-mobile-web-app-capable">` — same as above, but for iOS devices.
    ///
    /// Example: `<meta name="apple-mobile-web-app-capable" content="yes">`
    pub apple_mobile_web_app_capable: Option<bool>,
}

/// Metadata for social sharing platforms (Open Graph + Twitter Cards).
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SocialMetadata {
    /// Open Graph metadata (used by Facebook, LinkedIn, Slack, etc.).
    pub og: OgMetadata,

    /// Twitter Card metadata (used by X/Twitter).
    pub twitter: TwitterMetadata,
}

/// Open Graph metadata fields (Facebook, LinkedIn, Discord, etc.).
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct OgMetadata {
    /// `<meta property="og:title">` — title used when sharing on social platforms.
    ///
    /// Example: `<meta property="og:title" content="Utah’s #1 Roofing Contractor">`
    pub title: Option<String>,

    /// `<meta property="og:description">` — summary shown below the title.
    ///
    /// Example: `<meta property="og:description" content="Family-owned, 5-star rated. Serving Utah since 2001.">`
    pub description: Option<String>,

    /// `<meta property="og:url">` — canonical or shareable URL.
    ///
    /// Example: `<meta property="og:url" content="https://usasuperior.com/roofing">`
    pub url: Option<String>,

    /// `<meta property="og:site_name">` — the name of the site or brand.
    ///
    /// Example: `<meta property="og:site_name" content="USA Superior Construction">`
    pub site_name: Option<String>,

    /// `<meta property="og:type">` — object type (e.g., "website", "article").
    ///
    /// Example: `<meta property="og:type" content="website">`
    pub r#type: Option<String>,

    /// `<meta property="og:locale">` — regional language code (e.g., "en_US").
    ///
    /// Example: `<meta property="og:locale" content="en_US">`
    pub locale: Option<String>,

    /// `<meta property="og:image">` — representative image to display when shared.
    ///
    /// Example: `<meta property="og:image" content="https://usasuperior.com/preview.jpg">`
    pub image: Option<OgImage>,
}

/// Details about an Open Graph image (used in rich embeds).
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct OgImage {
    /// The image URL.
    ///
    /// Example: `<meta property="og:image" content="https://example.com/hero.jpg">`
    pub url: String,

    /// Optional image width in pixels.
    ///
    /// Example: `<meta property="og:image:width" content="1200">`
    pub width: Option<u32>,

    /// Optional image height in pixels.
    ///
    /// Example: `<meta property="og:image:height" content="630">`
    pub height: Option<u32>,

    /// Alternative text for accessibility or fallback.
    ///
    /// Example: `<meta property="og:image:alt" content="Roofers installing shingles in Utah">`
    pub alt: Option<String>,
}

/// Twitter Card metadata (used when pages are shared on X/Twitter).
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TwitterMetadata {
    /// `<meta name="twitter:card">` — type of card (e.g., "summary_large_image").
    ///
    /// Example: <meta name="twitter:card" content="summary_large_image">
    pub card: Option<String>,

    /// `<meta name="twitter:site">` — Twitter handle of the site (e.g., "@example").
    ///
    /// Example: <meta name="twitter:site" content="@usasuperior">
    pub site: Option<String>,

    /// `<meta name="twitter:creator">` — Twitter handle of the content creator.
    ///
    /// Example: <meta name="twitter:creator" content="@colbyn">
    pub creator: Option<String>,

    /// `<meta name="twitter:title">` — title shown on Twitter share preview.
    pub title: Option<String>,

    /// `<meta name="twitter:description">` — brief description below the title.
    pub description: Option<String>,

    /// `<meta name="twitter:image">` — image URL used in the Twitter Card.
    pub image: Option<String>,
}

/// Miscellaneous and rarely used metadata fields.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MiscMetadata {
    /// `<meta http-equiv="Content-Language">` — specifies the language of the page.
    ///
    /// Example: `<meta http-equiv="Content-Language" content="en">`
    pub content_language: Option<String>,

    /// `<meta name="format-detection">` — disables automatic linking of phone numbers.
    ///
    /// Example: `<meta name="format-detection" content="telephone=no">`
    pub format_detection: Option<String>,

    /// `<meta name="referrer">` — sets the referrer policy (e.g., "no-referrer-when-downgrade").
    ///
    /// Example: <meta name="referrer" content="strict-origin-when-cross-origin">
    pub referrer: Option<String>,

    /// `<meta name="permissions-policy">` — fine-grained feature policy controls.
    ///
    /// Example: `<meta name="permissions-policy" content="geolocation=(self)">`
    pub permissions_policy: Option<String>,
}

/// A <link> tag conveying metadata about the page (e.g., canonical, icon, preload).
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTag {
    /// The value of the `rel` attribute (e.g., "canonical", "icon", "preconnect").
    ///
    /// Example: `<link rel="canonical" href="https://example.com/page">`
    pub rel: String,

    /// The value of the `href` attribute — the linked resource URL.
    ///
    /// Example: `<link href="https://fonts.googleapis.com" rel="preconnect">`
    pub href: String,

    /// Optional `sizes` attribute, usually for icons.
    ///
    /// Example: <link sizes="32x32" href="/favicon-32x32.png" rel="icon">
    pub sizes: Option<String>,

    /// Optional `type` attribute — MIME type of the linked resource.
    ///
    /// Example: `<link type="image/png" href="...">`
    pub r#type: Option<String>,

    /// Optional `as` attribute, used with rel="preload"/"prefetch".
    ///
    /// Example: `<link rel="preload" as="font" href="...">`
    pub as_: Option<String>,
}

/// Represents a <script type="application/ld+json"> block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonLdBlock {
    /// The raw string contents of the script block.
    ///
    /// Example (truncated):
    /// ```
    /// {
    ///   "@context": "https://schema.org",
    ///   "@type": "LocalBusiness",
    ///   "name": "USA Superior Construction",
    ///   ...
    /// }
    /// ```
    pub raw: String,

    /// The parsed JSON value, if valid.
    pub parsed: Option<Value>,
}


// ————————————————————————————————————————————————————————————————————————————
// REPORT
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PageMetadata {
    pub url: Option<String>,
    pub head: HeadMetadata,
}


// ————————————————————————————————————————————————————————————————————————————
// METHODS HEAD-METADATA (`HeadMetadata`)
// ————————————————————————————————————————————————————————————————————————————

impl HeadMetadata {
    pub fn has_structured_data(&self) -> bool {
        self.json_ld.iter().any(|block| block.parsed.is_some())
    }

    pub fn canonical_url(&self) -> Option<&str> {
        if let Some(url) = &self.meta.seo.canonical_url {
            return Some(url);
        }
        self.links.iter()
            .find(|link| link.rel.eq_ignore_ascii_case("canonical"))
            .map(|link| link.href.as_str())
    }

    pub fn title_trimmed(&self) -> Option<String> {
        self.title.as_ref().map(|t| t.trim().to_string())
    }

    pub fn has_meta(&self) -> bool {
        !self.meta.is_empty()
    }
}

// ————————————————————————————————————————————————————————————————————————————
// METHODS META-TAGS (`MetaTags`)
// ————————————————————————————————————————————————————————————————————————————

impl MetaTags {
    pub fn has_robots_noindex(&self) -> bool {
        self.seo.robots
            .as_ref()
            .map(|val| val.to_ascii_lowercase().contains("noindex"))
            .unwrap_or(false)
    }

    pub fn is_empty(&self) -> bool {
        self.seo.is_empty()
            && self.mobile.is_empty()
            && self.social.is_empty()
            && self.misc.is_empty()
    }
}

// ————————————————————————————————————————————————————————————————————————————
// METHODS SEO-METADATA (`SeoMetadata`)
// ————————————————————————————————————————————————————————————————————————————

impl SeoMetadata {
    pub fn is_indexable(&self) -> bool {
        self.robots
            .as_ref()
            .map(|val| !val.to_ascii_lowercase().contains("noindex"))
            .unwrap_or(true)
    }

    pub fn has_keywords(&self) -> bool {
        self.keywords.as_ref().map(|k| !k.trim().is_empty()).unwrap_or(false)
    }

    pub fn is_empty(&self) -> bool {
        self.description.is_none()
            && self.robots.is_none()
            && self.keywords.is_none()
            && self.author.is_none()
            && self.generator.is_none()
            && self.canonical_url.is_none()
    }
}

// ————————————————————————————————————————————————————————————————————————————
// METHODS MOBILE-METADATA (`MobileMetadata`)
// ————————————————————————————————————————————————————————————————————————————

impl MobileMetadata {
    pub fn is_pwa_capable(&self) -> bool {
        self.mobile_web_app_capable.unwrap_or(false)
            || self.apple_mobile_web_app_capable.unwrap_or(false)
    }

    pub fn has_theme_color(&self) -> bool {
        self.theme_color.is_some()
    }

    pub fn is_empty(&self) -> bool {
        self.viewport.is_none()
            && self.theme_color.is_none()
            && self.mobile_web_app_capable.is_none()
            && self.apple_mobile_web_app_capable.is_none()
    }
}

// ————————————————————————————————————————————————————————————————————————————
// METHODS SOCIAL-METADATA (`SocialMetadata`)
// ————————————————————————————————————————————————————————————————————————————

impl SocialMetadata {
    pub fn has_social_preview(&self) -> bool {
        self.og.title.is_some() || self.twitter.title.is_some()
    }

    pub fn is_empty(&self) -> bool {
        self.og.is_empty() && self.twitter.is_empty()
    }
}

// ————————————————————————————————————————————————————————————————————————————
// METHODS OG-METADATA (`OgMetadata`)
// ————————————————————————————————————————————————————————————————————————————

impl OgMetadata {
    pub fn has_image(&self) -> bool {
        self.image.as_ref().map(|img| !img.url.is_empty()).unwrap_or(false)
    }

    pub fn is_empty(&self) -> bool {
        self.title.is_none()
            && self.description.is_none()
            && self.url.is_none()
            && self.site_name.is_none()
            && self.r#type.is_none()
            && self.locale.is_none()
            && self.image.is_none()
    }
}

// ————————————————————————————————————————————————————————————————————————————
// METHODS OG-IMAGE (`OgImage`)
// ————————————————————————————————————————————————————————————————————————————

impl OgImage {
    pub fn aspect_ratio(&self) -> Option<f32> {
        match (self.width, self.height) {
            (Some(w), Some(h)) if h != 0 => Some(w as f32 / h as f32),
            _ => None,
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.url.is_empty()
    }
}

// ————————————————————————————————————————————————————————————————————————————
// METHODS TWITTER-METADATA (`TwitterMetadata`)
// ————————————————————————————————————————————————————————————————————————————

impl TwitterMetadata {
    pub fn has_large_image_card(&self) -> bool {
        self.card.as_deref() == Some("summary_large_image")
    }

    pub fn is_empty(&self) -> bool {
        self.card.is_none()
            && self.site.is_none()
            && self.creator.is_none()
            && self.title.is_none()
            && self.description.is_none()
            && self.image.is_none()
    }
}

// ————————————————————————————————————————————————————————————————————————————
// METHODS MISC-METADATA (`MiscMetadata`)
// ————————————————————————————————————————————————————————————————————————————

impl MiscMetadata {
    pub fn has_strict_referrer_policy(&self) -> bool {
        matches!(
            self.referrer.as_deref(),
            Some("no-referrer") | Some("strict-origin") | Some("strict-origin-when-cross-origin")
        )
    }

    pub fn is_empty(&self) -> bool {
        self.content_language.is_none()
            && self.format_detection.is_none()
            && self.referrer.is_none()
            && self.permissions_policy.is_none()
    }
}

// ————————————————————————————————————————————————————————————————————————————
// METHODS LINK-TAG (`LinkTag`)
// ————————————————————————————————————————————————————————————————————————————

impl LinkTag {
    pub fn is_metadata_link(rel: &str) -> bool {
        matches!(
            rel,
            "canonical"
                | "alternate"
                | "preconnect"
                | "dns-prefetch"
                | "preload"
                | "manifest"
                | "icon"
                | "shortcut icon"
                | "apple-touch-icon"
                | "author"
        )
    }

    pub fn is_icon(&self) -> bool {
        self.rel.contains("icon")
    }
}

// ————————————————————————————————————————————————————————————————————————————
// METHODS JSON-LD-BLOCK (`JsonLdBlock`)
// ————————————————————————————————————————————————————————————————————————————

impl JsonLdBlock {
    pub fn is_valid(attributes: &AttributeMap) -> bool {
        attributes.contains_key_value("type", "application/ld+json")
    }

    pub fn is_schema_org(&self) -> bool {
        self.raw.contains("schema.org")
    }
}

// ————————————————————————————————————————————————————————————————————————————
// METHODS PAGE-METADATA (`PageMetadata`)
// ————————————————————————————————————————————————————————————————————————————

impl PageMetadata {
    pub fn has_title(&self) -> bool {
        self.head.title.is_some()
    }

    pub fn is_empty(&self) -> bool {
        self.url.is_none() && self.head.title.is_none() && self.head.meta.is_empty()
    }
}

// ————————————————————————————————————————————————————————————————————————————
// HTML AST EXTRACTOR
// ————————————————————————————————————————————————————————————————————————————

impl HeadMetadata {
    pub fn from_head_element_children(children: &Fragment) -> Self {
        let mut meta = MetaTags::default();
        let mut links = Vec::new();
        let mut json_ld = Vec::new();
        let mut title = None;
        let mut base_url = None;

        for el in children.clone().extract_elements() {
            let tag = el.tag.as_normalized();
            let attrs = &el.attributes;

            match tag {
                "title" => {
                    if let Ok(mut texts) = el.extract_child_text_strict() {
                        title = texts.pop();
                    }
                }
                "meta" => Self::extract_meta_tag(attrs, &mut meta),
                "link" => Self::extract_link_tag(attrs, &mut meta, &mut links),
                "base" => {
                    base_url = attrs.get("href").map(|v| v.to_string());
                }
                "script" => {
                    if JsonLdBlock::is_valid(attrs) {
                        let text = el.children.extract_text_strict().ok()
                            .and_then(|mut v| v.pop())
                            .unwrap_or_default();
                        json_ld.push(JsonLdBlock {
                            raw: text.clone(),
                            parsed: serde_json::from_str(&text).ok(),
                        });
                    }
                }
                _ => {}
            }
        }

        HeadMetadata {
            title,
            base_url,
            meta,
            links,
            json_ld,
        }
    }

    fn extract_meta_tag(attrs: &AttributeMap, meta: &mut MetaTags) {
        if let Some(name) = attrs.get("name").map(|v| v.as_str()) {
            match name.to_ascii_lowercase().as_str() {
                "description" => meta.seo.description = attrs.get("content").map(|v| v.to_string()),
                "robots" => meta.seo.robots = attrs.get("content").map(|v| v.to_string()),
                "keywords" => meta.seo.keywords = attrs.get("content").map(|v| v.to_string()),
                "author" => meta.seo.author = attrs.get("content").map(|v| v.to_string()),
                "generator" => meta.seo.generator = attrs.get("content").map(|v| v.to_string()),
                "viewport" => meta.mobile.viewport = attrs.get("content").map(|v| v.to_string()),
                "theme-color" => meta.mobile.theme_color = attrs.get("content").map(|v| v.to_string()),
                "mobile-web-app-capable" => meta.mobile.mobile_web_app_capable = Some(true),
                "apple-mobile-web-app-capable" => meta.mobile.apple_mobile_web_app_capable = Some(true),
                "referrer" => meta.misc.referrer = attrs.get("content").map(|v| v.to_string()),
                "format-detection" => meta.misc.format_detection = attrs.get("content").map(|v| v.to_string()),
                "permissions-policy" => meta.misc.permissions_policy = attrs.get("content").map(|v| v.to_string()),
                _ => {}
            }
        }

        if let Some(property) = attrs.get("property").map(|v| v.as_str()) {
            if property.starts_with("og:") {
                match &property[3..] {
                    "title" => meta.social.og.title = attrs.get("content").map(|v| v.to_string()),
                    "description" => meta.social.og.description = attrs.get("content").map(|v| v.to_string()),
                    "url" => meta.social.og.url = attrs.get("content").map(|v| v.to_string()),
                    "site_name" => meta.social.og.site_name = attrs.get("content").map(|v| v.to_string()),
                    "type" => meta.social.og.r#type = attrs.get("content").map(|v| v.to_string()),
                    "locale" => meta.social.og.locale = attrs.get("content").map(|v| v.to_string()),
                    "image" => meta.social.og.image = Some(OgImage {
                        url: attrs.get("content").map(|v| v.to_string()).unwrap_or_default(),
                        width: None,
                        height: None,
                        alt: None,
                    }),
                    _ => {}
                }
            }
        }

        if let Some(name) = attrs.get("name").map(|v| v.as_str()) {
            if name.starts_with("twitter:") {
                match &name[8..] {
                    "card" => meta.social.twitter.card = attrs.get("content").map(|v| v.to_string()),
                    "site" => meta.social.twitter.site = attrs.get("content").map(|v| v.to_string()),
                    "creator" => meta.social.twitter.creator = attrs.get("content").map(|v| v.to_string()),
                    "title" => meta.social.twitter.title = attrs.get("content").map(|v| v.to_string()),
                    "description" => meta.social.twitter.description = attrs.get("content").map(|v| v.to_string()),
                    "image" => meta.social.twitter.image = attrs.get("content").map(|v| v.to_string()),
                    _ => {}
                }
            }
        }
    }

    fn extract_link_tag(attrs: &AttributeMap, meta: &mut MetaTags, links: &mut Vec<LinkTag>) {
        if let Some(rel) = attrs.get("rel").map(|v| v.as_str()) {
            if LinkTag::is_metadata_link(rel) {
                links.push(LinkTag {
                    rel: rel.to_string(),
                    href: attrs.get("href").map(|v| v.to_string()).unwrap_or_default(),
                    sizes: attrs.get("sizes").map(|v| v.to_string()),
                    r#type: attrs.get("type").map(|v| v.to_string()),
                    as_: attrs.get("as").map(|v| v.to_string()),
                });
            }

            if rel.eq_ignore_ascii_case("canonical") {
                meta.seo.canonical_url = attrs.get("href").map(|v| v.to_string());
            }
        }
    }
}
