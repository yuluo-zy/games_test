//! End-to-end delivery checks with owned, streaming offline HTTP responses.
use anyhow::{Result, bail};
use std::{
    collections::VecDeque,
    fs,
    io::{Cursor, Write},
    path::PathBuf,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
        mpsc,
    },
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use tg_theme::{
    Config, ThemeClient, ThemeState,
    http::{Response, Transport},
};

static NEXT: AtomicU64 = AtomicU64::new(0);
struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "tg-delivery-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

type Calls = Arc<Mutex<Vec<(String, Option<String>)>>>;
struct Mock {
    replies: VecDeque<Response>,
    calls: Calls,
    dropped: Option<mpsc::Sender<()>>,
}
impl Mock {
    fn new(replies: Vec<Response>) -> (Self, Calls) {
        let calls = Arc::new(Mutex::new(Vec::new()));
        (
            Self {
                replies: replies.into(),
                calls: calls.clone(),
                dropped: None,
            },
            calls,
        )
    }
}
impl Drop for Mock {
    fn drop(&mut self) {
        if let Some(sender) = self.dropped.take() {
            let _ = sender.send(());
        }
    }
}
impl Transport for Mock {
    fn get(&mut self, url: &str, etag: Option<&str>) -> Result<Response> {
        self.calls
            .lock()
            .unwrap()
            .push((url.to_owned(), etag.map(str::to_owned)));
        match self.replies.pop_front() {
            Some(response) => Ok(response),
            None => bail!("unexpected request: {url}"),
        }
    }
}
fn response(status: u16, bytes: Vec<u8>, etag: Option<&str>) -> Response {
    Response {
        status,
        date: None,
        age: None,
        etag: etag.map(str::to_owned),
        cache_control: Some("max-age=10".into()),
        content_length: Some(bytes.len() as u64),
        body: Box::new(Cursor::new(bytes)),
    }
}
fn schedule(events: &[(&str, i64, i64)], etag: Option<&str>) -> Response {
    let events: Vec<_> = events
        .iter()
        .map(|(cid, start, end)| serde_json::json!({"cid":cid,"start":start,"end":end}))
        .collect();
    response(
        200,
        serde_json::to_vec(&serde_json::json!({"events":events})).unwrap(),
        etag,
    )
}
fn archive(contents: &str) -> (String, Vec<u8>) {
    let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
    writer
        .start_file("meta.json", zip::write::FileOptions::default())
        .unwrap();
    writer.write_all(contents.as_bytes()).unwrap();
    let bytes = writer.finish().unwrap().into_inner();
    (blake3::hash(&bytes).to_string(), bytes)
}
fn time(seconds: u64) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs(seconds)
}
fn client(temp: &Temp, replies: Vec<Response>) -> (ThemeClient<Mock>, Calls) {
    let (mock, calls) = Mock::new(replies);
    (
        ThemeClient::with_transport(Config::new(&temp.0), mock).unwrap(),
        calls,
    )
}

#[test]
fn rejected_schedule_does_not_commit_etag_and_304_keeps_accepted_data() {
    let temp = Temp::new();
    let (cid, bytes) = archive("{}");
    let (mut client, calls) = client(
        &temp,
        vec![
            response(200, b"broken json".to_vec(), Some("broken")),
            schedule(&[(&cid, 1, 100)], Some("accepted")),
            response(200, bytes, None),
            response(304, vec![], Some("ignored")),
        ],
    );
    let tick = Instant::now();
    assert!(matches!(client.poll(time(10), tick), ThemeState::Failed(_)));
    assert!(matches!(
        client.poll(time(11), tick + Duration::from_secs(1)),
        ThemeState::Active(_)
    ));
    assert!(matches!(
        client.poll(time(21), tick + Duration::from_secs(11)),
        ThemeState::Active(_)
    ));
    let requests = calls.lock().unwrap();
    assert_eq!(requests.len(), 4);
    assert_eq!(requests[0].1, None);
    assert_eq!(
        requests[1].1, None,
        "failed response ETag must not be acknowledged"
    );
    assert_eq!(requests[3].1.as_deref(), Some("accepted"));
}

#[test]
fn refresh_reuses_package_and_last_overlap_has_half_open_boundaries() {
    let temp = Temp::new();
    let (cid, bytes) = archive("{\"version\":1}");
    let (mut client, calls) = client(
        &temp,
        vec![
            schedule(&[(&cid, 10, 30), (&cid, 20, 40)], Some("one")),
            response(200, bytes, None),
            schedule(&[(&cid, 10, 30), (&cid, 20, 40)], Some("two")),
        ],
    );
    let tick = Instant::now();
    assert!(matches!(client.poll(time(10), tick), ThemeState::Active(_)));
    assert!(matches!(
        client.poll(time(20), tick + Duration::from_secs(10)),
        ThemeState::Active(_)
    ));
    let ThemeState::Active(event) = client.state_at(time(20)) else {
        panic!("overlap should be active")
    };
    assert_eq!(event.start_time, time(20));
    assert_eq!(client.state_at(time(9)), ThemeState::Idle);
    assert!(matches!(client.state_at(time(39)), ThemeState::Active(_)));
    assert_eq!(client.state_at(time(40)), ThemeState::Idle);
    let requests = calls.lock().unwrap();
    assert_eq!(
        requests.len(),
        3,
        "refresh must preserve downloaded CID state"
    );
    assert_eq!(
        requests
            .iter()
            .filter(|(url, _)| url.contains("/blob/"))
            .count(),
        1
    );
}

