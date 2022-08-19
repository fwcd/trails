use std::fs;

use trails_base::indoc::indoc;
use trails_base::log::info;
use trails_base::{VERSION, Result, bail};
use reqwest::{blocking::Client, header::USER_AGENT};
use url::Url;

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
    fn get(&mut self, url: Url) -> Result<Vec<u8>> {
        info!("Getting {}", url);
        // TODO: Async
        match url.scheme() {
            "http" | "https" => {
                // Fetch document via HTTP
                let mut response = self.client.get(url)
                    .header(USER_AGENT, &self.user_agent)
                    .send()?;
                let mut bytes: Vec<u8> = Vec::new();
                response.copy_to(&mut bytes)?;
                Ok(bytes)
            },
            "file" => {
                // Read local document
                println!("{}", url.path());
                let contents = fs::read(url.path())?;
                Ok(contents)
            },
            "about" => {
                // Return an empty document
                Ok(indoc! {"
                    <html>
                        <head>
                            <title>Trails</title>
                        </head>
                        <body></body>
                    </html>
                "}.as_bytes().into_iter().cloned().collect())
            },
            scheme => bail!("Unsupported scheme: {}", scheme),
        }
    }

    /// Fetches a string via a GET request.
    pub fn get_text(&mut self, url: Url) -> Result<String> {
        Ok(String::from_utf8(self.get(url)?)?)
    }
}
