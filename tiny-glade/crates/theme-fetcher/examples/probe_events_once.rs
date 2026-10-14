//! Explicit live probe. One GET, redirects disabled, no retries or blob downloads.
use anyhow::Result;
use std::{
    fs,
    path::PathBuf,
    time::{Duration, Instant, SystemTime},
};
use theme_fetcher::{EVENTS_URL, HttpResponse, ReqwestTransport, ThemeFetcher, Transport};

struct Probe {
    inner: ReqwestTransport,
    folder: PathBuf,
    calls: usize,
    response: Option<serde_json::Value>,
}

impl Transport for Probe {
    fn get(&mut self, url: &str, etag: Option<&str>) -> Result<HttpResponse> {
        assert_eq!(url, EVENTS_URL);
        assert_eq!(self.calls, 0, "only one request is allowed");
        self.calls += 1;
        fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(self.folder.join("request-attempted"))?;
        let response = self.inner.get(url, etag)?;
        self.response = Some(serde_json::json!({
            "status": response.status, "date": response.date,
            "etag": response.etag, "cache_control": response.cache_control,
            "content_length": response.content_length
        }));
        fs::write(
            self.folder.join("headers.json"),
            serde_json::to_vec_pretty(&self.response)?,
        )?;
        Ok(response)
    }

    fn read_body(&mut self, body: std::result::Result<Vec<u8>, String>) -> Result<Vec<u8>> {
        let body = self.inner.read_body(body)?;
        fs::write(self.folder.join("events-response.json"), &body)?;
        if let Some(response) = self.response.as_mut() {
            response["body_bytes"] = serde_json::json!(body.len());
            response["body_blake3"] = serde_json::json!(blake3::hash(&body).to_string());
        }
        fs::write(
            self.folder.join("headers.json"),
            serde_json::to_vec_pretty(&self.response)?,
        )?;
        Ok(body)
    }

    fn response_local_time(&self) -> Option<SystemTime> {
        self.inner.response_local_time()
    }

    fn response_header_error(&self, name: reqwest::header::HeaderName) -> Option<&str> {
        self.inner.response_header_error(name)
    }
}

fn replay(folder: PathBuf) -> Result<()> {
    struct Captured(Option<HttpResponse>);
    impl Transport for Captured {
        fn get(&mut self, url: &str, etag: Option<&str>) -> Result<HttpResponse> {
            assert_eq!(url, EVENTS_URL);
            assert!(etag.is_none());
            self.0
                .take()
                .ok_or_else(|| anyhow::anyhow!("capture already consumed"))
        }
    }
    let headers: serde_json::Value =
        serde_json::from_slice(&fs::read(folder.join("headers.json"))?)?;
    let body = fs::read(folder.join("events-response.json"))?;
    let wire: theme_server_proto::EventsResponse = serde_json::from_slice(&body)?;
    let string = |name: &str| headers[name].as_str().map(str::to_owned);
    let captured = Captured(Some(HttpResponse {
        status: headers["status"].as_u64().expect("captured status") as u16,
        date: string("date"),
        etag: string("etag"),
        cache_control: string("cache_control"),
        content_length: headers["content_length"].as_u64(),
        body: Ok(body),
    }));
    let mut fetcher =
        ThemeFetcher::with_transport(folder.join("unused-cache"), captured, Instant::now());
    let (outcome, refresh) = fetcher.fetch_events(SystemTime::now())?;
    let report = serde_json::json!({
        "url": EVENTS_URL, "live_get_invocations": 1, "replay_network_requests": 0,
        "redirects_enabled": false, "blob_downloads": 0,
        "response": headers, "producer": {
            "ok": true, "verification": "offline replay of the one captured live response",
            "outcome": format!("{outcome:?}"), "refresh_seconds": refresh.as_secs(),
            "events_count": fetcher.events.load().len()
        }, "wire_parse": {"ok": true, "events_count": wire.events.len(), "events": wire.events},
        "initial_probe_reporting_error": "serde_json 1.0.85 rejected elapsed_ms u128 after receiving/parsing; corrected to u64; no second request"
    });
    fs::write(
        folder.join("result.json"),
        serde_json::to_vec_pretty(&report)?,
    )?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

fn main() -> Result<()> {
    let mut args = std::env::args_os().skip(1);
    let first = args
        .next()
        .expect("supply output folder, or --replay output folder");
    if first == "--replay" {
        return replay(PathBuf::from(args.next().expect("supply capture folder")));
    }
    let folder = PathBuf::from(first);
    fs::create_dir_all(&folder)?;
    anyhow::ensure!(
        !folder.join("request-attempted").exists(),
        "select a fresh output folder; a request was already attempted here"
    );
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(60))
        .pool_max_idle_per_host(0)
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    let probe = Probe {
        inner: ReqwestTransport::new(client),
        folder: folder.clone(),
        calls: 0,
        response: None,
    };
    let mut fetcher =
        ThemeFetcher::with_transport(folder.join("unused-cache"), probe, Instant::now());
    let started = Instant::now();
    let result = fetcher.fetch_events(SystemTime::now());
    // The producer parser ran above. Parse saved bytes without another request
    // to record wire timestamps/CIDs independently of the local-time projection.
    let parsed = fs::read(folder.join("events-response.json"))
        .ok()
        .map(|body| serde_json::from_slice::<theme_server_proto::EventsResponse>(&body));
    let parse_report = match parsed {
        Some(Ok(events)) => serde_json::json!({
            "ok": true, "events_count": events.events.len(), "events": events.events
        }),
        Some(Err(error)) => serde_json::json!({"ok": false, "error": error.to_string()}),
        None => serde_json::json!({"ok": false, "error": "no 200 response body captured"}),
    };
    let producer = match result {
        Ok((outcome, refresh)) => serde_json::json!({
            "ok": true, "outcome": format!("{outcome:?}"),
            "refresh_seconds": refresh.as_secs(), "events_count": fetcher.events.load().len()
        }),
        Err(error) => serde_json::json!({"ok": false, "error": format!("{error:#}")}),
    };
    let report = serde_json::json!({
        "url": EVENTS_URL, "elapsed_ms": started.elapsed().as_millis() as u64,
        "get_invocations": 1, "redirects_enabled": false, "blob_downloads": 0,
        "response": fs::read(folder.join("headers.json")).ok()
            .and_then(|bytes| serde_json::from_slice::<serde_json::Value>(&bytes).ok()),
        "producer": producer, "wire_parse": parse_report
    });
    fs::write(
        folder.join("result.json"),
        serde_json::to_vec_pretty(&report)?,
    )?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}
