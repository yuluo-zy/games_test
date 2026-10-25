//! Dedicated render thread (reports/render-thread-2026-10-22/ARCHITECTURE.md).
//!
//! The game thread owns the game `World`. The renderer (including its Rc graph
//! session) is constructed and remains on the render thread. Only the Send
//! [`RenderSide`] containing the extracted World and feedback crosses the
//! frame boundary, while the renderer is idle:
//!
//! ```text
//! game:   update N+1 ──────────── reclaim ─ extract N+1 ─ submit ─ update N+2 …
//! render: ── declare/compile/record/submit/present N ──┘            └─ frame N+1 …
//! ```
//!
//! The game thread runs at most one frame ahead. `poll` returns to the event
//! loop while waiting; a wake callback runs after the result is published.
//! `reclaim` provides a blocking boundary for control and shutdown operations.
//! Results (screenshots, errors, submission acknowledgments and progress flags)
//! travel back as [`FrameFeedback`]; CPU completion does not imply GPU completion.
use bevy_ecs::{
    change_detection::{DetectChanges, Ref},
    component::{Component, Tick},
    entity::Entity,
    event::{Event, Events},
    query::QueryState,
    resource::Resource,
    world::World,
};
use bevy_state::state::{State, States};
use std::collections::HashSet;
use std::{
    panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
    sync::mpsc::{Receiver, SyncSender, sync_channel},
    thread::JoinHandle,
};

/// This thread owns visible drawing and animation. Configure its scheduler
/// role once, before creating the renderer, rather than on every frame.
fn configure_render_thread() -> anyhow::Result<()> {
    #[cfg(target_os = "macos")]
    {
        let code = unsafe { libc::pthread_set_qos_class_self_np(libc::qos_class_t::QOS_CLASS_USER_INTERACTIVE, 0) };
        if code != 0 {
            return Err(anyhow::anyhow!("render thread QoS: {}", std::io::Error::from_raw_os_error(code)));
        }
        log::info!(target: "tiny_glade::render_thread", "render_thread requested_qos=user_interactive relative_priority=0");
    }
    Ok(())
}

/// Only Send data crosses the frame boundary. The renderer, including its Rc
/// graph session, is constructed, used and destroyed on its resident thread.
#[derive(Default)]
pub struct RenderSide {
    // Cancel CPU read boundaries before dropping the World: a resource's last
    // GPU owner can call a host release during its destructor. This also covers
    // a queued packet returned by a disconnected sender on the game thread.
    pub(crate) read_boundary: Option<[crate::render_upload_boundary::ReadLease; 2]>,
    pub(crate) update_ready: std::sync::Arc<std::sync::atomic::AtomicBool>,
    update_wake: Option<std::sync::Arc<dyn Fn() + Send + Sync>>,
    pub world: World,
    pub feedback: FrameFeedback,
}

/// Held until all game-owned physical reads have been submitted or cancelled.
/// Opening admission also wakes the OS loop; GPU range retirement remains the
/// backend's responsibility and the render World stays on its resident worker.
pub(crate) struct UpdateAdmission {
    reads: Option<[crate::render_upload_boundary::ReadLease; 2]>,
    ready: std::sync::Arc<std::sync::atomic::AtomicBool>,
    wake: Option<std::sync::Arc<dyn Fn() + Send + Sync>>,
}
impl Drop for UpdateAdmission {
    fn drop(&mut self) {
        drop(self.reads.take());
        self.ready.store(true, std::sync::atomic::Ordering::Release);
        if let Some(wake) = &self.wake { wake(); }
    }
}
impl RenderSide {
    pub(crate) fn take_update_admission(&mut self) -> UpdateAdmission {
        UpdateAdmission { reads: self.read_boundary.take(), ready: self.update_ready.clone(), wake: self.update_wake.clone() }
    }
}

#[derive(Default)]
pub struct FrameFeedback {
    pub rendered: bool,
    pub submitted: bool,
    pub history_submitted: bool,
    pub temporal_candidate: Option<country_core::render::RenderTemporalState>,
    pub present_queued: bool,
    pub error: Option<anyhow::Error>,
    pub apply: Vec<Box<dyn FnOnce(&mut World) + Send>>,
}
impl FrameFeedback {
    pub fn defer(&mut self, apply: impl FnOnce(&mut World) + Send + 'static) {
        self.apply.push(Box::new(apply));
    }
}

pub type RenderFrameFn<R> = fn(&mut R, &mut RenderSide);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderStep { Pending, ReadyForUpdate, Complete }
pub type RenderStepFn<R> = fn(&mut R, &mut RenderSide) -> RenderStep;
type Command<R> = Box<dyn FnOnce(&mut R) + Send>;
enum Message<R> {
    Frame(RenderSide),
    Command(Command<R>),
    Wake(std::sync::Arc<dyn Fn() + Send + Sync>),
    Notify,
}
enum Returned {
    Side(RenderSide),
    Panicked(Box<dyn std::any::Any + Send>),
}

