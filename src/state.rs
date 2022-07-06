use std::sync::{Arc, Mutex};

use crate::{dom::Document, parse::HtmlParser, network::NetworkSession};
use druid::{Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub url: Arc<String>,
    pub parser: Arc<HtmlParser>,
    pub session: Arc<Mutex<NetworkSession>>,
    pub document: Option<Arc<Document>>,
}
