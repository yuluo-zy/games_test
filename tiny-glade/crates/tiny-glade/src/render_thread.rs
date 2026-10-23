//! Dedicated render thread (reports/render-thread-2026-10-22/ARCHITECTURE.md).
//!
//! The game thread owns the game `World`; the render thread owns a
//! [`RenderSide`]: the render `World` (everything declaration reads) and the
//! renderer (graph session, swapchain, resource pools). They meet once per
//! frame, at the hand-over point, while the render thread is idle:
//!
//! ```text
//! game:   update N+1 ──────────── reclaim ─ extract N+1 ─ submit ─ update N+2 …
//! render: ── declare/compile/record/submit/present N ──┘            └─ frame N+1 …
//! ```
//!
//! `reclaim` is the only back-pressure point: the game thread runs at most one
//! frame ahead. Results that must reach the game world (screenshots, errors,
//! progress flags) travel back as [`FrameFeedback`].
use bevy_ecs::{
    change_detection::{DetectChanges, DetectChangesMut, Ref},
    component::{Component, Tick},
    entity::Entity,
    event::{Event, Events},
    query::QueryState,
    resource::Resource,
    world::World,
};
use bevy_state::state::{State, States};
use std::collections::HashMap;
use std::{
    panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
    sync::mpsc::{Receiver, SyncSender, sync_channel},
    thread::JoinHandle,
};

/// Moves a self-contained object graph to another thread.
///
/// The render side keeps single-threaded shared ownership (`Rc`, `RefCell`)
/// internally. Moving the whole side at once is sound as long as no `Rc` clone
/// of its contents is kept outside it: every clone is created and dropped by
/// whichever thread currently owns the side. Constructors of `RenderSide`
/// state must uphold this; nothing it owns may be handed to the game world.
pub struct Exclusive<T>(T);
// SAFETY: see the type documentation; access is always through one owner.
unsafe impl<T> Send for Exclusive<T> {}
impl<T> Exclusive<T> {
    /// # Safety
    /// No `Rc`/`RefCell`/thread-affine handle reachable from `value` may be
    /// shared with anything outside `value`.
    pub unsafe fn new(value: T) -> Self {
        Self(value)
    }
    pub fn get(&mut self) -> &mut T {
        &mut self.0
    }
    pub fn into_inner(self) -> T {
        self.0
    }
}

/// Everything the render thread owns between two hand-overs.
pub struct RenderSide<R> {
    /// Extracted state: declaration reads only this world.
    pub world: World,
    pub renderer: R,
    /// Feedback of the frame rendered last, applied by the game thread on reclaim.
    pub feedback: FrameFeedback,
}

/// Results of one rendered frame destined for the game world.
#[derive(Default)]
pub struct FrameFeedback {
    pub rendered: bool,
    pub submitted: bool,
    pub present_queued: bool,
    pub error: Option<anyhow::Error>,
    /// Deferred mutations of the game world (screenshot completion, stats).
    pub apply: Vec<Box<dyn FnOnce(&mut World) + Send>>,
}
impl FrameFeedback {
    pub fn defer(&mut self, apply: impl FnOnce(&mut World) + Send + 'static) {
        self.apply.push(Box::new(apply));
    }
}

/// The per-frame render work, run on the render thread.
pub type RenderFrameFn<R> = fn(&mut RenderSide<R>);

enum Message<R> {
    Frame(Exclusive<RenderSide<R>>),
    Shutdown,
}
enum Returned<R> {
    Side(Exclusive<RenderSide<R>>),
    Panicked(Box<dyn std::any::Any + Send>),
}

pub struct RenderThread<R: 'static> {
    jobs: SyncSender<Message<R>>,
    done: Receiver<Returned<R>>,
    /// The side while the game thread holds it (between reclaim and submit).
    idle: Option<Exclusive<RenderSide<R>>>,
    handle: Option<JoinHandle<()>>,
    /// The render thread panicked; its side was lost with the frame.
    poisoned: bool,
}

