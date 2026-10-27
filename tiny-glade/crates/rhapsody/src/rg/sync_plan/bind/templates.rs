//! Bind templates cached by initial-state key and replayed as patches.
use crate::Result;
use ash::vk;
use crate::rg::sync_plan::{AliasEdge, BoundPatch};
use crate::native::access_table::AccessScope;
use crate::rg::queue_schedule::QueueDomain;
use crate::rg::sync_plan::BoundBatchKind;
use crate::rg::sync_plan::BoundPatchAction;
use crate::rg::sync_plan::RangeProgram;
use crate::rg::sync_plan::SharedBound;
use crate::tracking::range_state::NativeRange;
use crate::sync::MAX_QUEUES;
use crate::tracking::range_state::Frontier;
use crate::tracking::range_state::RangeOwner;
use crate::tracking::range_state::RangeStateConfig;
use std::sync::Arc;
use super::SynchronizationPlanner;
use super::shadow::shadow_bind_check;

pub(super) const MAX_TEMPLATES: usize = 4;

/// Frame-independent view of one imported initial cell. Timeline points are
/// reduced to a per-queue mask: their values only matter for cross-queue waits,
/// and a template is stored only when there are none.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct InitKey {
    resource: u32,
    range: NativeRange,
    layout: vk::ImageLayout,
    layout_known: bool,
    owner: Option<u32>,
    writer_scopes: [AccessScope; MAX_QUEUES],
    writer_host: AccessScope,
    writer_queues: u8,
    readers_scopes: [AccessScope; MAX_QUEUES],
    readers_host: AccessScope,
    readers_queues: u8,
}

pub(super) struct Template {
    id: usize,
    _pin: Arc<dyn std::any::Any + Send + Sync>,
    key: Vec<InitKey>,
    aliases: Vec<AliasEdge>,
    /// Frame-independent tables, handed to the planner by reference.
    shared: Arc<SharedBound>,
    patches: Vec<BoundPatch>,
    /// (initial row, revision delta) per patch.
    bases: Vec<(u32, u64)>,
    used: u64,
}

impl SynchronizationPlanner {
    /// Number of bindings served from a cached template (diagnostics/tests).
    pub fn template_hits(&self) -> u64 {
        self.template_hits
    }

    /// [`Self::bind`] with a per-program template cache. `id` identifies the
    /// program and `pin` keeps its allocation alive, so an address can never be
    /// reused by a different program while a template for it exists.
    ///
    /// A template is stored only when the binding is independent of timeline
    /// values: no ownership transfer, no cross-queue wait. A later frame whose
    /// imported state differs from the template only in revision numbers and
    /// timeline points then gets the identical output, with every patch's
    /// `expected_revision` rebased onto its own initial state.
    pub fn bind_cached(
        &mut self,
        program: &RangeProgram,
        id: usize,
        pin: Arc<dyn std::any::Any + Send + Sync>,
        family_domain: impl FnMut(u32) -> Result<QueueDomain>,
    ) -> Result<()> {
        self.start_binding()?;
        let key_ok = self.template_key()?;
        let key = std::mem::take(&mut self.key_scratch.values);
        self.borrowed_key_bytes = key.capacity() * size_of::<InitKey>();
        let result = self.bind_cached_keyed(program, id, pin, family_domain, key_ok, &key);
        self.key_scratch.values = key;
        self.borrowed_key_bytes = 0;
        result
    }

    fn bind_cached_keyed(
        &mut self,
        program: &RangeProgram,
        id: usize,
        pin: Arc<dyn std::any::Any + Send + Sync>,
        family_domain: impl FnMut(u32) -> Result<QueueDomain>,
        key_ok: bool,
        key: &[InitKey],
    ) -> Result<()> {
        if key_ok {
            let found = self.templates.iter().position(|template| {
                template.id == id
                    && template.key[..] == *key
                    && template.aliases == self.bound.aliases.values[..]
            });
            if let Some(index) = found {
                if self.load_template(index)? {
                    if shadow_bind_check() {
                        let produced = self.bound_fingerprint();
                        self.bound.shared = None;
                        self.bound.batches.values.clear();
                        self.bound.order.values.clear();
                        self.bound.transitions.values.clear();
                        self.bound.patches.values.clear();
                        self.bound.waits.values.clear();
                        self.patch_slots.values.clear();
                        self.bind_full(program, family_domain)?;
                        assert_eq!(
                            produced,
                            self.bound_fingerprint(),
                            "bind template differs from a full bind"
                        );
                    }
                    return Ok(());
                }
            }
        }
        self.bind_full(program, family_domain)?;
        if key_ok {
            self.save_template(id, pin, key.to_vec());
        }
        Ok(())
    }

    fn bound_fingerprint(&self) -> u64 {
        let text = format!(
            "{:?}|{:?}|{:?}|{:?}|{:?}",
            self.bound.execution_order(),
            self.bound.transitions(),
            self.bound.patches.values,
            self.bound.waits.values,
            self.bound.batches()
        );
        text.bytes().fold(0xcbf2_9ce4_8422_2325u64, |hash, byte| {
            (hash ^ byte as u64).wrapping_mul(0x0100_0000_01b3)
        })
    }

