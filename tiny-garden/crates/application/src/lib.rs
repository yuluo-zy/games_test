//! Editing transactions, bounded history and latest-wins generation requests.
//! This layer has no dependency on ECS, task executors or renderer.
pub mod context;
mod strokes;
pub mod tools;
use garden_domain::{BlockId, Building, BuildingDraft, BuildingEdit, BuildingId, DomainError};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fmt,
    sync::Arc,
};
pub use strokes::StrokeEdit;

#[derive(Debug, Clone)]
pub enum EditCommand {
    Create(BuildingDraft),
    Edit {
        building: BuildingId,
        edit: BuildingEdit,
    },
    Delete(BuildingId),
    CommitPreview(Preview),
    Undo,
    Redo,
    Stroke(StrokeEdit),
    Context(context::ContextEdit),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditError {
    Domain(DomainError),
    Missing,
    IdAlreadyUsed,
    BlockIdAlreadyUsed,
    EmptyHistory,
    CounterExhausted,
    DuplicateId,
    StalePreview,
}
impl From<DomainError> for EditError {
    fn from(e: DomainError) -> Self {
        Self::Domain(e)
    }
}
impl fmt::Display for EditError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for EditError {}

/// Consumers get aggregate-scoped change data, not an unbounded event outbox.
#[derive(Debug, Clone)]
pub struct Change {
    pub building: BuildingId,
    pub revision: u64,
    pub before: Option<Arc<Building>>,
    pub after: Option<Arc<Building>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JobTicket {
    pub session: u64,
    pub building: BuildingId,
    pub object_revision: u64,
    pub rules_revision: u64,
    pub request_serial: u64,
}

#[derive(Debug, Clone)]
pub struct BuildRequest {
    pub ticket: JobTicket,
    pub snapshot: Arc<Building>,
}

/// A drag owns this temporary candidate; update/cancel never touch scene or history.
#[derive(Debug, Clone)]
pub struct Preview {
    session: u64,
    revision: u64,
    candidate: Building,
}
impl Preview {
    pub fn candidate(&self) -> &Building {
        &self.candidate
    }
    pub fn update(&mut self, edit: BuildingEdit) -> Result<(), DomainError> {
        self.candidate = self.candidate.edited(edit)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Patch {
    id: BuildingId,
    before: Option<Arc<Building>>,
    after: Option<Arc<Building>>,
}
#[derive(Debug, Clone)]
enum History {
    Building(Patch),
    Stroke(strokes::StrokePatch),
    Context(Box<context::ContextPatch>),
}

#[derive(Clone)]
pub struct Editor {
    objects: BTreeMap<BuildingId, Arc<Building>>,
    // Tombstones prevent reuse after deletion; an Entity is never a domain ID.
    revisions: BTreeMap<BuildingId, u64>,
    used_blocks: BTreeMap<BuildingId, BTreeSet<BlockId>>,
    undo: VecDeque<History>,
    redo: Vec<History>,
    strokes: strokes::StrokeState,
    context: context::ContextState,
    history_limit: usize,
    clock: u64,
    session: u64,
    rules_revision: u64,
    latest: BTreeMap<BuildingId, JobTicket>,
    pending: BTreeMap<BuildingId, BuildRequest>,
    pending_order: BTreeSet<(u64, BuildingId)>,
}

impl Editor {
    pub fn new(history_limit: usize) -> Self {
        Self {
            objects: BTreeMap::new(),
            revisions: BTreeMap::new(),
            used_blocks: BTreeMap::new(),
            undo: VecDeque::new(),
            redo: Vec::new(),
            strokes: strokes::StrokeState::default(),
            context: context::ContextState::default(),
            history_limit,
            clock: 0,
            session: 1,
            rules_revision: 1,
            latest: BTreeMap::new(),
            pending: BTreeMap::new(),
            pending_order: BTreeSet::new(),
        }
    }
    pub fn get(&self, id: BuildingId) -> Option<&Arc<Building>> {
        self.objects.get(&id)
    }
    pub fn objects(&self) -> impl Iterator<Item = &Arc<Building>> {
        self.objects.values()
    }
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
    pub fn undo_count(&self) -> usize {
        self.undo.len()
    }
    /// Includes deleted IDs: UI must not recycle an identity after undo/delete.
    /// Allocation is observed here; Create remains the authoritative reservation.
    pub fn next_building_id(&self) -> Result<BuildingId, EditError> {
        let last = self
            .revisions
            .last_key_value()
            .map_or(0, |(id, _)| id.get());
        last.checked_add(1)
            .and_then(BuildingId::new)
            .ok_or(EditError::CounterExhausted)
    }
    pub fn revision(&self, id: BuildingId) -> Option<u64> {
        self.revisions.get(&id).copied()
    }
    /// Includes removed/undone blocks; ordinary edits never recycle identity.
    pub fn next_block_id(&self, building: BuildingId) -> Result<BlockId, EditError> {
        if !self.objects.contains_key(&building) {
            return Err(EditError::Missing);
        }
        self.used_blocks
            .get(&building)
            .and_then(|ids| ids.last())
            .map_or(0, |id| id.get())
            .checked_add(1)
            .and_then(BlockId::new)
            .ok_or(EditError::CounterExhausted)
    }
    pub fn rules_revision(&self) -> u64 {
        self.rules_revision
    }
    pub fn latest_ticket(&self, id: BuildingId) -> Option<JobTicket> {
        self.latest.get(&id).copied()
    }
    pub fn begin_preview(&self, id: BuildingId) -> Result<Preview, EditError> {
        Ok(Preview {
            session: self.session,
            revision: *self.revisions.get(&id).ok_or(EditError::Missing)?,
            candidate: self
                .objects
                .get(&id)
                .ok_or(EditError::Missing)?
                .as_ref()
                .clone(),
        })
    }

    pub fn execute(&mut self, command: EditCommand) -> Result<Option<Change>, EditError> {
        self.context.last = context::ChangeSet::default();
        match command {
            EditCommand::Undo => return self.undo(),
            EditCommand::Redo => return self.redo(),
            EditCommand::Stroke(edit) => return self.execute_stroke(edit),
            EditCommand::Context(edit) => return self.execute_context(edit),
            _ => (),
        }
        let patch = match command {
            EditCommand::Create(draft) => {
                if self.revisions.contains_key(&draft.id) {
                    return Err(EditError::IdAlreadyUsed);
                }
                let object = Arc::new(Building::try_new(draft)?);
                Patch {
                    id: object.id(),
                    before: None,
                    after: Some(object),
                }
            }
            EditCommand::Edit { building, edit } => {
                let before = self
                    .objects
                    .get(&building)
                    .ok_or(EditError::Missing)?
                    .clone();
                let after = Arc::new(before.edited(edit)?);
                if before == after {
                    return Ok(None);
                }
                Patch {
                    id: building,
                    before: Some(before),
                    after: Some(after),
                }
            }
            EditCommand::Delete(id) => Patch {
                id,
                before: Some(self.objects.get(&id).ok_or(EditError::Missing)?.clone()),
                after: None,
            },
            EditCommand::CommitPreview(preview) => {
                let id = preview.candidate.id();
                if preview.session != self.session || self.revision(id) != Some(preview.revision) {
                    return Err(EditError::StalePreview);
                }
                let before = self
                    .objects
                    .get(&id)
                    .ok_or(EditError::StalePreview)?
                    .clone();
                let after = Arc::new(preview.candidate);
                if before == after {
                    return Ok(None);
                }
                Patch {
                    id,
                    before: Some(before),
                    after: Some(after),
                }
            }
            EditCommand::Undo
            | EditCommand::Redo
            | EditCommand::Stroke(_)
            | EditCommand::Context(_) => unreachable!(),
        };
        if let (Some(after), Some(used)) = (&patch.after, self.used_blocks.get(&patch.id)) {
            for block in after.blocks() {
                let existed = patch
                    .before
                    .as_ref()
                    .is_some_and(|b| b.block(block.id).is_some());
                if !existed && used.contains(&block.id) {
                    return Err(EditError::BlockIdAlreadyUsed);
                }
            }
        }
        let change = self.apply(&patch, true)?;
        self.redo.clear();
        if self.history_limit > 0 {
            if self.undo.len() == self.history_limit {
                self.undo.pop_front();
            }
            self.undo.push_back(History::Building(patch));
        }
        Ok(Some(change))
    }

    fn undo(&mut self) -> Result<Option<Change>, EditError> {
        let patch = self.undo.back().ok_or(EditError::EmptyHistory)?.clone();
        let change = self.apply_history(&patch, false)?;
        self.undo.pop_back();
        self.redo.push(patch);
        Ok(change)
    }
    fn redo(&mut self) -> Result<Option<Change>, EditError> {
        let patch = self.redo.last().ok_or(EditError::EmptyHistory)?.clone();
        let change = self.apply_history(&patch, true)?;
        self.redo.pop();
        self.undo.push_back(patch);
        Ok(change)
    }
    fn next_serial(&mut self) -> Result<u64, EditError> {
        let next = self
            .clock
            .checked_add(1)
            .ok_or(EditError::CounterExhausted)?;
        self.clock = next;
        Ok(next)
    }
    fn apply(&mut self, patch: &Patch, forward: bool) -> Result<Change, EditError> {
        let serial = self.next_serial()?;
        let (before, after) = if forward {
            (&patch.before, &patch.after)
        } else {
            (&patch.after, &patch.before)
        };
        self.revisions.insert(patch.id, serial);
        if let Some(object) = after {
            self.used_blocks
                .entry(patch.id)
                .or_default()
                .extend(object.blocks().iter().map(|b| b.id));
            self.objects.insert(patch.id, object.clone());
            self.queue(patch.id, serial, serial);
        } else {
            self.objects.remove(&patch.id);
            self.latest.remove(&patch.id);
            if let Some(old) = self.pending.remove(&patch.id) {
                self.pending_order
                    .remove(&(old.ticket.request_serial, patch.id));
            }
        }
        let regions = before
            .iter()
            .chain(after.iter())
            .map(|b| Self::building_bounds(b))
            .collect();
        self.note_change(context::ObjectRef::Building(patch.id), regions, serial);
        Ok(Change {
            building: patch.id,
            revision: serial,
            before: before.clone(),
            after: after.clone(),
        })
    }
    fn queue(&mut self, id: BuildingId, revision: u64, serial: u64) {
        let ticket = JobTicket {
            session: self.session,
            building: id,
            object_revision: revision,
            rules_revision: self.rules_revision,
            request_serial: serial,
        };
        self.latest.insert(id, ticket);
        if let Some(old) = self.pending.insert(
            id,
            BuildRequest {
                ticket,
                snapshot: self.objects[&id].clone(),
            },
        ) {
            self.pending_order.remove(&(old.ticket.request_serial, id));
        }
        self.pending_order.insert((serial, id));
    }

    /// An external relation (e.g. a road) changed without changing building intent.
    /// The request serial still advances, so earlier jobs can never be applied.
    pub fn invalidate(&mut self, id: BuildingId) -> Result<(), EditError> {
        if !self.objects.contains_key(&id) {
            return Err(EditError::Missing);
        }
        let serial = self.next_serial()?;
        self.queue(id, self.revisions[&id], serial);
        Ok(())
    }
    pub fn take_requests(&mut self, limit: usize) -> Vec<BuildRequest> {
        self.take_requests_where(limit, |_| true)
    }
    /// Busy buildings keep their latest target pending without blocking other
    /// buildings behind them. History/commands are never coalesced here.
    pub fn take_requests_where(
        &mut self,
        limit: usize,
        mut eligible: impl FnMut(BuildingId) -> bool,
    ) -> Vec<BuildRequest> {
        let selected = self
            .pending_order
            .iter()
            .copied()
            .filter(|(_, id)| eligible(*id))
            .take(limit)
            .collect::<Vec<_>>();
        selected
            .into_iter()
            .filter_map(|key| {
                self.pending_order.remove(&key);
                self.pending.remove(&key.1)
            })
            .collect()
    }
    pub fn accepts(&self, ticket: JobTicket) -> bool {
        self.objects.contains_key(&ticket.building)
            && self.latest.get(&ticket.building) == Some(&ticket)
    }

    /// Replace a complete validated scene transactionally. Session fences old jobs.
    pub fn replace_scene(&mut self, buildings: Vec<Building>) -> Result<(), EditError> {
        let mut objects = BTreeMap::new();
        for b in buildings {
            if objects.insert(b.id(), Arc::new(b)).is_some() {
                return Err(EditError::DuplicateId);
            }
        }
        let session = self
            .session
            .checked_add(1)
            .ok_or(EditError::CounterExhausted)?;
        let serial = self
            .clock
            .checked_add(1)
            .ok_or(EditError::CounterExhausted)?;
        self.session = session;
        self.clock = serial;
        self.strokes = strokes::StrokeState {
            revision: serial,
            ..Default::default()
        };
        self.context = context::ContextState {
            revision: serial,
            ..Default::default()
        };
        self.objects = objects;
        self.revisions = self.objects.keys().map(|&id| (id, serial)).collect();
        self.used_blocks = self
            .objects
            .iter()
            .map(|(&id, b)| (id, b.blocks().iter().map(|b| b.id).collect()))
            .collect();
        self.undo.clear();
        self.redo.clear();
        self.pending.clear();
        self.pending_order.clear();
        self.latest.clear();
        for id in self.objects.keys().copied().collect::<Vec<_>>() {
            self.queue(id, serial, serial);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use garden_domain::{BlockId, Roof, sample_building};
    fn id() -> BuildingId {
        BuildingId::new(1).unwrap()
    }
    fn create() -> EditCommand {
        EditCommand::Create(sample_building(id(), 6.0, 3.0))
    }
    fn roof(roof: Roof) -> EditCommand {
        EditCommand::Edit {
            building: id(),
            edit: BuildingEdit::SetRoof {
                block: BlockId::new(1).unwrap(),
                roof,
            },
        }
    }
    #[test]
    fn busy_building_keeps_latest_target_without_blocking_other_buildings() {
        let mut editor = Editor::new(16);
        editor.execute(create()).unwrap();
        let other = BuildingId::new(2).unwrap();
        editor
            .execute(EditCommand::Create(sample_building(other, 3., 3.)))
            .unwrap();
        editor.execute(roof(Roof::Flat)).unwrap();
        editor.execute(roof(Roof::Hipped)).unwrap();
        let requests = editor.take_requests_where(4, |building| building != id());
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].ticket.building, other);
        let requests = editor.take_requests_where(4, |_| true);
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].ticket, editor.latest_ticket(id()).unwrap());
        assert_eq!(editor.undo_count(), 4);
    }
    #[test]
    fn dispatch_order_is_not_biased_toward_low_object_ids() {
        let mut editor = Editor::new(8);
        editor.execute(create()).unwrap();
        let other = BuildingId::new(100).unwrap();
        editor
            .execute(EditCommand::Create(sample_building(other, 3.0, 3.0)))
            .unwrap();
        editor.invalidate(id()).unwrap();
        assert_eq!(editor.take_requests(1)[0].ticket.building, other);
        assert_eq!(editor.take_requests(1)[0].ticket.building, id());
        assert!(editor.take_requests(1).is_empty());
    }
    #[test]
    fn history_is_bounded_and_undo_does_not_rewind_revision() {
        let mut editor = Editor::new(2);
        editor.execute(create()).unwrap();
        editor.execute(roof(Roof::Flat)).unwrap();
        editor.execute(roof(Roof::Hipped)).unwrap();
        let prior = editor.revision(id()).unwrap();
        assert_eq!(editor.undo_count(), 2);
        editor.execute(EditCommand::Undo).unwrap();
        assert!(editor.revision(id()).unwrap() > prior);
        assert_eq!(
            editor.get(id()).unwrap().blocks()[0].roof_intent,
            Roof::Flat
        );
        editor.execute(EditCommand::Redo).unwrap();
        assert_eq!(
            editor.get(id()).unwrap().blocks()[0].roof_intent,
            Roof::Hipped
        );
    }
    #[test]
    fn no_op_and_failed_edits_do_not_change_history() {
        let mut editor = Editor::new(8);
        editor.execute(create()).unwrap();
        assert!(editor.execute(roof(Roof::Gabled)).unwrap().is_none());
        assert!(
            editor
                .execute(EditCommand::Edit {
                    building: id(),
                    edit: BuildingEdit::RemoveBlock(BlockId::new(1).unwrap())
                })
                .is_err()
        );
        assert_eq!(editor.undo_count(), 1);
        assert_eq!(editor.revision(id()), Some(1));
    }
    #[test]
    fn coalesces_pending_and_rejects_dependency_stale_jobs() {
        let mut editor = Editor::new(8);
        editor.execute(create()).unwrap();
        let old = editor.take_requests(1).pop().unwrap();
        editor.invalidate(id()).unwrap();
        editor.invalidate(id()).unwrap();
        assert_eq!(editor.pending_count(), 1);
        assert!(!editor.accepts(old.ticket));
        let new = editor.take_requests(1).pop().unwrap();
        assert_eq!(old.ticket.object_revision, new.ticket.object_revision);
        assert!(editor.accepts(new.ticket));
    }
    #[test]
    fn scene_replacement_and_deletion_fence_jobs() {
        let mut editor = Editor::new(8);
        editor.execute(create()).unwrap();
        let old = editor.take_requests(1).pop().unwrap();
        editor
            .replace_scene(vec![
                Building::try_new(sample_building(id(), 6.0, 3.0)).unwrap(),
            ])
            .unwrap();
        assert!(!editor.accepts(old.ticket));
        let current = editor.take_requests(1).pop().unwrap();
        editor.execute(EditCommand::Delete(id())).unwrap();
        assert!(!editor.accepts(current.ticket));
        assert_eq!(
            editor.execute(create()).unwrap_err(),
            EditError::IdAlreadyUsed
        );
        editor.execute(EditCommand::Undo).unwrap();
        assert!(editor.get(id()).is_some());
    }
    #[test]
    fn editing_one_object_preserves_other_snapshot_identity() {
        let mut editor = Editor::new(8);
        editor.execute(create()).unwrap();
        let other = BuildingId::new(2).unwrap();
        editor
            .execute(EditCommand::Create(sample_building(other, 3.0, 3.0)))
            .unwrap();
        let snapshot = editor.get(other).unwrap().clone();
        editor.take_requests(8);
        editor.execute(roof(Roof::Flat)).unwrap();
        assert!(Arc::ptr_eq(&snapshot, editor.get(other).unwrap()));
        assert_eq!(editor.pending_count(), 1);
    }
    #[test]
    fn many_drag_updates_make_one_history_entry_and_cancel_makes_none() {
        let mut editor = Editor::new(8);
        editor.execute(create()).unwrap();
        let mut preview = editor.begin_preview(id()).unwrap();
        for roof in [Roof::Flat, Roof::Hipped, Roof::Flat] {
            preview
                .update(BuildingEdit::SetRoof {
                    block: BlockId::new(1).unwrap(),
                    roof,
                })
                .unwrap();
        }
        assert_eq!(
            editor.get(id()).unwrap().blocks()[0].roof_intent,
            Roof::Gabled
        );
        assert_eq!(editor.undo_count(), 1);
        editor.execute(EditCommand::CommitPreview(preview)).unwrap();
        assert_eq!(editor.undo_count(), 2);
        drop(editor.begin_preview(id()).unwrap());
        assert_eq!(editor.undo_count(), 2);
        let stale = editor.begin_preview(id()).unwrap();
        editor.execute(roof(Roof::Hipped)).unwrap();
        assert_eq!(
            editor
                .execute(EditCommand::CommitPreview(stale))
                .unwrap_err(),
            EditError::StalePreview
        );
    }
    #[test]
    fn removed_block_identity_cannot_be_reused_but_undo_can_restore_it() {
        let mut editor = Editor::new(8);
        editor.execute(create()).unwrap();
        let block = garden_domain::BlockDraft {
            id: BlockId::new(2).unwrap(),
            parent: Some(BlockId::new(1).unwrap()),
            footprint: garden_geometry_fixture(),
            height: 3.0,
            stories: garden_domain::Stories::Locked(1),
            roof_intent: Roof::Gabled,
            facade: garden_domain::Facade::Stone,
        };
        editor
            .execute(EditCommand::Edit {
                building: id(),
                edit: BuildingEdit::AddBlock(block.clone()),
            })
            .unwrap();
        editor
            .execute(EditCommand::Edit {
                building: id(),
                edit: BuildingEdit::RemoveBlock(block.id),
            })
            .unwrap();
        assert_eq!(
            editor
                .execute(EditCommand::Edit {
                    building: id(),
                    edit: BuildingEdit::AddBlock(block)
                })
                .unwrap_err(),
            EditError::BlockIdAlreadyUsed
        );
        editor.execute(EditCommand::Undo).unwrap();
        assert_eq!(editor.get(id()).unwrap().blocks().len(), 2);
    }
    fn garden_geometry_fixture() -> garden_geometry::Rect {
        garden_geometry::Rect {
            x: 1.0,
            z: 1.0,
            width: 2.0,
            depth: 2.0,
        }
    }
}
