use std::{sync::{Arc, Mutex}};

use crate::{model::dom::Document, parse::html, network::Session, error::Result};
use druid::{Data, Lens};
use log::error;
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::Url;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub bar_query: Arc<String>,
    pub parser: Arc<html::Parser>,
    pub session: Arc<Mutex<Session>>,
    pub document: Option<Arc<Document>>,
}

static SEARCH_QUERY: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[^\s\.:\[\]]+(?:\s+\S.*)?$").unwrap());

impl AppState {
    /// (Re)loads the document.
    pub fn reload(&mut self) -> Result<()> {
        let url = self.url()?;
        self.bar_query = Arc::new(url.to_string());
        let raw = self.session.lock().unwrap().get_text(url)?;
        let doc = self.parser.parse(raw.as_str())?;
        self.document = Some(Arc::new(doc));
        Ok(())
    }

    /// Performs a potentially erroring computation.
    pub fn perform(&mut self, result: impl FnOnce(&mut Self) -> Result<()>) {
        if let Err(e) = result(self) {
            // TODO: Better error-handling, perhaps store the error message in the state?
            error!("Error: {:?}", e);
        }
    }

    /// Fetches the parsed URL from the address bar.
    pub fn url(&self) -> Result<Url> {
        let query = self.bar_query.as_str();
        let url_result = if SEARCH_QUERY.is_match(query) {
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
