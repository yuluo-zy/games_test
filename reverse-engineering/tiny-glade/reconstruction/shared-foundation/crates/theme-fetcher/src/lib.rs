//! B theme producer reconstructed into the original lib.rs/checked_zip.rs tree.
//! Public Rust signatures are reconstruction interfaces, not claims about Rust ABI.
pub mod checked_zip;

use anyhow::{Context, Result, bail};
use arc_swap::ArcSwap;
use std::{
    fs,
    io::Cursor,
    path::{Path, PathBuf},
    sync::{Arc, mpsc::Sender},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use theme_defs::{ActiveEvent, EventDataError};
use theme_server_proto::EventsResponse;

pub const EVENTS_URL: &str = "http://ryeland.pouncelight.games/v1/events";
pub const BLOB_URL: &str = "http://ryeland.pouncelight.games/v1/blob";
pub const DEFAULT_REFRESH: Duration = Duration::from_secs(36_000);
pub const CAS_MAX_AGE: Duration = Duration::from_secs(1_209_600);

/// Only the response fields consumed by the original producer are projected.
#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub date: Option<String>,
    pub etag: Option<String>,
    pub cache_control: Option<String>,
    pub content_length: Option<u64>,
    pub body: std::result::Result<Vec<u8>, String>,
}

/// Test injection boundary around reqwest, not a substitute download implementation.
pub trait Transport {
    fn get(&mut self, url: &str, if_none_match: Option<&str>) -> Result<HttpResponse>;
}

pub struct ReqwestTransport {
    client: reqwest::blocking::Client,
}
impl ReqwestTransport {
    pub fn new(client: reqwest::blocking::Client) -> Self {
        Self { client }
    }
}
impl Transport for ReqwestTransport {
    fn get(&mut self, url: &str, if_none_match: Option<&str>) -> Result<HttpResponse> {
        let mut request = self.client.get(url);
        if let Some(etag) = if_none_match {
            request = request.header(reqwest::header::IF_NONE_MATCH, etag);
        }
        let response = request.send()?;
        let string = |name: reqwest::header::HeaderName| {
            response
                .headers()
                .get(name)
                .and_then(|x| x.to_str().ok())
                .map(str::to_owned)
        };
        let status = response.status().as_u16();
        let date = if url == EVENTS_URL {
            response
                .headers()
                .get(reqwest::header::DATE)
                .map(|header| header.to_str().map(str::to_owned))
                .transpose()?
        } else {
            None
        };
        let etag = if url == EVENTS_URL && status == 200 {
            response
                .headers()
                .get(reqwest::header::ETAG)
                .map(|header| header.to_str().map(str::to_owned))
                .transpose()?
        } else {
            None
        };
        let cache_control = if url == EVENTS_URL {
            string(reqwest::header::CACHE_CONTROL)
        } else {
            None
        };
        let content_length = response.content_length();
        let body = if status == 200 {
            response
                .bytes()
                .map(|bytes| bytes.to_vec())
                .map_err(|error| error.to_string())
        } else {
            Ok(Vec::new())
        };
        Ok(HttpResponse {
            status,
            date,
            etag,
            cache_control,
            content_length,
            body,
        })
    }
}

/// Original wire timestamps are adjusted by server-minus-local whole seconds,
/// with negative adjusted timestamps clamped to the Unix epoch.
pub fn event_local_time(timestamp: i64, server_minus_local: i64) -> SystemTime {
    let seconds = timestamp.wrapping_sub(server_minus_local).max(0) as u64;
    UNIX_EPOCH + Duration::from_secs(seconds)
}