    /// Frame-independent projection of the initial state; `None` when a
    /// released ownership makes the binding depend on a pending transfer.
    fn template_key(&mut self) -> Result<bool> {
        let needed = self.bound.initial.values.len();
        if self.key_scratch.values.capacity() < needed {
            let total = self.known_cpu_bytes() - self.key_scratch.bytes();
            self.key_scratch.grow(needed, total, RangeStateConfig { max_cells: self.config.max_cells, max_cpu_bytes: self.config.max_cpu_bytes }, &self.budget)?;
        }
        let mut key = std::mem::take(&mut self.key_scratch.values);
        key.clear();
        let ok = self.fill_template_key(&mut key);
        self.key_scratch.values = key;
        Ok(ok)
    }

    fn fill_template_key(&self, key: &mut Vec<InitKey>) -> bool {
        for initial in &self.bound.initial.values {
            let state = initial.cell.state;
            let owner = match state.owner {
                RangeOwner::Unclaimed => None,
                RangeOwner::Owned(family) => Some(family),
                RangeOwner::Released { .. } => return false,
            };
            let mask = |frontier: &Frontier| {
                frontier
                    .points
                    .points()
                    .fold(0u8, |mask, point| mask | 1 << point.queue.index())
            };
            key.push(InitKey {
                resource: initial.resource,
                range: initial.cell.range,
                layout: state.layout,
                layout_known: state.layout_known,
                owner,
                writer_scopes: state.writer.scopes,
                writer_host: state.writer.host,
                writer_queues: mask(&state.writer),
                readers_scopes: state.readers.scopes,
                readers_host: state.readers.host,
                readers_queues: mask(&state.readers),
            });
        }
        true
    }

    fn save_template(&mut self, id: usize, pin: Arc<dyn std::any::Any + Send + Sync>, key: Vec<InitKey>) {
        let bound = &self.bound;
        let single_queue = bound
            .batches
            .values
            .iter()
            .all(|batch| matches!(batch.kind, BoundBatchKind::Graph { .. }));
        let access_only = bound
            .patches
            .values
            .iter()
            .all(|patch| matches!(patch.action, BoundPatchAction::Access { .. }));
        if !single_queue
            || !access_only
            || !bound.waits.values.is_empty()
            || self.patch_slots.values.len() != bound.patches.values.len()
        {
            return;
        }
        let mut bases = Vec::with_capacity(bound.patches.values.len());
        for (patch, &slot) in bound.patches.values.iter().zip(&self.patch_slots.values) {
            let Some(initial) = bound.initial.values.get(slot as usize) else {
                return;
            };
            let Some(delta) = patch.expected_revision.checked_sub(initial.cell.state.revision)
            else {
                return;
            };
            bases.push((slot, delta));
        }
        self.template_clock += 1;
        let template = Template {
            id,
            _pin: pin,
            key,
            aliases: bound.aliases.values.to_vec(),
            shared: Arc::new(SharedBound {
                transitions: bound.transitions.values.to_vec(),
                batches: bound.batches.values.to_vec(),
                order: bound.order.values.to_vec(),
                index: bound.index.clone(),
            }),
            patches: bound.patches.values.to_vec(),
            bases,
            used: self.template_clock,
        };
        if let Some(existing) = self
            .templates
            .iter_mut()
            .find(|old| old.id == id && old.key == template.key && old.aliases == template.aliases)
        {
            *existing = template;
        } else if self.templates.len() < MAX_TEMPLATES {
            self.templates.push(template);
        } else if let Some(oldest) = self.templates.iter_mut().min_by_key(|old| old.used) {
            *oldest = template;
        }
    }

    /// Fills the bound tables from template `index`, rebasing revisions.
    /// Returns false (leaving the tables empty) if a revision would overflow.
    fn load_template(&mut self, index: usize) -> Result<bool> {
        let template = std::mem::take(&mut self.templates[index].patches);
        let result = self.load_template_rows(index, &template);
        self.templates[index].patches = template;
        result
    }

    fn load_template_rows(&mut self, index: usize, patches: &[BoundPatch]) -> Result<bool> {
        let bases = std::mem::take(&mut self.templates[index].bases);
        let mut rebased = true;
        for (patch, &(slot, delta)) in patches.iter().zip(&bases) {
            let revision = self.bound.initial.values[slot as usize]
                .cell
                .state
                .revision
                .checked_add(delta)
                // The patch's own increment must also stay below the reserved
                // maximum, exactly as `ShadowCell::increment` requires.
                .filter(|revision| revision.checked_add(1).is_some_and(|next| next != u64::MAX));
            let Some(revision) = revision else {
                rebased = false;
                break;
            };
            let mut row = *patch;
            row.expected_revision = revision;
            if let Err(error) = self.push_patch(row) {
                self.templates[index].bases = bases;
                return Err(error);
            }
        }
        self.templates[index].bases = bases;
        if !rebased {
            self.bound.patches.values.clear();
            return Ok(false);
        }
        // Shared by reference: no copy of transitions, batches, order or index.
        self.bound.shared = Some(self.templates[index].shared.clone());
        self.template_clock += 1;
        self.templates[index].used = self.template_clock;
        self.template_hits += 1;
        Ok(true)
    }
}
