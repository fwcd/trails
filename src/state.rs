use crate::{model::dom::Document, error::Result};
use druid::{Data, Lens};
use log::error;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub bar_query: String,
    pub document: Document,
}

impl AppState {
    /// Performs a potentially erroring computation.
    pub fn perform(&mut self, result: impl FnOnce(&mut Self) -> Result<()>) {
        if let Err(e) = result(self) {
            // TODO: Better error-handling, perhaps store the error message in the state?
            error!("Error: {:?}", e);
        }
    }
}