pub struct MaybePendingActiveEvent {
    pub cid: String,
    pub start_time: SystemTime,
    pub end_time: SystemTime,
    pub data: ArcSwap<std::result::Result<PathBuf, EventDataError>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FetchOutcome {
    Replaced,
    NotModified,
}

#[derive(Debug, Default)]
pub struct TrimReport {
    pub removed_rogue: Vec<PathBuf>,
    pub removed_old: Vec<PathBuf>,
    pub removed_corrupt: Vec<PathBuf>,
    pub errors: Vec<(PathBuf, String)>,
}

pub struct ThemeFetcher<T: Transport = ReqwestTransport> {
    cache_folder: PathBuf,
    transport: T,
    pub events: ArcSwap<Vec<MaybePendingActiveEvent>>,
    etag: Option<String>,
    server_minus_local: i64,
    fetch_error: Option<String>,
    next_fetch: Instant,
    retry_seconds: u64,
    last_published: Option<std::result::Result<ActiveEvent, EventDataError>>,
    sender: Option<Sender<std::result::Result<ActiveEvent, EventDataError>>>,
}

impl ThemeFetcher<ReqwestTransport> {
    /// Original constructor creates the supplied cache root, builds reqwest with
    /// a sixty-second timeout and no idle connection pooling, and retains sender.
    pub fn new(
        cache_folder: impl Into<PathBuf>,
        sender: Sender<std::result::Result<ActiveEvent, EventDataError>>,
    ) -> Result<Self> {
        let cache_folder = cache_folder.into();
        fs::create_dir_all(&cache_folder)?;
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(60))
            .pool_max_idle_per_host(0)
            .build()
            .expect("original ThemeFetcher client construction failed");
        let mut fetcher =
            Self::with_transport(cache_folder, ReqwestTransport::new(client), Instant::now());
        fetcher.sender = Some(sender);
        Ok(fetcher)
    }
}

impl<T: Transport> ThemeFetcher<T> {
    pub fn with_transport(cache_folder: impl Into<PathBuf>, transport: T, now: Instant) -> Self {
        Self {
            cache_folder: cache_folder.into(),
            transport,
            events: ArcSwap::from_pointee(Vec::new()),
            etag: None,
            server_minus_local: 0,
            fetch_error: Some(String::new()),
            next_fetch: now,
            retry_seconds: 1,
            last_published: None,
            sender: None,
        }
    }

    pub fn next_fetch(&self) -> Instant {
        self.next_fetch
    }
    pub fn retry_seconds(&self) -> u64 {
        self.retry_seconds
    }

    /// Read Date/Cache-Control for both 200 and 304. ETag is retained when the
    /// header is absent, and updated before body validation, matching the original.
    pub fn fetch_events(&mut self, local_now: SystemTime) -> Result<(FetchOutcome, Duration)> {
        let response = self.transport.get(EVENTS_URL, self.etag.as_deref())?;
        if let Some(date) = response
            .date
            .as_deref()
            .and_then(|x| httpdate::parse_http_date(x).ok())
        {
            let server = date.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
            let local = local_now.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
            self.server_minus_local = server.wrapping_sub(local);
        }
        let refresh = response
            .cache_control
            .as_deref()
            .and_then(cache_control::CacheControl::from_value)
            .and_then(|x| x.max_age)
            .unwrap_or(DEFAULT_REFRESH);
        match response.status {
            304 => Ok((FetchOutcome::NotModified, refresh)),
            200 => {
                if let Some(etag) = response.etag {
                    self.etag = Some(etag);
                }
                let expected = response
                    .content_length
                    .context("No Content-Length header")?;
                let body = response.body.map_err(anyhow::Error::msg)?;
                if body.len() as u64 != expected {
                    bail!("Response body length does not match Content-Length header");
                }
                let response: EventsResponse = serde_json::from_slice(&body)?;
                let events = response
                    .events
                    .into_iter()
                    .map(|event| MaybePendingActiveEvent {
                        cid: event.cid,
                        start_time: event_local_time(event.start, self.server_minus_local),
                        end_time: event_local_time(event.end, self.server_minus_local),
                        data: ArcSwap::from_pointee(Err(EventDataError::Pending)),
                    })
                    .collect();
                self.events.store(Arc::new(events));
                Ok((FetchOutcome::Replaced, refresh))
            }
            status => bail!("Failed to fetch events. Status: {status}"),
        }
    }

