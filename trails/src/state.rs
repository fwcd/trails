use std::sync::Arc;

use druid::{Data, Lens, im};
use trails_base::log::error;
use trails_base::Result;
use trails_model::dom::Document;
use trails_network::url::Url;

use crate::services::AppServices;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub bar_query: String,
    pub document: Arc<Document>,
    pub current_url: String,
    pub history: im::Vector<String>,
    pub forward_history: im::Vector<String>,
}

impl AppState {
    /// Creates a new state.
    pub fn new() -> Self {
        let start_page = "about:blank";
        Self {
            bar_query: start_page.to_owned(),
            current_url: start_page.to_owned(),
            document: Arc::new(Document::new()),
            history: im::Vector::new(),
            forward_history: im::Vector::new(),
        }
    }

    /// Performs a potentially erroring computation.
    pub fn perform(&mut self, result: impl FnOnce(&mut Self) -> Result<()>) {
        if let Err(e) = result(self) {
            // TODO: Better error-handling, perhaps store the error message in the state?
            error!("Error: {:?}", e);
        }
    }

    /// Visits the entered bar query.
    pub fn visit(&mut self, services: &AppServices) -> Result<()> {
        let url = self.parsed_bar_url(services)?;
        self.visit_url(url, services)
    }

    /// Reloads the current page.
    pub fn reload(&mut self, services: &AppServices) -> Result<()> {
        let url = self.url()?;
        self.open(url, services)
    }

    /// Opens the given page without changing the history.
    fn open(&mut self, url: Url, services: &AppServices) -> Result<()> {
        let url_string = url.to_string();
        self.bar_query = url_string.clone();
        self.document = Arc::new(services.load_document(url)?);
        self.current_url = url_string;
        Ok(())
    }

    /// Visits the given page.
    pub fn visit_url(&mut self, url: Url, services: &AppServices) -> Result<()> {
        self.history.push_back(self.current_url.clone());
        self.forward_history.clear();

        self.open(url, services)?;

        Ok(())
    }

    /// Pops the most next page from the forward history and visits it.
    pub fn go_forward(&mut self, services: &AppServices) -> Result<()> {
        if let Some(url) = self.forward_history.pop_back() {
            self.history.push_back(self.current_url.clone());

            let url = Url::parse(&url)?;
            self.open(url, services)?;
        }
        Ok(())
    }

    /// Pops the most recent page from history and visits it.
    pub fn go_back(&mut self, services: &AppServices) -> Result<()> {
        if let Some(url) = self.history.pop_back() {
            self.forward_history.push_back(self.current_url.clone());

            let url = Url::parse(&url)?;
            self.open(url, services)?;
        }
        Ok(())
    }

    /// Fetches the currently visited url.
    pub fn url(&self) -> Result<Url> {
        Ok(Url::parse(&self.current_url)?)
    }

    /// Fetches the url parsed from the address bar.
    pub fn parsed_bar_url(&self, services: &AppServices) -> Result<Url> {
        services.parse_bar_query(self.bar_query.as_str())
    }
}
