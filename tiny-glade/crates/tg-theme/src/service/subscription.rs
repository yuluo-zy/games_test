//! Latest-value delivery with time-based selection independent of blocking I/O.
use super::ThemeClient;
use crate::{http::Transport, model::ThemeState};
use anyhow::{Context, Result};
use std::{
    sync::{Arc, Condvar, Mutex},
    thread::{self, JoinHandle},
    time::{Duration, Instant, SystemTime},
};

struct Publication {
    events: Vec<(SystemTime, SystemTime, ThemeState)>,
    fallback: ThemeState,
}

impl Publication {
    fn from_client<T: Transport>(client: &ThemeClient<T>) -> Self {
        Self {
            events: client
                .events
                .iter()
                .map(|event| (event.start_time, event.end_time, client.event_state(event)))
                .collect(),
            fallback: client.inactive_state(),
        }
    }

    fn value_at(&self, now: SystemTime) -> ThemeState {
        self.events
            .iter()
            .rev()
            .find(|(start, end, _)| *start <= now && now < *end)
            .map(|(_, _, value)| value)
            .unwrap_or(&self.fallback)
            .clone()
    }

    fn until_boundary(&self, now: SystemTime) -> Duration {
        self.events
            .iter()
            .flat_map(|(start, end, _)| [start, end])
            .filter_map(|boundary| boundary.duration_since(now).ok())
            .filter(|duration| !duration.is_zero())
            .min()
            .unwrap_or(Duration::from_secs(60))
            .min(Duration::from_secs(60))
    }
}

struct State {
    publication: Publication,
    stopped: bool,
}
struct Shared {
    state: Mutex<State>,
    changed: Condvar,
}

impl Shared {
    fn is_stopped(&self) -> bool {
        self.state
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .stopped
    }

    fn publish<T: Transport>(&self, client: &ThemeClient<T>) {
        let publication = Publication::from_client(client);
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        if !state.stopped {
            state.publication = publication;
            self.changed.notify_all();
        }
    }
}

/// Wakes consumers during unwinding as well as ordinary shutdown. The join
/// handle still retains the original panic for explicit shutdown to report.
struct WorkerExit(Arc<Shared>);
impl Drop for WorkerExit {
    fn drop(&mut self) {
        let mut state = self
            .0
            .state
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        if !state.stopped {
            state.publication = Publication {
                events: Vec::new(),
                fallback: ThemeState::Failed("theme worker terminated unexpectedly".into()),
            };
        }
        state.stopped = true;
        self.0.changed.notify_all();
    }
}

/// A latest-value subscription, without an unbounded notification queue.
/// Activity boundaries are evaluated by the subscriber even while the worker
/// is performing I/O. Drop requests cancellation; shutdown also joins the worker.
pub struct ThemeSubscription {
    shared: Arc<Shared>,
    last_seen: Option<ThemeState>,
    worker: Option<JoinHandle<()>>,
}

impl ThemeSubscription {
    pub(super) fn spawn<T: Transport + 'static>(mut client: ThemeClient<T>) -> Result<Self> {
        let shared = Arc::new(Shared {
            state: Mutex::new(State {
                publication: Publication::from_client(&client),
                stopped: false,
            }),
            changed: Condvar::new(),
        });
        let worker_shared = shared.clone();
        let worker = thread::Builder::new()
            .name("theme-delivery".into())
            .spawn(move || {
                let _exit = WorkerExit(worker_shared.clone());
                loop {
                    if worker_shared.is_stopped() {
                        break;
                    }
                    client.poll_with_clock(
                        SystemTime::now,
                        Instant::now,
                        || worker_shared.is_stopped(),
                        |client| worker_shared.publish(client),
                    );
                    let wait = client.wait_duration(SystemTime::now(), Instant::now());
                    let state = worker_shared
                        .state
                        .lock()
                        .unwrap_or_else(|error| error.into_inner());
                    let (state, _) = worker_shared
                        .changed
                        .wait_timeout_while(state, wait, |state| !state.stopped)
                        .unwrap_or_else(|error| error.into_inner());
                    if state.stopped {
                        break;
                    }
                }
            })
            .context("starting theme worker")?;
        Ok(Self {
            shared,
            last_seen: None,
            worker: Some(worker),
        })
    }

    /// Wait for a different current state, returning None on timeout or after
    /// the worker's final state has been consumed. Panics in custom transports
    /// produce a Failed state rather than leaving consumers waiting indefinitely.
    pub fn recv_timeout(&mut self, timeout: Duration) -> Option<ThemeState> {
        let deadline = Instant::now().checked_add(timeout);
        let mut state = self
            .shared
            .state
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        loop {
            let now = SystemTime::now();
            let value = state.publication.value_at(now);
            if self.last_seen.as_ref() != Some(&value) {
                self.last_seen = Some(value.clone());
                return Some(value);
            }
            if state.stopped {
                return None;
            }
            let remaining = deadline
                .map(|deadline| deadline.saturating_duration_since(Instant::now()))
                .unwrap_or(Duration::from_secs(60));
            if remaining.is_zero() {
                return None;
            }
            let wait = remaining.min(state.publication.until_boundary(now));
            (state, _) = self
                .shared
                .changed
                .wait_timeout(state, wait)
                .unwrap_or_else(|error| error.into_inner());
        }
    }

    /// Cancel and join. In-flight network requests respect their configured
    /// deadline; filesystem operations and custom transports must finish too.
    pub fn shutdown(mut self) -> Result<()> {
        self.cancel();
        if let Some(worker) = self.worker.take() {
            worker
                .join()
                .map_err(|_| anyhow::anyhow!("theme worker panicked"))?;
        }
        Ok(())
    }

    fn cancel(&self) {
        self.shared
            .state
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .stopped = true;
        self.shared.changed.notify_all();
    }
}

impl Drop for ThemeSubscription {
    fn drop(&mut self) {
        self.cancel();
    }
}
