//! What the render thread reads (reports/render-thread-2026-10-22).
//!
//! Declaration runs on the render thread against its own `World`. Every module
//! that declares from the World lists the components and resources it reads in
//! a `render_extract` function, using the same type names as its accessors; the
//! host's extractor mirrors exactly those into the render World each frame.
use bevy_ecs::{component::Component, event::Event, resource::Resource};
use bevy_state::state::States;

mod component_write;
pub use component_write::{ComponentWrite, ComponentMut};

/// Exclusive World producers use the same journal as query producers. Before
/// the first extraction no journal is needed: that extraction copies all rows.
pub trait WorldComponentWrite {
    fn component_mut<C: Component<Mutability = bevy_ecs::component::Mutable>>(
        &mut self, entity: bevy_ecs::entity::Entity) -> Option<bevy_ecs::change_detection::Mut<'_, C>>;
}
impl WorldComponentWrite for bevy_ecs::world::World {
    fn component_mut<C: Component<Mutability = bevy_ecs::component::Mutable>>(
        &mut self, entity: bevy_ecs::entity::Entity) -> Option<bevy_ecs::change_detection::Mut<'_, C>> {
        if let Some(journal) = self.get_resource::<ComponentJournal<C>>() { journal.record(entity); }
        self.get_mut::<C>(entity)
    }
}
pub trait EntityComponentWrite {
    fn component_mut<C: Component<Mutability = bevy_ecs::component::Mutable>>(
        &mut self) -> Option<bevy_ecs::change_detection::Mut<'_, C>>;
}
impl EntityComponentWrite for bevy_ecs::world::EntityWorldMut<'_> {
    fn component_mut<C: Component<Mutability = bevy_ecs::component::Mutable>>(
        &mut self) -> Option<bevy_ecs::change_detection::Mut<'_, C>> {
        let entity = self.id();
        self.world_scope(|world| {
            if let Some(journal) = world.get_resource::<ComponentJournal<C>>() { journal.record(entity); }
        });
        self.get_mut::<C>()
    }
}

/// In the render world: the game world has an `R` (declaration checks that a
/// producer is installed without reading it).
#[derive(Resource)]
pub struct Present<R: Send + Sync + 'static>(core::marker::PhantomData<fn() -> R>);
impl<R: Send + Sync + 'static> Default for Present<R> {
    fn default() -> Self {
        Self(core::marker::PhantomData)
    }
}

/// A partial write-back preserves Bevy change detection: only dereferencing
/// the received owner mutably marks the game-side resource changed.
pub type WriteBack<R> = Box<dyn for<'a> FnOnce(bevy_ecs::change_detection::Mut<'a, R>) + Send>;

pub trait RenderExtract {
    /// Structural hooks and mutable producers publish the changed entities.
    /// Extraction visits only those entities between hand-overs.
    fn component_journaled<C: JournaledComponent>(&mut self);
    /// Copy resource `R` whenever it changed in the game world.
    fn resource<R: Resource + Clone>(&mut self);
    /// Publish only the declaration view, retaining the producer's CPU storage
    /// exclusively on the game side. Every reader must use the same projection.
    fn resource_projected<R: Resource>(&mut self, project: fn(&R) -> R);
    /// Combine ECS change detection with an interior-mutation revision.
    /// All declarations of the same resource must use the same revision source.
    fn resource_with_revision<R: Resource + Clone>(&mut self, revision: fn(&R) -> u64);
    /// Asset tables publish entry identities and reuse their snapshot allocations.
    fn asset_library<R: crate::render::asset_ack::AssetSnapshot>(&mut self);
    /// Copy this frame's pending events of `E`.
    fn events<E: Event + Clone>(&mut self);
    /// Copy the current value of state `S`.
    fn state<S: States>(&mut self);
    /// Mirror only whether `R` exists, as `Present<R>`.
    fn presence<R: Resource>(&mut self);
    /// Declaration writes `R` and the game reads it: whenever the render copy
    /// changes during a frame it is written back to the game world at the
    /// next hand-over, unless the game changed `R` itself since the extract.
    /// A render-side removal is written back too.
    fn write_back<R: Resource + Clone>(&mut self);
    /// A partial write-back: `take` reads the render copy after the frame
    /// and returns what to apply to the game's `R` (game-side changes since the
    /// extract win when `guarded`).
    fn write_back_with<R: Resource>(&mut self, guarded: bool, take: fn(&R) -> Option<WriteBack<R>>);
    /// Events of `E` that declaration sends are forwarded to the game world.
    fn forward_events<E: Event>(&mut self);
    /// `R` belongs to the render side: whenever the game world (re)creates it,
    /// it moves to the render world.
    fn render_owned<R: Resource>(&mut self);
}

