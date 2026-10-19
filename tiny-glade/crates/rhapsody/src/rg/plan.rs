//! Pure graph planning. A plan owns declarations and symbolic indices only;
//! physical objects, import states, callbacks and completion points belong to a frame.
use super::{
    compiled::{Batch, BatchCompiler},
    graph::{AccessDependency, Descriptor, GraphDeclaration, OriginSource, PassDeclaration},
    resource::{AccessRange, AccessVersion},
    resource_info::{AccessDeclaration, ExportDeclaration, OriginKind, ResourceInfo},
};
use std::collections::{BTreeSet, VecDeque};

pub const COMPILER_VERSION: u32 = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ResourceOriginKind {
    Create,
    ImportOwned,
    ImportUnowned,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ResourceDefinition {
    pub sharing: crate::resource::ResourceSharing,
    pub name: String,
    pub descriptor: Descriptor,
    pub origin: ResourceOriginKind,
}
impl ResourceDefinition {
    pub fn is_external(&self) -> bool {
        self.origin != ResourceOriginKind::Create
    }
    fn info_origin(&self) -> OriginKind {
        match self.origin {
            ResourceOriginKind::Create => OriginKind::Tag0 {
                descriptor_code: if self.descriptor.kind_code() == 1 {
                    7
                } else {
                    0
                },
                image_creation_flags: if self.descriptor.kind_code() == 0 {
                    self.descriptor.usage()
                } else {
                    0
                },
                buffer_creation_flags: if self.descriptor.kind_code() == 1 {
                    self.descriptor.usage()
                } else {
                    0
                },
            },
            ResourceOriginKind::ImportOwned => OriginKind::Tag2 {
                type_code: self.descriptor.kind_code(),
            },
            ResourceOriginKind::ImportUnowned => OriginKind::Tag1 {
                type_code: self.descriptor.kind_code(),
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AccessSemantics {
    pub code: u8,
    pub mask: u32,
    pub write: bool,
    pub stages: ash::vk::PipelineStageFlags,
    pub access: ash::vk::AccessFlags,
    pub layout: ash::vk::ImageLayout,
}

/// All policy values baked into planning are part of the complete cache key.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct PlanContext {
    pub queue_domains: Option<[super::queue_schedule::QueueDomain; 3]>,
    pub device_generation: u64,
    pub queue_family: u32,
    pub queue_index: u32,
    pub policy_signature: u64,
    /// The enabled feature controls whether depth and stencil layouts may vary.
    pub separate_depth_stencil_layouts: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GraphDefinition {
    pub subgraphs: Vec<super::subgraph::SubgraphDeclaration>,
    pub resources: Vec<ResourceDefinition>,
    pub passes: Vec<PassDeclaration>,
    pub exports: Vec<ExportDeclaration>,
    pub batch_per_pass: bool,
    pub access_semantics: Vec<AccessSemantics>,
    pub context: PlanContext,
    pub compiler_version: u32,
}
impl GraphDefinition {
    pub(crate) fn differing_key_reason(&self, new: &Self) -> super::cache::CacheReason {
        use super::cache::CacheReason;
        if self.context.device_generation != new.context.device_generation {
            CacheReason::DeviceGeneration
        } else if self.context != new.context {
            CacheReason::QueuePolicy
        } else if self.compiler_version != new.compiler_version {
            CacheReason::CompilerVersion
        } else if self.resources.len() == new.resources.len()
            && self
                .resources
                .iter()
                .zip(&new.resources)
                .any(|(a, b)| a.descriptor != b.descriptor)
        {
            CacheReason::Descriptor
        } else if self.resources.len() == new.resources.len()
            && self
                .resources
                .iter()
                .zip(&new.resources)
                .any(|(a, b)| a.sharing != b.sharing)
        {
            CacheReason::Sharing
        } else if self.subgraphs.iter().zip(&new.subgraphs).any(|(a, b)| {
            a.interface.implementation_signature != b.interface.implementation_signature
        }) {
            CacheReason::ShaderImplementation
        } else if self.subgraphs.iter().zip(&new.subgraphs).any(|(a, b)| {
            a.interface.shader_interface_signature != b.interface.shader_interface_signature
        }) {
            CacheReason::ShaderInterface
        } else if self.subgraphs != new.subgraphs {
            CacheReason::SubgraphInterface
        } else if self.access_semantics != new.access_semantics {
            CacheReason::AccessSemantics
        } else {
            CacheReason::GraphStructure
        }
    }
    /// Full borrowed key comparison for a frame binding. Only graph scope,
    /// import identity and initial access are dynamic. Explicit versions still
    /// have to equal the already validated normalized version stream, and an
    /// undefined import must be initialized again on this frame.
    pub(crate) fn matches_declaration(
        &self,
        graph: &GraphDeclaration,
        dependency: &mut impl AccessDependency,
        context: PlanContext,
    ) -> crate::Result<bool> {
        if self.compiler_version != COMPILER_VERSION
            || self.context != context
            || self.batch_per_pass != graph.batch_per_pass
            || self.subgraphs != graph.subgraphs
            || self.exports != graph.exports
            || self.resources.len() != graph.resources.len()
            || self.passes.len() != graph.passes.len()
        {
            return Ok(false);
        }
        for (cached, frame) in self.resources.iter().zip(&graph.resources) {
            let origin = match frame.source {
                OriginSource::Create => ResourceOriginKind::Create,
                OriginSource::ImportOwned { .. } => ResourceOriginKind::ImportOwned,
                OriginSource::ImportUnowned { .. } => ResourceOriginKind::ImportUnowned,
            };
            if cached.sharing != frame.sharing
                || cached.name != frame.name
                || cached.descriptor != frame.descriptor
                || cached.origin != origin
            {
                return Ok(false);
            }
        }
        for (cached, frame) in self.passes.iter().zip(&graph.passes) {
            if cached.name != frame.name
                || cached.queue_class != frame.queue_class
                || cached.side_effect != frame.side_effect
                || cached.accesses.len() != frame.accesses.len()
            {
                return Ok(false);
            }
            for (normalized, access) in cached.accesses.iter().zip(&frame.accesses) {
                if (AccessDeclaration {
                    version: normalized.version,
                    ..*access
                }) != *normalized
                    || access.version != AccessVersion::Automatic
                        && access.version != normalized.version
                {
                    return Ok(false);
                }
            }
        }
        // The identical pass/export/port access codes imply the identical set of
        // semantic keys. Compare the entire table, including unused-in-pass ports.
        for semantics in &self.access_semantics {
            let (native, layout) = dependency.native_semantics(semantics.code);
            if semantics.mask != dependency.access_mask(semantics.code)
                || semantics.write != dependency.is_write(semantics.code)
                || semantics.write != native.write
                || semantics.stages != native.stages
                || semantics.access != native.access
                || semantics.layout != layout
            {
                return Ok(false);
            }
        }
        graph.validate()?;
        super::resource_contracts::validate_matching_initialization(graph, |code| {
            self.is_write(code)
        })?;
        Ok(true)
    }

    /// Import identity and boundary state are deliberately dynamic parameters.
    /// Contracts are checked on every frame, including cache hits.
    pub fn from_declaration(
        graph: &GraphDeclaration,
        dependency: &mut impl AccessDependency,
        context: PlanContext,
    ) -> crate::Result<Self> {
        graph.validate()?;
        if graph
            .subgraphs
            .iter()
            .flat_map(|scope| {
                scope
                    .interface
                    .inputs
                    .iter()
                    .chain(&scope.interface.outputs)
            })
            .any(|port| {
                usize::from(port.access_code) >= crate::raw::access_table::ACCESS_CODE_COUNT
            })
        {
            return Err(crate::Error::invalid_usage(
                "subgraph port access code is invalid",
            ));
        }
        let resolved =
            super::resource_contracts::resolve_contracts(graph, |code| dependency.is_write(code))?;
        let mut codes = BTreeSet::new();
        for pass in &graph.passes {
            codes.extend(pass.accesses.iter().map(|a| a.access_code));
        }
        codes.extend(graph.exports.iter().map(|e| e.access_code));
        codes.extend(graph.subgraphs.iter().flat_map(|scope| {
            scope
                .interface
                .inputs
                .iter()
                .chain(&scope.interface.outputs)
                .map(|port| port.access_code)
        }));
        let access_semantics = codes
            .into_iter()
            .map(|code| {
                let (native, layout) = dependency.native_semantics(code);
                let write = dependency.is_write(code);
                if native.write != write {
                    return Err(crate::Error::invalid_usage(
                        "native access scope disagrees with graph write semantics",
                    ));
                }
                Ok(AccessSemantics {
                    code,
                    mask: dependency.access_mask(code),
                    write,
                    stages: native.stages,
                    access: native.access,
                    layout,
                })
            })
            .collect::<crate::Result<Vec<_>>>()?;
        let mut passes = graph.passes.clone();
        for (pass, accesses) in passes.iter_mut().zip(resolved) {
            pass.graph_id = None;
            pass.accesses = accesses;
        }
        Ok(Self {
            subgraphs: graph.subgraphs.clone(),
            resources: graph
                .resources
                .iter()
                .map(|r| ResourceDefinition {
                    sharing: r.sharing,
                    name: r.name.clone(),
                    descriptor: r.descriptor,
                    origin: match r.source {
                        OriginSource::Create => ResourceOriginKind::Create,
                        OriginSource::ImportOwned { .. } => ResourceOriginKind::ImportOwned,
                        OriginSource::ImportUnowned { .. } => ResourceOriginKind::ImportUnowned,
                    },
                })
                .collect(),
            passes,
            exports: graph.exports.clone(),
            batch_per_pass: graph.batch_per_pass,
            access_semantics,
            context,
            compiler_version: COMPILER_VERSION,
        })
    }
    pub fn is_write(&self, code: u8) -> bool {
        self.access_semantics
            .iter()
            .any(|s| s.code == code && s.write)
    }
}

#[derive(Clone, Debug)]
pub struct CompiledPlan {
    pub range_program: super::sync_plan::RangeProgram,
    pub definition: GraphDefinition,
    /// Original declaration indices. Frame callbacks use this exact mapping.
    pub retained_passes: Vec<usize>,
    /// Original indices, including storage anti-dependencies between versions.
    pub dependency_edges: Vec<(usize, usize)>,
    pub info: std::sync::Arc<ResourceInfo>,
    pub snapshot: std::sync::Arc<super::graph::SnapshotStatic>,
    pub batches: Vec<Batch>,
    pub queue_schedule: super::queue_schedule::QueueSchedule,
}
impl CompiledPlan {
    pub fn estimated_bytes(&self) -> usize {
        let d = &self.definition;
        std::mem::size_of::<Self>()
            .saturating_add(self.snapshot.estimated_bytes())
            .saturating_add(self.range_program.estimated_bytes())
            .saturating_add(
                d.resources
                    .iter()
                    .map(|r| std::mem::size_of_val(r) + r.name.len())
                    .sum::<usize>(),
            )
            .saturating_add(
                d.passes
                    .iter()
                    .map(|p| {
                        std::mem::size_of_val(p)
                            + p.name.len()
                            + p.accesses.len() * std::mem::size_of::<AccessDeclaration>()
                    })
                    .sum::<usize>(),
            )
            .saturating_add(
                self.info.accesses.len()
                    * std::mem::size_of::<super::graph::ResourceAccessRecord>(),
            )
            .saturating_add(
                self.info.lifetimes.len()
                    * std::mem::size_of::<super::graph::ResourceLifetimeRecord>(),
            )
            .saturating_add(self.dependency_edges.len() * std::mem::size_of::<(usize, usize)>())
            .saturating_add(
                self.queue_schedule.domains.len()
                    * std::mem::size_of::<super::queue_schedule::QueueDomain>(),
            )
            .saturating_add(
                self.queue_schedule
                    .dependencies
                    .iter()
                    .map(|dependencies| {
                        dependencies.len() * std::mem::size_of::<usize>()
                            + std::mem::size_of::<Vec<usize>>()
                    })
                    .sum::<usize>(),
            )
            .saturating_add(d.exports.len() * std::mem::size_of::<ExportDeclaration>())
            .saturating_add(d.access_semantics.len() * std::mem::size_of::<AccessSemantics>())
            .saturating_add(self.retained_passes.len() * std::mem::size_of::<usize>())
            .saturating_add(self.batches.len() * std::mem::size_of::<Batch>())
            .saturating_add(
                d.subgraphs
                    .iter()
                    .map(|scope| {
                        std::mem::size_of::<super::subgraph::SubgraphDeclaration>()
                            + scope.name.len()
                            + (scope.interface.inputs.len() + scope.interface.outputs.len())
                                * std::mem::size_of::<super::subgraph::SubgraphPort>()
                    })
                    .sum::<usize>(),
            )
    }
    pub fn dump(&self) -> String {
        use std::fmt::Write;
        let mut out = format!(
            "compiler={} passes={}/{} resources={}\n",
            self.definition.compiler_version,
            self.retained_passes.len(),
            self.definition.passes.len(),
            self.definition.resources.len()
        );
        for (scheduled, original) in self.retained_passes.iter().enumerate() {
            let p = &self.definition.passes[*original];
            let _ = writeln!(
                out,
                "pass {scheduled} original={original} {:?} side_effect={}",
                p.name, p.side_effect
            );
            for a in &p.accesses {
                let _ = writeln!(
                    out,
                    "  r{} {:?} {:?} access={:#x}",
                    a.resource_index, a.version, a.range, a.access_code
                );
            }
        }
        for (from, to) in &self.dependency_edges {
            let _ = writeln!(out, "dependency {from} -> {to}");
        }
        out
    }
}

pub fn ranges_overlap(a: AccessRange, b: AccessRange) -> bool {
    match (a, b) {
        (AccessRange::Whole, _) | (_, AccessRange::Whole) => true,
        (
            AccessRange::Buffer {
                offset: a,
                size: an,
            },
            AccessRange::Buffer {
                offset: b,
                size: bn,
            },
        ) => a < b.saturating_add(bn) && b < a.saturating_add(an),
        (
            AccessRange::Texture {
                aspect_mask: a,
                base_mip_level: am,
                level_count: an,
                base_array_layer: al,
                layer_count: ac,
            },
            AccessRange::Texture {
                aspect_mask: b,
                base_mip_level: bm,
                level_count: bn,
                base_array_layer: bl,
                layer_count: bc,
            },
        ) => {
            a & b != 0
                && am < bm.saturating_add(bn)
                && bm < am.saturating_add(an)
                && al < bl.saturating_add(bc)
                && bl < al.saturating_add(ac)
        }
        _ => false,
    }
}
fn range_covers(a: AccessRange, b: AccessRange) -> bool {
    match (a, b) {
        (AccessRange::Whole, _) => true,
        (_, AccessRange::Whole) => false,
        (
            AccessRange::Buffer {
                offset: a,
                size: an,
            },
            AccessRange::Buffer {
                offset: b,
                size: bn,
            },
        ) => a <= b && a.saturating_add(an) >= b.saturating_add(bn),
        (
            AccessRange::Texture {
                aspect_mask: a,
                base_mip_level: am,
                level_count: an,
                base_array_layer: al,
                layer_count: ac,
            },
            AccessRange::Texture {
                aspect_mask: b,
                base_mip_level: bm,
                level_count: bn,
                base_array_layer: bl,
                layer_count: bc,
            },
        ) => {
            a & b == b
                && am <= bm
                && am.saturating_add(an) >= bm.saturating_add(bn)
                && al <= bl
                && al.saturating_add(ac) >= bl.saturating_add(bc)
        }
        _ => false,
    }
}

/// Direct lookup from an access code to its semantics (codes are `u8`).
pub(crate) struct SemanticTable<'a> {
    index: [u8; 256],
    semantics: &'a [AccessSemantics],
}
impl<'a> SemanticTable<'a> {
    pub(crate) fn new(semantics: &'a [AccessSemantics]) -> Self {
        let mut index = [u8::MAX; 256];
        // The first entry wins, matching a linear `find`; definitions carry at
        // most one entry per code (one per distinct u8, so < 256 entries).
        for (position, semantic) in semantics.iter().enumerate().rev() {
            if let Ok(position) = u8::try_from(position) {
                index[semantic.code as usize] = position;
            }
        }
        Self { index, semantics }
    }
    pub(crate) fn get(&self, code: u8) -> Option<&'a AccessSemantics> {
        match self.index[code as usize] {
            u8::MAX => self.semantics.iter().find(|semantic| semantic.code == code),
            position => Some(&self.semantics[position as usize]),
        }
    }
    pub(crate) fn is_write(&self, code: u8) -> bool {
        self.get(code).is_some_and(|semantic| semantic.write)
    }
}

/// Dependency analysis contains only local symbolic indices and ranges:
/// content producers (for culling) and the current producer frontier.
#[derive(Clone, Debug)]
pub(crate) struct PassAnalysis {
    pub dependencies: Vec<BTreeSet<usize>>,
    pub producers: std::collections::BTreeMap<u32, Vec<(usize, AccessRange)>>,
}
impl PassAnalysis {
    pub(crate) fn estimated_bytes(&self) -> usize {
        std::mem::size_of::<Self>()
            .saturating_add(
                self.dependencies
                    .iter()
                    .map(|set| std::mem::size_of_val(set) + set.len() * 48)
                    .sum::<usize>(),
            )
            .saturating_add(
                self.producers
                    .values()
                    .map(|entries| 64 + entries.len() * std::mem::size_of::<(usize, AccessRange)>())
                    .sum::<usize>(),
            )
    }
}
/// Record content producers of one access; returns nothing. A write removes
/// every producer whose range it fully covers.
fn add_producer_access(
    analysis: &mut PassAnalysis,
    pass_index: usize,
    access: &AccessDeclaration,
    write: bool,
) {
    let producers = analysis.producers.entry(access.resource_index).or_default();
    if access.reads_contents(write) {
        for &(producer, range) in producers.iter() {
            if producer != pass_index && ranges_overlap(access.range, range) {
                analysis.dependencies[pass_index].insert(producer);
            }
        }
    }
    if write {
        producers.retain(|&(_, range)| !range_covers(access.range, range));
        producers.push((pass_index, access.range));
    }
}
pub(crate) fn analyze_passes(
    passes: &[PassDeclaration],
    semantics: &[AccessSemantics],
) -> PassAnalysis {
    let table = SemanticTable::new(semantics);
    let mut analysis = PassAnalysis {
        dependencies: vec![BTreeSet::new(); passes.len()],
        producers: Default::default(),
    };
    for (pass_index, pass) in passes.iter().enumerate() {
        for access in &pass.accesses {
            add_producer_access(&mut analysis, pass_index, access, table.is_write(access.access_code));
        }
    }
    analysis
}
/// One live access in the per-resource ordering frontier.
#[derive(Clone, Copy)]
struct FrontierEntry {
    pass: usize,
    range: AccessRange,
    native: Option<crate::resource::OwnershipRange>,
    layout: ash::vk::ImageLayout,
    write: bool,
}
/// Storage-order constraints between retained passes: write/any overlaps and
/// overlapping image accesses that need different native layouts. Only the
/// frontier is kept: an entry fully covered by a later write is ordered before
/// that write, and every later access that would conflict with the entry also
/// conflicts with the write, so the edge is implied transitively.
fn ordering_edges(
    definition: &GraphDefinition,
    retained: &[usize],
) -> crate::Result<Vec<(usize, usize)>> {
    let table = SemanticTable::new(&definition.access_semantics);
    let mut frontier = std::collections::HashMap::<u32, Vec<FrontierEntry>>::new();
    let mut edges = BTreeSet::new();
    for &pass in retained {
        for access in &definition.passes[pass].accesses {
            let semantic = table
                .get(access.access_code)
                .ok_or_else(|| crate::Error::invalid_usage("access semantics are absent"))?;
            let descriptor = definition.resources[access.resource_index as usize].descriptor;
            let native = match descriptor {
                Descriptor::Image(_) => Some(super::sync_plan::native_range(
                    access.range,
                    descriptor,
                    definition.context.separate_depth_stencil_layouts,
                )?),
                Descriptor::Buffer(_) => None,
            };
            let entries = frontier.entry(access.resource_index).or_default();
            for entry in entries.iter() {
                if entry.pass == pass {
                    continue;
                }
                let content = (semantic.write || entry.write)
                    && ranges_overlap(access.range, entry.range);
                let layout = matches!((native, entry.native), (Some(a), Some(b))
                    if entry.layout != semantic.layout
                        && crate::resource::range_state::intersection(a, b).is_some());
                if content || layout {
                    edges.insert((entry.pass, pass));
                }
            }
            if semantic.write {
                // A widened native range (merged depth/stencil) may reach
                // entries this declaration does not cover; keep those.
                let exact = match (native, access.range) {
                    (Some(crate::resource::OwnershipRange::Image { aspects, .. }), AccessRange::Texture { aspect_mask, .. }) => aspects == aspect_mask,
                    _ => true,
                };
                if exact {
                    entries.retain(|entry| !range_covers(access.range, entry.range));
                }
            }
            entries.push(FrontierEntry {
                pass,
                range: access.range,
                native,
                layout: semantic.layout,
                write: semantic.write,
            });
        }
    }
    Ok(edges.into_iter().collect())
}
/// Produces immutable symbolic work before asking the backend for any allocation.
pub fn compile_definition(definition: GraphDefinition) -> crate::Result<CompiledPlan> {
    super::plan_check::check_definition(&definition)?;
    let analysis = analyze_passes(&definition.passes, &definition.access_semantics);
    finish_plan(definition, analysis)
}
/// Reuses stable local analyses, then coordinates all cross-scope hazards and
/// rebuilds global liveness, symbolic barriers, batches and resource lifetimes.
pub fn compile_with_subgraphs(
    definition: GraphDefinition,
    cache: &mut super::subgraph::SubgraphPlanCache,
) -> crate::Result<CompiledPlan> {
    if definition.subgraphs.is_empty() || !cache.is_enabled() {
        return compile_definition(definition);
    }
    super::plan_check::check_definition(&definition)?;
    let mut analysis = PassAnalysis {
        dependencies: vec![BTreeSet::new(); definition.passes.len()],
        producers: Default::default(),
    };
    let table = SemanticTable::new(&definition.access_semantics);
    let mut next = 0;
    for scope in &definition.subgraphs {
        while next < scope.pass_range.start {
            for access in &definition.passes[next].accesses {
                add_producer_access(&mut analysis, next, access, table.is_write(access.access_code));
            }
            next += 1;
        }
        let local = cache.analysis(&definition, scope)?;
        compose_analysis(
            &definition,
            &table,
            local.range.clone(),
            &local.analysis,
            Some(&local.resources),
            &mut analysis,
        );
        next = local.range.end;
    }
    while next < definition.passes.len() {
        for access in &definition.passes[next].accesses {
            add_producer_access(&mut analysis, next, access, table.is_write(access.access_code));
        }
        next += 1;
    }
    finish_plan(definition, analysis)
}
fn compose_analysis(
    definition: &GraphDefinition,
    table: &SemanticTable<'_>,
    range: std::ops::Range<usize>,
    local: &PassAnalysis,
    resources: Option<&[u32]>,
    global: &mut PassAnalysis,
) {
    for (pass, dependencies) in local.dependencies.iter().enumerate() {
        global.dependencies[range.start + pass]
            .extend(dependencies.iter().map(|&source| range.start + source));
    }
    // Producers contain earlier scopes only. The cached local analysis
    // supplies all inner-scope relations without replaying them.
    for pass_index in range.clone() {
        for access in &definition.passes[pass_index].accesses {
            let write = table.is_write(access.access_code);
            let producers = global.producers.entry(access.resource_index).or_default();
            if access.reads_contents(write) {
                for &(source, written) in producers.iter() {
                    if ranges_overlap(access.range, written) {
                        global.dependencies[pass_index].insert(source);
                    }
                }
            }
            if write {
                producers.retain(|&(_, written)| !range_covers(access.range, written));
            }
        }
    }
    for (&resource, producers) in &local.producers {
        let global_resource = resources.map_or(resource, |mapping| mapping[resource as usize]);
        global.producers.entry(global_resource).or_default().extend(
            producers
                .iter()
                .map(|&(source, range_written)| (range.start + source, range_written)),
        );
    }
}
fn finish_plan(definition: GraphDefinition, analysis: PassAnalysis) -> crate::Result<CompiledPlan> {
    let count = definition.passes.len();
    let PassAnalysis {
        dependencies: data_dependencies,
        producers,
    } = analysis;
    let mut needed = vec![false; count];
    let mut pending = VecDeque::new();
    for (i, p) in definition.passes.iter().enumerate() {
        if p.side_effect {
            pending.push_back(i);
        }
    }
    for export in &definition.exports {
        if let Some(writers) = producers.get(&export.resource_index) {
            for &(producer, range) in writers {
                if ranges_overlap(export.range, range) {
                    pending.push_back(producer);
                }
            }
        }
    }
    while let Some(pass) = pending.pop_front() {
        if std::mem::replace(&mut needed[pass], true) {
            continue;
        }
        pending.extend(data_dependencies[pass].iter().copied());
    }
    let retained_passes = (0..count).filter(|&i| needed[i]).collect::<Vec<_>>();
    // Even two reads cannot overlap while asking for different native image
    // layouts. This is a storage constraint rather than a content producer edge.
    let dependency_edges = ordering_edges(&definition, &retained_passes)?;
    super::queue_schedule::validate_resource_queue_contracts(
        &definition,
        &retained_passes,
        &dependency_edges,
    )?;
    let origins = definition
        .resources
        .iter()
        .map(ResourceDefinition::info_origin)
        .collect::<Vec<_>>();
    let pass_slices = retained_passes
        .iter()
        .map(|&i| definition.passes[i].accesses.as_slice())
        .collect::<Vec<_>>();
    let mut info = ResourceInfo::default();
    let table = SemanticTable::new(&definition.access_semantics);
    info.calculate(&origins, &definition.exports, &pass_slices, |code| {
        table.get(code).map_or(0, |semantic| semantic.mask)
    })?;
    let mut batch_compiler = BatchCompiler::new();
    if definition.batch_per_pass {
        for i in 0..retained_passes.len() {
            batch_compiler.flush_batch(i);
        }
    }
    let base_batches = batch_compiler.finish(retained_passes.len());
    let (batches, queue_schedule) = super::queue_schedule::plan_queue_batches(
        &definition.passes,
        &retained_passes,
        &base_batches,
        &dependency_edges,
        &definition.exports,
        &definition.context,
    )?;
    let snapshot = std::sync::Arc::new(super::graph::SnapshotStatic::from_plan(
        &definition,
        &retained_passes,
        &info,
        &batches,
    ));
    let range_program = super::sync_plan::RangeProgram::compile(
        &definition,
        &retained_passes,
        &batches,
        &queue_schedule,
    )?;
    let plan = CompiledPlan {
        range_program,
        definition,
        retained_passes,
        dependency_edges,
        info: std::sync::Arc::new(info),
        snapshot,
        batches,
        queue_schedule,
    };
    super::plan_check::check_plan(&plan.definition, &plan)?;
    Ok(plan)
}

/// Version retained in public dumps and cache keys; no frame identity is stored.
pub fn access_version(access: &AccessDeclaration) -> AccessVersion {
    access.version
}

#[cfg(test)]
mod borrowed_definition_tests {
    use super::*;
    use crate::rg::{
        graph::{BufferDesc, RenderGraph},
        resource::{AccessRange, ResourceOrigin},
        vulkan::AccessTable,
    };
    fn declaration() -> GraphDeclaration {
        let mut graph = RenderGraph::new();
        let descriptor = Descriptor::Buffer(BufferDesc {
            word_at_00: 64,
            word_at_08: 0,
            bytes_at_0c_through_0e: [0; 3],
        });
        let input = graph.import_unowned("input", descriptor, 11, 0x25).unwrap();
        let output = graph
            .create_raw_resource(
                "output",
                ResourceOrigin {
                    descriptor,
                    source: OriginSource::Create,
                },
            )
            .unwrap();
        graph
            .subgraph("stable", |scope| {
                scope.input(input, AccessRange::Whole, 0x25)?;
                let mut pass = scope.graph().add_pass("copy");
                pass.add_resource_access(input, 0x25, 3)?;
                pass.add_resource_access(output, 0x34, 4)?;
                pass.render(|_| true);
                scope.output(output, AccessRange::Whole, 0x34)?;
                Ok(())
            })
            .unwrap();
        graph.export(output, 0x25).unwrap();
        graph.build().declaration
    }
    #[test]
    fn complete_plan_miss_reasons_distinguish_policy_interface_and_structure() {
        use super::super::cache::CacheReason;
        let old = GraphDefinition::from_declaration(
            &declaration(),
            &mut AccessTable::original(),
            PlanContext::default(),
        )
        .unwrap();
        let mut new = old.clone();
        new.context.device_generation += 1;
        assert_eq!(
            old.differing_key_reason(&new),
            CacheReason::DeviceGeneration
        );
        let mut new = old.clone();
        new.context.policy_signature += 1;
        assert_eq!(old.differing_key_reason(&new), CacheReason::QueuePolicy);
        let mut new = old.clone();
        new.subgraphs[0].interface.implementation_signature += 1;
        assert_eq!(
            old.differing_key_reason(&new),
            CacheReason::ShaderImplementation
        );
        let mut new = old.clone();
        new.subgraphs[0].interface.shader_interface_signature += 1;
        assert_eq!(old.differing_key_reason(&new), CacheReason::ShaderInterface);
        let mut new = old.clone();
        new.passes[0].name = "changed pass".into();
        assert_eq!(old.differing_key_reason(&new), CacheReason::GraphStructure);
    }
    #[test]
    fn borrowed_comparison_matches_the_complete_owned_definition() {
        let graph = declaration();
        let context = PlanContext::default();
        let key = GraphDefinition::from_declaration(&graph, &mut AccessTable::original(), context)
            .unwrap();
        let changes: &[fn(&mut GraphDeclaration)] = &[
            |g| g.resources[0].name.push('!'),
            |g| {
                g.resources[0].descriptor = Descriptor::Buffer(BufferDesc {
                    word_at_00: 128,
                    word_at_08: 0,
                    bytes_at_0c_through_0e: [0; 3],
                })
            },
            |g| {
                g.resources[0].sharing = crate::resource::ResourceSharing::Concurrent(
                    crate::resource::QueueFamilies::new(&[0, 1]).unwrap(),
                )
            },
            |g| {
                g.resources[0].source = OriginSource::ImportOwned {
                    token: 11,
                    initial_access: 0x25,
                }
            },
            |g| g.passes[0].name = format!("{}!", g.passes[0].name).into(),
            |g| g.passes[0].queue_class = crate::device::QueueClass::Compute,
            |g| g.passes[0].side_effect = true,
            |g| g.passes[0].accesses[0].synchronization_group += 1,
            |g| g.passes[0].accesses[0].access_code = 0x26,
            |g| {
                g.passes[0].accesses[0].range = AccessRange::Buffer {
                    offset: 0,
                    size: 32,
                }
            },
            |g| g.passes[0].accesses[0].reads_previous = true,
            |g| g.passes[0].accesses[0].allow_undefined_contents = true,
            |g| g.passes[0].accesses[0].version = AccessVersion::Read(9),
            |g| g.subgraphs[0].interface.implementation_signature += 1,
            |g| g.exports[0].access_code = 0x26,
            |g| {
                g.exports[0].range = AccessRange::Buffer {
                    offset: 0,
                    size: 32,
                }
            },
            |g| g.batch_per_pass = true,
        ];
        for change in changes {
            let mut changed = graph.clone();
            change(&mut changed);
            assert!(
                !key.matches_declaration(&changed, &mut AccessTable::original(), context)
                    .unwrap()
            );
            assert!(
                GraphDefinition::from_declaration(&changed, &mut AccessTable::original(), context)
                    .map_or(true, |owned| owned != key)
            );
        }
        let mut changed = graph.clone();
        changed.resources[0].source = OriginSource::ImportUnowned {
            token: 99,
            initial_access: 0,
        };
        changed.passes[0].graph_id = None;
        for (access, resolved) in changed.passes[0]
            .accesses
            .iter_mut()
            .zip(&key.passes[0].accesses)
        {
            access.version = resolved.version;
        }
        assert_eq!(
            GraphDefinition::from_declaration(&changed, &mut AccessTable::original(), context)
                .unwrap(),
            key
        );
        assert!(
            key.matches_declaration(&changed, &mut AccessTable::original(), context)
                .unwrap()
        );
        let mut policy = AccessTable::original();
        policy.0.get_mut(&0x34).unwrap().original_mask ^= 1;
        assert!(
            !key.matches_declaration(&graph, &mut policy, context)
                .unwrap()
        );
        let mut policy = AccessTable::original();
        policy.0.get_mut(&0x34).unwrap().write = false;
        assert!(
            !key.matches_declaration(&graph, &mut policy, context)
                .unwrap()
        );
        for changed_context in [
            PlanContext {
                separate_depth_stencil_layouts: true,
                ..context
            },
            PlanContext {
                device_generation: 1,
                ..context
            },
            PlanContext {
                queue_family: 1,
                ..context
            },
            PlanContext {
                queue_index: 1,
                ..context
            },
            PlanContext {
                policy_signature: 1,
                ..context
            },
            PlanContext {
                queue_domains: Some(
                    [super::super::queue_schedule::QueueDomain {
                        queue_index: 0,
                        family: 0,
                    }; 3],
                ),
                ..context
            },
        ] {
            assert!(
                !key.matches_declaration(&graph, &mut AccessTable::original(), changed_context)
                    .unwrap()
            );
        }
    }
    #[test]
    fn native_scope_and_layout_changes_invalidate_owned_and_borrowed_plan_keys() {
        let graph = declaration();
        let context = PlanContext::default();
        let original =
            GraphDefinition::from_declaration(&graph, &mut AccessTable::original(), context)
                .unwrap();
        let changes: &[fn(&mut super::super::vulkan::AccessSpec)] = &[
            |spec| spec.stages = ash::vk::PipelineStageFlags::ALL_COMMANDS,
            |spec| spec.access = ash::vk::AccessFlags::MEMORY_WRITE,
            |spec| spec.layout = ash::vk::ImageLayout::GENERAL,
        ];
        for change in changes {
            let mut policy = AccessTable::original();
            change(policy.0.get_mut(&0x34).unwrap());
            let changed = GraphDefinition::from_declaration(&graph, &mut policy, context).unwrap();
            assert_ne!(original, changed);
            assert_eq!(
                original.differing_key_reason(&changed),
                super::super::cache::CacheReason::AccessSemantics
            );
            assert!(
                !original
                    .matches_declaration(&graph, &mut policy, context)
                    .unwrap()
            );
        }
    }
    #[test]
    fn matching_definition_still_rejects_aliased_dynamic_imports() {
        let mut graph = declaration();
        graph.resources[1].source = OriginSource::ImportUnowned {
            token: 22,
            initial_access: 0x25,
        };
        let key = GraphDefinition::from_declaration(
            &graph,
            &mut AccessTable::original(),
            PlanContext::default(),
        )
        .unwrap();
        graph.resources[1].source = graph.resources[0].source;
        assert!(
            key.matches_declaration(&graph, &mut AccessTable::original(), PlanContext::default())
                .is_err()
        );
    }
    #[test]
    fn matching_import_revalidates_partial_initialization_and_read_modify_write() {
        let descriptor = crate::rg::vulkan::ResourceSpec::Image(
            crate::raw::texture::TextureDesc::new_2d(16, 16, ash::vk::Format::R8G8B8A8_UNORM)
                .mip_levels(2),
        )
        .descriptor();
        let access = |code, mip| AccessDeclaration {
            resource_index: 0,
            synchronization_group: 0,
            access_code: code,
            version: AccessVersion::Automatic,
            reads_previous: false,
            contents_unused: false,
            allow_undefined_contents: false,
            range: AccessRange::Texture {
                aspect_mask: 1,
                base_mip_level: mip,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
        };
        for full_write in [false, true] {
            let mut graph = GraphDeclaration::default();
            graph
                .resources
                .push(super::super::graph::ResourceDeclaration {
                    sharing: Default::default(),
                    name: String::from("image"),
                    descriptor,
                    source: OriginSource::ImportUnowned {
                        token: 1,
                        initial_access: 0x25,
                    },
                });
            let mut writes = vec![access(0x34, 0)];
            if full_write {
                writes.push(access(0x34, 1));
            }
            graph.passes.push(PassDeclaration {
                queue_class: crate::device::QueueClass::Graphics,
                graph_id: None,
                name: "initialize".into(),
                accesses: writes,
                side_effect: false,
            });
            graph.passes.push(PassDeclaration {
                queue_class: crate::device::QueueClass::Graphics,
                graph_id: None,
                name: "read".into(),
                accesses: vec![AccessDeclaration {
                    range: AccessRange::Whole,
                    ..access(0x25, 0)
                }],
                side_effect: true,
            });
            let key = GraphDefinition::from_declaration(
                &graph,
                &mut AccessTable::original(),
                PlanContext::default(),
            )
            .unwrap();
            graph.resources[0].source = OriginSource::ImportUnowned {
                token: 2,
                initial_access: 0,
            };
            let matched = key.matches_declaration(
                &graph,
                &mut AccessTable::original(),
                PlanContext::default(),
            );
            assert_eq!(matched.is_ok(), full_write);
            assert_eq!(
                GraphDefinition::from_declaration(
                    &graph,
                    &mut AccessTable::original(),
                    PlanContext::default()
                )
                .is_ok(),
                full_write
            );
            if full_write {
                assert!(matched.unwrap());
            }
        }
        let mut graph = declaration();
        graph.passes[0].accesses[1].reads_previous = true;
        graph.resources[1].source = OriginSource::ImportUnowned {
            token: 2,
            initial_access: 0x25,
        };
        let key = GraphDefinition::from_declaration(
            &graph,
            &mut AccessTable::original(),
            PlanContext::default(),
        )
        .unwrap();
        // Buffer imports are initialized independently of image layout codes.
        graph.resources[1].source = OriginSource::ImportUnowned {
            token: 3,
            initial_access: 0,
        };
        assert!(
            key.matches_declaration(&graph, &mut AccessTable::original(), PlanContext::default())
                .unwrap()
        );
    }
    #[test]
    fn undefined_read_permission_does_not_bypass_cached_ordinary_boundary_validation() {
        let descriptor = super::super::vulkan::ResourceSpec::Image(
            crate::raw::texture::TextureDesc::new_2d(8, 8, ash::vk::Format::R32_SFLOAT))
            .descriptor();
        for permitted in [false, true] {
            let mut graph = RenderGraph::new();
            let h = graph.import_unowned("history", descriptor, 9, 0x19).unwrap();
            let mut pass = graph.add_pass("history");
            if permitted {
                pass.add_undefined_content_read_range(h, 0x19, 0, AccessRange::Whole).unwrap();
            } else {
                pass.add_resource_access(h, 0x19, 0).unwrap();
            }
            pass.side_effect(); pass.render(|_| true);
            let mut raw = graph.build().declaration;
            let context = PlanContext::default();
            let key = GraphDefinition::from_declaration(&raw, &mut AccessTable::original(), context).unwrap();
            raw.resources[0].source = OriginSource::ImportUnowned { token: 10, initial_access: 0 };
            let borrowed = key.matches_declaration(&raw, &mut AccessTable::original(), context);
            assert_eq!(borrowed.is_ok(), permitted);
            if permitted { assert!(borrowed.unwrap()); }
            assert_eq!(GraphDefinition::from_declaration(&raw, &mut AccessTable::original(), context).is_ok(), permitted);
        }
    }

}

#[cfg(test)]
mod ordering_tests {
    use super::*;
    use crate::rg::{
        graph::{BufferDesc, RenderGraph},
        resource::ResourceOrigin,
        vulkan::AccessTable,
    };
    const READ: u8 = 0x25;
    const WRITE: u8 = 0x34;
    /// `passes[i] = (code, side_effect)` all on one created buffer.
    fn plan(passes: &[(u8, bool)]) -> CompiledPlan {
        let mut graph = RenderGraph::new();
        let descriptor = Descriptor::Buffer(BufferDesc {
            word_at_00: 64,
            word_at_08: 0,
            bytes_at_0c_through_0e: [0; 3],
        });
        let buffer = graph
            .create_raw_resource(
                "b",
                ResourceOrigin {
                    descriptor,
                    source: OriginSource::Create,
                },
            )
            .unwrap();
        for (index, &(code, side_effect)) in passes.iter().enumerate() {
            let mut pass = graph.add_pass(format!("p{index}"));
            if side_effect {
                pass = pass.with_side_effect();
            }
            pass.add_resource_access(buffer, code, 0).unwrap();
            pass.render(|_| true);
        }
        let declaration = graph.build().declaration;
        let definition = GraphDefinition::from_declaration(
            &declaration,
            &mut AccessTable::original(),
            PlanContext::default(),
        )
        .unwrap();
        compile_definition(definition).unwrap()
    }
    fn reaches(edges: &[(usize, usize)], from: usize, to: usize) -> bool {
        let mut pending = vec![from];
        let mut seen = BTreeSet::new();
        while let Some(next) = pending.pop() {
            for &(a, b) in edges {
                if a == next && seen.insert(b) {
                    pending.push(b);
                }
            }
        }
        seen.contains(&to)
    }
    #[test]
    fn culled_middle_writer_keeps_the_surviving_writers_ordered() {
        let plan = plan(&[(WRITE, true), (WRITE, false), (WRITE, true)]);
        assert_eq!(plan.retained_passes, [0, 2]);
        assert_eq!(plan.dependency_edges, [(0, 2)]);
    }
    #[test]
    fn frontier_keeps_every_conflict_transitively() {
        // W0 R1 R2 W3 W4 R5: each conflicting pair stays ordered.
        let passes = [
            (WRITE, true),
            (READ, true),
            (READ, true),
            (WRITE, true),
            (WRITE, true),
            (READ, true),
        ];
        let plan = plan(&passes);
        for later in 0..passes.len() {
            for earlier in 0..later {
                if passes[earlier].0 == WRITE || passes[later].0 == WRITE {
                    assert!(
                        reaches(&plan.dependency_edges, earlier, later),
                        "{earlier} -> {later} in {:?}",
                        plan.dependency_edges
                    );
                }
            }
        }
        assert!(!reaches(&plan.dependency_edges, 1, 2), "two reads stay unordered");
        // Covered entries are pruned: W4 does not need a direct edge from W0.
        assert!(!plan.dependency_edges.contains(&(0, 4)));
    }
}