    /// Existing CAS directories bypass download. Startup trim is the separate verifier.
    pub fn download_blob(&mut self, cid: &str) -> Result<PathBuf> {
        let path = self.cache_folder.join(cid);
        if path.is_dir() {
            return Ok(path);
        }
        let response = self.transport.get(&format!("{BLOB_URL}/{cid}"), None)?;
        if response.status != 200 {
            bail!("Failed to fetch {cid}. Status: {}", response.status);
        }
        let body = response.body.map_err(anyhow::Error::msg)?;
        if response
            .content_length
            .is_some_and(|length| body.len() as u64 != length)
        {
            bail!("Response body length does not match Content-Length header");
        }
        if blake3::hash(&body).to_string() != cid {
            bail!("Checksum mismatch");
        }
        let archive = zip::ZipArchive::new(Cursor::new(body))?;
        checked_zip::extract_with_checksums(archive, &path)?;
        Ok(path)
    }

    /// Compare modified age before checksum validation; age errors still validate.
    pub fn validate_cas_cache_entry(path: &Path, now: SystemTime) -> Result<bool> {
        let metadata = fs::metadata(path)?;
        if metadata
            .modified()
            .ok()
            .and_then(|modified| now.duration_since(modified).ok())
            .is_some_and(|age| age > CAS_MAX_AGE)
        {
            fs::remove_dir_all(path)?;
            return Ok(false);
        }
        if checked_zip::verify_folder_checksums(path).is_err() {
            fs::remove_dir_all(path)?;
            return Ok(false);
        }
        Ok(true)
    }

    /// Entry errors are isolated. A non-directory is rogue; directories older
    /// than fourteen days are removed; remaining directories validate .checksum.
    pub fn trim_cas(&self, now: SystemTime) -> Result<TrimReport> {
        let mut report = TrimReport::default();
        for entry in fs::read_dir(&self.cache_folder).context("reading CAS cache folder")? {
            let entry = match entry {
                Ok(entry) => entry,
                Err(error) => {
                    report
                        .errors
                        .push((self.cache_folder.clone(), error.to_string()));
                    continue;
                }
            };
            let path = entry.path();
            let process = || -> Result<u8> {
                let metadata = entry.metadata()?;
                if !metadata.is_dir() {
                    fs::remove_file(&path)?;
                    return Ok(0);
                }
                if metadata
                    .modified()
                    .ok()
                    .and_then(|modified| now.duration_since(modified).ok())
                    .is_some_and(|age| age > CAS_MAX_AGE)
                {
                    fs::remove_dir_all(&path)?;
                    return Ok(1);
                }
                if checked_zip::verify_folder_checksums(&path).is_err() {
                    fs::remove_dir_all(&path)?;
                    return Ok(2);
                }
                Ok(3)
            };
            match process() {
                Ok(0) => report.removed_rogue.push(path),
                Ok(1) => report.removed_old.push(path),
                Ok(2) => report.removed_corrupt.push(path),
                Ok(_) => {}
                Err(error) => report.errors.push((path, error.to_string())),
            }
        }
        Ok(report)
    }

    /// One original run-loop iteration: fetch when due, retry every event whose
    /// blob is not yet Ok (fresh Pending and previously failed downloads), then
    /// choose the last active interval. Failed downloads are retried each loop
    /// iteration (run @ 0x140618310), not only after a new events response.
    pub fn run_once(
        &mut self,
        now: SystemTime,
        monotonic_now: Instant,
    ) -> Option<std::result::Result<ActiveEvent, EventDataError>> {
        if monotonic_now >= self.next_fetch {
            match self.fetch_events(now) {
                Ok((outcome, refresh)) => {
                    self.next_fetch = monotonic_now + refresh;
                    if outcome == FetchOutcome::Replaced {
                        self.fetch_error = None;
                    }
                }
                Err(error) => {
                    self.fetch_error = Some(error.to_string());
                    self.next_fetch = monotonic_now + Duration::from_secs(self.retry_seconds);
                    self.retry_seconds = (self.retry_seconds * 2).min(300);
                }
            }
        }
        let events = self.events.load_full();
        for event in events.iter() {
            // Original download loop triggers on ANY Err variant, not only
            // Pending: run @ 0x140618310, `if (SBORROW8(0, data[2])) goto
            // download` (Result tag word; Ok tag = 0, every Err tag != 0).
            // A failed download is therefore retried on every 1s loop pass.
            if event.data.load().as_ref().is_err() {
                let data = self
                    .download_blob(&event.cid)
                    .map_err(|error| EventDataError::Error(error.to_string()));
                event.data.store(Arc::new(data));
            }
        }
        let mut active = Err(EventDataError::NoActiveEvent);
        for event in events.iter() {
            if theme_defs::is_active_interval(event.start_time, event.end_time, now) {
                active = event
                    .data
                    .load_full()
                    .as_ref()
                    .clone()
                    .map(|data_folder| ActiveEvent {
                        data_folder,
                        start_time: event.start_time,
                        end_time: event.end_time,
                    });
            }
        }
        match &self.fetch_error {
            Some(error) if matches!(active, Err(EventDataError::NoActiveEvent)) => {
                active = Err(EventDataError::Error(error.clone()))
            }
            _ => {}
        }
        if self.last_published.as_ref() == Some(&active) {
            return None;
        }
        self.last_published = Some(active.clone());
        Some(active)
    }

