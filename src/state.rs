use std::sync::{Arc, Mutex};

use crate::{model::dom::Document, parse::html, network::Session};
use druid::{Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub url: Arc<String>,
    pub parser: Arc<html::Parser>,
    pub session: Arc<Mutex<Session>>,
    pub document: Option<Arc<Document>>,
}
