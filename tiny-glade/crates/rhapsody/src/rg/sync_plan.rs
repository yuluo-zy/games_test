//! A single native range synchronization program. Static accesses are compiled
//! once; bindings overlay the physical SyncCell tables on each frame. Serial,
//! parallel and ownership-prelude recorders consume this same bound projection.
use super::{graph::Descriptor, queue_schedule::QueueDomain, resource::AccessRange};
use crate::{
    Error, Result,
    resource::{
        ResourceSharing,
        range_state::{
            AccessScope, NativeRange, NativeState, PatchAction, RangeCell, RangeTransfer,
            StatePatch,
        },
    },
    sync::{QueueId, SyncPoint},
};
use ash::vk;
use std::ops::Range;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceDomain {
    pub range: NativeRange,
    pub sharing: ResourceSharing,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RangeUse {
    pub resource: u32,
    pub range: NativeRange,
    pub scope: AccessScope,
    pub layout: vk::ImageLayout,
    pub access_code: u8,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramPass {
    /// Retained/scheduled pass index, not original declaration index.
    pub pass: usize,
    pub uses: Range<usize>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramBatch {
    pub domain: QueueDomain,
    pub passes: Range<usize>,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RangeProgram {
    pub separate_depth_stencil: bool,
    pub dependencies: Vec<(usize, usize)>,
    pub resources: Vec<ResourceDomain>,
    pub uses: Vec<RangeUse>,
    pub passes: Vec<ProgramPass>,
    pub exports: Vec<RangeUse>,
    pub batches: Vec<ProgramBatch>,
}
/// No placeholder timeline values. A symbolic dependency must resolve to the
/// actual receipt of an already successful batch in this bound execution.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DependencyRef {
    Point(SyncPoint),
    Batch(usize),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BarrierKind {
    Memory,
    Release { pair: u64 },
    Acquire { pair: u64 },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BarrierPhase {
    Initial,
    BeforePass(usize),
    Export,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoundTransition {
    pub batch: usize,
    pub phase: BarrierPhase,
    pub resource: u32,
    pub range: NativeRange,
    pub source: AccessScope,
    pub destination: AccessScope,
    pub old_layout: vk::ImageLayout,
    pub new_layout: vk::ImageLayout,
    pub source_family: u32,
    pub destination_family: u32,
    pub kind: BarrierKind,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoundPatchAction {
    Access {
        scope: AccessScope,
        layout: vk::ImageLayout,
        queue: QueueId,
        family: u32,
    },
    Release(RangeTransfer),
    Acquire {
        transfer: RangeTransfer,
        release: DependencyRef,
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoundPatch {
    pub batch: usize,
    pub phase: BarrierPhase,
    pub resource: u32,
    pub range: NativeRange,
    pub expected_revision: u64,
    pub action: BoundPatchAction,
}
impl BoundPatch {
    pub fn resolve(self, receipt: impl FnOnce(usize) -> Option<SyncPoint>) -> Result<StatePatch> {
        let action = match self.action {
            BoundPatchAction::Access {
                scope,
                layout,
                queue,
                family,
            } => PatchAction::Access {
                scope,
                layout,
                queue,
                family,
            },
            BoundPatchAction::Release(transfer) => PatchAction::Release(transfer),
            BoundPatchAction::Acquire { transfer, release } => PatchAction::Acquire {
                transfer,
                release_point: match release {
                    DependencyRef::Point(point) => point,
                    DependencyRef::Batch(batch) => receipt(batch).ok_or_else(|| {
                        Error::invalid_usage(
                            "ownership acquire refers to an unsubmitted release batch",
                        )
                    })?,
                },
            },
        };
        Ok(StatePatch {
            range: self.range,
            expected_revision: self.expected_revision,
            action,
        })
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoundWait {
    pub batch: usize,
    pub dependency: DependencyRef,
    pub stages: vk::PipelineStageFlags,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BoundBatchKind {
    Graph {
        original: usize,
        passes: Range<usize>,
    },
    OwnershipPrelude,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundBatch {
    pub domain: QueueDomain,
    pub kind: BoundBatchKind,
}
/// Two transient resources share backing memory. `next` discards its contents
/// on first use, and every access to `previous` recorded so far (plus `next`'s
/// own initial frontier) becomes a synthetic previous writer of `next` with an
/// UNDEFINED image layout. The binder therefore always emits the aliasing
/// barrier or cross-queue wait, and the bound checker verifies it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AliasEdge {
    pub previous: u32,
    pub next: u32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InitialState {
    pub resource: u32,
    pub cell: RangeCell,
}
/// Read-only projection of the exact operations consumed by native recording.
/// Accessors borrow existing tables; diagnostics never allocate capture arrays.
pub struct BoundSynchronization {
    pub(crate) initial: crate::resource::range_state::Storage<InitialState>,
    pub(crate) aliases: crate::resource::range_state::Storage<AliasEdge>,
    pub(crate) batches: crate::resource::range_state::Storage<BoundBatch>,
    pub(crate) order: crate::resource::range_state::Storage<usize>,
    pub(crate) transitions: crate::resource::range_state::Storage<BoundTransition>,
    pub(crate) patches: crate::resource::range_state::Storage<BoundPatch>,
    pub(crate) waits: crate::resource::range_state::Storage<BoundWait>,
}
impl BoundSynchronization {
    pub fn initial_inputs(&self) -> &[InitialState] {
        &self.initial.values
    }
    pub fn aliases(&self) -> &[AliasEdge] {
        &self.aliases.values
    }
    pub fn batches(&self) -> &[BoundBatch] {
        &self.batches.values
    }
    pub fn execution_order(&self) -> &[usize] {
        &self.order.values
    }
    pub fn transitions(&self) -> &[BoundTransition] {
        &self.transitions.values
    }
    pub fn patches(&self) -> &[BoundPatch] {
        &self.patches.values
    }
    pub fn waits(&self) -> &[BoundWait] {
        &self.waits.values
    }
}

pub fn native_range(
    range: AccessRange,
    descriptor: Descriptor,
    separate_depth_stencil: bool,
) -> Result<NativeRange> {
    let normalized = range
        .normalized(descriptor)
        .map_err(|error| Error::invalid_usage(format!("native range: {error:?}")))?;
    let result = match normalized {
        AccessRange::Buffer { offset, size } => NativeRange::Buffer { offset, size },
        AccessRange::Texture {
            mut aspect_mask,
            base_mip_level,
            level_count,
            base_array_layer,
            layer_count,
        } => {
            let all = AccessRange::Whole
                .normalized(descriptor)
                .map_err(|error| Error::invalid_usage(format!("native domain: {error:?}")))?;
            let ds = (vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL).as_raw();
            if !separate_depth_stencil
                && let AccessRange::Texture {
                    aspect_mask: all, ..
                } = all
                && all & ds == ds
                && aspect_mask & ds != 0
            {
                aspect_mask |= ds;
            }
            NativeRange::Image {
                aspects: aspect_mask,
                base_mip: base_mip_level,
                mip_count: level_count,
                base_layer: base_array_layer,
                layer_count,
            }
        }
        AccessRange::Whole => unreachable!("normalized range"),
    };
    crate::resource::range_state::validate_range(result)?;
    Ok(result)
}

mod bind;
pub use bind::{SynchronizationConfig, SynchronizationPlanner};

impl RangeProgram {
    pub(crate) fn compile(
        definition: &super::plan::GraphDefinition,
        retained: &[usize],
        batches: &[super::compiled::Batch],
        schedule: &super::queue_schedule::QueueSchedule,
    ) -> Result<Self> {
        let separate_depth_stencil = definition.context.separate_depth_stencil_layouts;
        let mut result = Self {
            separate_depth_stencil,
            ..Self::default()
        };
        for resource in &definition.resources {
            result.resources.push(ResourceDomain {
                range: native_range(
                    AccessRange::Whole,
                    resource.descriptor,
                    separate_depth_stencil,
                )?,
                sharing: resource.sharing,
            });
        }
        for (scheduled, &original) in retained.iter().enumerate() {
            let start = result.uses.len();
            for access in &definition.passes[original].accesses {
                let usage = range_use(
                    definition,
                    access.resource_index,
                    access.range,
                    access.access_code,
                )?;
                combine_uses(&mut result.uses, start, usage)?;
            }
            result.passes.push(ProgramPass {
                pass: scheduled,
                uses: start..result.uses.len(),
            });
        }
        for export in &definition.exports {
            let usage = range_use(
                definition,
                export.resource_index,
                export.range,
                export.access_code,
            )?;
            combine_uses(&mut result.exports, 0, usage)?;
        }
        for (index, batch) in batches.iter().enumerate() {
            result.batches.push(ProgramBatch {
                domain: schedule.domains[index],
                passes: batch.pass_range.clone(),
            });
            for &source in &schedule.dependencies[index] {
                result.dependencies.push((source, index));
            }
        }
        Ok(result)
    }
    pub fn estimated_bytes(&self) -> usize {
        size_of::<Self>()
            + self.resources.capacity() * size_of::<ResourceDomain>()
            + self.uses.capacity() * size_of::<RangeUse>()
            + self.passes.capacity() * size_of::<ProgramPass>()
            + self.exports.capacity() * size_of::<RangeUse>()
            + self.batches.capacity() * size_of::<ProgramBatch>()
            + self.dependencies.capacity() * size_of::<(usize, usize)>()
    }
}
fn range_use(
    definition: &super::plan::GraphDefinition,
    resource: u32,
    range: AccessRange,
    code: u8,
) -> Result<RangeUse> {
    let descriptor = definition
        .resources
        .get(resource as usize)
        .ok_or_else(|| Error::invalid_usage("native use resource is absent"))?
        .descriptor;
    let semantics = definition
        .access_semantics
        .iter()
        .find(|entry| entry.code == code)
        .ok_or_else(|| Error::invalid_usage("native use semantics are absent"))?;
    let range = native_range(
        range,
        descriptor,
        definition.context.separate_depth_stencil_layouts,
    )?;
    if matches!(descriptor, Descriptor::Image(_))
        && (semantics.layout == vk::ImageLayout::UNDEFINED
            || semantics.layout == vk::ImageLayout::PREINITIALIZED)
    {
        return Err(Error::invalid_usage(
            "an image access cannot target UNDEFINED or PREINITIALIZED",
        ));
    }
    Ok(RangeUse {
        resource,
        range,
        scope: AccessScope {
            stages: semantics.stages,
            access: semantics.access,
            write: semantics.write,
        },
        layout: if matches!(descriptor, Descriptor::Buffer(_)) {
            vk::ImageLayout::UNDEFINED
        } else {
            semantics.layout
        },
        access_code: code,
    })
}
/// Simultaneous declarations in one callback become one nonoverlapping set of
/// native scopes. Barriers are placed before the callback, never between two
/// declarations that execute together.
fn combine_uses(uses: &mut Vec<RangeUse>, start: usize, added: RangeUse) -> Result<()> {
    let before = uses.len();
    let mut remaining = smallvec::SmallVec::<[NativeRange; 8]>::from_slice(&[added.range]);
    for index in start..before {
        let previous = uses[index];
        if previous.resource != added.resource {
            continue;
        }
        let Some(overlap) = crate::resource::range_state::intersection(previous.range, added.range)
        else {
            continue;
        };
        if previous.layout != added.layout {
            return Err(Error::invalid_usage(
                "overlapping image declarations in one pass require one native layout",
            ));
        }
        let outside = crate::resource::range_state::difference(previous.range, overlap);
        uses[index] = RangeUse {
            range: overlap,
            scope: previous.scope.union(added.scope),
            ..previous
        };
        uses.extend(
            outside
                .into_iter()
                .map(|range| RangeUse { range, ..previous }),
        );
        let mut next = smallvec::SmallVec::<[NativeRange; 8]>::new();
        for range in remaining {
            next.extend(crate::resource::range_state::difference(
                range,
                previous.range,
            ));
        }
        remaining = next;
    }
    uses.extend(
        remaining
            .into_iter()
            .map(|range| RangeUse { range, ..added }),
    );
    Ok(())
}
impl std::fmt::Debug for BoundSynchronization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BoundSynchronization")
            .field("initial", &self.initial_inputs())
            .field("aliases", &self.aliases())
            .field("batches", &self.batches())
            .field("order", &self.execution_order())
            .field("transitions", &self.transitions())
            .field("patches", &self.patches())
            .field("waits", &self.waits())
            .finish()
    }
}

/// Initial state for a previously untracked external object. Existing physical
/// tables ignore import scalars and preserve their exact partition and points.
pub fn initial_native_state(
    scope: AccessScope,
    layout: vk::ImageLayout,
    queue: QueueId,
    owner: crate::resource::range_state::RangeOwner,
) -> NativeState {
    let mut state = NativeState {
        layout,
        owner,
        ..NativeState::default()
    };
    let frontier = if scope.write {
        &mut state.writer
    } else {
        &mut state.readers
    };
    if scope.stages == vk::PipelineStageFlags::HOST {
        frontier.host = scope;
    } else {
        frontier.add(queue, scope);
    }
    state
}
