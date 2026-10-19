//! B wall-attached decoration changes.
//!
//! Original B: add 1412acd70/1412ad540, remove 1412add20/1412ae1c0,
//! with_diffs 1412ae660/1412af3a0. The 96-byte defaults at 14010b940/ba60
//! construct the tracker, not a generated/generation-bearing DecoDiff.
//! See TG-bd3d82a9ddc435cfb2/deco-trackers-20261020/{DESIGN,CALLER_CONTRACT}.md.
//!
//! The scoped consumption API is a safe Rust adaptation of the observed B
//! protocol. It does not reconstruct the geometry/audio hosts or establish C
//! equivalence. A caller must explicitly consume removals for every dirty wall.

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use bevy_ecs::resource::Resource;
use country_core::resources::walls::WallId;
use country_core::resources::walls::decorator::DecoratorId;
use country_core::resources::walls::decorator_storage::DecoratorStorageAddr;

pub use super::chimney_assemble_bricks::ChimneyAssemblyParams;
pub use super::door_stairs_assemble_bricks::DoorStairsAssemblyParams;

/// The names describe the recovered B transitions, not recovered source names.
#[derive(Debug)]
pub enum DecoDiff<P> {
    Current(Box<P>),
    Added(Box<P>),
    Removed(Box<P>),
    Changed {
        current: Box<P>,
        previous: Box<P>,
    },
    /// Temporary replacement state; never a valid consumer input.
    #[doc(hidden)]
    Poison,
}

impl<P> DecoDiff<P> {
    /// Removed entries have no current payload. Unchanged entries remain visible.
    pub fn current(&self) -> Option<&P> {
        match self {
            Self::Current(params) | Self::Added(params) => Some(params),
            Self::Changed { current, .. } => Some(current),
            Self::Removed(_) => None,
            Self::Poison => panic!("invalid decoration diff state"),
        }
    }
}

type WallLog<P> = HashMap<DecoratorId, DecoDiff<P>>;

/// Two original resources use this same tracker with distinct canonical Params.
#[derive(Debug)]
pub struct WallAttachedDecoTracker<P> {
    wall_logs: HashMap<WallId, WallLog<P>>,
    dirty_walls: HashSet<WallId>,
}

impl<P: Send + Sync + 'static> Resource for WallAttachedDecoTracker<P> {}

impl<P> Default for WallAttachedDecoTracker<P> {
    fn default() -> Self {
        Self {
            wall_logs: HashMap::new(),
            dirty_walls: HashSet::new(),
        }
    }
}

impl<P> WallAttachedDecoTracker<P> {
    /// Add is not an upsert. Only a removed old value can become a changed value.
    pub fn add(&mut self, storage: DecoratorStorageAddr, id: DecoratorId, params: &P)
    where
        P: Clone,
    {
        let wall = storage.as_wall();
        let current = Box::new(params.clone());
        match self.wall_logs.entry(wall).or_default().entry(id) {
            Entry::Vacant(slot) => {
                slot.insert(DecoDiff::Added(current));
            }
            Entry::Occupied(mut slot) => {
                let old = std::mem::replace(slot.get_mut(), DecoDiff::Poison);
                match old {
                    DecoDiff::Removed(previous) => {
                        *slot.get_mut() = DecoDiff::Changed { current, previous };
                    }
                    _ => panic!("decoration already tracked: {id:?} on {wall:?}"),
                }
            }
        }
        self.dirty_walls.insert(wall);
    }

    /// Removing a changed entry keeps the original previous payload.
    /// Missing, newly added, and already removed entries are protocol errors.
    pub fn remove(&mut self, storage: DecoratorStorageAddr, id: DecoratorId) {
        let wall = storage.as_wall();
        let log = self
            .wall_logs
            .get_mut(&wall)
            .unwrap_or_else(|| panic!("wall not tracked: {wall:?}"));
        let diff = log
            .get_mut(&id)
            .unwrap_or_else(|| panic!("decoration not tracked: {id:?} on {wall:?}"));
        let old = std::mem::replace(diff, DecoDiff::Poison);
        match old {
            DecoDiff::Current(previous) => *diff = DecoDiff::Removed(previous),
            DecoDiff::Changed { current, previous } => {
                *diff = DecoDiff::Removed(previous);
                drop(current);
            }
            _ => panic!("decoration cannot be removed in its current state: {id:?}"),
        }
        self.dirty_walls.insert(wall);
    }

    /// Wall geometry can change without any decoration entries for that wall.
    pub fn on_wall_changed(&mut self, wall: WallId) {
        self.dirty_walls.insert(wall);
    }