impl<R: 'static> RenderThread<R> {
    /// # Safety
    /// `side` must satisfy [`Exclusive::new`]'s contract.
    pub unsafe fn spawn(side: RenderSide<R>, render: RenderFrameFn<R>) -> std::io::Result<Self> {
        let (jobs, inbox) = sync_channel::<Message<R>>(1);
        let (outbox, done) = sync_channel::<Returned<R>>(1);
        let handle = std::thread::Builder::new()
            .name("tg-render".into())
            .spawn(move || {
                while let Ok(Message::Frame(mut side)) = inbox.recv() {
                    let result = catch_unwind(AssertUnwindSafe(|| render(side.get())));
                    let (returned, panicked) = match result {
                        Ok(()) => (Returned::Side(side), false),
                        Err(payload) => (Returned::Panicked(payload), true),
                    };
                    // A side that panicked mid-frame is gone; so is the thread.
                    if outbox.send(returned).is_err() || panicked {
                        break;
                    }
                }
            })?;
        Ok(Self {
            jobs,
            done,
            // SAFETY: forwarded from the caller.
            idle: Some(unsafe { Exclusive::new(side) }),
            handle: Some(handle),
            poisoned: false,
        })
    }

    /// Whether a frame is being rendered right now.
    pub fn in_flight(&self) -> bool {
        self.idle.is_none()
    }

    /// Wait for the frame in flight (if any) and borrow the side. A panic on
    /// the render thread resumes on the game thread with its payload.
    pub fn reclaim(&mut self) -> &mut RenderSide<R> {
        if self.poisoned {
            panic!("the render thread panicked earlier; the renderer is gone");
        }
        if self.idle.is_none() {
            match self.done.recv() {
                Ok(Returned::Side(side)) => self.idle = Some(side),
                Ok(Returned::Panicked(payload)) => {
                    self.poisoned = true;
                    resume_unwind(payload)
                }
                Err(_) => {
                    self.poisoned = true;
                    panic!("render thread exited while a frame was in flight")
                }
            }
        }
        self.idle.as_mut().expect("reclaimed side").get()
    }

    /// Reclaim, extract the next frame into the render world, and start it.
    pub fn submit_frame(&mut self, game: &mut World, extract: &mut ExtractSet) {
        let side = self.reclaim();
        extract.run(game, &mut side.world);
        self.start();
    }

    /// Start rendering with the side as the game thread left it.
    pub fn start(&mut self) {
        self.reclaim();
        let side = self.idle.take().expect("reclaimed side");
        if self.jobs.send(Message::Frame(side)).is_err() {
            panic!("render thread is gone");
        }
    }

    /// Apply the feedback of the last rendered frame to the game world.
    pub fn apply_feedback(&mut self, game: &mut World) -> FrameFeedback {
        let side = self.reclaim();
        let mut feedback = std::mem::take(&mut side.feedback);
        for apply in feedback.apply.drain(..) {
            apply(game);
        }
        feedback
    }

    /// Finish the frame in flight and stop the thread, returning the side.
    pub fn shutdown(mut self) -> RenderSide<R> {
        self.reclaim();
        let side = self.idle.take().expect("reclaimed side");
        let _ = self.jobs.send(Message::Shutdown);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
        side.into_inner()
    }
}

