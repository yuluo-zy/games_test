//! One optional saved-background read/decode, with bounded owned publication.
//! The worker owns no World, renderer, TextureHost, device or library handles.
//! Dropping the loading App cancels publication and disconnects the one-slot
//! result channel. File/codec calls are not forcibly interrupted: the detached
//! CPU-only worker drops its owned bytes on return, without blocking OS input.
use std::{path::{Path, PathBuf}, sync::{Arc, atomic::{AtomicBool, Ordering}}};

use crossbeam_channel::{Receiver, TryRecvError};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoadingBackgroundStatus { Pending, Ready }

type Decoded = Result<image::RgbaImage, DecodeFailure>;

enum DecodeFailure { Optional(String), Panic(String) }

struct Pending {
    result: Receiver<Decoded>,
    cancelled: Arc<AtomicBool>,
    path: PathBuf,
}

enum State {
    Unrequested,
    Pending(Pending),
    Ready(Option<image::RgbaImage>),
    Consumed,
    Failed(String),
}

pub(super) struct BackgroundPreparation { state: State }

impl Default for BackgroundPreparation {
    fn default() -> Self { Self { state: State::Unrequested } }
}

impl BackgroundPreparation {
    pub(super) fn needs_selection(&self) -> bool { matches!(&self.state, State::Unrequested) }

    pub(super) fn is_ready(&self) -> bool { matches!(&self.state, State::Ready(_) | State::Consumed) }

    pub(super) fn is_pending(&self) -> bool { matches!(&self.state, State::Pending(_)) }

    pub(super) fn poll(&mut self, path: Option<PathBuf>) -> Result<LoadingBackgroundStatus, String> {
        self.begin(path, |path| image::open(path).map(|image| image.to_rgba8()).map_err(|error| error.to_string()));
        self.poll_result()
    }

    fn begin(&mut self, path: Option<PathBuf>, decode: impl FnOnce(&Path) -> Result<image::RgbaImage, String> + Send + 'static) {
        if !matches!(&self.state, State::Unrequested) { return; }
        let Some(path) = path else {
            self.state = State::Ready(None);
            return;
        };
        let (sender, result) = crossbeam_channel::bounded(1);
        let cancelled = Arc::new(AtomicBool::new(false));
        let worker_cancelled = cancelled.clone();
        let worker_path = path.clone();
        match std::thread::Builder::new().name("loading background decode".into()).spawn(move || {
            if worker_cancelled.load(Ordering::Acquire) { return; }
            let decoded = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| decode(&worker_path).map_err(DecodeFailure::Optional)))
                .unwrap_or_else(|payload| {
                    let message = payload.downcast_ref::<String>().map(String::as_str)
                        .or_else(|| payload.downcast_ref::<&'static str>().copied())
                        .unwrap_or("non-string panic payload");
                    Err(DecodeFailure::Panic(format!("background decoder panicked: {message}")))
                });
            if worker_cancelled.load(Ordering::Acquire) { return; }
            // One producer, one result, one slot: never wait for the renderer.
            // Disconnect or cancellation drops the complete owned image here.
            let _ = sender.try_send(decoded);
        }) {
            Ok(worker) => {
                drop(worker); // CPU-only task; no joining from frame callbacks.
                self.state = State::Pending(Pending { result, cancelled, path });
            }
            Err(error) => {
                self.state = State::Failed(format!("Loading background {} worker could not start: {error}", path.display()));
            }
        }
    }

    fn poll_result(&mut self) -> Result<LoadingBackgroundStatus, String> {
        if let State::Failed(error) = &self.state { return Err(error.clone()); }
        let State::Pending(pending) = &self.state else { return Ok(LoadingBackgroundStatus::Ready); };
        match pending.result.try_recv() {
            Err(TryRecvError::Empty) => Ok(LoadingBackgroundStatus::Pending),
            Ok(Ok(rgba)) => {
                self.state = State::Ready(Some(rgba));
                Ok(LoadingBackgroundStatus::Ready)
            }
            Ok(Err(DecodeFailure::Optional(error))) => {
                log::warn!(target: "tiny_glade::startup",
                    "Optional loading background {} read/decode failed; retaining original absent-background fallback: {error}", pending.path.display());
                self.state = State::Ready(None);
                Ok(LoadingBackgroundStatus::Ready)
            }
            outcome => {
                let error = match outcome {
                    Ok(Err(DecodeFailure::Panic(error))) => error,
                    Err(TryRecvError::Disconnected) => "background worker disconnected before publishing its result".to_owned(),
                    _ => unreachable!("handled pending, image and optional decoder errors"),
                };
                let error = format!("Loading background {} preparation failed: {error}", pending.path.display());
                self.state = State::Failed(error.clone());
                Err(error)
            }
        }
    }

    /// Consume at most once. Pending is never interpreted as a missing image.
    pub(super) fn take_ready(&mut self) -> Result<Option<image::RgbaImage>, &'static str> {
        match &self.state {
            State::Pending(_) | State::Unrequested => Err("loading background CPU preparation is pending; defer the first loading render before advancing reveal"),
            State::Consumed => Ok(None),
            State::Failed(_) => Err("loading background preparation failed; rendering cannot proceed"),
            State::Ready(_) => {
                let State::Ready(image) = std::mem::replace(&mut self.state, State::Consumed) else { unreachable!() };
                Ok(image)
            }
        }
    }
}