/// Insert/remove hooks publish structural changes. Mutable implementations
/// additionally route every producer through ComponentWrite or component_mut;
/// structural and mutable producers must use the same journal.
pub trait JournaledComponent: Component + Clone {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComponentCursor(ChangeCursor);

/// Installed when a producer query is initialized or at first extraction.
/// Earlier spawns are covered by that extraction's full initial copy, so hooks
/// need not allocate resources or issue commands during entity creation.
#[derive(Resource)]
pub struct ComponentJournal<C: Component> {
    changes: ChangeJournal<bevy_ecs::entity::Entity>,
    marker: core::marker::PhantomData<fn() -> C>,
}
impl<C: Component> Default for ComponentJournal<C> {
    fn default() -> Self {
        Self { changes: ChangeJournal::default(), marker: core::marker::PhantomData }
    }
}
impl<C: Component> ComponentJournal<C> {
    pub fn record(&self, entity: bevy_ecs::entity::Entity) { self.changes.record(entity); }
    pub fn cursor(&self) -> ComponentCursor { ComponentCursor(self.changes.cursor()) }
    pub fn since(&self, cursor: Option<ComponentCursor>)
        -> Option<(ComponentCursor, Vec<bevy_ecs::entity::Entity>)> {
        self.changes.since(cursor.map(|cursor| cursor.0))
            .map(|(cursor, entities)| (ComponentCursor(cursor), entities))
    }
}
pub fn record_component_change<C: Component>(
    world: bevy_ecs::world::DeferredWorld,
    context: bevy_ecs::component::HookContext,
) {
    if let Some(journal) = world.get_resource::<ComponentJournal<C>>() {
        journal.changes.record(context.entity);
    }
}

/// A bounded journal records actual mutation entry points, independently of ECS
/// resource ticks. Every owner has a unique identity, including cloned owners.
/// Consumers that miss retained history must rebuild instead of losing edits.
pub(crate) struct ChangeJournal<K> {
    owner: u64,
    state: parking_lot::Mutex<JournalState<K>>,
}
struct JournalState<K> {
    revision: u64,
    entries: std::collections::VecDeque<(u64, K)>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ChangeCursor { owner: u64, pub(crate) revision: u64 }
const JOURNAL_CAPACITY: usize = 4096;

pub(crate) fn next_publication() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(1);
    NEXT.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| n.checked_add(1))
        .expect("asset journal revision exhausted")
}

impl<K> Default for ChangeJournal<K> {
    fn default() -> Self {
        let owner = next_publication();
        Self { owner, state: parking_lot::Mutex::new(JournalState {
            revision: owner, entries: std::collections::VecDeque::new(),
        }) }
    }
}
impl<K> ChangeJournal<K> {
    pub fn record(&self, key: K) {
        let mut state = self.state.lock();
        let previous = state.revision;
        state.revision = next_publication();
        if state.entries.len() == JOURNAL_CAPACITY { state.entries.pop_front(); }
        state.entries.push_back((previous, key));
    }
    pub fn cursor(&self) -> ChangeCursor {
        ChangeCursor { owner: self.owner, revision: self.state.lock().revision }
    }
    pub fn revision(&self) -> u64 { self.state.lock().revision }
}
impl<K: Clone> ChangeJournal<K> {
    pub fn since(&self, cursor: Option<ChangeCursor>) -> Option<(ChangeCursor, Vec<K>)> {
        let cursor = cursor?;
        if cursor.owner != self.owner { return None; }
        let state = self.state.lock();
        let current = ChangeCursor { owner: self.owner, revision: state.revision };
        if cursor == current { return Some((current, Vec::new())); }
        let start = state.entries.binary_search_by_key(&cursor.revision, |(before, _)| *before).ok()?;
        Some((current, state.entries.iter().skip(start).map(|(_, key)| key.clone()).collect()))
    }
}

#[cfg(test)]
mod journal_tests {
    use super::*;
    #[test]
    fn journal_detects_expired_history_and_different_sources() {
        let journal = ChangeJournal::default();
        let initial = journal.cursor();
        journal.record(7);
        let first = journal.cursor();
        journal.record(9);
        assert_eq!(journal.since(Some(initial)).unwrap().1, [7, 9]);
        assert_eq!(journal.since(Some(first)).unwrap().1, [9]);
        assert!(ChangeJournal::<u32>::default().since(Some(first)).is_none());
        for n in 0..JOURNAL_CAPACITY { journal.record(n as u32); }
        assert!(journal.since(Some(first)).is_none());
        assert!(journal.since(Some(journal.cursor())).unwrap().1.is_empty());
        assert_eq!(journal.state.lock().entries.len(), JOURNAL_CAPACITY);
        assert!(journal.state.lock().entries.capacity() <= JOURNAL_CAPACITY);
    }
}
