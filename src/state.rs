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
        let url = Url::parse(self.url.as_str())?;
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
}