pub struct RenderThread<R: 'static> {
    jobs: SyncSender<Message<R>>,
    done: Receiver<Returned>,
    idle: Option<RenderSide>,
    handle: Option<JoinHandle<()>>,
    poisoned: bool,
    update_ready: std::sync::Arc<std::sync::atomic::AtomicBool>,
    stopping: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl<R: 'static> RenderThread<R> {
    /// The factory, rather than R, crosses the thread boundary. R need not be
    /// Send: Rust checks every captured startup owner and every message.
    pub fn spawn(
        factory: impl FnOnce() -> anyhow::Result<R> + Send + 'static,
        render: RenderFrameFn<R>,
    ) -> anyhow::Result<Self> {
        Self::spawn_with_control_poll(factory, render, |_| false)
    }

    /// Poll a coalesced control mailbox on the resident owner before sleeping
    /// and after frame/command work. Notify never waits for a busy worker.
    pub fn spawn_with_control_poll(
        factory: impl FnOnce() -> anyhow::Result<R> + Send + 'static,
        render: RenderFrameFn<R>,
        poll_control: fn(&mut R) -> bool,
    ) -> anyhow::Result<Self> {
        Self::spawn_worker(factory, move |renderer, side| {
            render(renderer, side);
            RenderStep::Complete
        }, move |renderer, _| poll_control(renderer))
    }

    pub fn spawn_steps(
        factory: impl FnOnce() -> anyhow::Result<R> + Send + 'static,
        step: RenderStepFn<R>,
        controls: fn(&mut R, Option<&mut RenderSide>) -> bool,
    ) -> anyhow::Result<Self> {
        Self::spawn_worker(factory, step, controls)
    }

    fn spawn_worker(
        factory: impl FnOnce() -> anyhow::Result<R> + Send + 'static,
        mut step: impl FnMut(&mut R, &mut RenderSide) -> RenderStep + Send + 'static,
        mut poll_control: impl FnMut(&mut R, Option<&mut RenderSide>) -> bool + Send + 'static,
    ) -> anyhow::Result<Self> {
        let (jobs, inbox) = sync_channel::<Message<R>>(1);
        let (outbox, done) = sync_channel::<Returned>(1);
        let (ready, initialized) = sync_channel(1);
        let stopping = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let worker_stopping = stopping.clone();
        let idle = RenderSide::default();
        let update_ready = idle.update_ready.clone();
        let worker_update_ready = update_ready.clone();
        let handle = std::thread::Builder::new().name("tg-render".into()).spawn(move || {
            if let Err(error) = configure_render_thread() { let _ = ready.send(Err(error)); return; }
            let mut renderer = match factory() {
                Ok(renderer) => { if ready.send(Ok(())).is_err() { return; } renderer }
                Err(error) => { let _ = ready.send(Err(error)); return; }
            };
            let mut wake: Option<std::sync::Arc<dyn Fn() + Send + Sync>> = None;
            let mut active: Option<RenderSide> = None;
            loop {
                if worker_stopping.load(std::sync::atomic::Ordering::Acquire) {
                    // Stop may race the first dequeue. Return the accepted
                    // packet without executing it, whether queued or Pending.
                    while let Ok(message) = inbox.try_recv() {
                        if let Message::Frame(side) = message {
                            assert!(active.is_none(), "one accepted frame");
                            active = Some(side);
                        }
                    }
                    if let Some(mut side) = active.take() {
                        drop(side.read_boundary.take());
                        side.update_ready.store(true, std::sync::atomic::Ordering::Release);
                        let _ = outbox.send(Returned::Side(side));
                    }
                    break;
                }
                match catch_unwind(AssertUnwindSafe(|| poll_control(&mut renderer, active.as_mut()))) {
                    Ok(true) => { if let Some(wake) = &wake { wake(); } },
                    Ok(false) => {},
                    Err(payload) => {
                        drop(active.take());
                        worker_update_ready.store(true, std::sync::atomic::Ordering::Release);
                        let _ = outbox.send(Returned::Panicked(payload));
                        if let Some(wake) = &wake { wake(); }
                        break;
                    }
                }
                if let Some(side) = &mut active {
                    match catch_unwind(AssertUnwindSafe(|| step(&mut renderer, side))) {
                        Ok(RenderStep::Complete) => {
                            let mut side = active.take().expect("active frame");
                            drop(side.read_boundary.take());
                            side.update_ready.store(true, std::sync::atomic::Ordering::Release);
                            let sent = outbox.send(Returned::Side(side)).is_ok();
                            if let Some(wake) = &wake { wake(); }
                            if !sent { break; }
                        }
                        Ok(RenderStep::Pending) => {},
                        Ok(RenderStep::ReadyForUpdate) => {
                            side.update_ready.store(true, std::sync::atomic::Ordering::Release);
                            if let Some(wake) = &wake { wake(); }
                        },
                        Err(payload) => {
                            drop(active.take());
                            worker_update_ready.store(true, std::sync::atomic::Ordering::Release);
                            let _ = outbox.send(Returned::Panicked(payload));
                            if let Some(wake) = &wake { wake(); }
                            break;
                        }
                    }
                }
                let message = if active.is_some() {
                    match inbox.recv_timeout(std::time::Duration::from_millis(1)) {
                        Ok(message) => message,
                        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => continue,
                        Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
                    }
                } else {
                    let Ok(message) = inbox.recv() else { break; };
                    message
                };
                match message {
                    Message::Frame(mut side) => {
                        side.update_wake = wake.clone();
                        assert!(active.is_none(), "only one accepted frame may be preparing");
                        active = Some(side);
                    }
                    Message::Command(command) => command(&mut renderer),
                    Message::Wake(callback) => wake = Some(callback),
                    Message::Notify => {},
                }
            }
            // Release a pending packet's read boundary before its World and
            // before the resident renderer's native owners are destroyed.
            drop(active);
            // Drop the non-Send renderer here, never on the game thread.
        })?;
        match initialized.recv() {
            Ok(Ok(())) => {},
            result => {
                let joined = handle.join();
                if let Err(payload) = joined { resume_unwind(payload); }
                return Err(match result {
                    Ok(Err(error)) => error,
                    Err(error) => error.into(),
                    Ok(Ok(())) => unreachable!(),
                });
            }
        }
        Ok(Self {
            jobs, done, idle: Some(idle), update_ready, stopping,
            handle: Some(handle), poisoned: false,
        })
    }

    pub fn in_flight(&self) -> bool { self.idle.is_none() }

    /// Start closes this gate before publishing a packet. Submission or cancellation must
    /// explicitly open it, so Update cannot race unregistered physical reads.
    pub fn update_allowed(&self) -> bool {
        !self.in_flight() || self.update_ready.load(std::sync::atomic::Ordering::Acquire)
    }

    pub fn failed(&self) -> bool { self.poisoned }

    /// Cleanup after a propagated panic must not try to reclaim another frame.
    /// Control panics leave the worker waiting for a command; frame panics exit
    /// its loop. Both paths still join before native surface owners can drop.
    pub fn join_failed(&mut self) {
        assert!(self.poisoned, "join_failed requires a failed render worker");
        self.idle.take();
        self.stopping.store(true, std::sync::atomic::Ordering::Release);
        let _ = self.jobs.try_send(Message::Notify);
        if let Some(handle) = self.handle.take() { let _ = handle.join(); }
    }

    fn accept(&mut self, result: Result<Returned, std::sync::mpsc::RecvError>) {
        match result {
            Ok(Returned::Side(side)) => self.idle = Some(side),
            Ok(Returned::Panicked(payload)) => { self.poisoned = true; resume_unwind(payload) }
            Err(_) => { self.poisoned = true; panic!("render thread exited while a frame was in flight") }
        }
    }

    /// Nonblocking receive for the event loop's AwaitRender state.
    pub fn poll(&mut self) -> bool {
        assert!(!self.poisoned, "the render thread panicked earlier; the renderer is gone");
        match self.done.try_recv() {
            Ok(result) => self.accept(Ok(result)),
            Err(std::sync::mpsc::TryRecvError::Empty) => return self.idle.is_some(),
            Err(std::sync::mpsc::TryRecvError::Disconnected) => self.accept(Err(std::sync::mpsc::RecvError)),
        }
        true
    }

    pub fn reclaim(&mut self) -> &mut RenderSide {
        assert!(!self.poisoned, "the render thread panicked earlier; the renderer is gone");
        if !self.poll() { let result = self.done.recv(); self.accept(result); }
        self.idle.as_mut().expect("reclaimed side")
    }

    /// Infrequent control operations run on the same owner as frame work. No
    /// renderer reference can escape; returned data must independently be Send.
    pub fn call<T: Send + 'static>(&mut self, command: impl FnOnce(&mut R) -> T + Send + 'static) -> T {
        self.reclaim();
        let (send, recv) = sync_channel(1);
        self.jobs.send(Message::Command(Box::new(move |renderer| {
            let result = catch_unwind(AssertUnwindSafe(|| command(renderer)));
            let _ = send.send(result);
        }))).unwrap_or_else(|_| panic!("render thread is gone"));
        match recv.recv().expect("render control response") {
            Ok(result) => result,
            Err(payload) => { self.poisoned = true; resume_unwind(payload) }
        }
    }

    /// A full slot already guarantees another mailbox poll. No command or
    /// unbounded queue is allocated per control update, and no frame is reclaimed.
    pub fn notify_control(&self) -> anyhow::Result<()> {
        match self.jobs.try_send(Message::Notify) {
            Ok(()) | Err(std::sync::mpsc::TrySendError::Full(_)) => Ok(()),
            Err(std::sync::mpsc::TrySendError::Disconnected(_)) => anyhow::bail!("render control worker exited"),
        }
    }

    pub fn set_wake(&mut self, wake: std::sync::Arc<dyn Fn() + Send + Sync>) {
        self.jobs.send(Message::Wake(wake)).unwrap_or_else(|_| panic!("render thread is gone"));
    }

    pub fn submit_frame(&mut self, game: &mut World, extract: &mut ExtractSet) {
        extract.run(game, &mut self.reclaim().world);
        self.start();
    }

    pub fn start(&mut self) {
        self.reclaim();
        let side = self.idle.take().expect("reclaimed side");
        side.update_ready.store(false, std::sync::atomic::Ordering::Release);
        if self.jobs.send(Message::Frame(side)).is_err() { panic!("render thread is gone"); }
    }

    pub fn apply_feedback(&mut self, game: &mut World) -> FrameFeedback {
        let mut feedback = std::mem::take(&mut self.reclaim().feedback);
        for apply in feedback.apply.drain(..) { apply(game); }
        feedback
    }

    /// Cancel queued/Pending work on the resident owner and return its packet.
    /// Callers needing graceful completion explicitly reclaim before shutdown.
    pub fn shutdown(mut self) -> RenderSide {
        self.stopping.store(true, std::sync::atomic::Ordering::Release);
        let _ = self.jobs.try_send(Message::Notify);
        if self.idle.is_none() { self.reclaim(); }
        let side = self.idle.take().expect("reclaimed side");
        if let Some(handle) = self.handle.take() { let _ = handle.join(); }
        side
    }
}

impl<R: 'static> Drop for RenderThread<R> {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            // A Pending frame cannot complete until compilation does. Stop the
            // resident loop first; it owns both cancellation and renderer Drop.
            self.stopping.store(true, std::sync::atomic::Ordering::Release);
            let _ = self.jobs.try_send(Message::Notify);
            // Drain a completion/panic already being published so a full return
            // slot cannot prevent the worker from observing the stop flag.
            while !handle.is_finished() {
                let _ = self.done.recv_timeout(std::time::Duration::from_millis(1));
            }
            let _ = handle.join();
        }
    }
}

/// The hand-over: copies per-frame state (category A) from the game world
/// into the render world, mirrors entity components, and applies queued
/// library changes (category B).
///
/// Change detection uses explicit ticks: `App::update` ends with
/// `clear_trackers`, so `is_changed` relative to the world's last change tick
/// would never report what this update changed.
#[derive(Default)]
pub struct ExtractSet {
    steps: Vec<Box<dyn FnMut(&mut World, &mut World, Tick, &mut ExtractStats) + Send>>,
    last_run: Option<Tick>,
    stats: ExtractStats,
}

