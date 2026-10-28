//! Instantiation of a compiled plan into a frame with storage.
use crate::rg::resource_registry::Buffer;
use crate::rg::resource_registry::RegistryResource;
use crate::rg::resource_registry::Texture;
use super::Pool;
use super::Registry;
use super::backend::ResourceBackend;
use super::backend::populate_resources_into;
use super::declaration::AccessDependency;
use super::declaration::GraphDeclaration;
use super::declaration::OriginSource;
use super::declaration::ResourceDeclaration;
use super::snapshot::FrameInstance;
use super::snapshot::FrameInstanceTables;
use super::snapshot::PhysicalMapping;
use super::snapshot::Snapshot;

pub fn compile_graph(
    graph: GraphDeclaration,
    pool: &mut Pool,
    backend: &mut impl ResourceBackend,
    dependency: &impl AccessDependency,
) -> crate::Result<FrameInstance> {
    let definition = super::super::plan::GraphDefinition::from_declaration(
        &graph,
        dependency,
        super::super::plan::PlanContext::default(),
    )?;
    let plan = std::sync::Arc::new(super::super::plan::compile_definition(definition)?);
    instantiate_validated_plan(plan, graph, pool, backend, dependency)
}

/// Bind physical resources and dynamic import states to an immutable plan.
/// The complete normalized declaration must match, even if its hash collides.
pub fn instantiate_plan(
    plan: std::sync::Arc<super::super::plan::CompiledPlan>,
    graph: GraphDeclaration,
    pool: &mut Pool,
    backend: &mut impl ResourceBackend,
    dependency: &impl AccessDependency,
) -> crate::Result<FrameInstance> {
    if plan.definition.context.device_generation != 0
        && backend.device_generation().map(|g| g.value())
            != Some(plan.definition.context.device_generation)
    {
        return Err(crate::Error::invalid_usage(
            "compiled plan belongs to a different device generation",
        ));
    }
    let definition = super::super::plan::GraphDefinition::from_declaration(
        &graph,
        dependency,
        plan.definition.context,
    )?;
    if definition != plan.definition {
        return Err(crate::Error::invalid_usage(
            "frame declaration differs from compiled plan",
        ));
    }
    if super::super::validation_enabled() {
        super::super::plan_check::check_plan(&definition, &plan)?;
    }
    instantiate_validated_plan(plan, graph, pool, backend, dependency)
}

pub(crate) fn instantiate_validated_plan(
    plan: std::sync::Arc<super::super::plan::CompiledPlan>,
    graph: GraphDeclaration,
    pool: &mut Pool,
    backend: &mut impl ResourceBackend,
    dependency: &impl AccessDependency,
) -> crate::Result<FrameInstance> {
    let storage = backend
        .frame_storage_pool()
        .map(|pool| pool.checkout(&plan))
        .transpose()?;
    instantiate_with_storage(plan, graph, pool, backend, dependency, storage)
}