    /// Original wall-deletion cleanup drops the log and removes its dirty marker.
    /// It does not emit per-decoration removal callbacks.
    pub fn on_wall_removed(&mut self, wall: WallId) {
        self.wall_logs.remove(&wall);
        self.dirty_walls.remove(&wall);
    }

    /// Drain only dirty walls, retaining the set allocation and its hasher.
    ///
    /// A callback must return the completion produced by consuming this exact
    /// wall/log. There is no implicit no-op removal callback or Drop cleanup.
    pub fn with_diffs<F>(&mut self, mut consume_wall: F)
    where
        F: for<'a> FnMut(WallId, DecoDiffLog<'a, P>) -> ConsumedDiffs<'a, P>,
    {
        let Self {
            wall_logs,
            dirty_walls,
        } = self;
        for wall in dirty_walls.drain() {
            let entries = wall_logs.get_mut(&wall);
            let expected_log = log_identity(entries.as_deref());
            let mut identity = BatchIdentity {
                wall,
                log: expected_log,
            };
            let expected_batch = std::ptr::from_ref(&identity);
            let completed = consume_wall(
                wall,
                DecoDiffLog {
                    entries,
                    identity: &mut identity,
                },
            );
            assert!(
                std::ptr::eq(std::ptr::from_ref(&*completed.identity), expected_batch)
                    && completed.identity.wall == wall
                    && completed.identity.log == expected_log
                    && log_identity(completed.entries.as_deref()) == expected_log,
                "decoration consumer returned completion for another wall/log",
            );
        }
    }
}

// A nonzero-sized, stack-scoped identity also distinguishes absent logs and
// batches belonging to different trackers. It is never exposed or persisted.
struct BatchIdentity {
    wall: WallId,
    log: Option<usize>,
}

fn log_identity<P>(entries: Option<&WallLog<P>>) -> Option<usize> {
    entries.map(|log| std::ptr::from_ref(log) as usize)
}

/// A scoped view of all stored changes for one dirty wall.
#[must_use = "explicitly visit changes and consume removals for this wall"]
pub struct DecoDiffLog<'a, P> {
    entries: Option<&'a mut WallLog<P>>,
    identity: &'a mut BatchIdentity,
}

impl<'a, P> DecoDiffLog<'a, P> {
    pub fn wall(&self) -> WallId {
        self.identity.wall
    }

    /// Visit all entries in map order, including Removed and unchanged Current.
    pub fn with_each_deco_diff<F>(self, mut visit: F) -> DecoRemovalLog<'a, P>
    where
        F: FnMut(DecoratorId, &DecoDiff<P>),
    {
        if let Some(entries) = self.entries.as_deref() {
            for (id, diff) in entries {
                visit(*id, diff);
            }
        }
        DecoRemovalLog {
            entries: self.entries,
            identity: self.identity,
        }
    }

    /// Explicitly skip current geometry when the real host cannot find the wall.
    /// Callers must choose this branch from their actual PublicWalls lookup;
    /// it is not a fallback for an unimplemented current-value consumer.
    pub fn into_removals(self) -> DecoRemovalLog<'a, P> {
        DecoRemovalLog {
            entries: self.entries,
            identity: self.identity,
        }
    }
}

/// Removal/normalization continuation for the same wall and borrowed log.
#[must_use = "explicitly consume removed decorations for this wall"]
pub struct DecoRemovalLog<'a, P> {
    entries: Option<&'a mut WallLog<P>>,
    identity: &'a mut BatchIdentity,
}

impl<'a, P> DecoRemovalLog<'a, P> {
    /// Normalize current values and remove deleted values. Changed drops its old
    /// payload without invoking the true deletion callback.
    pub fn with_removed<F>(mut self, mut removed: F) -> ConsumedDiffs<'a, P>
    where
        F: FnMut(DecoratorId, &P),
    {
        if let Some(entries) = self.entries.as_deref_mut() {
            entries.retain(|id, diff| match std::mem::replace(diff, DecoDiff::Poison) {
                DecoDiff::Current(current) | DecoDiff::Added(current) => {
                    *diff = DecoDiff::Current(current);
                    true
                }
                DecoDiff::Removed(previous) => {
                    removed(*id, &previous);
                    false
                }
                DecoDiff::Changed { current, previous } => {
                    *diff = DecoDiff::Current(current);
                    drop(previous);
                    true
                }
                DecoDiff::Poison => panic!("invalid decoration diff state"),
            });
        }
        ConsumedDiffs {
            entries: self.entries,
            identity: self.identity,
        }
    }
}