impl<R: 'static> Drop for RenderThread<R> {
    fn drop(&mut self) {
        if self.handle.is_some() {
            if self.idle.is_none() && !self.poisoned {
                let _ = self.done.recv();
            }
            let _ = self.jobs.send(Message::Shutdown);
            if let Some(handle) = self.handle.take() {
                let _ = handle.join();
            }
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
    steps: Vec<Box<dyn FnMut(&mut World, &mut World, Tick) + Send>>,
    last_run: Option<Tick>,
}
impl ExtractSet {
    pub fn add(&mut self, step: impl FnMut(&mut World, &mut World, Tick) + Send + 'static) -> &mut Self {
        self.steps.push(Box::new(step));
        self
    }
    /// Category A: the render world receives a copy of `T` every frame, or
    /// loses it when the game world has none.
    pub fn copy<T: Resource + Clone>(&mut self) -> &mut Self {
        self.add(|game, render, _| match game.get_resource::<T>() {
            Some(value) => {
                let value = value.clone();
                render.insert_resource(value);
            }
            None => {
                render.remove_resource::<T>();
            }
        })
    }
    /// Copy only when the game-side value changed since the previous extract,
    /// so unchanged state costs one tick comparison.
    pub fn copy_changed<T: Resource + Clone>(&mut self) -> &mut Self {
        self.add(|game, render, last_run| {
            let this_run = game.read_change_tick();
            let Some(value) = game.get_resource_ref::<T>() else {
                render.remove_resource::<T>();
                return;
            };
            if value.last_changed().is_newer_than(last_run, this_run) || !render.contains_resource::<T>() {
                let value = value.clone();
                render.insert_resource(value);
            }
        })
    }
    /// Mirror component `C` onto the same entity ids in the render world:
    /// changed or new values are cloned, removed ones are removed.
    pub fn mirror<C: Component + Clone>(&mut self) -> &mut Self {
        let mut query: Option<QueryState<(Entity, Ref<'static, C>)>> = None;
        let mut seen: HashMap<Entity, u64> = HashMap::new();
        let mut frame = 0u64;
        let mut pending: Vec<(Entity, C)> = Vec::new();
        self.add(move |game, render, last_run| {
            frame += 1;
            let this_run = game.read_change_tick();
            let query = query.get_or_insert_with(|| game.query::<(Entity, Ref<C>)>());
            for (entity, value) in query.iter(game) {
                let fresh = seen.insert(entity, frame).is_none();
                if fresh || value.last_changed().is_newer_than(last_run, this_run) {
                    pending.push((entity, value.clone()));
                }
            }
            seen.retain(|entity, stamp| {
                let keep = *stamp == frame;
                if !keep && let Ok(mut mirrored) = render.get_entity_mut(*entity) {
                    mirrored.remove::<C>();
                }
                keep
            });
            insert_mirrored(render, &mut pending);
        })
    }
    /// The events of `E` sent since the previous extract.
    pub fn events<E: Event + Clone>(&mut self) -> &mut Self {
        let mut cursor = bevy_ecs::event::EventCursor::<E>::default();
        self.add(move |game, render, _| {
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
            let Some(mut queue) = game.get_resource_mut::<Q>() else {
                return;
            };
            let pending = std::mem::take(&mut *queue);
            if let Some(mut target) = render.get_resource_mut::<T>() {
                apply(pending, &mut target);
            }
        })
    }
    pub fn run(&mut self, game: &mut World, render: &mut World) {
        // The first extract copies everything.
        let last_run = self.last_run.unwrap_or(Tick::new(0));
        let this_run = game.change_tick();
        for step in &mut self.steps {
            step(game, render, last_run);
        }
        self.last_run = Some(this_run);
        let render_tick = render.change_tick();
        render.insert_resource(ExtractedAt { game: this_run, render: render_tick });
        // Declaration's writes must carry a later tick than the extract's.
        render.increment_change_tick();
    }
}

/// A mutation of the game world produced on the render thread.
pub type GameApply = Box<dyn FnOnce(&mut World) + Send>;

/// Ticks of the last extract, in both worlds.
#[derive(Resource, Clone, Copy)]
pub struct ExtractedAt {
    pub game: Tick,
    pub render: Tick,
}

/// Runs on the render thread after declaration: turns render-side writes the
/// game needs into `GameApply`s, applied at the next hand-over.
#[derive(Default)]
pub struct WriteBackSet {
    steps: Vec<Box<dyn FnMut(&mut World, ExtractedAt, &mut Vec<GameApply>) + Send>>,
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
    fn first<T: 'static>(&mut self, kind: u8) -> bool {
        self.registered.insert((kind, std::any::TypeId::of::<T>()))
    }
}

impl country_core::render::extract::RenderExtract for RenderExchange {
    fn component<C: Component + Clone>(&mut self) {
        if self.first::<C>(0) {
            self.extract.mirror::<C>();
        }
    }
    fn resource<R: Resource + Clone>(&mut self) {
        if self.first::<R>(1) {
            self.extract.copy_changed::<R>();
        }
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
        let present = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let seen = present.clone();
        self.extract.add(move |_, render, _| {
            seen.store(render.contains_resource::<R>(), std::sync::atomic::Ordering::Relaxed);
        });
        self.write_back.steps.push(Box::new(move |render, at, out| {
            let this_run = render.read_change_tick();
            match render.get_resource_ref::<R>() {
                Some(value) if value.last_changed().is_newer_than(at.render, this_run) => {
                    let value = value.clone();
                    out.push(Box::new(move |game: &mut World| {
                        let now = game.read_change_tick();
                        match game.get_resource_mut::<R>() {
                            Some(current) if current.last_changed().is_newer_than(at.game, now) => {}
                            Some(mut current) => *current.bypass_change_detection() = value,
                            None => game.insert_resource(value),
                        }
                    }));
                }
                Some(_) => {}
                None if present.load(std::sync::atomic::Ordering::Relaxed) => {
                    out.push(Box::new(|game: &mut World| {
                        game.remove_resource::<R>();
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
        self.write_back.steps.push(Box::new(move |render, at, out| {
            let Some(apply) = render.get_resource::<R>().and_then(take) else {
                return;
            };
            out.push(Box::new(move |game: &mut World| {
                let now = game.read_change_tick();
                if let Some(mut current) = game.get_resource_mut::<R>()
                    && !(guarded && current.last_changed().is_newer_than(at.game, now))
                {
                    apply(current.bypass_change_detection());
                }
            }));
        }));
    }
    fn forward_events<E: Event>(&mut self) {
        self.extract.add(|_, render, _| {
            render.init_resource::<Events<E>>();
        });
        self.write_back.steps.push(Box::new(|render, _, out| {
            let Some(mut events) = render.get_resource_mut::<Events<E>>() else {
                return;
            };
            let sent: Vec<E> = events.drain().collect();
            if !sent.is_empty() {
                out.push(Box::new(move |game: &mut World| {
                    if let Some(mut events) = game.get_resource_mut::<Events<E>>() {
                        events.send_batch(sent);
                    }
                }));
            }
        }));
    }
}


/// Insert mirrored values at their game entity ids. An index the game reused
/// with a new generation replaces the stale render entity first.
fn insert_mirrored<C: Component>(render: &mut World, pending: &mut Vec<(Entity, C)>) {
    for (entity, value) in pending.drain(..) {
        if let Ok(mut mirrored) = render.get_entity_mut(entity) {
            mirrored.insert(value);
            continue;
        }
        if let Some(stale) = render.entities().resolve_from_id(entity.index())
            && stale != entity
        {
            render.despawn(stale);
        }
        #[allow(deprecated)]
        let _ = render.insert_or_spawn_batch(std::iter::once((entity, (value,))));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Resource, Clone, Default, PartialEq, Debug)]
    struct Frame(u64);

    fn render(side: &mut RenderSide<u64>) {
        side.renderer += side.world.resource::<Frame>().0;
        side.feedback.rendered = true;
        let seen = side.renderer;
        side.feedback.defer(move |game| game.insert_resource(Frame(seen)));
    }

    #[test]
    fn frames_hand_over_extracted_state_and_return_feedback() {
        let side = RenderSide { world: World::new(), renderer: 0u64, feedback: Default::default() };
        // SAFETY: plain data, nothing shared.
        let mut thread = unsafe { RenderThread::spawn(side, render) }.unwrap();
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
        assert_eq!(thread.shutdown().renderer, 6);
    }

    #[test]
    fn a_render_panic_resumes_on_the_game_thread() {
        fn boom(_: &mut RenderSide<()>) {
            panic!("render failure");
        }
        let side = RenderSide { world: World::new(), renderer: (), feedback: Default::default() };
        // SAFETY: plain data, nothing shared.
        let mut thread = unsafe { RenderThread::spawn(side, boom) }.unwrap();
        thread.submit_frame(&mut World::new(), &mut ExtractSet::default());
        let result = catch_unwind(AssertUnwindSafe(|| {
            thread.reclaim();
        }));
        assert!(result.is_err());
        // Later reclaims report the lost renderer instead of blocking.
        assert!(catch_unwind(AssertUnwindSafe(|| {
            thread.reclaim();
        }))
        .is_err());
        drop(thread);
    }
}
