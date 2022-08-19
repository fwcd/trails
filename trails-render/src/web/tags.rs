use std::collections::HashSet;

use trails_base::once_cell::sync::Lazy;

pub static RENDERED_TAGS: Lazy<HashSet<&str>> = Lazy::new(|| {
    let mut set = HashSet::new();
    set.insert("$root");
    set.insert("html");
    set.insert("body");
    set.insert("div");
    set.insert("a");
    set.insert("ul");
    set.insert("li");
    set.insert("p");
    set.insert("span");
    set.insert("b");
    set.insert("i");
    set.insert("u");
    set.insert("strong");
    set.insert("em");
    set.insert("h1");
    set.insert("h2");
    set.insert("h3");
    set.insert("h4");
    set.insert("h5");
    set.insert("h6");
    set.insert("table");
    set.insert("th");
    set.insert("tr");
    set.insert("td");
    set.insert("nav");
    set.insert("section");
    set.insert("article");
    set.insert("footer");
    set.insert("aside");
    set.insert("main");
    set.insert("label");
    set.insert("noscript");
    set.insert("abbr");
    set.insert("nobr");
    set.insert("wbr");
    set.insert("center");
    set
});

pub static INLINE_TAGS: Lazy<HashSet<&str>> = Lazy::new(|| {
    let mut set = HashSet::new();
    set.insert("a");
    set.insert("span");
    set.insert("b");
    set.insert("i");
    set.insert("u");
    set.insert("strong");
    set.insert("em");
    set
});