/// Counts actual extraction work. Counts deliberately do not claim to measure
/// heap bytes: size_of::<T>() would hide the large allocations cloned by T.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ExtractStats {
    pub resources_copied: u64,
    pub component_rows_scanned: u64,
    pub components_copied: u64,
    pub components_removed: u64,
    pub component_removal_events: u64,
    pub component_removal_rescans: u64,
}
impl ExtractSet {
    pub fn add(&mut self, mut step: impl FnMut(&mut World, &mut World, Tick) + Send + 'static) -> &mut Self {
        self.add_measured(move |game, render, tick, _| step(game, render, tick))
    }
    fn add_measured(&mut self, step: impl FnMut(&mut World, &mut World, Tick, &mut ExtractStats) + Send + 'static) -> &mut Self {
        self.steps.push(Box::new(step));
        self
    }
    pub fn stats(&self) -> ExtractStats { self.stats }
    /// Category A: the render world receives a copy of `T` every frame, or
    /// loses it when the game world has none.
    pub fn copy<T: Resource + Clone>(&mut self) -> &mut Self {
        self.add_measured(|game, render, _, stats| match game.get_resource::<T>() {
            Some(value) => {
                let value = value.clone();
                render.insert_resource(value);
                stats.resources_copied += 1;
            }
            None => {
                render.remove_resource::<T>();
            }
        })
    }
    /// Copy only when the game-side value changed since the previous extract,
    /// so unchanged state costs one tick comparison.
    pub fn copy_changed<T: Resource + Clone>(&mut self) -> &mut Self {
        self.copy_changed_with_revision::<T>(|_| 0)
    }
    pub fn copy_changed_with_revision<T: Resource + Clone>(
        &mut self, revision: fn(&T) -> u64,
    ) -> &mut Self {
        self.copy_projected::<T>(revision, T::clone)
    }
    pub fn copy_projected<T: Resource>(&mut self, revision: fn(&T) -> u64, project: fn(&T) -> T) -> &mut Self {
        let mut previous_revision = None;
        self.add_measured(move |game, render, last_run, stats| {
            let this_run = game.read_change_tick();
            let Some(value) = game.get_resource_ref::<T>() else {
                render.remove_resource::<T>();
                if let Some(mut applied) = render.get_resource_mut::<country_core::render::asset_ack::AppliedAssetRevisions>() {
                    applied.remove::<T>();
                }
                previous_revision = None;
                return;
            };
            let current_revision = (game.id(), render.id(), revision(&value));
            if value.last_changed().is_newer_than(last_run, this_run)
                || previous_revision != Some(current_revision)
                || !render.contains_resource::<T>() {
                let value = project(&value);
                render.insert_resource(value);
                stats.resources_copied += 1;
                previous_revision = Some(current_revision);
                if std::any::TypeId::of::<T>() == std::any::TypeId::of::<country_core::asset_libraries::texture_library::AssetTextureLibrary>() {
                    render.get_resource_or_init::<country_core::render::asset_ack::AppliedAssetRevisions>()
                        .record::<T>(current_revision.2);
                }
            }
        })
    }
    /// Update only entries published by the producer; retain the render-side
    /// table and unchanged immutable owners across hand-overs.
    pub fn copy_asset_library<T: country_core::render::asset_ack::AssetSnapshot>(&mut self) -> &mut Self {
        use country_core::render::asset_ack::{AppliedAssetOperations, AppliedAssetRevisions,
            AssetOperation, AssetOperationKind};
        let mut previous = None;
        let mut cursor = None;
        let mut source_world = None;
        self.add_measured(move |game, render, last_run, stats| {
            let worlds = (game.id(), render.id());
            if source_world.is_some_and(|source| source != game.id()) {
                render.remove_resource::<T>();
                if let Some(mut applied) = render.get_resource_mut::<AppliedAssetOperations>() {
                    applied.remove_library::<T>();
                }
                if let Some(mut applied) = render.get_resource_mut::<AppliedAssetRevisions>() { applied.remove::<T>(); }
                cursor = None;
            }
            source_world = Some(game.id());
            let Some(value) = game.get_resource_ref::<T>() else {
                if let Some(old) = render.remove_resource::<T>() {
                    let revision = country_core::render::asset_ack::next_asset_publication();
                    let mut applied = render.get_resource_or_init::<AppliedAssetOperations>();
                    for entry in old.asset_keys() { applied.record(AssetOperation {
                        library: std::any::TypeId::of::<T>(), entry, revision, kind: AssetOperationKind::Remove,
                    }); }
                    render.get_resource_or_init::<AppliedAssetRevisions>().record::<T>(revision);
                }
                previous = None;
                cursor = None;
                return;
            };
            let before = (worlds.0, worlds.1, value.asset_revision());
            let tick_changed = value.last_changed().is_newer_than(last_run, game.read_change_tick());
            if previous == Some(before) && render.contains_resource::<T>() && !tick_changed { return; }
            // Reserve the hand-over sequence before capturing its checkpoint:
            // replacement/reinsertion also gets a newer operation id, while
            // concurrent writes after that checkpoint can never be ACKed early.
            let publication = country_core::render::asset_ack::next_asset_publication();
            let checkpoint = value.asset_cursor();
            let publication = publication.max(checkpoint.revision());
            let current = (worlds.0, worlds.1, checkpoint.revision());
            let same_worlds = previous.is_some_and(|(source, destination, _)| (source, destination) == worlds);
            let keys = if same_worlds {
                value.asset_changes(cursor).map(|(_, keys)| keys)
            } else { None };
            let keys = keys.unwrap_or_else(|| {
                let mut keys = value.asset_keys();
                if let Some(old) = render.get_resource::<T>() { keys.extend(old.asset_keys()); }
                keys
            });
            let operations = keys.into_iter().collect::<HashSet<_>>().into_iter().map(|entry| {
                let kind = if value.asset_contains(&entry) { AssetOperationKind::Upsert } else { AssetOperationKind::Remove };
                AssetOperation { library: std::any::TypeId::of::<T>(), entry, revision: publication, kind }
            }).collect::<Vec<_>>();
            if let Some(mut destination) = render.get_resource_mut::<T>() {
                value.update_asset_snapshot(&mut destination);
            } else { render.insert_resource(value.clone()); }
            let mut applied = render.get_resource_or_init::<AppliedAssetOperations>();
            for operation in operations { applied.record(operation); }
            render.get_resource_or_init::<AppliedAssetRevisions>().record::<T>(publication);
            cursor = Some(checkpoint);
            previous = Some(current);
            stats.resources_copied += 1;
        })
    }
    /// Component hooks and opted-in mutable producers publish entity changes.
    /// Only the initial copy or a bounded-journal gap scans
    /// the component table; ordinary frames visit the changed entities only.
    pub fn mirror_journaled<C: country_core::render::extract::JournaledComponent>(&mut self) -> &mut Self {
        use country_core::render::extract::ComponentJournal;
        let mut query: Option<QueryState<(Entity, &'static C)>> = None;
        let mut source_world = None;
        let mut destination_world = None;
        let mut cursor = None;
        let mut seen = HashSet::new();
        let mut changed = HashSet::new();
        let mut pending = Vec::new();
        self.add_measured(move |game, render, _, stats| {
            let new_source = source_world != Some(game.id());
            let new_destination = destination_world != Some(render.id());
            if new_source {
                if !new_destination {
                    for &entity in &seen { remove_mirrored::<C>(render, entity, stats); }
                }
                query = None;
                cursor = None;
                source_world = Some(game.id());
            }
            if new_source || new_destination { seen.clear(); }
            destination_world = Some(render.id());
            game.init_resource::<ComponentJournal<C>>();
            let journal = game.resource::<ComponentJournal<C>>();
            let changes = journal.since(cursor);
            let checkpoint = journal.cursor();
            let query = query.get_or_insert_with(|| game.query::<(Entity, &C)>());
            if new_source || new_destination || changes.is_none() {
                if !new_source && !new_destination {
                    stats.component_removal_rescans += 1;
                    seen.retain(|&entity| {
                        if query.get(game, entity).is_ok() { return true; }
                        remove_mirrored::<C>(render, entity, stats);
                        false
                    });
                }
                for (entity, value) in query.iter(game) {
                    stats.component_rows_scanned += 1;
                    stats.components_copied += 1;
                    seen.insert(entity);
                    pending.push((entity, value.clone()));
                }
                cursor = Some(checkpoint);
            } else if let Some((checkpoint, entities)) = changes {
                changed.clear();
                for entity in entities {
                    if !changed.insert(entity) { continue; }
                    if let Ok((_, value)) = query.get(game, entity) {
                        seen.insert(entity);
                        pending.push((entity, value.clone()));
                        stats.components_copied += 1;
                    } else if seen.remove(&entity) {
                        remove_mirrored::<C>(render, entity, stats);
                    }
                }
                cursor = Some(checkpoint);
            }
            insert_mirrored(render, &mut pending);
        })
    }
    /// The events of `E` sent since the previous extract.
    pub fn events<E: Event + Clone>(&mut self) -> &mut Self {
        let mut cursor = bevy_ecs::event::EventCursor::<E>::default();
        let mut source_world = None;
        self.add(move |game, render, _| {
            if source_world != Some(game.id()) {
                cursor = Default::default();
                source_world = Some(game.id());
            }
            let mut events = Events::<E>::default();
            if let Some(source) = game.get_resource::<Events<E>>() {
                for event in cursor.read(source) {
                    events.send(event.clone());
                }
            }
            render.insert_resource(events);
        })
    }
    /// The current value of state `S`.
    pub fn state<S: States>(&mut self) -> &mut Self {
        self.add(|game, render, _| match game.get_resource::<State<S>>() {
            Some(state) => {
                let value = state.get().clone();
                render.insert_resource(State::new(value));
            }
            None => {
                render.remove_resource::<State<S>>();
            }
        })
    }
    /// Mirror whether the game world has `R`, as `Present<R>`.
    pub fn presence<R: Resource>(&mut self) -> &mut Self {
        use country_core::render::extract::Present;
        self.add(|game, render, _| {
            if game.contains_resource::<R>() {
                render.init_resource::<Present<R>>();
            } else {
                render.remove_resource::<Present<R>>();
            }
        })
    }
    /// Render-owned: move `T` to the render world whenever the game world
    /// (re)inserts it; the render world keeps it otherwise.
    pub fn move_owned<T: Resource>(&mut self) -> &mut Self {
        self.add(|game, render, _| {
            if let Some(value) = game.remove_resource::<T>() {
                render.insert_resource(value);
            }
        })
    }
    /// Category B: the resource lives on the render side for good; the game
    /// world keeps a command queue `Q` whose pending changes are applied here.
    pub fn apply_queue<Q: Resource + Default, T: Resource>(
        &mut self,
        apply: fn(Q, &mut T),
    ) -> &mut Self {
        self.add(move |game, render, _| {
            // A missing render owner is not an acknowledgement. Keep the
            // producer queue intact until the destination can accept it.
            let Some(mut target) = render.get_resource_mut::<T>() else {
                return;
            };
            let Some(mut queue) = game.get_resource_mut::<Q>() else {
                return;
            };
            let pending = std::mem::take(&mut *queue);
            apply(pending, &mut target);
        })
    }
    pub fn run(&mut self, game: &mut World, render: &mut World) {
        if render.get_resource::<ExtractedAt>().is_some_and(|at| at.source != game.id()) {
            render.remove_resource::<country_core::render::asset_ack::AppliedAssetRevisions>();
            render.remove_resource::<country_core::render::asset_ack::AppliedAssetOperations>();
        }
        // The first extract copies everything.
        let last_run = self.last_run.unwrap_or(Tick::new(0));
        let this_run = game.change_tick();
        self.stats = ExtractStats::default();
        for step in &mut self.steps {
            step(game, render, last_run, &mut self.stats);
        }
        self.last_run = Some(this_run);
        let render_tick = render.change_tick();
        render.insert_resource(ExtractedAt { source: game.id(), game: this_run, render: render_tick });
        // Declaration's writes must carry a later tick than the extract's.
        render.increment_change_tick();
        // Separate subsequent game writes from the extracted source version,
        // even when the caller mutates directly without another App update.
        game.increment_change_tick();
    }
}

/// A mutation of the game world produced on the render thread.
pub type GameApply = Box<dyn FnOnce(&mut World) + Send>;

/// Identity of the game-side value at extraction. Comparing both insertion
/// and modification ticks also rejects remove/reinsert and late updates.
/// World identity prevents a queued result from crossing an App switch.
#[derive(Clone, Copy, PartialEq, Eq)]
struct SourceVersion {
    world: bevy_ecs::world::WorldId,
    ticks: Option<(Tick, Tick)>,
}
impl SourceVersion {
    fn of<R: Resource>(world: &World) -> Self {
        Self {
            world: world.id(),
            ticks: world.get_resource_change_ticks::<R>().map(|t| (t.added, t.changed)),
        }
    }
}

/// Ticks of the last extract, in both worlds.
#[derive(Resource, Clone, Copy)]
pub struct ExtractedAt {
    pub source: bevy_ecs::world::WorldId,
    pub game: Tick,
    pub render: Tick,
}

/// Runs on the render thread after declaration: turns render-side writes the
/// game needs into `GameApply`s, applied at the next hand-over.
type WriteBackStep = Box<dyn FnMut(&mut World, ExtractedAt, &mut Vec<GameApply>) + Send>;
#[derive(Default)]
pub struct WriteBackSet {
    steps: Vec<WriteBackStep>,
    submitted: Vec<WriteBackStep>,
    unsubmitted: Vec<fn(&mut World)>,
}
impl WriteBackSet {
    pub fn add(&mut self, step: impl FnMut(&mut World, ExtractedAt, &mut Vec<GameApply>) + Send + 'static) -> &mut Self {
        self.steps.push(Box::new(step));
        self
    }
    pub fn run(&mut self, render: &mut World, out: &mut Vec<GameApply>) {
        let Some(at) = render.get_resource::<ExtractedAt>().copied() else {
            return;
        };
        for step in &mut self.steps {
            step(render, at, out);
        }
    }
    pub fn abort_unsubmitted(&mut self, render: &mut World) {
        for invalidate in &self.unsubmitted { invalidate(render); }
    }
    pub fn run_submitted(&mut self, render: &mut World, out: &mut Vec<GameApply>) {
        let Some(at) = render.get_resource::<ExtractedAt>().copied() else { return; };
        for step in &mut self.submitted { step(render, at, out); }
    }

}

/// Both directions of the hand-over, as registered by the declaration modules.
#[derive(Default)]
pub struct RenderExchange {
    pub extract: ExtractSet,
    pub write_back: WriteBackSet,
    /// Several modules read the same type; each is extracted once.
    registered: std::collections::HashSet<(u8, std::any::TypeId)>,
}
impl RenderExchange {
    /// The game still owns dirty data if submission fails. Discard the mutated
    /// render copy so unchanged source ticks cannot suppress the next retry.
    pub fn invalidate_unsubmitted<R: Resource>(&mut self) {
        self.write_back.unsubmitted.push(|render| { render.remove_resource::<R>(); });
    }
    /// Register GPU consumption acknowledgments separately from CPU-only
    /// feedback. A failed declaration/compile must leave producer data dirty.
    pub fn after_submission(&mut self, register: impl FnOnce(&mut Self)) {
        let start = self.write_back.steps.len();
        register(self);
        self.write_back.submitted.extend(self.write_back.steps.drain(start..));
    }

