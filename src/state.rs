use std::sync::{Arc, Mutex};

use crate::{model::dom::Document, parse::html, network::Session, error::Result};
use druid::{Data, Lens};
use log::error;
use reqwest::Url;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub url: Arc<String>,
    pub parser: Arc<html::Parser>,
    pub session: Arc<Mutex<Session>>,
    pub document: Option<Arc<Document>>,
}

impl AppState {
    /// (Re)loads the document.
    pub fn reload(&mut self) -> Result<()> {
        let url = Self::parse_url(self.url.as_str())?;
        self.url = Arc::new(url.to_string());
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

    /// Parses a URL.
    fn parse_url(url: &str) -> Result<Url> {
        Ok(Url::parse(url).or_else(|e| match e {
            url::ParseError::RelativeUrlWithoutBase => {
                // TODO: Google if the address doesn't look like a URL or file path?
                // TODO: Windows paths?
                let fallback = if url.starts_with("/") {
                    format!("file://{}", url)
                } else {
                    format!("https://{}", url)
                };
                Url::parse(fallback.as_str())
            },
            _ => Err(e)
        })?)
    }
}
