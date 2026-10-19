//! Stable symbolic subgraphs. Cached entries never own frame resources.
use super::{
    cache::{BoundedCache, CacheDiagnostics, CacheReason},
    graph::{PassDeclaration, RenderGraph},
    plan::{AccessSemantics, GraphDefinition, PlanContext, ResourceDefinition},
    resource::{AccessRange, AccessVersion, ResourceHandle},
};
use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Range,
    sync::Arc,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SubgraphQueueConstraint {
    #[default]
    Inherit,
    Exact {
        family: u32,
        index: u32,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SubgraphPort {
    pub resource: u32,
    pub range: AccessRange,
    pub access_code: u8,
    pub version: u32,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SubgraphInterface {
    pub inputs: Vec<SubgraphPort>,
    pub outputs: Vec<SubgraphPort>,
    pub queue: SubgraphQueueConstraint,
    pub side_effect: bool,
    pub implementation_signature: u64,
    pub shader_interface_signature: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SubgraphDeclaration {
    pub name: String,
    pub pass_range: Range<usize>,
    pub interface: SubgraphInterface,
}
pub struct SubgraphBuilder<'a> {
    graph: &'a mut RenderGraph,
    start: usize,
    interface: SubgraphInterface,
}
impl SubgraphBuilder<'_> {
    pub fn graph(&mut self) -> &mut RenderGraph {
        self.graph
    }
    pub fn interface_mut(&mut self) -> &mut SubgraphInterface {
        &mut self.interface
    }
    pub fn input(
        &mut self,
        resource: ResourceHandle,
        range: AccessRange,
        access_code: u8,
    ) -> crate::Result<()> {
        if self.graph.declaration.passes.len() != self.start {
            return Err(crate::Error::invalid_usage(
                "subgraph inputs must be declared before its passes",
            ));
        }
        let port = self.port(resource, range, access_code)?;
        self.interface.inputs.push(port);
        Ok(())
    }
    pub fn output(
        &mut self,
        resource: ResourceHandle,
        range: AccessRange,
        access_code: u8,
    ) -> crate::Result<()> {
        let port = self.port(resource, range, access_code)?;
        self.interface.outputs.push(port);
        Ok(())
    }
    fn port(
        &self,
        resource: ResourceHandle,
        range: AccessRange,
        access_code: u8,
    ) -> crate::Result<SubgraphPort> {
        self.graph.check_handle(resource)?;
        range
            .validate(self.graph.declaration.resources[resource.raw() as usize].descriptor)
            .map_err(|error| super::resource_contracts::range_error(resource.raw(), error))?;
        if usize::from(access_code) >= crate::raw::access_table::ACCESS_CODE_COUNT {
            return Err(crate::Error::invalid_usage(
                "subgraph port access code is invalid",
            ));
        }
        Ok(SubgraphPort {
            resource: resource.raw(),
            range,
            access_code,
            version: self.graph.current_version(resource)?,
        })
    }
}
impl RenderGraph {
    /// Declare a contiguous scope, checking resource ownership at its ports.
    /// Nested scopes are rejected; independent scopes may follow one another.
    /// Failed scopes roll back passes, exports and batch policy. Resource IDs
    /// remain frame-wide and append-only, so escaped handles cannot alias a later
    /// allocation. Unused declarations do not allocate GPU storage. Retirement
    /// callbacks are also frame-wide and remain attached after scope failure.
    pub fn subgraph<T>(
        &mut self,
        name: impl Into<String>,
        build: impl FnOnce(&mut SubgraphBuilder<'_>) -> crate::Result<T>,
    ) -> crate::Result<T> {
        let start = self.declaration.passes.len();
        let exports = self.declaration.exports.len();
        let batch_per_pass = self.declaration.batch_per_pass;
        let scopes = self.declaration.subgraphs.len();
        let mut builder = SubgraphBuilder {
            graph: self,
            start,
            interface: SubgraphInterface::default(),
        };
        let result = build(&mut builder);
        let interface = builder.interface;
        if result.is_err()
            || self.declaration.subgraphs.len() != scopes
            || self.declaration.passes.len() == start
        {
            self.declaration.passes.truncate(start);
            self.callbacks.truncate(start);
            self.declaration.exports.truncate(exports);
            self.declaration.batch_per_pass = batch_per_pass;
            self.declaration.subgraphs.truncate(scopes);
            return result.and_then(|_| {
                Err(crate::Error::invalid_usage(
                    "subgraphs must be nonempty, nonnested scopes",
                ))
            });
        }
        if interface.side_effect {
            for pass in &mut self.declaration.passes[start..] {
                pass.side_effect = true;
            }
        }
        self.declaration.subgraphs.push(SubgraphDeclaration {
            name: name.into(),
            pass_range: start..self.declaration.passes.len(),
            interface,
        });
        result
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SubgraphCacheStats {
    pub diagnostics: CacheDiagnostics,
    pub hits: u64,
    pub misses: u64,
    pub compilations: u64,
    pub evictions: u64,
    pub bytes: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct LocalKey {
    resources: Vec<ResourceDefinition>,
    passes: Vec<PassDeclaration>,
    interface: SubgraphInterface,
    semantics: Vec<AccessSemantics>,
    context: PlanContext,
    compiler_version: u32,
}
pub struct SubgraphPlanCache {
    cache: BoundedCache<LocalKey, Arc<super::plan::PassAnalysis>>,
    enabled: bool,
    compilations: u64,
}
impl Default for SubgraphPlanCache {
    fn default() -> Self {
        Self::new(128, 16 * 1024 * 1024)
    }
}
impl SubgraphPlanCache {
    pub fn new(max_entries: usize, max_bytes: usize) -> Self {
        Self {
            // A zero budget must not turn a renderer-owned cache unbounded.
            cache: BoundedCache::new(max_entries.max(1), max_bytes.max(1)),
            enabled: true,
            compilations: 0,
        }
    }
    pub fn clear(&mut self) {
        self.cache.clear();
    }
    pub(crate) fn is_enabled(&self) -> bool {
        self.enabled
    }
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.cache.clear_with_reason(CacheReason::Disabled);
        }
    }
    pub fn stats(&self) -> SubgraphCacheStats {
        let stats = self.cache.stats();
        SubgraphCacheStats {
            diagnostics: stats.diagnostics,
            hits: stats.hits,
            misses: stats.misses,
            compilations: self.compilations,
            evictions: stats.evictions,
            bytes: self.cache.bytes(),
        }
    }
    pub(crate) fn analysis(
        &mut self,
        definition: &GraphDefinition,
        scope: &SubgraphDeclaration,
    ) -> crate::Result<CachedAnalysis> {
        let (key, resources) = local_key(definition, scope)?;
        let analysis = if let Some(hit) = self.enabled.then(|| self.cache.get(&key)).flatten() {
            hit.clone()
        } else {
            if self.enabled {
                let reason = self
                    .cache
                    .iter()
                    .map(|(old, _)| local_key_difference(old, &key))
                    .min_by_key(|reason| *reason as usize)
                    .unwrap_or(CacheReason::Cold);
                self.cache.classify_miss(reason);
            } else {
                self.cache.record_bypass(CacheReason::Disabled);
            }
            let analysis = Arc::new(super::plan::analyze_passes(&key.passes, &key.semantics));
            self.compilations += 1;
            if self.enabled {
                let bytes =
                    std::mem::size_of::<LocalKey>()
                        .saturating_add(
                            key.resources.len() * std::mem::size_of::<ResourceDefinition>(),
                        )
                        .saturating_add(
                            key.passes
                                .iter()
                                .map(|pass| {
                                    std::mem::size_of::<PassDeclaration>()
                                        + pass.accesses.len()
                                            * std::mem::size_of::<
                                                super::resource_info::AccessDeclaration,
                                            >()
                                })
                                .sum::<usize>(),
                        )
                        .saturating_add(analysis.estimated_bytes());
                let bytes = bytes
                    .saturating_add(
                        (key.interface.inputs.len() + key.interface.outputs.len())
                            * std::mem::size_of::<SubgraphPort>(),
                    )
                    .saturating_add(key.semantics.len() * std::mem::size_of::<AccessSemantics>());
                self.cache
                    .insert(key, analysis.clone(), bytes.saturating_mul(2));
            }
            analysis
        };
        Ok(CachedAnalysis {
            range: scope.pass_range.clone(),
            resources,
            analysis,
        })
    }
}
fn local_key_difference(old: &LocalKey, new: &LocalKey) -> CacheReason {
    if old.context.device_generation != new.context.device_generation {
        CacheReason::DeviceGeneration
    } else if old.context != new.context {
        CacheReason::QueuePolicy
    } else if old.compiler_version != new.compiler_version {
        CacheReason::CompilerVersion
    } else if old.resources.len() == new.resources.len()
        && old
            .resources
            .iter()
            .zip(&new.resources)
            .any(|(a, b)| a.descriptor != b.descriptor)
    {
        CacheReason::Descriptor
    } else if old.resources.len() == new.resources.len()
        && old
            .resources
            .iter()
            .zip(&new.resources)
            .any(|(a, b)| a.sharing != b.sharing)
    {
        CacheReason::Sharing
    } else if old.interface.implementation_signature != new.interface.implementation_signature {
        CacheReason::ShaderImplementation
    } else if old.interface.shader_interface_signature != new.interface.shader_interface_signature {
        CacheReason::ShaderInterface
    } else if old.interface != new.interface {
        CacheReason::SubgraphInterface
    } else if old.semantics != new.semantics {
        CacheReason::AccessSemantics
    } else {
        CacheReason::GraphStructure
    }
}
pub(crate) struct CachedAnalysis {
    pub range: Range<usize>,
    pub resources: Vec<u32>,
    pub analysis: Arc<super::plan::PassAnalysis>,
}

pub(crate) fn validate_subgraphs(definition: &GraphDefinition) -> crate::Result<()> {
    let mut end = 0;
    for scope in &definition.subgraphs {
        if scope.pass_range.start < end
            || scope.pass_range.start >= scope.pass_range.end
            || scope.pass_range.end > definition.passes.len()
        {
            return Err(crate::Error::invalid_usage(
                "subgraph pass ranges overlap or are invalid",
            ));
        }
        end = scope.pass_range.end;
        if let SubgraphQueueConstraint::Exact { family, index } = scope.interface.queue
            && (family, index)
                != (
                    definition.context.queue_family,
                    definition.context.queue_index,
                )
        {
            return Err(crate::Error::unsupported(
                "subgraph queue constraint differs from the parent plan queue",
            ));
        }
        for port in scope
            .interface
            .inputs
            .iter()
            .chain(&scope.interface.outputs)
        {
            let resource = definition
                .resources
                .get(port.resource as usize)
                .ok_or_else(|| {
                    crate::Error::invalid_usage("subgraph port references an unknown resource")
                })?;
            port.range
                .validate(resource.descriptor)
                .map_err(|error| super::resource_contracts::range_error(port.resource, error))?;
            if usize::from(port.access_code) >= crate::raw::access_table::ACCESS_CODE_COUNT {
                return Err(crate::Error::invalid_usage(
                    "subgraph port access code is invalid",
                ));
            }
            let semantic = definition
                .access_semantics
                .iter()
                .find(|entry| entry.code == port.access_code)
                .ok_or_else(|| {
                    crate::Error::invalid_usage("subgraph port access has no classification")
                })?;
            if matches!(resource.descriptor, super::graph::Descriptor::Image(_)) {
                crate::raw::resource_usage::image_usage_for_access(semantic.declared_access)?;
            }
        }
        if scope.interface.side_effect
            && definition.passes[scope.pass_range.clone()]
                .iter()
                .any(|pass| !pass.side_effect)
        {
            return Err(crate::Error::invalid_usage(
                "subgraph side effects are not retained by its passes",
            ));
        }
        validate_boundary(definition, scope)?;
    }
    Ok(())
}

fn validate_boundary(
    definition: &GraphDefinition,
    scope: &SubgraphDeclaration,
) -> crate::Result<()> {
    let version_at = |resource, boundary| {
        definition.passes[..boundary]
            .iter()
            .flat_map(|pass| &pass.accesses)
            .filter(|access| access.resource_index == resource)
            .filter_map(|access| match access.version {
                AccessVersion::Write { current, .. } => Some(current),
                _ => None,
            })
            .next_back()
            .unwrap_or(0)
    };
    for (ports, boundary) in [
        (&scope.interface.inputs, scope.pass_range.start),
        (&scope.interface.outputs, scope.pass_range.end),
    ] {
        for port in ports {
            if port.version != version_at(port.resource, boundary) {
                return Err(crate::Error::invalid_usage(
                    "subgraph port version differs from its boundary version",
                ));
            }
        }
    }
    let mut written = BTreeMap::<u32, Vec<AccessRange>>::new();
    for access in definition.passes[scope.pass_range.clone()]
        .iter()
        .flat_map(|pass| &pass.accesses)
    {
        let resource = &definition.resources[access.resource_index as usize];
        let write = definition.is_write(access.access_code);
        if access.reads_contents(write) {
            let available = written
                .get(&access.resource_index)
                .into_iter()
                .flatten()
                .copied()
                .chain(
                    scope
                        .interface
                        .inputs
                        .iter()
                        .filter(|port| port.resource == access.resource_index)
                        .map(|port| port.range),
                )
                .collect::<Vec<_>>();
            if !super::resource_contracts::covers(&available, access.range, resource.descriptor)
                .map_err(|error| {
                    super::resource_contracts::range_error(access.resource_index, error)
                })?
            {
                return Err(crate::Error::invalid_usage(
                    "subgraph reads outside its initialized internal content and input ports",
                ));
            }
        }
        if write {
            written
                .entry(access.resource_index)
                .or_default()
                .push(access.range);
            let escapes = resource.is_external()
                || definition.exports.iter().any(|export| {
                    export.resource_index == access.resource_index
                        && export.range.overlaps(access.range)
                })
                || definition.passes[scope.pass_range.end..]
                    .iter()
                    .flat_map(|pass| &pass.accesses)
                    .any(|later| {
                        later.resource_index == access.resource_index
                            && later.range.overlaps(access.range)
                    });
            if escapes {
                let outputs = scope
                    .interface
                    .outputs
                    .iter()
                    .filter(|port| port.resource == access.resource_index)
                    .map(|port| port.range)
                    .collect::<Vec<_>>();
                if !super::resource_contracts::covers(&outputs, access.range, resource.descriptor)
                    .map_err(|error| {
                    super::resource_contracts::range_error(access.resource_index, error)
                })? {
                    return Err(crate::Error::invalid_usage(
                        "subgraph writes outside its declared output ports",
                    ));
                }
            }
        }
    }
    Ok(())
}

fn local_key(
    definition: &GraphDefinition,
    scope: &SubgraphDeclaration,
) -> crate::Result<(LocalKey, Vec<u32>)> {
    let passes = &definition.passes[scope.pass_range.clone()];
    let mut mapping = BTreeMap::new();
    let mut resources = Vec::new();
    for resource in passes
        .iter()
        .flat_map(|pass| pass.accesses.iter().map(|access| access.resource_index))
        .chain(
            scope
                .interface
                .inputs
                .iter()
                .chain(&scope.interface.outputs)
                .map(|port| port.resource),
        )
    {
        mapping.entry(resource).or_insert_with(|| {
            resources.push(resource);
            (resources.len() - 1) as u32
        });
    }
    let mut bases = vec![None; resources.len()];
    for access in passes.iter().flat_map(|pass| &pass.accesses) {
        let index = mapping[&access.resource_index] as usize;
        let version = match access.version {
            AccessVersion::Read(version) => version,
            AccessVersion::Write { previous, .. } => previous,
            AccessVersion::Automatic => {
                return Err(crate::Error::invalid_usage(
                    "subgraph has an unresolved access version",
                ));
            }
        };
        bases[index].get_or_insert(version);
    }
    for port in scope
        .interface
        .inputs
        .iter()
        .chain(&scope.interface.outputs)
    {
        let index = mapping[&port.resource] as usize;
        bases[index] = Some(bases[index].map_or(port.version, |base| base.min(port.version)));
    }
    let rebase = |resource: u32, version: u32| {
        version
            .checked_sub(bases[mapping[&resource] as usize].unwrap_or(0))
            .ok_or_else(|| crate::Error::invalid_usage("subgraph version precedes its boundary"))
    };
    let mut local_passes = passes.to_vec();
    let no_name: std::sync::Arc<str> = "".into();
    for pass in &mut local_passes {
        pass.name = no_name.clone();
        pass.graph_id = None;
        for access in &mut pass.accesses {
            access.range = access
                .range
                .normalized(definition.resources[access.resource_index as usize].descriptor)
                .unwrap_or(access.range);
            access.version = match access.version {
                AccessVersion::Read(version) => {
                    AccessVersion::Read(rebase(access.resource_index, version)?)
                }
                AccessVersion::Write { previous, current } => AccessVersion::Write {
                    previous: rebase(access.resource_index, previous)?,
                    current: rebase(access.resource_index, current)?,
                },
                AccessVersion::Automatic => unreachable!(),
            };
            access.resource_index = mapping[&access.resource_index];
        }
    }
    let mut interface = scope.interface.clone();
    for port in interface.inputs.iter_mut().chain(&mut interface.outputs) {
        port.range = port
            .range
            .normalized(definition.resources[port.resource as usize].descriptor)
            .unwrap_or(port.range);
        port.version = rebase(port.resource, port.version)?;
        port.resource = mapping[&port.resource];
    }
    interface.inputs.sort_by_key(port_key);
    interface.inputs.dedup();
    interface.outputs.sort_by_key(port_key);
    interface.outputs.dedup();
    let mut local_resources = resources
        .iter()
        .map(|&resource| definition.resources[resource as usize].clone())
        .collect::<Vec<_>>();
    for resource in &mut local_resources {
        resource.name = String::new();
    }
    let codes: BTreeSet<_> = local_passes
        .iter()
        .flat_map(|pass| pass.accesses.iter().map(|access| access.access_code))
        .chain(
            interface
                .inputs
                .iter()
                .chain(&interface.outputs)
                .map(|port| port.access_code),
        )
        .collect();
    let semantics = definition
        .access_semantics
        .iter()
        .filter(|semantic| codes.contains(&semantic.code))
        .copied()
        .collect();
    Ok((
        LocalKey {
            resources: local_resources,
            passes: local_passes,
            interface,
            semantics,
            context: definition.context,
            compiler_version: definition.compiler_version,
        },
        resources,
    ))
}

fn port_key(port: &SubgraphPort) -> (u32, u32, u8, u8, [u64; 5]) {
    let (kind, range) = match port.range {
        AccessRange::Whole => (0, [0; 5]),
        AccessRange::Buffer { offset, size } => (1, [offset, size, 0, 0, 0]),
        AccessRange::Texture {
            aspect_mask,
            base_mip_level,
            level_count,
            base_array_layer,
            layer_count,
        } => (
            2,
            [
                u64::from(aspect_mask),
                u64::from(base_mip_level),
                u64::from(level_count),
                u64::from(base_array_layer),
                u64::from(layer_count),
            ],
        ),
    };
    (port.resource, port.version, port.access_code, kind, range)
}
