use crate::{model::dom::Document, error::Result, services::AppServices};
use druid::{Data, Lens};
use log::error;
use reqwest::Url;

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

    /// Reloads the current page.
    pub fn reload(&mut self, services: &AppServices) -> Result<()> {
        let url = self.url(services)?;
        self.visit(url, services)
    }

    /// Visits the given page.
    pub fn visit(&mut self, url: Url, services: &AppServices) -> Result<()> {
        self.bar_query = url.to_string();
        self.document = services.load_document(url)?;
        Ok(())
    }

    /// Fetches the parsed url.
    pub fn url(&self, services: &AppServices) -> Result<Url> {
        services.parse_bar_query(self.bar_query.as_str())
    }
}