#[test]
fn failed_selected_package_does_not_starve_other_downloads() {
    let temp = Temp::new();
    let (good, bytes) = archive("good");
    let bad = "b".repeat(64);
    let (mut client, calls) = client(
        &temp,
        vec![
            schedule(&[(&good, 1, 100), (&bad, 1, 100)], None),
            response(500, vec![], None),
            response(200, bytes, None),
        ],
    );
    let tick = Instant::now();
    assert!(matches!(client.poll(time(10), tick), ThemeState::Failed(_)));
    // Advance to the retry deadline: both packages are eligible, and fairness
    // must still give the unrelated package a turn.
    assert!(matches!(
        client.poll(time(11), tick + Duration::from_secs(1)),
        ThemeState::Failed(_)
    ));
    let requests = calls.lock().unwrap();
    assert_eq!(requests.len(), 3);
    assert!(requests[1].0.ends_with(&bad));
    assert!(requests[2].0.ends_with(&good));
    assert!(temp.0.join(good).join("meta.json").is_file());
}

#[test]
fn response_count_and_stream_size_limits_reject_before_package_download() {
    for oversized_stream in [false, true] {
        let temp = Temp::new();
        let (mut mock, calls) = Mock::new(vec![if oversized_stream {
            let mut reply = response(200, vec![b' '; 65], None);
            reply.content_length = None; // Exercise the streaming bound too.
            reply
        } else {
            schedule(
                &[(&"a".repeat(64), 1, 100), (&"b".repeat(64), 1, 100)],
                None,
            )
        }]);
        mock.dropped = None;
        let mut config = Config::new(&temp.0);
        config.max_events = 1;
        if oversized_stream {
            config.max_events_bytes = 64;
        }
        let mut client = ThemeClient::with_transport(config, mock).unwrap();
        assert!(matches!(
            client.poll(time(10), Instant::now()),
            ThemeState::Failed(_)
        ));
        assert_eq!(calls.lock().unwrap().len(), 1);
    }
}

#[test]
fn dropping_consumed_subscription_wakes_unchanged_worker_and_shutdown_joins() {
    for explicit_shutdown in [false, true] {
        let temp = Temp::new();
        let (mut mock, _) = Mock::new(vec![schedule(&[], Some("empty"))]);
        let (sender, receiver) = mpsc::channel();
        mock.dropped = Some(sender);
        let client = ThemeClient::with_transport(Config::new(&temp.0), mock).unwrap();
        let mut subscription = client.spawn().unwrap();
        let deadline = Instant::now() + Duration::from_secs(3);
        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            assert!(
                !remaining.is_zero(),
                "worker did not publish accepted empty schedule"
            );
            if subscription.recv_timeout(remaining) == Some(ThemeState::Idle) {
                break;
            }
        }
        if explicit_shutdown {
            subscription.shutdown().unwrap();
        } else {
            drop(subscription);
        }
        receiver
            .recv_timeout(Duration::from_secs(3))
            .expect("transport must drop even without a subsequent state change");
    }
}

#[test]
fn conditional_response_recalculates_boundaries_after_clock_correction() {
    let temp = Temp::new();
    let (cid, bytes) = archive("clock correction");
    let mut initial = schedule(&[(&cid, 100, 200)], Some("schedule"));
    initial.date = Some(httpdate::fmt_http_date(time(100)));
    let mut unchanged = response(304, vec![], None);
    unchanged.date = Some(httpdate::fmt_http_date(time(110)));
    let (mut client, calls) = client(&temp, vec![initial, response(200, bytes, None), unchanged]);
    let tick = Instant::now();
    let ThemeState::Active(first) = client.poll(time(90), tick) else {
        panic!("initial corrected schedule should be active")
    };
    assert_eq!((first.start_time, first.end_time), (time(90), time(190)));
    let ThemeState::Active(corrected) = client.poll(time(110), tick + Duration::from_secs(10))
    else {
        panic!("304 should preserve downloaded package")
    };
    assert_eq!(
        (corrected.start_time, corrected.end_time),
        (time(100), time(200))
    );
    assert!(matches!(client.state_at(time(199)), ThemeState::Active(_)));
    assert_eq!(client.state_at(time(200)), ThemeState::Idle);
    let requests = calls.lock().unwrap();
    assert_eq!(requests.len(), 3);
    assert_eq!(requests[2].1.as_deref(), Some("schedule"));
}