/// Private construction, no Default/Copy/Clone: this carries the consumed batch
/// identity and its actual wall-log borrow back to the tracker for validation.
pub struct ConsumedDiffs<'a, P> {
    entries: Option<&'a mut WallLog<P>>,
    identity: &'a mut BatchIdentity,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wall(id: u64) -> DecoratorStorageAddr {
        DecoratorStorageAddr::Wall(WallId(id))
    }

    #[test]
    fn changed_then_removed_reports_the_original_value() {
        let mut tracker = WallAttachedDecoTracker::<u32>::default();
        tracker.add(wall(7), DecoratorId(3), &10);
        tracker.with_diffs(|_, log| {
            log.with_each_deco_diff(|_, diff| assert_eq!(diff.current(), Some(&10)))
                .with_removed(|_, _| panic!("added value is not deleted"))
        });
        tracker.remove(wall(7), DecoratorId(3));
        tracker.add(wall(7), DecoratorId(3), &20);
        tracker.remove(wall(7), DecoratorId(3));
        let mut deleted = Vec::new();
        tracker.with_diffs(|wall, log| {
            assert_eq!(wall, WallId(7));
            log.with_each_deco_diff(|_, diff| assert!(diff.current().is_none()))
                .with_removed(|id, old| deleted.push((id, *old)))
        });
        assert_eq!(deleted, vec![(DecoratorId(3), 10)]);
        tracker.with_diffs(|_, _| panic!("drained wall must not run again"));
    }

    #[test]
    fn moving_between_walls_keeps_removal_and_current_separate() {
        let mut tracker = WallAttachedDecoTracker::<u32>::default();
        tracker.add(wall(1), DecoratorId(9), &11);
        tracker.with_diffs(|_, log| log.with_each_deco_diff(|_, _| {}).with_removed(|_, _| {}));
        tracker.remove(wall(1), DecoratorId(9));
        tracker.add(wall(2), DecoratorId(9), &22);
        let mut current = Vec::new();
        let mut removed = Vec::new();
        tracker.with_diffs(|wall, log| {
            log.with_each_deco_diff(|id, diff| {
                if let Some(params) = diff.current() {
                    current.push((wall, id, *params));
                }
            })
            .with_removed(|id, old| removed.push((wall, id, *old)))
        });
        assert_eq!(current, vec![(WallId(2), DecoratorId(9), 22)]);
        assert_eq!(removed, vec![(WallId(1), DecoratorId(9), 11)]);
    }

    #[test]
    fn changed_consumption_keeps_new_value_without_deletion_callback() {
        let mut tracker = WallAttachedDecoTracker::<u32>::default();
        tracker.add(wall(6), DecoratorId(2), &13);
        tracker.with_diffs(|_, log| log.with_each_deco_diff(|_, _| {}).with_removed(|_, _| {}));
        tracker.remove(wall(6), DecoratorId(2));
        tracker.add(wall(6), DecoratorId(2), &17);
        tracker.with_diffs(|_, log| {
            log.with_each_deco_diff(|_, diff| match diff {
                DecoDiff::Changed { current, previous } => {
                    assert_eq!(**current, 17);
                    assert_eq!(**previous, 13);
                }
                _ => panic!("remove/add must produce changed"),
            })
            .with_removed(|_, _| panic!("changed is not a deletion"))
        });
        tracker.on_wall_changed(WallId(6));
        tracker.with_diffs(|_, log| {
            log.with_each_deco_diff(|_, diff| {
                assert!(matches!(diff, DecoDiff::Current(value) if **value == 17));
            })
            .with_removed(|_, _| panic!("wall-only change does not delete decoration"))
        });
    }

    #[test]
    fn dirty_missing_wall_is_explicitly_consumed_and_wall_delete_drops_pending_work() {
        let mut tracker = WallAttachedDecoTracker::<u32>::default();
        tracker.on_wall_changed(WallId(4));
        let mut walls = Vec::new();
        tracker.with_diffs(|wall, log| {
            walls.push(wall);
            // Fixture models a real host's explicit PublicWalls miss branch.
            log.into_removals()
                .with_removed(|_, _| panic!("missing log"))
        });
        assert_eq!(walls, vec![WallId(4)]);
        tracker.add(wall(4), DecoratorId(1), &8);
        tracker.on_wall_removed(WallId(4));
        tracker.with_diffs(|_, _| panic!("deleted wall must not be consumed"));
    }

    #[test]
    fn strict_protocol_rejects_unconsumed_add_then_remove() {
        let mut tracker = WallAttachedDecoTracker::<u32>::default();
        tracker.add(wall(2), DecoratorId(1), &8);
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                tracker.remove(wall(2), DecoratorId(1));
            }))
            .is_err()
        );
        // No recovery-and-continue contract is asserted after the panic.
    }
}
