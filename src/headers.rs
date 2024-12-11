use crate::{IntoUrl, Ollama};

pub use http::header::*;

impl Ollama {
    /// # Panics
    ///
    /// Panics if the host is not a valid URL or if the URL cannot have a port.
    pub fn new_with_request_headers(host: impl IntoUrl, port: u16, headers: HeaderMap) -> Self {
        let mut ollama = Self::new(host, port);
        ollama.set_headers(Some(headers));

        ollama
    }

    pub fn set_headers(&mut self, headers: Option<HeaderMap>) {
        match headers {
            Some(h) => self.request_headers = h,
            None => self.request_headers = HeaderMap::new(),
        }
    }
}
