use std::sync::Mutex;

use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::Url;

use crate::{parse::html, network::Session, model::dom::Document, error::Result};

static SEARCH_QUERY: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[^\s\.:\[\]]+(?:\s+\S.*)?$").unwrap());

/// Central services used by the application.
pub struct AppServices {
    html_parser: html::Parser,
    session: Mutex<Session>,
}

impl AppServices {
    pub fn new() -> Self {
        Self {
            html_parser: html::Parser::default(),
            session: Mutex::new(Session::default()),
        }
    }

    /// Loads a document.
    pub fn load_document(&self, url: Url) -> Result<Document> {
        let raw = self.session.lock().unwrap().get_text(url)?;
        let doc = self.html_parser.parse(raw.as_str())?;
        Ok(doc)
    }

    /// Parses an address-bar query to a URL.
    pub fn parse_bar_query(&self, query: &str) -> Result<Url> {
        let url_result = if query.is_empty() {
            Url::parse("about:blank")
        } else if SEARCH_QUERY.is_match(query) {
            Url::parse_with_params("https://www.google.com/search", &[("q", query)])
        } else {
            Url::parse(query)
        };
        Ok(url_result.or_else(|e| match e {
            url::ParseError::RelativeUrlWithoutBase => {
                // TODO: Windows paths?
                let fallback = if query.starts_with("/") {
                    format!("file://{}", query)
                } else {
                    format!("https://{}", query)
                };
                Url::parse(fallback.as_str())
            },
            _ => Err(e)
        })?)
    }
}
