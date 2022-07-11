use std::sync::{Arc, Mutex};

use crate::{model::dom::Document, parse::html, network::Session};
use druid::{Data, Lens};
use log::error;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub url: Arc<String>,
    pub parser: Arc<html::Parser>,
    pub session: Arc<Mutex<Session>>,
    pub document: Option<Arc<Document>>,
}

impl AppState {
    /// (Re)loads the document.
    pub fn reload(&mut self) {
        // TODO: Better error-handling, perhaps store the error message in the state?
        let doc = self.session.lock().unwrap().get_text(self.url.as_str())
            .and_then(|raw| self.parser.parse(raw.as_str()));
        match doc {
            Ok(doc) => self.document = Some(Arc::new(doc)),
            Err(e) => error!("Error while fetching/parsing HTML: {:?}", e),
        };
    }
}