    /// Run until the consumer disconnects. Original loop sleeps one second and
    /// publishes only changes. Startup trim failure does not stop event fetching.
    pub fn run(mut self) -> Result<()> {
        let sender = self
            .sender
            .take()
            .context("injected provider has no consumer; use run_with_sender")?;
        self.run_with_sender(sender);
        Ok(())
    }

    /// Explicit adapter for injected transports; production new()/run() retain
    /// the original sender ownership structure.
    pub fn run_with_sender(
        mut self,
        sender: Sender<std::result::Result<ActiveEvent, EventDataError>>,
    ) {
        let _ = self.trim_cas(SystemTime::now());
        loop {
            if self
                .run_once(SystemTime::now(), Instant::now())
                .is_some_and(|value| sender.send(value).is_err())
            {
                return;
            }
            std::thread::sleep(Duration::from_secs(1));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        collections::VecDeque,
        io::Write,
        sync::atomic::{AtomicU64, Ordering},
    };
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Fixture {
        responses: VecDeque<HttpResponse>,
        requests: Vec<(String, Option<String>)>,
    }
    impl Transport for Fixture {
        fn get(&mut self, url: &str, etag: Option<&str>) -> Result<HttpResponse> {
            self.requests.push((url.into(), etag.map(str::to_owned)));
            self.responses.pop_front().context("fixture exhausted")
        }
    }
    fn response(status: u16, body: Vec<u8>) -> HttpResponse {
        HttpResponse {
            status,
            date: None,
            etag: None,
            cache_control: None,
            content_length: Some(body.len() as u64),
            body: Ok(body),
        }
    }
    fn fixture(responses: Vec<HttpResponse>) -> Fixture {
        Fixture {
            responses: responses.into(),
            requests: Vec::new(),
        }
    }
    fn folder() -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "tg-theme-fetch-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&path).unwrap();
        path
    }
    fn zip_bytes() -> Vec<u8> {
        let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
        writer
            .start_file("theme.json", zip::write::FileOptions::default())
            .unwrap();
        writer.write_all(b"fixture theme").unwrap();
        writer.finish().unwrap().into_inner()
    }
    #[test]
    fn constructor_creates_root_and_propagates_filesystem_failure() {
        let base = folder();
        let root = base.join("new-root");
        let (sender, _receiver) = std::sync::mpsc::channel();
        let fetcher = ThemeFetcher::new(&root, sender).unwrap();
        assert!(root.is_dir());
        assert!(fetcher.sender.is_some());
        let file = base.join("existing-file");
        fs::write(&file, b"not a directory").unwrap();
        let (sender, _receiver) = std::sync::mpsc::channel();
        assert!(ThemeFetcher::new(file, sender).is_err());
        drop(fetcher);
        fs::remove_dir_all(base).unwrap();
    }

