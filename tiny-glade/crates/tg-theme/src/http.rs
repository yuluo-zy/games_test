//! Stateless HTTP adapter. Response bodies remain streaming and have one owner.
use anyhow::{Context, Result, ensure};
use std::{io::Read, time::Duration};

pub struct Response {
    pub status: u16,
    pub date: Option<String>,
    pub age: Option<Duration>,
    pub etag: Option<String>,
    pub cache_control: Option<String>,
    pub content_length: Option<u64>,
    pub body: Box<dyn Read + Send>,
}

/// Injection boundary used by offline tests and alternate service transports.
/// Implementations must bound request and body-read time so worker shutdown
/// cannot be held indefinitely by an unresponsive peer.
pub trait Transport: Send {
    fn get(&mut self, url: &str, etag: Option<&str>) -> Result<Response>;
}

pub struct HttpTransport(reqwest::blocking::Client);

impl HttpTransport {
    pub fn new(timeout: Duration) -> Result<Self> {
        ensure!(
            !timeout.is_zero() && timeout <= Duration::from_secs(300),
            "HTTP timeout must be between 1 ns and 300 seconds"
        );
        Ok(Self(
            reqwest::blocking::Client::builder()
                .timeout(timeout)
                // Direct service requests avoid implicit OS proxy discovery (which
                // can panic in headless macOS sessions). Custom proxy users can
                // provide their own Transport.
                .no_proxy()
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .context("building theme HTTP client")?,
        ))
    }
}

impl Transport for HttpTransport {
    fn get(&mut self, url: &str, etag: Option<&str>) -> Result<Response> {
        let mut request = self.0.get(url);
        if let Some(etag) = etag {
            request = request.header(reqwest::header::IF_NONE_MATCH, etag);
        }
        let response = request.send().context("requesting theme service")?;
        let header = |name: reqwest::header::HeaderName| -> Result<Option<String>> {
            response
                .headers()
                .get(name)
                .map(|value| {
                    value
                        .to_str()
                        .map(str::to_owned)
                        .context("invalid theme response header")
                })
                .transpose()
        };
        Ok(Response {
            status: response.status().as_u16(),
            date: header(reqwest::header::DATE)?,
            age: header(reqwest::header::AGE)?
                .map(|value| {
                    ensure!(
                        !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit()),
                        "invalid theme response Age"
                    );
                    value
                        .parse::<u64>()
                        .map(Duration::from_secs)
                        .context("theme response Age exceeds supported range")
                })
                .transpose()?,
            etag: header(reqwest::header::ETAG)?,
            cache_control: header(reqwest::header::CACHE_CONTROL)?,
            content_length: response.content_length(),
            body: Box::new(response),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn invalid_timeouts_are_rejected_before_building_client() {
        for timeout in [Duration::ZERO, Duration::from_secs(301), Duration::MAX] {
            assert!(HttpTransport::new(timeout).is_err(), "{timeout:?}");
        }
    }
}
