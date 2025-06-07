use once_cell::sync::Lazy;
use std::collections::HashSet;


pub static REQUIRES_REGULAR_DEPENDENCY_TRACKING: Lazy<HashSet<(&'static str, &'static str)>> = Lazy::new(|| {
    HashSet::from([
        ("a", "href"),
        ("area", "href"),
        ("link", "href"),
        ("img", "src"),
        ("video", "src"),
        ("video", "poster"),
        ("source", "src"),
        ("script", "src"),
        ("iframe", "src"),
        ("audio", "src"),
        ("track", "src"),
        ("embed", "src"),
        ("object", "data"),
        ("form", "action"),
        ("input", "formaction"),
        ("button", "formaction"),
        ("use", "href"),
        ("use", "xlink:href"),
        ("image", "href"),
        ("image", "xlink:href"),
    ])
});

pub static REQUIRES_SRC_SET_DEPENDENCY_TRACKING: Lazy<HashSet<(&'static str, &'static str)>> = Lazy::new(|| {
    HashSet::from([
        ("img", "srcset"),
        ("source", "srcset"),
    ])
});

pub static REQUIRES_DYNAMIC_SITE_LINK_DEPENDENCY_TRACKING: Lazy<HashSet<(&'static str, &'static str)>> = Lazy::new(|| {
    HashSet::from([
        ("a", "href"),
    ])
});

pub static TAG_MAY_REQUIRE_DEPENDENCY_TRACKING: Lazy<HashSet<&'static str>> = {
    fn tags_only() -> HashSet<&'static str> {
        let xs = REQUIRES_REGULAR_DEPENDENCY_TRACKING
            .iter()
            .chain(REQUIRES_SRC_SET_DEPENDENCY_TRACKING.iter())
            .map(|(x, _)| *x);
        let result: HashSet<&'static str> = HashSet::from_iter(xs);
        result
    }
    Lazy::new(|| { tags_only() })
};

