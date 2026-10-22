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
use bevy_ecs::{change_detection::DetectChanges, resource::Resource, world::World};
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
                    let returned = match result {
                        Ok(()) => Returned::Side(side),
                        Err(payload) => Returned::Panicked(payload),
                    };
                    if outbox.send(returned).is_err() {
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
        })
    }

    /// Whether a frame is being rendered right now.
    pub fn in_flight(&self) -> bool {
        self.idle.is_none()
    }

    /// Wait for the frame in flight (if any) and borrow the side. A panic on
    /// the render thread resumes on the game thread with its payload.
    pub fn reclaim(&mut self) -> &mut RenderSide<R> {
        if self.idle.is_none() {
            match self.done.recv() {
                Ok(Returned::Side(side)) => self.idle = Some(side),
                Ok(Returned::Panicked(payload)) => resume_unwind(payload),
                Err(_) => panic!("render thread exited while a frame was in flight"),
            }
        }
        self.idle.as_mut().expect("reclaimed side").get()
    }

    /// Reclaim, extract the next frame into the render world, and start it.
    pub fn submit_frame(&mut self, game: &mut World, extract: &mut ExtractSet) {
        let side = self.reclaim();
        extract.run(game, &mut side.world);
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
            if self.idle.is_none() {
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
/// into the render world and applies queued library changes (category B).
#[derive(Default)]
pub struct ExtractSet {
    steps: Vec<Box<dyn FnMut(&mut World, &mut World) + Send>>,
}
impl ExtractSet {
    pub fn add(&mut self, step: impl FnMut(&mut World, &mut World) + Send + 'static) -> &mut Self {
        self.steps.push(Box::new(step));
        self
    }
    /// Category A: the render world receives a copy of `T` every frame, or
    /// loses it when the game world has none.
    pub fn copy<T: Resource + Clone>(&mut self) -> &mut Self {
        self.add(|game, render| match game.get_resource::<T>() {
            Some(value) => {
                let value = value.clone();
                render.insert_resource(value);
            }
            None => {
                render.remove_resource::<T>();
            }
        })
    }
    /// Copy only when the game-side value changed since the previous extract
    /// (bevy change detection), so unchanged state costs one tick comparison.
    pub fn copy_changed<T: Resource + Clone>(&mut self) -> &mut Self {
        self.add(|game, render| {
            let Some(value) = game.get_resource_ref::<T>() else {
                render.remove_resource::<T>();
                return;
            };
            if value.is_changed() || !render.contains_resource::<T>() {
                let value = value.clone();
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
        self.add(move |game, render| {
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
        for step in &mut self.steps {
            step(game, render);
        }
        game.clear_trackers();
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
        std::mem::forget(thread);
    }
}