    #[test]
    fn fetch_conditional_headers_clock_and_not_modified() {
        let at = Instant::now();
        let mut first = response(
            200,
            br#"{"events":[{"start":120,"end":130,"cid":"one"}]}"#.to_vec(),
        );
        first.date = Some(httpdate::fmt_http_date(
            UNIX_EPOCH + Duration::from_secs(110),
        ));
        first.etag = Some("original-tag".into());
        first.cache_control = Some("max-age=7".into());
        let mut fetcher = ThemeFetcher::with_transport(
            "unused",
            fixture(vec![first, response(304, Vec::new())]),
            at,
        );
        assert_eq!(
            fetcher
                .fetch_events(UNIX_EPOCH + Duration::from_secs(100))
                .unwrap(),
            (FetchOutcome::Replaced, Duration::from_secs(7))
        );
        assert_eq!(
            fetcher.events.load()[0].start_time,
            UNIX_EPOCH + Duration::from_secs(110)
        );
        assert_eq!(
            fetcher
                .fetch_events(UNIX_EPOCH + Duration::from_secs(100))
                .unwrap(),
            (FetchOutcome::NotModified, DEFAULT_REFRESH)
        );
        assert_eq!(
            fetcher.transport.requests[1].1.as_deref(),
            Some("original-tag")
        );
        assert_eq!(fetcher.events.load().len(), 1);
        assert_eq!(event_local_time(-3, 0), UNIX_EPOCH);
    }
    #[test]
    fn bad_fetch_keeps_etag_for_retry_and_requires_exact_length() {
        let mut bad = response(200, b"{}".to_vec());
        bad.content_length = None;
        bad.etag = Some("retry-tag".into());
        let mut fetcher = ThemeFetcher::with_transport(
            "unused",
            fixture(vec![bad, response(200, br#"{"events":[]}"#.to_vec())]),
            Instant::now(),
        );
        assert_eq!(
            fetcher.fetch_events(UNIX_EPOCH).unwrap_err().to_string(),
            "No Content-Length header"
        );
        fetcher.fetch_events(UNIX_EPOCH).unwrap();
        assert_eq!(
            fetcher.transport.requests[1].1.as_deref(),
            Some("retry-tag")
        );
        let mut bad = response(200, br#"{"events":[]}"#.to_vec());
        bad.content_length = Some(999);
        fetcher.transport.responses.push_back(bad);
        assert_eq!(
            fetcher.fetch_events(UNIX_EPOCH).unwrap_err().to_string(),
            "Response body length does not match Content-Length header"
        );
        let mut failed = response(200, Vec::new());
        failed.etag = Some("body-failure-tag".into());
        failed.body = Err("body read failed".into());
        fetcher.transport.responses.push_back(failed);
        assert_eq!(
            fetcher.fetch_events(UNIX_EPOCH).unwrap_err().to_string(),
            "body read failed"
        );
        assert_eq!(fetcher.etag.as_deref(), Some("body-failure-tag"));
    }
    #[test]
    fn not_modified_preserves_last_fetch_error_and_cached_events() {
        let at = Instant::now();
        let mut fetcher = ThemeFetcher::with_transport(
            "unused",
            fixture(vec![response(500, Vec::new()), response(304, Vec::new())]),
            at,
        );
        let first = fetcher.run_once(UNIX_EPOCH, at).unwrap();
        assert_eq!(
            first,
            Err(EventDataError::Error(
                "Failed to fetch events. Status: 500".into()
            ))
        );
        assert!(
            fetcher
                .run_once(UNIX_EPOCH, at + Duration::from_secs(1))
                .is_none()
        );
        assert_eq!(
            fetcher.fetch_error.as_deref(),
            Some("Failed to fetch events. Status: 500")
        );
    }

    #[test]
    fn downloads_hashes_extracts_and_then_uses_existing_cas() {
        let root = folder();
        let bytes = zip_bytes();
        let cid = blake3::hash(&bytes).to_string();
        let mut fetcher = ThemeFetcher::with_transport(
            &root,
            fixture(vec![response(200, bytes)]),
            Instant::now(),
        );
        let path = fetcher.download_blob(&cid).unwrap();
        assert!(path.join("theme.json").exists());
        assert!(checked_zip::verify_folder_checksums(&path).is_ok());
        assert_eq!(fetcher.download_blob(&cid).unwrap(), path);
        assert_eq!(fetcher.transport.requests.len(), 1);
        assert!(fetcher.transport.requests[0].0.ends_with(&cid));
        fs::write(root.join("file-entry"), b"rogue file").unwrap();
        fetcher
            .transport
            .responses
            .push_back(response(404, Vec::new()));
        assert!(fetcher.download_blob("file-entry").is_err());
        assert_eq!(fetcher.transport.requests.len(), 2);
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn checksum_failure_does_not_create_cas_and_future_events_are_downloaded() {
        let root = folder();
        let at = Instant::now();
        let events = br#"{"events":[{"start":100,"end":200,"cid":"wrong"}]}"#.to_vec();
        let mut fetcher = ThemeFetcher::with_transport(
            &root,
            fixture(vec![
                response(200, events),
                response(200, zip_bytes()),
                response(200, zip_bytes()),
            ]),
            at,
        );
        assert_eq!(
            fetcher.run_once(UNIX_EPOCH, at),
            Some(Err(EventDataError::NoActiveEvent))
        );
        assert_eq!(fetcher.transport.requests.len(), 2); // events fetch + first download attempt
        assert!(!root.join("wrong").exists());
        let active = fetcher
            .run_once(
                UNIX_EPOCH + Duration::from_secs(100),
                at + Duration::from_secs(1),
            )
            .unwrap();
        assert_eq!(
            active,
            Err(EventDataError::Error("Checksum mismatch".into()))
        );
        // Failed downloads are retried every loop iteration in the original.
        assert_eq!(fetcher.transport.requests.len(), 3);
        assert!(!root.join("wrong").exists());
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn retry_exponential_cap_and_last_matching_active_wins() {
        let root = folder();
        let at = Instant::now();
        let mut fetcher = ThemeFetcher::with_transport(
            &root,
            fixture((0..10).map(|_| response(500, Vec::new())).collect()),
            at,
        );
        let mut tick = at;
        for _ in 0..10 {
            fetcher.run_once(UNIX_EPOCH, tick);
            tick = fetcher.next_fetch();
        }
        assert_eq!(fetcher.retry_seconds(), 300);
        let make = |cid: &str| MaybePendingActiveEvent {
            cid: cid.into(),
            start_time: UNIX_EPOCH,
            end_time: UNIX_EPOCH + Duration::from_secs(10),
            data: ArcSwap::from_pointee(Ok(root.join(cid))),
        };
        fetcher
            .events
            .store(Arc::new(vec![make("first"), make("last")]));
        fetcher.fetch_error = None;
        let active = fetcher
            .run_once(UNIX_EPOCH, tick - Duration::from_millis(1))
            .unwrap()
            .unwrap();
        assert_eq!(active.data_folder, root.join("last"));
        assert!(
            fetcher
                .run_once(UNIX_EPOCH, tick - Duration::from_millis(1))
                .is_none()
        );
        assert_eq!(
            fetcher.run_once(
                UNIX_EPOCH + Duration::from_secs(10),
                tick - Duration::from_millis(1)
            ),
            Some(Err(EventDataError::NoActiveEvent))
        );
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn trim_removes_rogue_old_and_corrupt_but_keeps_valid() {
        let root = folder();
        fs::write(root.join("rogue"), b"rogue").unwrap();
        fs::create_dir_all(root.join("corrupt")).unwrap();
        let bytes = zip_bytes();
        let cid = blake3::hash(&bytes).to_string();
        let mut fetcher = ThemeFetcher::with_transport(
            &root,
            fixture(vec![response(200, bytes)]),
            Instant::now(),
        );
        let valid = fetcher.download_blob(&cid).unwrap();
        let report = fetcher.trim_cas(SystemTime::now()).unwrap();
        assert_eq!(report.removed_rogue, vec![root.join("rogue")]);
        assert_eq!(report.removed_corrupt, vec![root.join("corrupt")]);
        assert!(valid.exists());
        let report = fetcher
            .trim_cas(SystemTime::now() + CAS_MAX_AGE + Duration::from_secs(1))
            .unwrap();
        assert_eq!(report.removed_old, vec![valid]);
        fs::remove_dir_all(root).unwrap();
    }
}