impl Drop for BackgroundPreparation {
    fn drop(&mut self) {
        if let State::Pending(pending) = &self.state {
            pending.cancelled.store(true, Ordering::Release);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    fn await_result(preparation: &mut BackgroundPreparation) {
        let deadline = Instant::now() + Duration::from_secs(2);
        loop {
            if preparation.poll_result().unwrap() == LoadingBackgroundStatus::Ready { return; }
            assert!(Instant::now() < deadline, "background worker did not publish in time");
            std::thread::yield_now();
        }
    }

    #[test]
    fn absent_first_choice_is_attempted_once_without_a_worker() {
        let mut preparation = BackgroundPreparation::default();
        preparation.begin(None, |_| panic!("no path must not spawn a decoder"));
        assert_eq!(preparation.poll_result().unwrap(), LoadingBackgroundStatus::Ready);
        preparation.begin(Some("later-session.png".into()), |_| panic!("first selection is frozen"));
        assert!(preparation.take_ready().unwrap().is_none());
        assert!(preparation.take_ready().unwrap().is_none());
    }

    #[test]
    fn pending_is_not_absent_and_first_session_choice_cannot_change() {
        let (started, start) = crossbeam_channel::bounded(1);
        let (release, released) = crossbeam_channel::bounded(1);
        let mut preparation = BackgroundPreparation::default();
        preparation.begin(Some("first-session.png".into()), move |path| {
            started.send(path.to_owned()).unwrap();
            released.recv().unwrap();
            Ok(image::RgbaImage::from_raw(2, 1, vec![3, 7, 11, 13, 17, 19, 23, 29]).unwrap())
        });
        assert_eq!(start.recv_timeout(Duration::from_secs(2)).unwrap(), PathBuf::from("first-session.png"));
        assert!(preparation.is_pending());
        assert_eq!(preparation.poll(Some("second-session.png".into())).unwrap(), LoadingBackgroundStatus::Pending);
        assert!(preparation.take_ready().is_err());
        release.send(()).unwrap();
        await_result(&mut preparation);
        let image = preparation.take_ready().unwrap().unwrap();
        assert_eq!(image.dimensions(), (2, 1));
        assert_eq!(image.as_raw(), &[3, 7, 11, 13, 17, 19, 23, 29]);
        assert!(preparation.take_ready().unwrap().is_none());
    }

    #[test]
    fn corrupt_source_resolves_to_original_optional_absence() {
        let mut preparation = BackgroundPreparation::default();
        preparation.begin(Some("invalid-session.png".into()), move |_| Err("injected corrupt image".into()));
        await_result(&mut preparation);
        assert!(!preparation.is_pending());
        assert!(preparation.take_ready().unwrap().is_none());
    }

    #[test]
    fn file_decoder_keeps_original_dimensions_rgba_bytes_and_alpha() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("background.png");
        let bytes = vec![3, 7, 11, 13, 17, 19, 23, 29];
        image::RgbaImage::from_raw(2, 1, bytes.clone()).unwrap().save(&path).unwrap();
        let mut preparation = BackgroundPreparation::default();
        preparation.poll(Some(path)).unwrap();
        await_result(&mut preparation);
        let decoded = preparation.take_ready().unwrap().unwrap();
        assert_eq!(decoded.dimensions(), (2, 1));
        assert_eq!(decoded.into_raw(), bytes);
    }

    #[test]
    fn missing_file_keeps_original_optional_background_fallback() {
        let directory = tempfile::tempdir().unwrap();
        let mut preparation = BackgroundPreparation::default();
        preparation.poll(Some(directory.path().join("missing.png"))).unwrap();
        await_result(&mut preparation);
        assert!(preparation.take_ready().unwrap().is_none());
    }

    #[test]
    fn disconnected_worker_is_an_error_and_never_becomes_ready() {
        let (sender, result) = crossbeam_channel::bounded(1);
        drop(sender);
        let mut preparation = BackgroundPreparation { state: State::Pending(Pending {
            result, cancelled: Arc::new(AtomicBool::new(false)), path: "lost.png".into(),
        }) };
        assert!(preparation.poll_result().unwrap_err().contains("disconnected"));
        assert!(preparation.poll_result().is_err());
        assert!(preparation.take_ready().is_err());
    }

    #[test]
    fn worker_panic_is_returned_and_not_downgraded_to_optional_absence() {
        let mut preparation = BackgroundPreparation::default();
        preparation.begin(Some("panic.png".into()), |_| panic!("injected decoder panic"));
        let deadline = Instant::now() + Duration::from_secs(2);
        loop {
            match preparation.poll_result() {
                Err(error) => { assert!(error.contains("injected decoder panic")); break; }
                Ok(LoadingBackgroundStatus::Pending) => {}
                Ok(LoadingBackgroundStatus::Ready) => panic!("panic became ready"),
            }
            assert!(Instant::now() < deadline);
            std::thread::yield_now();
        }
        assert!(preparation.take_ready().is_err());
    }

    #[test]
    fn dropping_loading_owner_cancels_publication_without_waiting_for_io() {
        struct Finished(crossbeam_channel::Sender<()>);
        impl Drop for Finished { fn drop(&mut self) { let _ = self.0.send(()); } }
        let (started, start) = crossbeam_channel::bounded(1);
        let (release, released) = crossbeam_channel::bounded(1);
        let (finished, finish) = crossbeam_channel::bounded(1);
        let mut preparation = BackgroundPreparation::default();
        preparation.begin(Some("slow.png".into()), move |_| {
            let _finished = Finished(finished);
            started.send(()).unwrap();
            released.recv().unwrap();
            Ok(image::RgbaImage::new(1, 1))
        });
        start.recv_timeout(Duration::from_secs(2)).unwrap();
        let State::Pending(pending) = &preparation.state else { unreachable!() };
        let cancelled = pending.cancelled.clone();
        drop(preparation); // Must return even though the worker has not been released.
        assert!(cancelled.load(Ordering::Acquire));
        release.send(()).unwrap();
        finish.recv_timeout(Duration::from_secs(2)).unwrap();
    }

    #[test]
    fn queued_decoded_pixels_are_released_with_the_loading_owner() {
        let (sender, result) = crossbeam_channel::bounded(1);
        sender.send(Ok(image::RgbaImage::new(1, 1))).unwrap();
        let cancelled = Arc::new(AtomicBool::new(false));
        let preparation = BackgroundPreparation { state: State::Pending(Pending {
            result, cancelled: cancelled.clone(), path: "queued.png".into(),
        }) };
        drop(preparation);
        assert!(cancelled.load(Ordering::Acquire));
        assert!(sender.try_send(Ok(image::RgbaImage::new(1, 1))).is_err());
    }
}