pub(crate) fn instantiate_with_storage(
    plan: std::sync::Arc<super::super::plan::CompiledPlan>,
    mut graph: GraphDeclaration,
    pool: &mut Pool,
    backend: &mut impl ResourceBackend,
    _dependency: &impl AccessDependency,
    mut storage: Option<super::super::frame_storage::FrameStorageLease>,
) -> crate::Result<FrameInstance> {
    let mut original = 0usize;
    let mut retained = plan.retained_passes.iter().copied().peekable();
    graph.passes.retain_mut(|pass| {
        let index = original;
        original += 1;
        if retained.peek() != Some(&index) {
            return false;
        }
        retained.next();
        pass.accesses
            .copy_from_slice(&plan.definition.passes[index].accesses);
        true
    });
    backend.prepare_transient_resources(&plan, &graph)?;
    let mut tables = storage.as_mut().map_or_else(
        super::super::frame_storage::FrameTables::default,
        super::super::frame_storage::FrameStorageLease::take_tables,
    );
    // Public CPU backends without a recycler use the same allocation/failure
    // path. Vulkan has already admitted and reserved these exact table sizes.
    if storage.is_none() {
        tables.reserve_plan(&plan)?;
    }
    let info = plan.info.clone();
    let mut frame = FrameInstance {
        tables: FrameInstanceTables {
            snapshot: Snapshot {
                shared: plan.snapshot.clone(),
                physical_mapping: tables.physical_mapping,
                pool_hits: tables.pool_hits,
            },
            registry: tables.registry,
            history_validity: Some(tables.history_validity),
        },
        storage,
        plan,
        declarations: graph,
        info,
    };
    if let Err(error) = populate_resources_into(
        &frame.info,
        &frame.declarations.resources,
        pool,
        backend,
        &mut frame.tables.registry,
        &mut frame.tables.snapshot.pool_hits,
    ) {
        reclaim_failed_resources(
            &mut frame.tables.registry,
            &frame.declarations.resources,
            &frame.tables.snapshot.pool_hits,
            pool,
            backend,
        );
        return Err(error);
    }
    let registry = &mut frame.tables.registry;
    let validity = frame
        .tables
        .history_validity
        .as_mut()
        .expect("frame validity table");
    validity.resize(frame.declarations.resources.len(), None);
    for (logical, mapping) in registry.resources.iter().enumerate() {
        if let Some(payload) = registry
            .backing_resources
            .get(mapping.backing_index as usize)
            && let (_, Some(token)) = payload_projection(payload)
        {
            validity[logical] = backend.history_validity(token);
        }
    }
    for (logical, mapping) in registry.resources.iter().enumerate() {
        let (payload_tag, token) = registry
            .backing_resources
            .get(mapping.backing_index as usize)
            .map_or((255, None), payload_projection);
        frame
            .tables
            .snapshot
            .physical_mapping
            .push(PhysicalMapping {
                logical: logical as u32,
                physical: mapping.backing_index,
                reused: mapping.reused,
                initial_access: registry.resource_initial_access[logical],
                payload_tag,
                token,
            });
    }
    Ok(frame)
}

/// 失败路径归还：本图 Create 资源中取自资源池的放回资源池，
/// 本次新建的经 [`ResourceBackend::release_created_resource`] 交还后端；导入资源不动。
/// 已归还的物理槽置为 `Empty`，避免同一资源被再次取出。只能在 GPU 不会再使用这些资源时调用。
pub fn reclaim_failed_resources(
    registry: &mut Registry,
    declarations: &[ResourceDeclaration],
    pool_hits: &[u32],
    pool: &mut Pool,
    backend: &mut impl ResourceBackend,
) {
    for (logical, declaration) in declarations.iter().enumerate() {
        if !matches!(declaration.source, OriginSource::Create) {
            continue;
        }
        let Some(reuse) = registry.resources.get(logical) else {
            continue;
        };
        let Some(slot) = registry
            .backing_resources
            .get_mut(usize::from(reuse.backing_index))
        else {
            continue;
        };
        let pooled = pool_hits.contains(&(logical as u32));
        match std::mem::replace(slot, RegistryResource::Empty) {
            RegistryResource::OwnedTexture(texture) if pooled => {
                let token = texture.handle;
                pool.insert_texture(texture);
                backend.park_pooled(token);
            }
            RegistryResource::OwnedBuffer(buffer) if pooled => {
                let token = buffer.handle;
                pool.insert_buffer(buffer);
                backend.park_pooled(token);
            }
            RegistryResource::OwnedTexture(Texture { handle, .. })
            | RegistryResource::OwnedBuffer(Buffer { handle, .. }) => {
                backend.release_created_resource(handle)
            }
            other => *slot = other,
        }
    }
}

pub fn payload_projection(resource: &RegistryResource<u64, u64, ()>) -> (u8, Option<u64>) {
    match resource {
        RegistryResource::OwnedTexture(texture) => (0, Some(texture.handle)),
        RegistryResource::UnownedTexture(texture) => (1, Some(texture.handle)),
        RegistryResource::OwnedBuffer(buffer) => (2, Some(buffer.handle)),
        RegistryResource::UnownedBuffer(buffer) => (3, Some(buffer.handle)),
        RegistryResource::Other(_) => (4, None),
        RegistryResource::Empty => (5, None),
    }
}
