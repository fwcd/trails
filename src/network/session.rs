use log::info;
use reqwest::{blocking::{Client, Response}, header::USER_AGENT};

use crate::{constants::VERSION, error::Result};

/// A facility for performing HTTP requests that may hold state
/// (e.g. cookies).
pub struct Session {
    client: Client,
    user_agent: String,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            user_agent: format!("Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36 Trails/{}", VERSION),
            client: Client::new(),
        }
    }
}

impl Session {
    /// Performs a GET request to the given URL.
    fn get(&mut self, url: &str) -> Result<Response> {
        info!("Getting {}", url);
        // TODO: Async
        let response = self.client.get(url)
            .header(USER_AGENT, &self.user_agent)
            .send()?;
        Ok(response)
    }

    /// Fetches a string via a GET request.
    pub fn get_text(&mut self, url: &str) -> Result<String> {
        Ok(self.get(url)?.text()?)
    }
}