#[test]
fn cached_date_uses_age_and_headerless_304_keeps_refresh_policy() {
    let temp = Temp::new();
    let (cid, bytes) = archive("cached response");
    let mut initial = schedule(&[(&cid, 100, 200)], Some("cached"));
    initial.date = Some(httpdate::fmt_http_date(time(100)));
    initial.age = Some(Duration::from_secs(50));
    initial.cache_control = Some("max-age=60".into());
    let unchanged = |date| {
        let mut reply = response(304, vec![], None);
        reply.cache_control = None;
        reply.date = Some(httpdate::fmt_http_date(time(date)));
        reply.age = Some(Duration::from_secs(50));
        reply
    };
    let (mut client, calls) = client(
        &temp,
        vec![
            initial,
            response(200, bytes, None),
            unchanged(110),
            unchanged(120),
        ],
    );
    let tick = Instant::now();
    let ThemeState::Active(active) = client.poll(time(150), tick) else {
        panic!("cached schedule is active")
    };
    assert_eq!((active.start_time, active.end_time), (time(100), time(200)));
    client.poll(time(160), tick + Duration::from_secs(10));
    client.poll(time(169), tick + Duration::from_secs(19));
    assert_eq!(calls.lock().unwrap().len(), 3);
    client.poll(time(170), tick + Duration::from_secs(20));
    assert_eq!(
        calls.lock().unwrap().len(),
        4,
        "304 inherits the 60s lifetime minus 50s Age"
    );
}

struct GatedTransport {
    inner: Mock,
    entered: mpsc::Sender<()>,
    release: mpsc::Receiver<()>,
}
impl Transport for GatedTransport {
    fn get(&mut self, url: &str, etag: Option<&str>) -> Result<Response> {
        if self.inner.calls.lock().unwrap().len() == 2 {
            self.entered.send(())?;
            self.release.recv_timeout(Duration::from_secs(6))?;
        }
        self.inner.get(url, etag)
    }
}

#[test]
fn subscription_expires_active_theme_while_prefetch_is_blocked() {
    let temp = Temp::new();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let (current, first_bytes) = archive("active");
    let (future, second_bytes) = archive("prefetch");
    let (inner, _) = Mock::new(vec![
        schedule(
            &[
                (&current, (now - 1) as i64, (now + 3) as i64),
                (&future, (now + 100) as i64, (now + 200) as i64),
            ],
            None,
        ),
        response(200, first_bytes, None),
        response(200, second_bytes, None),
    ]);
    let (entered, waiting) = mpsc::channel();
    let (release, gate) = mpsc::channel();
    let mut client = ThemeClient::with_transport(
        Config::new(&temp.0),
        GatedTransport {
            inner,
            entered,
            release: gate,
        },
    )
    .unwrap();
    assert!(matches!(
        client.poll(time(now), Instant::now()),
        ThemeState::Active(_)
    ));
    let mut subscription = client.spawn().unwrap();
    waiting
        .recv_timeout(Duration::from_secs(2))
        .expect("prefetch reached the gate");
    assert!(matches!(
        subscription.recv_timeout(Duration::ZERO),
        Some(ThemeState::Active(_))
    ));
    let expired = subscription.recv_timeout(Duration::from_secs(4));
    release.send(()).unwrap();
    subscription.shutdown().unwrap();
    assert_eq!(
        expired,
        Some(ThemeState::Idle),
        "expiry must not wait for blocked prefetch"
    );
}

#[test]
fn worker_panic_notifies_consumers_and_is_reported_by_shutdown() {
    struct PanickingTransport;
    impl Transport for PanickingTransport {
        fn get(&mut self, _: &str, _: Option<&str>) -> Result<Response> {
            panic!("transport fixture panic")
        }
    }
    let temp = Temp::new();
    let mut subscription = ThemeClient::with_transport(Config::new(&temp.0), PanickingTransport)
        .unwrap()
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        let remaining = deadline.saturating_duration_since(Instant::now());
        assert!(!remaining.is_zero(), "panic must wake the subscription");
        if let Some(ThemeState::Failed(error)) = subscription.recv_timeout(remaining) {
            assert!(error.contains("terminated"));
            break;
        }
    }
    assert_eq!(subscription.recv_timeout(Duration::from_secs(1)), None);
    assert!(subscription.shutdown().is_err());
}

#[test]
fn revalidation_directives_override_max_age_and_no_store_discards_validator() {
    for (directive, conditional) in [
        ("no-cache, max-age=3600", true),
        ("no-store, max-age=3600", false),
    ] {
        let temp = Temp::new();
        let mut initial = schedule(&[], Some("accepted"));
        initial.cache_control = Some(directive.into());
        let (mut client, calls) = client(&temp, vec![initial, schedule(&[], None)]);
        let tick = Instant::now();
        assert_eq!(client.poll(time(10), tick), ThemeState::Idle);
        assert_eq!(
            client.poll(time(11), tick + Duration::from_secs(1)),
            ThemeState::Idle
        );
        let calls = calls.lock().unwrap();
        assert_eq!(
            calls.len(),
            2,
            "{directive} must revalidate at the minimum polling interval"
        );
        assert_eq!(calls[1].1.as_deref(), conditional.then_some("accepted"));
    }
}