    fn first<T: 'static>(&mut self, kind: u8) -> bool {
        self.registered.insert((kind, std::any::TypeId::of::<T>()))
    }
}

impl country_core::render::extract::RenderExtract for RenderExchange {
    fn component_journaled<C: country_core::render::extract::JournaledComponent>(&mut self) {
        if self.first::<C>(0) { self.extract.mirror_journaled::<C>(); }
    }
    fn resource<R: Resource + Clone>(&mut self) {
        if self.first::<R>(1) {
            self.extract.copy_changed::<R>();
        }
    }
    fn resource_projected<R: Resource>(&mut self, project: fn(&R) -> R) {
        if self.first::<R>(1) { self.extract.copy_projected::<R>(|_| 0, project); }
    }
    fn resource_with_revision<R: Resource + Clone>(&mut self, revision: fn(&R) -> u64) {
        if self.first::<R>(1) {
            self.extract.copy_changed_with_revision::<R>(revision);
        }
    }
    fn asset_library<R: country_core::render::asset_ack::AssetSnapshot>(&mut self) {
        if self.first::<R>(1) { self.extract.copy_asset_library::<R>(); }
    }
    fn events<E: Event + Clone>(&mut self) {
        if self.first::<E>(2) {
            self.extract.events::<E>();
        }
    }
    fn state<S: States>(&mut self) {
        if self.first::<S>(3) {
            self.extract.state::<S>();
        }
    }
    fn presence<R: Resource>(&mut self) {
        if self.first::<R>(4) {
            self.extract.presence::<R>();
        }
    }
    fn render_owned<R: Resource>(&mut self) {
        if self.first::<R>(5) {
            self.extract.move_owned::<R>();
        }
    }
    fn write_back<R: Resource + Clone>(&mut self) {
        // Whether the render world had `R` right after the extract, so that a
        // removal during declaration can be told from "never there".
        if !self.first::<R>(6) {
            return;
        }
        let source = std::sync::Arc::new(parking_lot::Mutex::new(None));
        let seen = source.clone();
        self.extract.add(move |game, render, _| {
            *seen.lock() = Some((SourceVersion::of::<R>(game), render.contains_resource::<R>()));
        });
        self.write_back.steps.push(Box::new(move |render, at, out| {
            let Some((source, present)) = *source.lock() else { return };
            let this_run = render.read_change_tick();
            match render.get_resource_ref::<R>() {
                Some(value) if value.last_changed().is_newer_than(at.render, this_run) => {
                    let value = value.clone();
                    out.push(Box::new(move |game: &mut World| {
                        if SourceVersion::of::<R>(game) != source {
                            return;
                        }
                        match game.get_resource_mut::<R>() {
                            Some(mut current) => *current = value,
                            None => game.insert_resource(value),
                        }
                    }));
                }
                Some(_) => {}
                None if present => {
                    out.push(Box::new(move |game: &mut World| {
                        if SourceVersion::of::<R>(game) == source {
                            game.remove_resource::<R>();
                        }
                    }));
                }
                None => {}
            }
        }));
    }
    fn write_back_with<R: Resource>(
        &mut self,
        guarded: bool,
        take: fn(&R) -> Option<country_core::render::extract::WriteBack<R>>,
    ) {
        let source = std::sync::Arc::new(parking_lot::Mutex::new(None));
        let seen = source.clone();
        self.extract.add(move |game, _, _| {
            *seen.lock() = Some(SourceVersion::of::<R>(game));
        });
        self.write_back.steps.push(Box::new(move |render, _, out| {
            let Some(source) = *source.lock() else { return };
            let Some(apply) = render.get_resource::<R>().and_then(take) else {
                return;
            };
            out.push(Box::new(move |game: &mut World| {
                let current = SourceVersion::of::<R>(game);
                if current.world != source.world || (guarded && current != source) {
                    return;
                }
                if let Some(current) = game.get_resource_mut::<R>()
                {
                    apply(current);
                }
            }));
        }));
    }
    fn forward_events<E: Event>(&mut self) {
        self.extract.add(|_, render, _| {
            render.init_resource::<Events<E>>();
        });
        self.write_back.steps.push(Box::new(|render, at, out| {
            let Some(mut events) = render.get_resource_mut::<Events<E>>() else {
                return;
            };
            let sent: Vec<E> = events.drain().collect();
            if !sent.is_empty() {
                out.push(Box::new(move |game: &mut World| {
                    if game.id() != at.source { return; }
                    if let Some(mut events) = game.get_resource_mut::<Events<E>>() {
                        events.send_batch(sent);
                    }
                }));
            }
        }));
    }
}


