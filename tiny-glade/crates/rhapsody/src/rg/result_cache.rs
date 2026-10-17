//! Content-versioned persistent results, owned by a graph session, never a plan.
use super::{
    cache::{BoundedCache, CacheDiagnostics, CacheReason, Inserted},
    graph::{Descriptor, GraphDeclaration, OriginSource, RenderGraph},
    resource::{GraphId, ResourceHandle},
    vulkan::{GpuResource, ResourceSpec, VulkanGraphSession},
};
use crate::sync::{CompletedValues, SyncSet};
use std::{
    collections::{BTreeSet, HashMap},
    ops::Range,
    sync::Arc,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResultNodeId(pub u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DependencyId {
    External(u64),
    Result(ResultNodeId),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct DependencyVersion {
    id: DependencyId,
    version: u64,
}
/// Result dependencies carry their transitive provenance. Evicting a parent
/// result therefore cannot sever a cached child's invalidation path.
#[derive(Clone, Debug)]
pub struct ContentDependency {
    value: DependencyVersion,
    lineage: Vec<DependencyVersion>,
    cacheable: bool,
}
impl ContentDependency {
    pub fn external(id: u64, version: u64) -> Self {
        Self {
            value: DependencyVersion {
                id: DependencyId::External(id),
                version,
            },
            lineage: Vec::new(),
            cacheable: true,
        }
    }
    pub fn id(&self) -> DependencyId {
        self.value.id
    }
    pub fn version(&self) -> u64 {
        self.value.version
    }
}
#[derive(Clone, Debug, Default)]
pub enum ResultDependencies {
    Complete(Vec<ContentDependency>),
    #[default]
    Unknown,
}
#[derive(Clone, Debug)]
pub struct ResultRequest {
    pub node: ResultNodeId,
    pub dependencies: ResultDependencies,
    pub parameters: Vec<u8>,
    pub shader_implementation: u64,
    pub shader_interface: u64,
}
#[derive(Clone, Debug)]
pub struct ResultLookup {
    pub handle: ResourceHandle,
    pub hit: bool,
    pub content_version: u64,
    /// A real submitted completion set, empty only for a new pending production.
    pub produced: SyncSet,
    dependency: ContentDependency,
}
impl ResultLookup {
    pub fn dependency(&self) -> ContentDependency {
        self.dependency.clone()
    }
}
// The cache's map and LRU share this immutable payload. Cloning a publication
// key after submission only increments the Arc count; its nested vectors were
// allocated while building the frame, before any GPU work can be submitted.
type ResultKey = Arc<ResultKeyPayload>;
#[derive(Debug, PartialEq, Eq, Hash)]
struct ResultKeyPayload {
    node: ResultNodeId,
    device_generation: u64,
    inputs: Vec<DependencyVersion>,
    parameters: Vec<u8>,
    shader_implementation: u64,
    shader_interface: u64,
    descriptor: Descriptor,
    sharing: crate::resource::ResourceSharing,
    final_access: u8,
}
impl ResultKeyPayload {
    fn bytes(&self) -> usize {
        std::mem::size_of::<Self>()
            + self.inputs.len() * std::mem::size_of::<DependencyVersion>()
            + self.parameters.len()
    }
}
struct RetiredOutput {
    token: u64,
    resource: GpuResource,
}
type Retired = Arc<parking_lot::Mutex<Vec<RetiredOutput>>>;
struct OwnedOutput {
    token: u64,
    resource: Option<GpuResource>,
    retirement: Retired,
}
impl OwnedOutput {
    fn resource(&self) -> &GpuResource {
        self.resource
            .as_ref()
            .expect("live result owns its resource")
    }
}
impl Drop for OwnedOutput {
    fn drop(&mut self) {
        if let Some(resource) = self.resource.take() {
            self.retirement.lock().push(RetiredOutput {
                token: self.token,
                resource,
            });
        }
    }
}
struct CachedOutput {
    owned: OwnedOutput,
    version: u64,
    produced: SyncSet,
}
struct PendingProduction {
    key: Option<ResultKey>,
    owned: OwnedOutput,
    handle: ResourceHandle,
    producers: Range<usize>,
    version: u64,
}
#[derive(Clone)]
struct HitBinding {
    key: ResultKey,
    lookup: ResultLookup,
    token: u64,
}
#[derive(Default)]
struct PendingFrame {
    productions: Vec<PendingProduction>,
    hits: Vec<HitBinding>,
    invalidated: bool,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ResultCacheStats {
    pub diagnostics: CacheDiagnostics,
    pub hits: u64,
    pub misses: u64,
    pub bypassed: u64,
    pub productions: u64,
    pub publications: u64,
    pub invalidations: u64,
    pub evictions: u64,
    pub entries: usize,
    pub bytes: usize,
    pub pending: usize,
    pub retired: usize,
}
pub struct ResultCache {
    cache: BoundedCache<ResultKey, CachedOutput>,
    retired: Retired,
    frames: HashMap<GraphId, PendingFrame>,
    enabled: bool,
    next_version: u64,
    counters: ResultCacheStats,
}
impl Default for ResultCache {
    fn default() -> Self {
        Self::new(128, 64 * 1024 * 1024)
    }
}
impl ResultCache {
    pub fn new(entries: usize, bytes: usize) -> Self {
        Self {
            cache: BoundedCache::new(entries.max(1), bytes.max(1)),
            retired: Default::default(),
            frames: HashMap::new(),
            enabled: true,
            next_version: 1,
            counters: Default::default(),
        }
    }
    pub fn stats(&self) -> ResultCacheStats {
        let mut diagnostics = self.counters.diagnostics;
        diagnostics.merge_invalidations(self.cache.stats().diagnostics);
        ResultCacheStats {
            diagnostics,
            evictions: self.counters.evictions + self.cache.stats().evictions,
            entries: self.cache.len(),
            bytes: self.cache.bytes(),
            pending: self
                .frames
                .values()
                .map(|frame| frame.productions.len())
                .sum(),
            retired: self.retired.lock().len(),
            ..self.counters
        }
    }
    pub fn clear(&mut self) {
        self.cache.clear();
    }
    fn set_limits(&mut self, entries: usize, bytes: usize) {
        self.counters.evictions += self.cache.stats().evictions;
        self.counters
            .diagnostics
            .merge_invalidations(self.cache.stats().diagnostics);
        self.counters
            .diagnostics
            .invalidate(CacheReason::CapacityChanged, self.cache.len());
        self.cache = BoundedCache::new(entries.max(1), bytes.max(1));
    }
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.cache.clear_with_reason(CacheReason::Disabled);
        }
    }
    pub(crate) fn abort(&mut self, graph: GraphId) {
        if let Some(frame) = self.frames.remove(&graph) {
            self.counters
                .diagnostics
                .invalidate(CacheReason::AbortedFrame, frame.productions.len());
        }
    }
    pub(crate) fn abandon_pending(&mut self) {
        self.counters.diagnostics.invalidate(
            CacheReason::AbortedFrame,
            self.frames
                .values()
                .map(|frame| frame.productions.len())
                .sum(),
        );
        self.frames.clear();
    }
    pub(crate) fn has_work(&self) -> bool {
        !self.frames.is_empty() || !self.cache.is_empty() || !self.retired.lock().is_empty()
    }
    pub(crate) fn evict_completed(&mut self, completed: &CompletedValues) {
        let keys: Vec<_> = self
            .cache
            .iter()
            .filter(|(_, output)| resource_use(output.owned.resource()).is_reached(completed))
            .map(|(key, _)| key.clone())
            .collect();
        for key in keys {
            self.cache.remove_with_reason(&key, CacheReason::Pressure);
        }
    }
    pub(crate) fn owns_token(&self, token: u64) -> bool {
        self.cache
            .iter()
            .any(|(_, output)| output.owned.token == token)
            || self.frames.values().any(|frame| {
                frame.hits.iter().any(|hit| hit.token == token)
                    || frame
                        .productions
                        .iter()
                        .any(|production| production.owned.token == token)
            })
    }
    fn reserve(&mut self, graph: GraphId) -> crate::Result<()> {
        if !self.frames.contains_key(&graph) && self.frames.len() >= 8 {
            return Err(crate::Error::invalid_usage(
                "result-cache pending graph budget exhausted",
            ));
        }
        let frame = self.frames.entry(graph).or_default();
        if frame.invalidated {
            return Err(crate::Error::invalid_usage(
                "result frame is already invalidated",
            ));
        }
        if frame.hits.len() + frame.productions.len() >= 128 {
            return Err(crate::Error::invalid_usage(
                "result-cache per-frame request budget exhausted",
            ));
        }
        frame
            .productions
            .try_reserve(1)
            .map_err(|_| crate::Error::OutOfHostMemory)?;
        frame
            .hits
            .try_reserve(1)
            .map_err(|_| crate::Error::OutOfHostMemory)?;
        let pending: usize = self
            .frames
            .values()
            .map(|frame| frame.productions.len())
            .sum();
        self.retired
            .lock()
            .try_reserve(self.cache.len() + pending + 1)
            .map_err(|_| crate::Error::OutOfHostMemory)?;
        Ok(())
    }
    pub(crate) fn prepare(
        &mut self,
        graph: GraphId,
        declaration: &GraphDeclaration,
        generation: u64,
    ) -> crate::Result<()> {
        let Some(frame) = self.frames.get(&graph) else {
            return Ok(());
        };
        if frame.invalidated {
            return Err(crate::Error::invalid_usage(
                "result dependencies changed while this graph was being built",
            ));
        }
        for binding in &frame.hits {
            let resource = declaration
                .resources
                .get(binding.lookup.handle.raw() as usize)
                .ok_or_else(|| {
                    crate::Error::invalid_usage("cached result declaration is missing")
                })?;
            if binding.key.device_generation != generation
                || resource.descriptor != binding.key.descriptor
                || resource.sharing != binding.key.sharing
                || !matches!(resource.source, OriginSource::ImportUnowned { token, .. } if token == binding.token)
            {
                return Err(crate::Error::invalid_usage(
                    "cached result binding was changed",
                ));
            }
            if declaration
                .passes
                .iter()
                .flat_map(|pass| &pass.accesses)
                .any(|access| {
                    access.resource_index == binding.lookup.handle.raw()
                        && crate::raw::access_table::is_write(access.access_code)
                })
            {
                return Err(crate::Error::invalid_usage(
                    "cached results are immutable; write to a separate output",
                ));
            }
        }
        let mut logical = declaration.clone();
        for production in &frame.productions {
            if production.owned.resource().generation().value() != generation {
                return Err(crate::Error::invalid_usage(
                    "pending result belongs to another device generation",
                ));
            }
            let index = production.handle.raw() as usize;
            let resource = logical.resources.get_mut(index).ok_or_else(|| {
                crate::Error::invalid_usage("result output declaration is missing")
            })?;
            if !matches!(resource.source, OriginSource::ImportUnowned { token, .. } if token == production.owned.token)
                || resource.descriptor != production.owned.resource().descriptor()
                || resource.sharing != resource_sharing(production.owned.resource())
            {
                return Err(crate::Error::invalid_usage(
                    "result output binding was changed",
                ));
            }
            resource.source = OriginSource::Create;
            for (pass, declaration) in declaration.passes.iter().enumerate() {
                if !production.producers.contains(&pass)
                    && declaration.accesses.iter().any(|access| {
                        access.resource_index == index as u32
                            && crate::raw::access_table::is_write(access.access_code)
                    })
                {
                    return Err(crate::Error::invalid_usage(
                        "result output is modified outside its producer",
                    ));
                }
            }
        }
        super::resource_contracts::initialized_resources(
            &logical,
            crate::raw::access_table::is_write,
        )?;
        self.cache
            .try_reserve(frame.productions.len())
            .map_err(|_| crate::Error::OutOfHostMemory)?;
        // Every existing/pending owner may retire through clear, a changed
        // budget, replacement, or an aborted frame. Preserve enough slots for
        // all of them before the first submit; OwnedOutput::drop cannot fail.
        let pending: usize = self
            .frames
            .values()
            .map(|frame| frame.productions.len())
            .sum();
        self.retired
            .lock()
            .try_reserve(self.cache.len() + pending)
            .map_err(|_| crate::Error::OutOfHostMemory)?;
        Ok(())
    }
    pub(crate) fn commit(&mut self, graph: GraphId) {
        let Some(frame) = self.frames.remove(&graph) else {
            return;
        };
        for production in frame.productions {
            self.counters.productions += 1;
            let produced = resource_use(production.owned.resource());
            // A culled/empty producer cannot create a published content version.
            if produced.is_empty() {
                continue;
            }
            if let Some(key) = production.key.filter(|_| self.enabled) {
                let bytes = resource_bytes(production.owned.resource())
                    .saturating_add(key.bytes())
                    .saturating_add(2 * std::mem::size_of::<ResultKey>());
                let output = CachedOutput {
                    owned: production.owned,
                    version: production.version,
                    produced,
                };
                if matches!(self.cache.insert(key, output, bytes), Inserted::Stored(_)) {
                    self.counters.publications += 1;
                }
            }
        }
    }
    fn invalidate(&mut self, id: DependencyId, version: u64) {
        let mut sources = vec![(id, Some(version))];
        let mut visited = BTreeSet::new();
        while let Some((source, version)) = sources.pop() {
            if !visited.insert(source) {
                continue;
            }
            let affected = |key: &ResultKey| {
                key.inputs.iter().any(|input| {
                    input.id == source && version.is_none_or(|version| input.version != version)
                })
            };
            for frame in self.frames.values_mut() {
                frame.invalidated |= frame.hits.iter().any(|hit| affected(&hit.key))
                    || frame
                        .productions
                        .iter()
                        .filter_map(|production| production.key.as_ref())
                        .any(affected);
            }
            let keys: Vec<_> = self.cache.iter().filter(|(key, output)| affected(key)
                || matches!(source, DependencyId::Result(node) if key.node == node && version.is_none_or(|version| output.version != version)))
                .map(|(key, _)| key.clone()).collect();
            for key in keys {
                sources.push((DependencyId::Result(key.node), None));
                self.cache
                    .remove_with_reason(&key, CacheReason::DependencyVersion);
                self.counters.invalidations += 1;
            }
        }
    }
}

fn resource_use(resource: &GpuResource) -> SyncSet {
    match resource {
        GpuResource::Buffer(buffer) => buffer.last_use.snapshot(),
        GpuResource::Image(image) => image.last_use.snapshot(),
    }
}
fn resource_bytes(resource: &GpuResource) -> usize {
    let bytes = match resource {
        GpuResource::Buffer(buffer) => buffer.allocation_bytes(),
        GpuResource::Image(image) => image.allocation_bytes(),
    };
    usize::try_from(bytes).unwrap_or(usize::MAX)
}
fn resource_sharing(resource: &GpuResource) -> crate::resource::ResourceSharing {
    match resource {
        GpuResource::Buffer(buffer) => buffer.sharing,
        GpuResource::Image(image) => image.sharing,
    }
}
fn mark_cached(resource: &GpuResource, completed: &CompletedValues) {
    match resource {
        GpuResource::Buffer(buffer) => buffer
            .device
            .allocator_stats
            .mark_cached(buffer.allocation_id(), completed),
        GpuResource::Image(image) => image
            .device
            .allocator_stats
            .mark_cached(image.allocation_id(), completed),
    }
}
fn result_key_difference(old: &ResultKeyPayload, new: &ResultKeyPayload) -> CacheReason {
    if old.device_generation != new.device_generation {
        CacheReason::DeviceGeneration
    } else if old.descriptor != new.descriptor {
        CacheReason::Descriptor
    } else if old.sharing != new.sharing {
        CacheReason::Sharing
    } else if old.inputs != new.inputs {
        CacheReason::DependencyVersion
    } else if old.parameters != new.parameters {
        CacheReason::Parameters
    } else if old.shader_implementation != new.shader_implementation {
        CacheReason::ShaderImplementation
    } else if old.shader_interface != new.shader_interface {
        CacheReason::ShaderInterface
    } else {
        CacheReason::GraphStructure
    }
}
fn key(
    request: ResultRequest,
    generation: u64,
    descriptor: Descriptor,
    sharing: crate::resource::ResourceSharing,
    final_access: u8,
    enabled: bool,
) -> crate::Result<Option<ResultKey>> {
    if usize::from(final_access) >= crate::raw::access_table::ACCESS_CODE_COUNT {
        return Err(crate::Error::invalid_usage(
            "result final access code is invalid",
        ));
    }
    if matches!(descriptor, Descriptor::Image(_)) {
        crate::raw::resource_usage::image_access_mask_to_usage_flags(
            crate::raw::access_table::access_info(final_access)
                .unwrap()
                .access,
        )?;
    }
    let ResultDependencies::Complete(dependencies) = request.dependencies else {
        return Ok(None);
    };
    if !enabled
        || request.parameters.len() > 65536
        || dependencies.iter().any(|dependency| !dependency.cacheable)
    {
        return Ok(None);
    }
    let mut inputs = BTreeSet::new();
    for dependency in dependencies {
        for value in std::iter::once(dependency.value).chain(dependency.lineage) {
            inputs.insert(value);
            if inputs.len() > 1024 {
                return Ok(None);
            }
        }
    }
    if inputs.len() > 1024 {
        return Ok(None);
    }
    Ok(Some(Arc::new(ResultKeyPayload {
        node: request.node,
        device_generation: generation,
        inputs: inputs.into_iter().collect(),
        parameters: request.parameters,
        shader_implementation: request.shader_implementation,
        shader_interface: request.shader_interface,
        descriptor,
        sharing,
        final_access,
    })))
}
fn lookup(
    handle: ResourceHandle,
    hit: bool,
    version: u64,
    produced: SyncSet,
    node: ResultNodeId,
    key: Option<&ResultKey>,
) -> ResultLookup {
    ResultLookup {
        handle,
        hit,
        content_version: version,
        produced,
        dependency: ContentDependency {
            value: DependencyVersion {
                id: DependencyId::Result(node),
                version,
            },
            lineage: key.map_or_else(Vec::new, |key| key.inputs.clone()),
            cacheable: key.is_some(),
        },
    }
}

impl VulkanGraphSession {
    /// Reuse a persistent immutable GPU result or build its complete producer.
    /// Complete dependencies must include every actual influence, including
    /// camera/light/history versions; Unknown always executes and never caches.
    pub fn cached_result(
        &mut self,
        graph: &mut RenderGraph,
        request: ResultRequest,
        spec: ResourceSpec,
        final_access: u8,
        build: impl FnOnce(
            &mut VulkanGraphSession,
            &mut RenderGraph,
            ResourceHandle,
        ) -> crate::Result<()>,
    ) -> crate::Result<ResultLookup> {
        if self.is_poisoned() || self.backend.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        let graph_id = graph.graph_id();
        let node = request.node;
        let bypass_reason = if !self.result_cache.enabled {
            CacheReason::Disabled
        } else if matches!(&request.dependencies, ResultDependencies::Unknown)
            || matches!(&request.dependencies, ResultDependencies::Complete(inputs) if inputs.iter().any(|input| !input.cacheable))
        {
            CacheReason::UnknownDependencies
        } else {
            CacheReason::MetadataLimit
        };
        let key = key(
            request,
            self.backend.device.generation().value(),
            spec.descriptor(),
            spec.sharing(),
            final_access,
            self.result_cache.enabled,
        )?;
        self.result_cache.reserve(graph_id)?;
        if let Some(key) = &key {
            let frame = &self.result_cache.frames[&graph_id];
            if let Some(hit) = frame.hits.iter().find(|hit| &hit.key == key) {
                return Ok(hit.lookup.clone());
            }
            if let Some(pending) = frame
                .productions
                .iter()
                .find(|pending| pending.key.as_ref() == Some(key))
            {
                return Ok(lookup(
                    pending.handle,
                    false,
                    pending.version,
                    SyncSet::EMPTY,
                    node,
                    Some(key),
                ));
            }
            if let Some(output) = self.result_cache.cache.get(key) {
                let (token, version, produced) =
                    (output.owned.token, output.version, output.produced);
                let handle = graph.import_unowned_with_sharing(
                    format!(
                        "result:{}:{}",
                        node.0,
                        self.result_cache.frames[&graph_id].hits.len()
                            + self.result_cache.frames[&graph_id].productions.len()
                    ),
                    spec.descriptor(),
                    token,
                    final_access,
                    spec.sharing(),
                )?;
                let result = lookup(handle, true, version, produced, node, Some(key));
                self.result_cache
                    .frames
                    .get_mut(&graph_id)
                    .unwrap()
                    .hits
                    .push(HitBinding {
                        key: key.clone(),
                        lookup: result.clone(),
                        token,
                    });
                self.result_cache.counters.hits += 1;
                return Ok(result);
            }
        }
        self.result_cache.counters.misses += 1;
        let reason = key.as_ref().map_or(bypass_reason, |key| {
            self.result_cache
                .cache
                .iter()
                .filter(|(old, _)| old.node == key.node)
                .map(|(old, _)| result_key_difference(old, key))
                .min_by_key(|reason| *reason as usize)
                .unwrap_or(CacheReason::Cold)
        });
        self.result_cache.counters.diagnostics.miss(reason);
        self.result_cache.counters.bypassed += u64::from(key.is_none());
        let version = self.result_cache.next_version;
        self.result_cache.next_version = version
            .checked_add(1)
            .ok_or_else(|| crate::Error::invalid_usage("result content version overflow"))?;
        let resource = spec.create_resource(self.backend.device.clone(), 0)?;
        let token = self.backend.bind_import(resource.clone())?;
        let owned = OwnedOutput {
            token,
            resource: Some(resource),
            retirement: self.result_cache.retired.clone(),
        };
        let exports = graph.declaration.exports.len();
        let start = graph.declaration.passes.len();
        let scopes = graph.declaration.subgraphs.len();
        let batch_per_pass = graph.declaration.batch_per_pass;
        let handle = graph.import_unowned_with_sharing(
            format!(
                "result:{}:{}",
                node.0,
                self.result_cache.frames[&graph_id].hits.len()
                    + self.result_cache.frames[&graph_id].productions.len()
            ),
            spec.descriptor(),
            token,
            0,
            spec.sharing(),
        )?;
        self.result_cache
            .frames
            .get_mut(&graph_id)
            .unwrap()
            .invalidated = true;
        if let Err(error) = build(self, graph, handle)
            .and_then(|()| graph.export(handle, final_access).map_err(Into::into))
        {
            graph.declaration.passes.truncate(start);
            graph.callbacks.truncate(start);
            graph.declaration.exports.truncate(exports);
            graph.declaration.subgraphs.truncate(scopes);
            graph.declaration.batch_per_pass = batch_per_pass;
            self.result_cache
                .frames
                .get_mut(&graph_id)
                .unwrap()
                .invalidated = true;
            return Err(error);
        }
        self.result_cache
            .frames
            .get_mut(&graph_id)
            .unwrap()
            .invalidated = false;
        let result = lookup(handle, false, version, SyncSet::EMPTY, node, key.as_ref());
        self.result_cache
            .frames
            .get_mut(&graph_id)
            .unwrap()
            .productions
            .push(PendingProduction {
                key,
                owned,
                handle,
                producers: start..graph.declaration.passes.len(),
                version,
            });
        Ok(result)
    }
    pub fn result_cache_stats(&self) -> ResultCacheStats {
        self.result_cache.stats()
    }
    pub fn set_result_cache_enabled(&mut self, enabled: bool) {
        self.result_cache.set_enabled(enabled);
    }
    pub fn clear_result_cache(&mut self) {
        self.result_cache.clear();
    }
    pub fn set_result_cache_limits(&mut self, entries: usize, bytes: usize) {
        self.result_cache.set_limits(entries, bytes);
    }
    pub fn invalidate_result_dependency(&mut self, id: DependencyId, version: u64) {
        self.result_cache.invalidate(id, version);
    }
    pub(crate) fn collect_cached_results(
        &mut self,
        completed: &CompletedValues,
    ) -> crate::Result<()> {
        for (_, output) in self.result_cache.cache.iter() {
            mark_cached(output.owned.resource(), completed);
        }
        let pinned: BTreeSet<_> = self
            .result_cache
            .frames
            .values()
            .flat_map(|frame| {
                frame.hits.iter().map(|hit| hit.token).chain(
                    frame
                        .productions
                        .iter()
                        .map(|production| production.owned.token),
                )
            })
            .collect();
        // Keep the queue allocation: moving it out and extending a fresh Vec
        // discarded the spare slots reserved for still-live cache owners.
        let retirement = self.result_cache.retired.clone();
        let mut retired = retirement.lock();
        let mut index = 0;
        while let Some(output) = retired.get(index) {
            if pinned.contains(&output.token)
                || !resource_use(&output.resource).is_reached(completed)
                || self.result_import_in_flight(output.token)
            {
                index += 1;
            } else {
                self.release_import(output.token)?;
                retired.swap_remove(index);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn request(dependencies: ResultDependencies) -> ResultRequest {
        ResultRequest {
            node: ResultNodeId(1),
            dependencies,
            parameters: vec![1, 2],
            shader_implementation: 3,
            shader_interface: 4,
        }
    }
    fn descriptor() -> Descriptor {
        ResourceSpec::Buffer(crate::raw::buffer::BufferDesc::new_gpu_only(
            64,
            ash::vk::BufferUsageFlags::TRANSFER_SRC,
        ))
        .descriptor()
    }
    #[test]
    fn result_miss_reasons_compare_the_real_complete_key() {
        let request = || {
            request(ResultDependencies::Complete(vec![
                ContentDependency::external(3, 1),
            ]))
        };
        let build = |request| {
            key(
                request,
                9,
                descriptor(),
                crate::resource::ResourceSharing::Exclusive,
                0x25,
                true,
            )
            .unwrap()
            .unwrap()
        };
        let old = build(request());
        let mut changed = request();
        changed.parameters.push(9);
        assert_eq!(
            result_key_difference(&old, &build(changed)),
            CacheReason::Parameters
        );
        let mut changed = request();
        changed.shader_implementation += 1;
        assert_eq!(
            result_key_difference(&old, &build(changed)),
            CacheReason::ShaderImplementation
        );
        let mut changed = request();
        changed.shader_interface += 1;
        assert_eq!(
            result_key_difference(&old, &build(changed)),
            CacheReason::ShaderInterface
        );
        let mut changed = request();
        changed.dependencies =
            ResultDependencies::Complete(vec![ContentDependency::external(3, 2)]);
        assert_eq!(
            result_key_difference(&old, &build(changed)),
            CacheReason::DependencyVersion
        );
        let changed = key(
            request(),
            10,
            descriptor(),
            crate::resource::ResourceSharing::Exclusive,
            0x25,
            true,
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            result_key_difference(&old, &changed),
            CacheReason::DeviceGeneration
        );
    }
    #[test]
    fn complete_keys_include_all_provenance_and_policy_and_normalize_dependency_order() {
        let dependencies = || {
            vec![
                ContentDependency::external(2, 4),
                ContentDependency::external(1, 3),
            ]
        };
        let reference = key(
            request(ResultDependencies::Complete(dependencies())),
            9,
            descriptor(),
            crate::resource::ResourceSharing::Exclusive,
            0x25,
            true,
        )
        .unwrap()
        .unwrap();
        let mut reversed = dependencies();
        reversed.reverse();
        reversed.push(ContentDependency::external(1, 3));
        assert_eq!(
            Some(reference.clone()),
            key(
                request(ResultDependencies::Complete(reversed)),
                9,
                descriptor(),
                crate::resource::ResourceSharing::Exclusive,
                0x25,
                true
            )
            .unwrap()
        );
        let mut shader = request(ResultDependencies::Complete(dependencies()));
        shader.shader_implementation += 1;
        assert_ne!(
            Some(reference.clone()),
            key(
                shader,
                9,
                descriptor(),
                crate::resource::ResourceSharing::Exclusive,
                0x25,
                true
            )
            .unwrap()
        );
        assert_ne!(
            Some(reference),
            key(
                request(ResultDependencies::Complete(dependencies())),
                10,
                descriptor(),
                crate::resource::ResourceSharing::Exclusive,
                0x25,
                true
            )
            .unwrap()
        );
        let concurrent = crate::resource::ResourceSharing::Concurrent(
            crate::resource::QueueFamilies::new(&[2, 4]).unwrap(),
        );
        let exclusive_key = key(
            request(ResultDependencies::Complete(dependencies())),
            9,
            descriptor(),
            crate::resource::ResourceSharing::Exclusive,
            0x25,
            true,
        )
        .unwrap();
        let concurrent_key = key(
            request(ResultDependencies::Complete(dependencies())),
            9,
            descriptor(),
            concurrent,
            0x25,
            true,
        )
        .unwrap();
        assert_ne!(
            exclusive_key, concurrent_key,
            "Vk sharing mode and family list belong to physical result identity"
        );
    }
    #[test]
    fn publication_key_clones_share_the_preallocated_nested_payload() {
        let key = key(
            request(ResultDependencies::Complete(vec![
                ContentDependency::external(1, 2),
            ])),
            9,
            descriptor(),
            crate::resource::ResourceSharing::Exclusive,
            0x25,
            true,
        )
        .unwrap()
        .unwrap();
        let cloned = key.clone();
        assert!(Arc::ptr_eq(&key, &cloned));
        assert_eq!(key.inputs.as_ptr(), cloned.inputs.as_ptr());
        assert_eq!(key.parameters.as_ptr(), cloned.parameters.as_ptr());
        let mut cache = BoundedCache::new(2, 4096);
        cache.try_reserve(1).unwrap();
        cache.insert(key.clone(), 1, key.bytes());
        let stored = cache.iter().next().unwrap().0;
        assert!(Arc::ptr_eq(&key, stored));
        assert_eq!(
            Arc::strong_count(&key),
            4,
            "map and LRU share the same payload"
        );
    }
    #[test]
    fn lineage_keeps_evicted_parent_inputs_and_unknown_parent_never_becomes_cacheable() {
        let parent = key(
            request(ResultDependencies::Complete(vec![
                ContentDependency::external(8, 2),
            ])),
            9,
            descriptor(),
            crate::resource::ResourceSharing::Exclusive,
            0x25,
            true,
        )
        .unwrap()
        .unwrap();
        let parent = lookup(
            ResourceHandle::from_raw(0),
            true,
            11,
            SyncSet::EMPTY,
            ResultNodeId(1),
            Some(&parent),
        );
        let child = key(
            request(ResultDependencies::Complete(vec![parent.dependency()])),
            9,
            descriptor(),
            crate::resource::ResourceSharing::Exclusive,
            0x25,
            true,
        )
        .unwrap()
        .unwrap();
        assert!(child.inputs.contains(&DependencyVersion {
            id: DependencyId::External(8),
            version: 2
        }));
        assert!(child.inputs.contains(&DependencyVersion {
            id: DependencyId::Result(ResultNodeId(1)),
            version: 11
        }));
        let unknown = lookup(
            ResourceHandle::from_raw(0),
            false,
            12,
            SyncSet::EMPTY,
            ResultNodeId(1),
            None,
        );
        assert!(
            key(
                request(ResultDependencies::Complete(vec![unknown.dependency()])),
                9,
                descriptor(),
                crate::resource::ResourceSharing::Exclusive,
                0x25,
                true
            )
            .unwrap()
            .is_none()
        );
        assert!(
            key(
                request(ResultDependencies::Unknown),
                9,
                descriptor(),
                crate::resource::ResourceSharing::Exclusive,
                0x25,
                true
            )
            .unwrap()
            .is_none()
        );
    }
}
