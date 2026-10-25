use crate::{
    cache::{Cache, CacheLimits},
    http::{HttpTransport, Transport},
    model::{ActiveEvent, ContentId, ScheduledEvent, ThemeState},
    protocol::EventsResponse,
};
use anyhow::{Context, Result, ensure};
use std::{
    collections::{HashMap, HashSet},
    io::Read,
    path::PathBuf,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

mod subscription;
pub use subscription::ThemeSubscription;

const INITIAL_RETRY: Duration = Duration::from_secs(1);
const MAX_RETRY: Duration = Duration::from_secs(300);

/// Resource limits and endpoints are explicit so private servers and offline
/// transports use the same state machine. Redirects are never followed.
#[derive(Clone, Debug)]
pub struct Config {
    pub cache_dir: PathBuf,
    pub events_url: String,
    pub blobs_url: String,
    pub request_timeout: Duration,
    pub default_refresh: Duration,
    pub cache_max_age: Duration,
    pub max_events: usize,
    pub max_events_bytes: u64,
    pub cache_limits: CacheLimits,
}

impl Config {
    pub fn new(cache_dir: impl Into<PathBuf>) -> Self {
        Self {
            cache_dir: cache_dir.into(),
            events_url: "https://ryeland.pouncelight.games/v1/events".into(),
            blobs_url: "https://ryeland.pouncelight.games/v1/blob".into(),
            request_timeout: Duration::from_secs(30),
            default_refresh: Duration::from_secs(36_000),
            cache_max_age: Duration::from_secs(1_209_600),
            max_events: 1024,
            max_events_bytes: 1024 * 1024,
            cache_limits: CacheLimits::default(),
        }
    }

    fn validate(&self) -> Result<()> {
        ensure!(
            !self.cache_dir.as_os_str().is_empty(),
            "cache directory is empty"
        );
        ensure!(
            self.max_events > 0 && self.max_events_bytes > 0 && self.max_events_bytes < u64::MAX,
            "event limits must be positive and bounded"
        );
        ensure!(
            !self.request_timeout.is_zero() && self.request_timeout <= Duration::from_secs(300),
            "HTTP timeout must be between 1 ns and 300 seconds"
        );
        ensure!(
            !self.default_refresh.is_zero(),
            "refresh interval must be positive"
        );
        for endpoint in [&self.events_url, &self.blobs_url] {
            let url = reqwest::Url::parse(endpoint).context("invalid theme endpoint")?;
            ensure!(
                matches!(url.scheme(), "http" | "https") && url.host_str().is_some(),
                "theme endpoint must be an HTTP(S) URL"
            );
            ensure!(
                url.query().is_none()
                    && url.fragment().is_none()
                    && url.username().is_empty()
                    && url.password().is_none(),
                "theme endpoints cannot contain credentials, queries or fragments"
            );
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Package {
    folder: Option<PathBuf>,
    error: Option<String>,
    next_attempt: Instant,
    retry: Duration,
    last_attempt: u64,
}

impl Package {
    fn pending(now: Instant) -> Self {
        Self {
            folder: None,
            error: None,
            next_attempt: now,
            retry: INITIAL_RETRY,
            last_attempt: 0,
        }
    }
}

/// Single-owner state machine; no per-event locks or shared mutable snapshots.
pub struct ThemeClient<T: Transport = HttpTransport> {
    config: Config,
    transport: T,
    cache: Cache,
    events: Vec<ScheduledEvent>,
    packages: HashMap<ContentId, Package>,
    etag: Option<String>,
    next_refresh: Instant,
    refresh_retry: Duration,
    refresh_error: Option<String>,
    has_schedule: bool,
    wire_schedule: Option<EventsResponse>,
    server_minus_local: i64,
    attempt_sequence: u64,
    accepted_refresh: Duration,
}

impl ThemeClient<HttpTransport> {
    pub fn new(config: Config) -> Result<Self> {
        config.validate()?;
        let transport = HttpTransport::new(config.request_timeout)?;
        Self::with_transport(config, transport)
    }
}

impl<T: Transport> ThemeClient<T> {
    pub fn with_transport(config: Config, transport: T) -> Result<Self> {
        config.validate()?;
        let cache = Cache::new(config.cache_dir.clone(), config.cache_limits.clone())?;
        let report = cache.trim(SystemTime::now(), config.cache_max_age)?;
        for (path, error) in report.errors {
            log::warn!("cannot clean theme cache {}: {error}", path.display());
        }
        let accepted_refresh = config
            .default_refresh
            .clamp(Duration::from_secs(1), Duration::from_secs(86_400));
        Ok(Self {
            config,
            transport,
            cache,
            events: Vec::new(),
            packages: HashMap::new(),
            etag: None,
            next_refresh: Instant::now(),
            refresh_retry: INITIAL_RETRY,
            refresh_error: None,
            has_schedule: false,
            wire_schedule: None,
            server_minus_local: 0,
            attempt_sequence: 0,
            accepted_refresh,
        })
    }

    /// Refresh the schedule if due, then attempt at most one package. Bounded
    /// work per call prevents large schedules from monopolizing the worker.
    pub fn poll(&mut self, now: SystemTime, monotonic: Instant) -> ThemeState {
        self.poll_with_clock(|| now, || monotonic, || false, |_| {})
    }

    fn poll_with_clock(
        &mut self,
        wall_clock: impl Fn() -> SystemTime,
        clock: impl Fn() -> Instant,
        cancelled: impl Fn() -> bool,
        on_update: impl Fn(&Self),
    ) -> ThemeState {
        let monotonic = clock();
        if cancelled() {
            return self.state_at(wall_clock());
        }
        if monotonic >= self.next_refresh {
            match self.refresh(&wall_clock, &clock, &cancelled) {
                Ok(deadline) => {
                    self.refresh_error = None;
                    self.refresh_retry = INITIAL_RETRY;
                    self.next_refresh = deadline;
                }
                Err(error) => {
                    log::warn!("cannot refresh theme schedule: {error:#}");
                    self.refresh_error = Some(format!("{error:#}"));
                    self.next_refresh = clock() + self.refresh_retry;
                    self.refresh_retry = (self.refresh_retry * 2).min(MAX_RETRY);
                }
            }
            on_update(self);
        }
        let now = wall_clock();
        let monotonic = clock();
        // Prefer the selected active event, then prefetch the remaining live
        // schedule. Failed packages have independent deadlines and cannot starve
        // unrelated downloads.
        let selected = self.events.iter().rev().find(|event| event.is_active(now));
        let candidate = selected
            .into_iter()
            .chain(self.events.iter())
            .filter(|event| {
                event.end_time > now
                    && self.packages.get(&event.cid).is_some_and(|package| {
                        package.folder.is_none() && monotonic >= package.next_attempt
                    })
            })
            .min_by_key(|event| self.packages[&event.cid].last_attempt)
            .map(|event| event.cid.clone());
        if let Some(cid) = candidate.filter(|_| !cancelled()) {
            let result = self.load_package(&cid, &cancelled);
            self.attempt_sequence = self.attempt_sequence.saturating_add(1);
            if let Some(package) = self.packages.get_mut(&cid) {
                package.last_attempt = self.attempt_sequence;
                match result {
                    Ok(folder) => {
                        package.folder = Some(folder);
                        package.error = None;
                        package.retry = INITIAL_RETRY;
                    }
                    Err(error) => {
                        package.error = Some(format!("{error:#}"));
                        package.next_attempt = clock() + package.retry;
                        package.retry = (package.retry * 2).min(MAX_RETRY);
                    }
                }
            }
        }
        on_update(self);
        self.state_at(wall_clock())
    }

    /// Select using a fresh wall clock after blocking I/O in production.
    pub fn state_at(&self, now: SystemTime) -> ThemeState {
        self.events
            .iter()
            .rev()
            .find(|event| event.is_active(now))
            .map(|event| self.event_state(event))
            .unwrap_or_else(|| self.inactive_state())
    }

    fn event_state(&self, event: &ScheduledEvent) -> ThemeState {
        if let Some(package) = self.packages.get(&event.cid) {
            if let Some(folder) = &package.folder {
                return ThemeState::Active(ActiveEvent {
                    data_folder: folder.clone(),
                    start_time: event.start_time,
                    end_time: event.end_time,
                });
            }
            if let Some(error) = &package.error {
                return ThemeState::Failed(error.clone());
            }
        }
        ThemeState::Pending
    }

    fn inactive_state(&self) -> ThemeState {
        match &self.refresh_error {
            Some(error) => ThemeState::Failed(error.clone()),
            None if self.has_schedule => ThemeState::Idle,
            None => ThemeState::Pending,
        }
    }

    fn refresh(
        &mut self,
        wall_clock: &impl Fn() -> SystemTime,
        clock: &impl Fn() -> Instant,
        cancelled: &impl Fn() -> bool,
    ) -> Result<Instant> {
        let response = self
            .transport
            .get(&self.config.events_url, self.etag.as_deref())?;
        let local_now = wall_clock();
        let received_at = clock();
        ensure!(
            matches!(response.status, 200 | 304),
            "events HTTP status {}",
            response.status
        );
        let policy = response
            .cache_control
            .as_deref()
            .and_then(cache_control::CacheControl::from_value);
        let can_store = policy.as_ref().is_none_or(|policy| !policy.no_store);
        let must_revalidate = policy.as_ref().is_some_and(|policy| {
            policy.no_store || policy.cachability == Some(cache_control::Cachability::NoCache)
        });
        let fresh_for = policy
            .as_ref()
            .and_then(|policy| policy.max_age)
            .unwrap_or_else(|| {
                if response.status == 304 && response.cache_control.is_none() {
                    self.accepted_refresh
                } else {
                    self.config.default_refresh
                }
            })
            .clamp(Duration::from_secs(1), Duration::from_secs(86_400));
        let fresh_for = if must_revalidate {
            Duration::from_secs(1)
        } else {
            fresh_for
        };
        let refresh = fresh_for
            .saturating_sub(response.age.unwrap_or_default())
            .max(Duration::from_secs(1));
        let skew = response
            .date
            .as_deref()
            .map(|date| -> Result<i64> {
                let server = httpdate::parse_http_date(date)
                    .context("invalid server date")?
                    .duration_since(UNIX_EPOCH)
                    .context("server date before epoch")?
                    .as_secs();
                let local = local_now
                    .duration_since(UNIX_EPOCH)
                    .context("local time before epoch")?
                    .as_secs();
                i64::try_from(
                    i128::from(server) + i128::from(response.age.unwrap_or_default().as_secs())
                        - i128::from(local),
                )
                .context("clock offset out of range")
            })
            .transpose()?
            .unwrap_or(self.server_minus_local);
        if response.status == 304 {
            ensure!(
                self.has_schedule && self.etag.is_some(),
                "304 without an accepted conditional schedule"
            );
            if let Some(wire) = &self.wire_schedule {
                let events = wire.validate(skew, self.config.max_events)?;
                self.events = events;
                self.server_minus_local = skew;
            }
            if !can_store {
                self.etag = None;
            } else if response.etag.is_some() {
                self.etag = response.etag;
            }
            self.accepted_refresh = fresh_for;
            return Ok(received_at + refresh);
        }
        let limit = self.config.max_events_bytes;
        ensure!(
            response.content_length.is_none_or(|length| length <= limit),
            "events response exceeds size limit"
        );
        let mut bytes = Vec::new();
        CancellableReader {
            inner: response.body,
            cancelled,
        }
        .take(limit + 1)
        .read_to_end(&mut bytes)
        .context("reading events response")?;
        ensure!(
            bytes.len() as u64 <= limit,
            "events response exceeds size limit"
        );
        ensure!(
            response
                .content_length
                .is_none_or(|length| length == bytes.len() as u64),
            "events response length mismatch"
        );
        let wire =
            serde_json::from_slice::<EventsResponse>(&bytes).context("decoding events response")?;
        let events = wire.validate(skew, self.config.max_events)?;
        let needed: HashSet<_> = events.iter().map(|event| event.cid.clone()).collect();
        self.packages.retain(|cid, _| needed.contains(cid));
        for cid in needed {
            self.packages
                .entry(cid)
                .or_insert_with(|| Package::pending(clock()));
        }
        // Commit only after the complete response has passed validation. A body
        // failure must not make the next conditional request acknowledge it.
        self.events = events;
        self.wire_schedule = Some(wire);
        self.server_minus_local = skew;
        self.etag = if can_store { response.etag } else { None };
        self.has_schedule = true;
        self.accepted_refresh = fresh_for;
        Ok(received_at + refresh)
    }

    fn load_package(&mut self, cid: &ContentId, cancelled: &impl Fn() -> bool) -> Result<PathBuf> {
        if let Some(folder) = self.cache.lookup(cid)? {
            return Ok(folder);
        }
        ensure!(!cancelled(), "theme download cancelled");
        let url = format!("{}/{cid}", self.config.blobs_url.trim_end_matches('/'));
        let response = self.transport.get(&url, None)?;
        ensure!(
            response.status == 200,
            "blob HTTP status {}",
            response.status
        );
        ensure!(
            response
                .content_length
                .is_none_or(|length| length <= self.config.cache_limits.max_archive_bytes),
            "blob response exceeds size limit"
        );
        // The cache streams, hashes and bounds the body before extracting it.
        self.cache.install(
            cid,
            CancellableReader {
                inner: response.body,
                cancelled,
            },
        )
    }

    fn wait_duration(&self, now: SystemTime, monotonic: Instant) -> Duration {
        // Recheck wall-clock jumps at least once per minute.
        let mut wait = self
            .next_refresh
            .saturating_duration_since(monotonic)
            .min(Duration::from_secs(60));
        for event in &self.events {
            for boundary in [event.start_time, event.end_time] {
                if let Ok(until) = boundary.duration_since(now)
                    && !until.is_zero()
                {
                    wait = wait.min(until);
                }
            }
            if event.end_time > now
                && let Some(package) = self
                    .packages
                    .get(&event.cid)
                    .filter(|package| package.folder.is_none())
            {
                wait = wait.min(package.next_attempt.saturating_duration_since(monotonic));
            }
        }
        wait
    }

    pub fn spawn(self) -> Result<ThemeSubscription>
    where
        T: 'static,
    {
        ThemeSubscription::spawn(self)
    }
}

struct CancellableReader<R, F> {
    inner: R,
    cancelled: F,
}

impl<R: Read, F: Fn() -> bool> Read for CancellableReader<R, F> {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        if (self.cancelled)() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "theme download cancelled",
            ));
        }
        self.inner.read(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::Response;
    use std::{
        collections::VecDeque,
        io::Cursor,
        sync::Arc,
        sync::atomic::{AtomicU64, Ordering},
    };

    struct AdvancingTransport {
        elapsed: Arc<AtomicU64>,
        replies: VecDeque<(u64, u64, u16)>,
    }
    impl Transport for AdvancingTransport {
        fn get(&mut self, _: &str, _: Option<&str>) -> Result<Response> {
            let (delay, body_delay, status) =
                self.replies.pop_front().context("unexpected request")?;
            let elapsed = self.elapsed.fetch_add(delay, Ordering::Relaxed) + delay;
            let body = br#"{"events":[]}"#.to_vec();
            Ok(Response {
                status,
                age: None,
                date: Some(httpdate::fmt_http_date(
                    UNIX_EPOCH + Duration::from_secs(90 + elapsed),
                )),
                etag: None,
                cache_control: Some("max-age=5".into()),
                content_length: Some(body.len() as u64),
                body: Box::new(DelayedBody {
                    inner: Cursor::new(body),
                    elapsed: self.elapsed.clone(),
                    delay: body_delay,
                }),
            })
        }
    }

    struct DelayedBody {
        inner: Cursor<Vec<u8>>,
        elapsed: Arc<AtomicU64>,
        delay: u64,
    }
    impl Read for DelayedBody {
        fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
            self.elapsed
                .fetch_add(std::mem::take(&mut self.delay), Ordering::Relaxed);
            self.inner.read(buffer)
        }
    }

    #[test]
    fn clocks_are_sampled_after_io_and_success_resets_backoff() {
        let directory = std::env::temp_dir().join(format!("tg-theme-clock-{}", std::process::id()));
        let elapsed = Arc::new(AtomicU64::new(0));
        let transport = AdvancingTransport {
            elapsed: elapsed.clone(),
            replies: VecDeque::from([(10, 20, 200), (20, 0, 500), (3, 0, 200)]),
        };
        let mut client = ThemeClient::with_transport(Config::new(&directory), transport).unwrap();
        let tick = Instant::now();
        let wall = || UNIX_EPOCH + Duration::from_secs(90 + elapsed.load(Ordering::Relaxed));
        let clock = || tick + Duration::from_secs(elapsed.load(Ordering::Relaxed));
        assert_eq!(
            client.poll_with_clock(wall, clock, || false, |_| {}),
            ThemeState::Idle
        );
        assert_eq!(
            client.server_minus_local, 0,
            "network delay is not clock skew"
        );
        assert_eq!(
            client.next_refresh,
            tick + Duration::from_secs(15),
            "body reading must consume freshness lifetime"
        );
        assert_eq!(elapsed.load(Ordering::Relaxed), 30);
        assert!(matches!(
            client.poll_with_clock(wall, clock, || false, |_| {}),
            ThemeState::Failed(_)
        ));
        assert_eq!(client.next_refresh, tick + Duration::from_secs(51));
        assert_eq!(client.refresh_retry, Duration::from_secs(2));
        elapsed.store(51, Ordering::Relaxed);
        assert_eq!(
            client.poll_with_clock(wall, clock, || false, |_| {}),
            ThemeState::Idle
        );
        assert_eq!(client.next_refresh, tick + Duration::from_secs(59));
        assert_eq!(client.refresh_retry, INITIAL_RETRY);
        drop(client);
        std::fs::remove_dir_all(directory).unwrap();
    }
}
