//! Offline producer/contract integration; Transport never opens a socket.
use anyhow::{Result, bail};
use std::{
    collections::VecDeque,
    fs,
    io::{Cursor, Write},
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use theme_defs::EventDataError;
use theme_fetcher::{BLOB_URL, EVENTS_URL, HttpResponse, ThemeFetcher, Transport, checked_zip};
use theme_server_proto::{ActiveEvent, EventsResponse};

static NEXT: AtomicU64 = AtomicU64::new(0);

struct Cache(PathBuf);
impl Cache {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "tg-theme-chain-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }
}
impl Drop for Cache {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

struct Offline(VecDeque<(String, Option<String>, HttpResponse)>);
impl Transport for Offline {
    fn get(&mut self, url: &str, etag: Option<&str>) -> Result<HttpResponse> {
        let Some((expected_url, expected_etag, response)) = self.0.pop_front() else {
            bail!("unexpected offline request: {url}");
        };
        assert_eq!(url, expected_url);
        assert_eq!(etag, expected_etag.as_deref());
        Ok(response)
    }
}

fn response(status: u16, body: Vec<u8>) -> HttpResponse {
    HttpResponse {
        status,
        date: None,
        etag: None,
        cache_control: Some("max-age=5".into()),
        content_length: Some(body.len() as u64),
        body: Ok(body),
    }
}

fn package() -> Vec<u8> {
    let mut zip = zip::ZipWriter::new(Cursor::new(Vec::new()));
    zip.start_file("theme.json", zip::write::FileOptions::default())
        .unwrap();
    // Opaque fixture: these crates transport bytes and do not own theme.json schema.
    zip.write_all(br#"{"fixture":"theme"}"#).unwrap();
    zip.finish().unwrap().into_inner()
}

fn time(seconds: u64) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs(seconds)
}

#[test]
fn wire_to_verified_cas_to_active_payload_then_304_and_expiry() {
    let cache = Cache::new();
    let bytes = package();
    let cid = blake3::hash(&bytes).to_string();
    let wire = EventsResponse {
        events: vec![
            ActiveEvent {
                start: 1_000,
                end: 1_020,
                cid: cid.clone(),
            },
            ActiveEvent {
                start: 1_005,
                end: 1_015,
                cid: cid.clone(),
            },
        ],
    };
    let mut events = response(200, serde_json::to_vec(&wire).unwrap());
    // Server is ten seconds ahead. Local intervals become [990,1010), [995,1005).
    events.date = Some(httpdate::fmt_http_date(time(1_010)));
    events.etag = Some("\"fixture-v1\"".into());
    let fixture = Offline(VecDeque::from([
        (EVENTS_URL.into(), None, events),
        (format!("{BLOB_URL}/{cid}"), None, response(200, bytes)),
        (
            EVENTS_URL.into(),
            Some("\"fixture-v1\"".into()),
            response(304, Vec::new()),
        ),
    ]));
    let tick = Instant::now();
    let mut fetcher = ThemeFetcher::with_transport(&cache.0, fixture, tick);
    let first = fetcher.run_once(time(1_000), tick).unwrap().unwrap();
    assert_eq!(first.data_folder, cache.0.join(&cid));
    assert_eq!(first.start_time, time(995));
    assert_eq!(first.end_time, time(1_005));
    assert_eq!(
        fs::read(first.data_folder.join("theme.json")).unwrap(),
        br#"{"fixture":"theme"}"#
    );
    assert_eq!(
        checked_zip::verify_folder_checksums(&first.data_folder)
            .unwrap()
            .files
            .len(),
        1
    );
    assert!(
        fetcher
            .run_once(time(1_001), tick + Duration::from_secs(1))
            .is_none()
    );
    // 304 preserves payloads. At the final interval's exclusive end the earlier one wins.
    let earlier = fetcher
        .run_once(time(1_005), tick + Duration::from_secs(5))
        .unwrap()
        .unwrap();
    assert_eq!(earlier.start_time, time(990));
    assert_eq!(earlier.end_time, time(1_010));
    assert_eq!(
        fetcher.run_once(time(1_010), tick + Duration::from_secs(6)),
        Some(Err(EventDataError::NoActiveEvent))
    );
}

#[test]
fn inactive_packages_are_prefetched_and_failed_blob_retries_before_refresh() {
    let cache = Cache::new();
    let bytes = package();
    let cid = blake3::hash(&bytes).to_string();
    let body = serde_json::to_vec(&EventsResponse {
        events: vec![ActiveEvent {
            start: 100,
            end: 200,
            cid: cid.clone(),
        }],
    })
    .unwrap();
    let fixture = Offline(VecDeque::from([
        (EVENTS_URL.into(), None, response(200, body)),
        (format!("{BLOB_URL}/{cid}"), None, response(503, Vec::new())),
        (format!("{BLOB_URL}/{cid}"), None, response(200, bytes)),
    ]));
    let tick = Instant::now();
    let mut fetcher = ThemeFetcher::with_transport(&cache.0, fixture, tick);
    assert_eq!(
        fetcher.run_once(time(99), tick),
        Some(Err(EventDataError::NoActiveEvent))
    );
    let active = fetcher
        .run_once(time(100), tick + Duration::from_secs(1))
        .unwrap()
        .unwrap();
    assert_eq!(active.data_folder, cache.0.join(cid));
    checked_zip::verify_folder_checksums(&active.data_folder).unwrap();
}

#[test]
fn worker_exits_on_disconnected_consumer_without_waiting_or_network() {
    let cache = Cache::new();
    let fixture = Offline(VecDeque::from([(
        EVENTS_URL.into(),
        None,
        response(200, br#"{"events":[]}"#.to_vec()),
    )]));
    let fetcher = ThemeFetcher::with_transport(&cache.0, fixture, Instant::now());
    let (sender, receiver) = std::sync::mpsc::channel();
    drop(receiver);
    fetcher.run_with_sender(sender);
}