/// Empty mirror shells have no rendering identity to preserve. Removing them
/// immediately also covers a live game entity whose last mirrored component
/// disappears before that game entity is eventually despawned.
fn remove_mirrored<C: Component>(render: &mut World, entity: Entity, stats: &mut ExtractStats) {
    if let Ok(mut mirrored) = render.get_entity_mut(entity) {
        if mirrored.contains::<C>() {
            mirrored.remove::<C>();
            stats.components_removed += 1;
        }
        if mirrored.archetype().component_count() == 0 { mirrored.despawn(); }
    }
}

/// Insert mirrored values at their game entity ids. An index the game reused
/// with a new generation replaces the stale render entity first.
fn insert_mirrored<C: Component>(render: &mut World, pending: &mut Vec<(Entity, C)>) {
    if pending.is_empty() { return; }
    for (entity, _) in pending.iter() {
        if render.entities().contains(*entity) { continue; }
        if let Some(stale) = render.entities().resolve_from_id(entity.index())
            && stale != *entity
        {
            render.despawn(stale);
        }
    }
    // One bundle inserter/spawner per component batch, rather than rebuilding
    // it for every row. The pending Vec retains capacity for the next frame.
    #[allow(deprecated)]
    render.insert_or_spawn_batch(pending.drain(..).map(|(entity, value)| (entity, (value,))))
        .expect("render entity generations reconciled before batch insertion");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Resource, Clone, Default, PartialEq, Debug)]
    struct Frame(u64);

    fn exchange() -> RenderExchange {
        use country_core::render::extract::RenderExtract;
        let mut exchange = RenderExchange::default();
        exchange.resource::<Frame>();
        exchange.write_back::<Frame>();
        exchange
    }

    #[test]
    fn pending_worker_drop_cancels_on_its_owner_without_waiting_for_ready() {
        let (prepared, preparing) = std::sync::mpsc::channel();
        let (dropped, destroyed) = std::sync::mpsc::channel();
        struct Owner(std::sync::mpsc::Sender<std::thread::ThreadId>);
        impl Drop for Owner { fn drop(&mut self) { self.0.send(std::thread::current().id()).unwrap(); } }
        let mut worker = RenderThread::spawn_worker(move || Ok(Owner(dropped)), move |_, _| {
            let _ = prepared.send(());
            RenderStep::Pending
        }, |_, _| false).unwrap();
        let owner = worker.call(|_| std::thread::current().id());
        worker.start();
        preparing.recv_timeout(std::time::Duration::from_secs(5)).unwrap();
        assert!(!worker.update_allowed());
        let (finished, joined) = std::sync::mpsc::channel();
        let dropper = std::thread::spawn(move || { drop(worker); finished.send(()).unwrap(); });
        joined.recv_timeout(std::time::Duration::from_secs(5)).unwrap();
        assert_eq!(destroyed.recv().unwrap(), owner);
        dropper.join().unwrap();
    }

    #[test]
    fn preparation_panic_opens_admission_before_waking_the_event_loop() {
        let mut worker = RenderThread::spawn_worker(|| Ok(()), |_, _| {
            panic!("prepare failed before admission");
        }, |_, _| false).unwrap();
        let (wake, woken) = std::sync::mpsc::channel();
        worker.set_wake(std::sync::Arc::new(move || { let _ = wake.send(()); }));
        worker.start();
        woken.recv_timeout(std::time::Duration::from_secs(5)).unwrap();
        assert!(worker.update_allowed(), "the OS must reach poll after a failed Prepare");
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| worker.poll())).is_err());
        assert!(worker.failed());
        worker.join_failed();
    }

    #[test]
    fn shutdown_returns_a_pending_packet_without_waiting_for_ready() {
        #[derive(Resource)]
        struct ShutdownMarker(u32);
        let (prepared, preparing) = std::sync::mpsc::channel();
        let mut worker = RenderThread::spawn_worker(|| Ok(()), move |_, _| {
            let _ = prepared.send(());
            RenderStep::Pending
        }, |_, _| false).unwrap();
        worker.reclaim().world.insert_resource(ShutdownMarker(42));
        worker.start();
        preparing.recv_timeout(std::time::Duration::from_secs(5)).unwrap();
        let (returned, received) = std::sync::mpsc::channel();
        let join = std::thread::spawn(move || { returned.send(worker.shutdown()).unwrap(); });
        let side = received.recv_timeout(std::time::Duration::from_secs(5)).unwrap();
        assert_eq!(side.world.resource::<ShutdownMarker>().0, 42);
        assert!(side.read_boundary.is_none());
        join.join().unwrap();
    }

    #[test]
    fn admission_releases_writers_and_wakes_before_presentation_completes() {
        use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
        let buffers = Arc::new(crate::render_upload_boundary::UploadBoundary::default());
        let textures = Arc::new(crate::render_upload_boundary::UploadBoundary::default());
        let (submitted, registered) = std::sync::mpsc::channel();
        let (present, presentation) = std::sync::mpsc::channel();
        let mut worker = RenderThread::spawn_worker(|| Ok(()), move |_, side| {
            drop(side.take_update_admission());
            submitted.send(()).unwrap();
            presentation.recv().unwrap();
            RenderStep::Complete
        }, |_, _| false).unwrap();
        let woke = Arc::new(AtomicBool::new(false));
        let signal = woke.clone();
        worker.set_wake(Arc::new(move || { signal.store(true, Ordering::Release); }));
        let owner = worker.call(|_| std::thread::current().id());
        worker.reclaim().read_boundary = Some([buffers.begin_read(owner), textures.begin_read(owner)]);
        worker.start();
        registered.recv_timeout(std::time::Duration::from_secs(5)).unwrap();
        assert!(worker.in_flight());
        assert!(worker.update_allowed());
        assert!(woke.load(Ordering::Acquire));
        let (wrote, received) = std::sync::mpsc::channel();
        let writer = std::thread::spawn(move || {
            let first = wrote.clone();
            buffers.write_or_defer(0, move || { first.send(()).unwrap(); Ok(()) }).unwrap();
            textures.write_or_defer(0, move || { wrote.send(()).unwrap(); Ok(()) }).unwrap();
        });
        received.recv_timeout(std::time::Duration::from_secs(5)).unwrap();
        received.recv_timeout(std::time::Duration::from_secs(5)).unwrap();
        present.send(()).unwrap();
        worker.reclaim();
        writer.join().unwrap();
    }

    #[test]
    fn pending_preparation_keeps_the_packet_and_services_controls_before_execute() {
        use std::sync::{Arc, atomic::{AtomicBool, AtomicUsize, Ordering}, mpsc::channel};
        struct Renderer {
            prepared: Arc<AtomicUsize>,
            prepared_signal: std::sync::mpsc::Sender<()>,
            ready: Arc<AtomicBool>,
            execute: Arc<AtomicBool>,
            control: Arc<AtomicBool>,
            observed: std::sync::mpsc::Sender<()>,
            phase: u8,
        }
        fn step(renderer: &mut Renderer, side: &mut RenderSide) -> RenderStep {
            match renderer.phase {
                0 => { renderer.prepared.fetch_add(1, Ordering::Relaxed); renderer.phase = 1; renderer.prepared_signal.send(()).unwrap(); RenderStep::Pending },
                1 if renderer.ready.load(Ordering::Acquire) => { renderer.phase = 2; RenderStep::ReadyForUpdate },
                2 if renderer.execute.load(Ordering::Acquire) => { side.feedback.rendered = true; RenderStep::Complete },
                _ => RenderStep::Pending,
            }
        }
        fn controls(renderer: &mut Renderer, active: Option<&mut RenderSide>) -> bool {
            if renderer.control.swap(false, Ordering::AcqRel) {
                assert!(active.is_some(), "control must be serviced without returning the pending packet");
                renderer.observed.send(()).unwrap();
                true
            } else { false }
        }
        let prepared = Arc::new(AtomicUsize::new(0));
        let ready = Arc::new(AtomicBool::new(false));
        let execute = Arc::new(AtomicBool::new(false));
        let control = Arc::new(AtomicBool::new(false));
        let (observed, controls_done) = channel();
        let (prepared_signal, prepared_done) = channel();
        let (wake, wakes) = channel();
        let (p, r, e, c) = (prepared.clone(), ready.clone(), execute.clone(), control.clone());
        let mut worker = RenderThread::spawn_steps(move || Ok(Renderer {
            prepared: p, prepared_signal, ready: r, execute: e, control: c, observed, phase: 0,
        }), step, controls).unwrap();
        worker.set_wake(Arc::new(move || { let _ = wake.send(()); }));
        worker.start();
        assert!(!worker.update_allowed(), "admission must close before the worker can start");
        assert!(!worker.poll());
        prepared_done.recv_timeout(std::time::Duration::from_secs(2)).unwrap();
        control.store(true, Ordering::Release);
        worker.notify_control().unwrap();
        controls_done.recv_timeout(std::time::Duration::from_secs(2)).unwrap();
        wakes.recv_timeout(std::time::Duration::from_secs(2)).unwrap();
        assert!(worker.in_flight());
        assert!(!worker.update_allowed());
        ready.store(true, Ordering::Release);
        worker.notify_control().unwrap();
        wakes.recv_timeout(std::time::Duration::from_secs(2)).unwrap();
        assert!(worker.update_allowed());
        assert!(!worker.poll(), "Ready permits Update but must not return a not-yet-executed packet");
        execute.store(true, Ordering::Release);
        worker.notify_control().unwrap();
        assert!(worker.reclaim().feedback.rendered);
        assert_eq!(prepared.load(Ordering::Relaxed), 1, "Pending must never redeclare");
        worker.shutdown();
    }

    #[test]
    fn busy_frame_control_notifications_coalesce_without_reclaim_or_lost_idle_wake() {
        use std::sync::{Arc, atomic::{AtomicU64, Ordering}, mpsc::channel};
        struct Renderer {
            requested: Arc<AtomicU64>,
            entered: std::sync::mpsc::Sender<()>,
            release: std::sync::mpsc::Receiver<()>,
            applied: std::sync::mpsc::Sender<u64>,
            seen: u64,
        }
        fn render(renderer: &mut Renderer, _: &mut RenderSide) {
            renderer.entered.send(()).unwrap();
            renderer.release.recv().unwrap();
        }
        fn controls(renderer: &mut Renderer) -> bool {
            let desired = renderer.requested.load(Ordering::Acquire);
            if renderer.seen == desired { return false; }
            renderer.seen = desired;
            renderer.applied.send(desired).unwrap();
            true
        }
        let requested = Arc::new(AtomicU64::new(0));
        let (entered, started) = channel();
        let (release, wait) = channel();
        let (applied, observed) = channel();
        let desired = requested.clone();
        let mut worker = RenderThread::spawn_with_control_poll(move || Ok(Renderer {
            requested: desired, entered, release: wait, applied, seen: 0,
        }), render, controls).unwrap();
        worker.start();
        started.recv_timeout(std::time::Duration::from_secs(2)).unwrap();
        for revision in 1..=10_000 {
            requested.store(revision, Ordering::Release);
            worker.notify_control().unwrap();
        }
        assert!(worker.in_flight());
        assert!(!worker.poll());
        assert!(observed.try_recv().is_err());
        release.send(()).unwrap();
        assert_eq!(observed.recv_timeout(std::time::Duration::from_secs(2)).unwrap(), 10_000);
        worker.reclaim();
        requested.store(10_001, Ordering::Release);
        worker.notify_control().unwrap();
        assert_eq!(observed.recv_timeout(std::time::Duration::from_secs(2)).unwrap(), 10_001);
        worker.shutdown();
    }

    #[test]
    fn asset_snapshot_publications_are_idempotent_and_remain_eligible_after_failure() {
        use country_core::{asset_libraries::mesh_library::{AssetMeshLibrary, MeshAsset},
            render::{mesh::Mesh, asset_ack::AppliedAssetRevisions}};
        let (mut game, mut render) = (World::new(), World::new());
        game.init_resource::<AssetMeshLibrary>();
        let mut extract = ExtractSet::default();
        extract.copy_asset_library::<AssetMeshLibrary>();
        extract.run(&mut game, &mut render);
        let first = render.resource::<AppliedAssetRevisions>().0.clone();
        assert_eq!(first.len(), 1);
        extract.run(&mut game, &mut render);
        assert_eq!(extract.stats().resources_copied, 0);
        assert_eq!(render.resource::<AppliedAssetRevisions>().0, first,
            "unchanged versions remain eligible after a failed frame");
        let handle = game.resource::<AssetMeshLibrary>().add(MeshAsset::new(Mesh::new("new"), None));
        extract.run(&mut game, &mut render);
        assert_eq!(extract.stats().resources_copied, 1);
        assert!(render.resource::<AppliedAssetRevisions>().0[0].1 > first[0].1);
        assert!(render.resource::<AssetMeshLibrary>().get(handle).is_some());
        game.remove_resource::<AssetMeshLibrary>();
        extract.run(&mut game, &mut render);
        assert!(!render.contains_resource::<AssetMeshLibrary>());
        assert_eq!(render.resource::<AppliedAssetRevisions>().0.len(), 1,
            "resource removal must also retain its publication until submitted");
    }

    #[test]
    fn asset_entry_changes_keep_deletions_through_journal_gaps_and_resource_replacement() {
        use country_core::{asset_libraries::mesh_library::{AssetMeshLibrary, MeshAsset},
            render::{mesh::Mesh, asset_ack::{AppliedAssetOperations, AssetEntryKey, AssetOperationKind}}};
        let (mut game, mut render) = (World::new(), World::new());
        game.init_resource::<AssetMeshLibrary>();
        let mut extract = ExtractSet::default();
        extract.copy_asset_library::<AssetMeshLibrary>();
        extract.run(&mut game, &mut render);
        let removed = game.resource::<AssetMeshLibrary>().add(MeshAsset::new(Mesh::new("removed"), Some("removed".into())));
        let kept = game.resource::<AssetMeshLibrary>().add(MeshAsset::new(Mesh::new("kept"), Some("kept".into())));
        extract.run(&mut game, &mut render);
        let first = render.resource_mut::<AppliedAssetOperations>().feedback(true);
        assert_eq!(first.len(), 4);
        assert!(first.iter().all(|operation| operation.kind == AssetOperationKind::Upsert));
        extract.run(&mut game, &mut render);
        assert!(render.resource_mut::<AppliedAssetOperations>().feedback(false).is_empty());
        game.resource_mut::<AssetMeshLibrary>().remove(removed);
        for _ in 0..4100 { let _ = game.resource_mut::<AssetMeshLibrary>().get_mut(kept); }
        extract.run(&mut game, &mut render);
        let retry = render.resource_mut::<AppliedAssetOperations>().feedback(false);
        assert!(retry.iter().any(|operation| operation.entry == AssetEntryKey::Id(removed.id().raw())
            && operation.kind == AssetOperationKind::Remove));
        assert!(retry.iter().any(|operation| operation.entry == AssetEntryKey::Name("removed".into())
            && operation.kind == AssetOperationKind::Remove));
        // No new producer write between an unsuccessful frame and its retry.
        extract.run(&mut game, &mut render);
        assert_eq!(render.resource_mut::<AppliedAssetOperations>().feedback(true).len(), retry.len());
        let same_owner = game.remove_resource::<AssetMeshLibrary>().unwrap();
        extract.run(&mut game, &mut render);
        let removal = render.resource_mut::<AppliedAssetOperations>().feedback(true);
        assert!(removal.iter().all(|operation| operation.kind == AssetOperationKind::Remove));
        let removed_at = removal.iter().map(|operation| operation.revision).max().unwrap();
        game.insert_resource(same_owner);
        extract.run(&mut game, &mut render);
        let reinserted = render.resource_mut::<AppliedAssetOperations>().feedback(true);
        assert!(reinserted.iter().all(|operation| operation.kind == AssetOperationKind::Upsert && operation.revision > removed_at));
        game.insert_resource(AssetMeshLibrary::new());
        extract.run(&mut game, &mut render);
        let retired = render.resource_mut::<AppliedAssetOperations>().feedback(false);
        assert!(retired.iter().all(|operation| operation.kind == AssetOperationKind::Remove));
        assert!(retired.iter().any(|operation| operation.entry == AssetEntryKey::Id(kept.id().raw())));
        let mut incoming = World::new();
        incoming.init_resource::<AssetMeshLibrary>();
        extract.run(&mut incoming, &mut render);
        assert!(render.resource_mut::<AppliedAssetOperations>().feedback(false).is_empty(), "old World operations crossed Apps");
    }

    #[test]
    fn placement_logged_clear_and_reload_publish_removals_and_new_owners() {
        use country_core::{asset_libraries::placement_data_library::AssetPlacementDataLibrary,
            render::{test_device::TestDevice, asset_ack::{AppliedAssetOperations, AssetEntryKey, AssetOperationKind}}};
        let dir = tempfile::tempdir().unwrap();
        let first = dir.path().join("first.json");
        let second = dir.path().join("second.json");
        std::fs::write(&first, "[]").unwrap(); std::fs::write(&second, "[]").unwrap();
        let (mut game, mut render) = (World::new(), World::new());
        let mut library = AssetPlacementDataLibrary::new();
        let (_, old) = library.load(&TestDevice(1), first);
        game.insert_resource(library);
        let mut extract = ExtractSet::default();
        extract.copy_asset_library::<AssetPlacementDataLibrary>();
        extract.run(&mut game, &mut render);
        render.resource_mut::<AppliedAssetOperations>().feedback(true);
        game.resource_mut::<AssetPlacementDataLibrary>().clear();
        extract.run(&mut game, &mut render);
        assert!(render.resource::<AssetPlacementDataLibrary>().paths().is_empty());
        assert!(render.resource::<AssetPlacementDataLibrary>().buffers().is_empty());
        let removed = render.resource_mut::<AppliedAssetOperations>().feedback(true);
        assert_eq!(removed.len(), 2);
        assert!(removed.iter().all(|operation| operation.kind == AssetOperationKind::Remove));
        assert!(removed.iter().any(|operation| operation.entry == AssetEntryKey::Id(old.id().raw())));
        let mut library = game.resource_mut::<AssetPlacementDataLibrary>();
        let (_, new) = library.load(&TestDevice(1), second);
        extract.run(&mut game, &mut render);
        let snapshot = render.resource::<AssetPlacementDataLibrary>();
        assert!(!snapshot.buffers().contains_key(&old) && snapshot.buffers().contains_key(&new));
        let operations = render.resource_mut::<AppliedAssetOperations>().feedback(true);
        assert!(operations.iter().any(|operation| operation.entry == AssetEntryKey::Id(new.id().raw())
            && operation.kind == AssetOperationKind::Upsert));
    }

    #[test]
    fn extraction_counts_visit_only_journaled_writes_after_initial_copy() {
        #[derive(Component, Clone)]
        #[component(on_insert = country_core::render::extract::record_component_change::<Position>,
            on_remove = country_core::render::extract::record_component_change::<Position>)]
        struct Position;
        impl country_core::render::extract::JournaledComponent for Position {}
        let (mut game, mut render) = (World::new(), World::new());
        game.insert_resource(Frame(1));
        let entity = game.spawn(Position).id();
        let mut extract = ExtractSet::default();
        extract.copy_changed::<Frame>().mirror_journaled::<Position>();
        extract.run(&mut game, &mut render);
        assert_eq!(extract.stats().resources_copied, 1);
        assert_eq!(extract.stats().components_copied, 1);
        extract.run(&mut game, &mut render);
        assert_eq!(extract.stats().component_rows_scanned, 0);
        assert_eq!(extract.stats().resources_copied, 0);
        assert_eq!(extract.stats().components_copied, 0);
        // Structural and mutable producers share the same journal.
        let other = game.spawn(Position).id();
        extract.run(&mut game, &mut render);
        assert_eq!(extract.stats().components_copied, 1);
        use country_core::render::extract::WorldComponentWrite;
        bevy_ecs::change_detection::DetectChangesMut::set_changed(&mut game.component_mut::<Position>(entity).unwrap());
        extract.run(&mut game, &mut render);
        assert_eq!((extract.stats().components_copied, extract.stats().component_rows_scanned), (1, 0));
        game.despawn(other);
        extract.run(&mut game, &mut render);
        game.despawn(entity);
        extract.run(&mut game, &mut render);
        assert_eq!(extract.stats().components_removed, 1);
        assert!(render.get_entity(entity).is_err(), "despawned mirrors must not accumulate");
    }

    #[test]
    fn component_removal_journal_handles_reinsertion_gaps_and_empty_shells() {
        #[derive(Component, Clone, PartialEq, Debug)]
        #[component(on_insert = country_core::render::extract::record_component_change::<Position>,
            on_remove = country_core::render::extract::record_component_change::<Position>)]
        struct Position(u32);
        impl country_core::render::extract::JournaledComponent for Position {}
        #[derive(Component)]
        struct SimulationOnly;
        let (mut game, mut render) = (World::new(), World::new());
        let entity = game.spawn((Position(1), SimulationOnly)).id();
        let mut extract = ExtractSet::default();
        extract.mirror_journaled::<Position>();
        extract.run(&mut game, &mut render);
        game.entity_mut(entity).remove::<Position>().insert(Position(2));
        extract.run(&mut game, &mut render);
        assert_eq!(render.get::<Position>(entity), Some(&Position(2)));
        assert_eq!(extract.stats().components_copied, 1);
        assert_eq!(extract.stats().component_rows_scanned, 0);
        game.entity_mut(entity).remove::<Position>();
        extract.run(&mut game, &mut render);
        assert!(game.get_entity(entity).is_ok());
        assert!(render.get_entity(entity).is_err(), "last component removal must reclaim the mirror shell");
        game.entity_mut(entity).insert(Position(3));
        extract.run(&mut game, &mut render);
        assert_eq!(render.get::<Position>(entity), Some(&Position(3)));
        game.entity_mut(entity).remove::<Position>();
        for _ in 0..4100 { game.entity_mut(entity).insert(Position(3)).remove::<Position>(); }
        extract.run(&mut game, &mut render);
        assert_eq!(extract.stats().component_removal_rescans, 1);
        assert!(render.get_entity(entity).is_err(), "a dropped journal entry must trigger reconciliation");
    }

    #[test]
    fn mutable_transform_producers_extract_only_journaled_entities_and_rebuild_after_gaps() {
        use country_core::{utils::components::transform::Transform,
            render::extract::{ComponentWrite, WorldComponentWrite}};
        let (mut game, mut render) = (World::new(), World::new());
        let entities = (0..100).map(|_| game.spawn(Transform::identity()).id()).collect::<Vec<_>>();
        let mut extract = ExtractSet::default();
        extract.mirror_journaled::<Transform>();
        extract.run(&mut game, &mut render);
        assert_eq!(extract.stats().component_rows_scanned, 100);
        extract.run(&mut game, &mut render);
        assert_eq!((extract.stats().component_rows_scanned, extract.stats().components_copied), (0, 0));
        game.query::<ComponentWrite<Transform>>().get_mut(&mut game, entities[1]).unwrap().translation.x = 4.;
        game.component_mut::<Transform>(entities[2]).unwrap().translation.y = 7.;
        extract.run(&mut game, &mut render);
        assert_eq!((extract.stats().component_rows_scanned, extract.stats().components_copied), (0, 2));
        assert_eq!(render.get::<Transform>(entities[1]).unwrap().translation.x, 4.);
        assert_eq!(render.get::<Transform>(entities[2]).unwrap().translation.y, 7.);
        game.entity_mut(entities[1]).remove::<Transform>();
        game.entity_mut(entities[2]).remove::<Transform>().insert(Transform::from_scale(glam::Vec3::splat(2.)));
        extract.run(&mut game, &mut render);
        assert!(render.get_entity(entities[1]).is_err());
        assert_eq!(render.get::<Transform>(entities[2]).unwrap().scale.x, 2.);
        for n in 0..4100 { game.component_mut::<Transform>(entities[2]).unwrap().translation.x = n as f32; }
        extract.run(&mut game, &mut render);
        assert_eq!(extract.stats().component_rows_scanned, 99);
        assert_eq!(render.get::<Transform>(entities[2]).unwrap().translation.x, 4099.);
        let mut incoming = World::new();
        let new_entity = incoming.spawn(Transform::from_scale(glam::Vec3::splat(3.))).id();
        extract.run(&mut incoming, &mut render);
        assert_eq!(render.get::<Transform>(new_entity).unwrap().scale.x, 3.);
        assert!(render.get::<Transform>(entities[2]).is_none());
    }

    #[test]
    fn changed_resources_are_republished_to_an_already_populated_new_destination() {
        let (mut game, mut first, mut second) = (World::new(), World::new(), World::new());
        game.insert_resource(Frame(4));
        second.insert_resource(Frame(99));
        let mut extract = ExtractSet::default();
        extract.copy_changed::<Frame>();
        extract.run(&mut game, &mut first);
        extract.run(&mut game, &mut second);
        assert_eq!(second.resource::<Frame>(), &Frame(4));
    }

    #[test]
    fn component_batches_replace_reused_entity_generations_without_leaving_old_rows() {
        #[derive(Component, Clone, PartialEq, Debug)]
        #[component(on_insert = country_core::render::extract::record_component_change::<Position>,
            on_remove = country_core::render::extract::record_component_change::<Position>)]
        struct Position(u32);
        impl country_core::render::extract::JournaledComponent for Position {}
        let (mut game, mut render) = (World::new(), World::new());
        let mut extract = ExtractSet::default();
        extract.mirror_journaled::<Position>();
        let old = game.spawn(Position(1)).id();
        extract.run(&mut game, &mut render);
        game.despawn(old);
        let next = game.spawn(Position(2)).id();
        assert_eq!(next.index(), old.index());
        assert_ne!(next, old);
        extract.run(&mut game, &mut render);
        assert!(render.get_entity(old).is_err());
        assert_eq!(render.get::<Position>(next), Some(&Position(2)));
    }

    #[test]
    fn retargeting_an_unchanged_source_populates_the_new_render_world() {
        #[derive(Component, Clone, PartialEq, Debug)]
        #[component(on_insert = country_core::render::extract::record_component_change::<Position>,
            on_remove = country_core::render::extract::record_component_change::<Position>)]
        struct Position(u32);
        impl country_core::render::extract::JournaledComponent for Position {}
        let mut game = World::new();
        let mut first = World::new();
        let mut second = World::new();
        let entity = game.spawn(Position(5)).id();
        let mut extract = ExtractSet::default();
        extract.mirror_journaled::<Position>();
        extract.run(&mut game, &mut first);
        extract.run(&mut game, &mut second);
        assert_eq!(second.get::<Position>(entity), Some(&Position(5)));
        assert_eq!(first.get::<Position>(entity), Some(&Position(5)));
    }

    #[test]
    fn mirrored_queries_rebind_when_the_source_world_changes() {
        #[derive(Component, Clone, PartialEq, Debug)]
        #[component(on_insert = country_core::render::extract::record_component_change::<Position>,
            on_remove = country_core::render::extract::record_component_change::<Position>)]
        struct Position(u32);
        impl country_core::render::extract::JournaledComponent for Position {}
        let mut first = World::new();
        let mut second = World::new();
        let mut render = World::new();
        let one = first.spawn(Position(1)).id();
        let two = second.spawn(Position(2)).id();
        let mut extract = ExtractSet::default();
        extract.mirror_journaled::<Position>();
        extract.run(&mut first, &mut render);
        assert_eq!(render.get::<Position>(one), Some(&Position(1)));
        extract.run(&mut second, &mut render);
        assert_eq!(render.get::<Position>(two), Some(&Position(2)));
    }

    #[test]
    fn forwarded_events_cannot_cross_application_worlds() {
        use country_core::render::extract::RenderExtract;
        #[derive(bevy_ecs::event::Event)]
        struct Completed;
        let mut game = World::new();
        let mut other = World::new();
        other.init_resource::<Events<Completed>>();
        let mut render = World::new();
        let mut exchange = RenderExchange::default();
        exchange.forward_events::<Completed>();
        exchange.extract.run(&mut game, &mut render);
        render.send_event(Completed);
        let mut feedback = Vec::new();
        exchange.write_back.run(&mut render, &mut feedback);
        for apply in feedback { apply(&mut other); }
        assert!(other.resource::<Events<Completed>>().is_empty());
    }

    #[test]
    fn queued_updates_wait_for_the_render_owner() {
        #[derive(Resource, Default)]
        struct Pending(Vec<u64>);
        let (mut game, mut render) = (World::new(), World::new());
        game.insert_resource(Pending(vec![2, 3]));
        let mut extract = ExtractSet::default();
        extract.apply_queue::<Pending, Frame>(|pending, frame| {
            frame.0 += pending.0.into_iter().sum::<u64>();
        });
        extract.run(&mut game, &mut render);
        assert_eq!(game.resource::<Pending>().0, [2, 3]);
        render.insert_resource(Frame(1));
        extract.run(&mut game, &mut render);
        assert!(game.resource::<Pending>().0.is_empty());
        assert_eq!(render.resource::<Frame>().0, 6);
        extract.run(&mut game, &mut render);
        assert_eq!(render.resource::<Frame>().0, 6);
    }

    fn apply(exchange: &mut RenderExchange, game: &mut World, render: &mut World) {
        let mut feedback = Vec::new();
        exchange.write_back.run(render, &mut feedback);
        for update in feedback { update(game); }
    }

    #[test]
    fn write_back_does_not_resurrect_a_deleted_game_resource() {
        let (mut game, mut render) = (World::new(), World::new());
        game.insert_resource(Frame(1));
        let mut exchange = exchange();
        exchange.extract.run(&mut game, &mut render);
        render.resource_mut::<Frame>().0 = 2;
        game.remove_resource::<Frame>();
        apply(&mut exchange, &mut game, &mut render);
        assert!(!game.contains_resource::<Frame>());
    }

    #[test]
    fn render_removal_does_not_delete_a_replaced_game_resource() {
        let (mut game, mut render) = (World::new(), World::new());
        game.insert_resource(Frame(1));
        let mut exchange = exchange();
        exchange.extract.run(&mut game, &mut render);
        render.remove_resource::<Frame>();
        game.remove_resource::<Frame>();
        game.insert_resource(Frame(3));
        apply(&mut exchange, &mut game, &mut render);
        assert_eq!(game.resource::<Frame>().0, 3);
    }

    #[test]
    fn concurrent_game_write_wins_without_an_app_update() {
        let (mut game, mut render) = (World::new(), World::new());
        game.insert_resource(Frame(1));
        let mut exchange = exchange();
        exchange.extract.run(&mut game, &mut render);
        render.resource_mut::<Frame>().0 = 2;
        game.resource_mut::<Frame>().0 = 3;
        apply(&mut exchange, &mut game, &mut render);
        assert_eq!(game.resource::<Frame>().0, 3);
    }

    #[test]
    fn write_back_notifies_change_detection_and_rejects_another_world() {
        let (mut game, mut render) = (World::new(), World::new());
        game.insert_resource(Frame(1));
        let mut exchange = exchange();
        exchange.extract.run(&mut game, &mut render);
        let before = game.get_resource_change_ticks::<Frame>().unwrap().changed;
        render.resource_mut::<Frame>().0 = 2;
        let mut other = World::new();
        other.insert_resource(Frame(4));
        apply(&mut exchange, &mut other, &mut render);
        assert_eq!(other.resource::<Frame>().0, 4);
        apply(&mut exchange, &mut game, &mut render);
        assert_eq!(game.resource::<Frame>().0, 2);
        assert_ne!(game.get_resource_change_ticks::<Frame>().unwrap().changed, before);
    }

    fn render(renderer: &mut u64, side: &mut RenderSide) {
        *renderer += side.world.resource::<Frame>().0;
        side.feedback.rendered = true;
        let seen = *renderer;
        side.feedback.defer(move |game| game.insert_resource(Frame(seen)));
    }

    #[test]
    fn frames_hand_over_extracted_state_and_return_feedback() {
        let mut thread = RenderThread::spawn(|| Ok(0u64), render).unwrap();
        let mut extract = ExtractSet::default();
        extract.copy::<Frame>();
        let mut game = World::new();
        for frame in 1..=3 {
            game.insert_resource(Frame(frame));
            thread.submit_frame(&mut game, &mut extract);
            let feedback = thread.apply_feedback(&mut game);
            assert!(feedback.rendered);
        }
        assert_eq!(*game.resource::<Frame>(), Frame(6));
        assert_eq!(thread.call(|renderer| *renderer), 6);
        thread.shutdown();
    }

    #[test]
    fn a_render_panic_resumes_on_the_game_thread() {
        fn boom(_: &mut (), _: &mut RenderSide) {
            panic!("render failure");
        }
        let mut thread = RenderThread::spawn(|| Ok(()), boom).unwrap();
        thread.submit_frame(&mut World::new(), &mut ExtractSet::default());
        let result = catch_unwind(AssertUnwindSafe(|| {
            thread.reclaim();
        }));
        assert!(result.is_err());
        assert!(thread.failed());
        // Later reclaims report the lost renderer instead of blocking.
        assert!(catch_unwind(AssertUnwindSafe(|| {
            thread.reclaim();
        }))
        .is_err());
        thread.join_failed();
        assert!(thread.handle.is_none());
    }

    #[test]
    fn non_send_renderer_is_created_called_and_dropped_on_one_thread() {
        use std::{cell::Cell, rc::Rc, sync::mpsc::channel};
        struct Local {
            value: Rc<Cell<u64>>,
            owner: std::thread::ThreadId,
            dropped: std::sync::mpsc::Sender<std::thread::ThreadId>,
        }
        impl Drop for Local {
            fn drop(&mut self) { self.dropped.send(std::thread::current().id()).unwrap(); }
        }
        let (dropped, observed) = channel();
        let mut thread = RenderThread::spawn(move || Ok(Local {
            value: Rc::new(Cell::new(0)), owner: std::thread::current().id(), dropped,
        }), |local, side| {
            assert_eq!(local.owner, std::thread::current().id());
            local.value.set(local.value.get() + 1);
            side.feedback.rendered = true;
        }).unwrap();
        let owner = thread.call(|local| local.owner);
        assert_ne!(owner, std::thread::current().id());
        for _ in 0..3 { thread.start(); thread.reclaim(); }
        assert_eq!(thread.call(|local| local.value.get()), 3);
        thread.shutdown();
        assert_eq!(observed.recv().unwrap(), owner);
    }

    #[test]
    fn completion_is_published_before_wake_and_poll_never_waits_for_render() {
        use std::sync::mpsc::channel;
        let (release, blocked) = channel();
        let (woke, wakes) = channel();
        let mut thread = RenderThread::spawn(move || Ok(blocked), |release, side| {
            release.recv().unwrap();
            side.feedback.rendered = true;
        }).unwrap();
        thread.set_wake(std::sync::Arc::new(move || { woke.send(()).unwrap(); }));
        thread.start();
        assert!(!thread.poll());
        release.send(()).unwrap();
        wakes.recv_timeout(std::time::Duration::from_secs(5)).unwrap();
        assert!(thread.poll());
        assert!(thread.reclaim().feedback.rendered);
    }

    #[test]
    fn startup_failure_is_returned_and_control_panics_poison_the_owner() {
        let result = RenderThread::<()>::spawn(|| anyhow::bail!("initialization failed"), |_, _| {});
        assert!(matches!(result, Err(error) if error.to_string() == "initialization failed"));
        let mut thread = RenderThread::spawn(|| Ok(()), |_, _| {}).unwrap();
        assert!(catch_unwind(AssertUnwindSafe(|| thread.call::<()>(|_| panic!("control failure")))).is_err());
        assert!(catch_unwind(AssertUnwindSafe(|| { thread.reclaim(); })).is_err());
        thread.join_failed();
        assert!(thread.handle.is_none());
    }

}
