use super::*;
/// Namespace of the cache the pre-existing tests pretend to run in.
const NS: u64 = 1;
/// Namespace of a different cache.
const FOREIGN: u64 = 2;
fn key(
    request: ResultRequest,
    generation: u64,
    descriptor: Descriptor,
    sharing: crate::tracking::ResourceSharing,
    final_access: u8,
    enabled: bool,
) -> crate::Result<Option<ResultKey>> {
    super::key(request, NS, generation, descriptor, sharing, final_access, enabled)
}
fn lookup(
    handle: ResourceHandle,
    hit: bool,
    version: u64,
    produced: SyncSet,
    node: ResultNodeId,
    key: Option<&ResultKey>,
) -> ResultLookup {
    super::lookup(handle, hit, version, produced, NS, node, key)
}
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
    ResourceSpec::Buffer(crate::resource::buffer::BufferDesc::new_gpu_only(
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
            crate::tracking::ResourceSharing::Exclusive,
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
        crate::tracking::ResourceSharing::Exclusive,
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
        crate::tracking::ResourceSharing::Exclusive,
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
            crate::tracking::ResourceSharing::Exclusive,
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
            crate::tracking::ResourceSharing::Exclusive,
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
            crate::tracking::ResourceSharing::Exclusive,
            0x25,
            true
        )
        .unwrap()
    );
    let concurrent = crate::tracking::ResourceSharing::Concurrent(
        crate::tracking::QueueFamilies::new(&[2, 4]).unwrap(),
    );
    let exclusive_key = key(
        request(ResultDependencies::Complete(dependencies())),
        9,
        descriptor(),
        crate::tracking::ResourceSharing::Exclusive,
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
        crate::tracking::ResourceSharing::Exclusive,
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
        crate::tracking::ResourceSharing::Exclusive,
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
        crate::tracking::ResourceSharing::Exclusive,
        0x25,
        true,
    )
    .unwrap()
    .unwrap();
    assert!(child.inputs.contains(&DependencyVersion {
        id: DependencyId::External(8),
        version: 2,
        namespace: 0
    }));
    assert!(child.inputs.contains(&DependencyVersion {
        id: DependencyId::Result(ResultNodeId(1)),
        version: 11,
        namespace: NS
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
            crate::tracking::ResourceSharing::Exclusive,
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
            crate::tracking::ResourceSharing::Exclusive,
            0x25,
            true
        )
        .unwrap()
        .is_none()
    );
}

fn result_dependency(namespace: u64, node: u64, version: u64) -> ContentDependency {
    ContentDependency {
        value: DependencyVersion {
            id: DependencyId::Result(ResultNodeId(node)),
            version,
            namespace,
        },
        lineage: Vec::new(),
        cacheable: true,
    }
}
fn key_in(namespace: u64, dependency: ContentDependency) -> Option<ResultKey> {
    super::key(
        request(ResultDependencies::Complete(vec![dependency])),
        namespace,
        9,
        descriptor(),
        crate::tracking::ResourceSharing::Exclusive,
        0x25,
        true,
    )
    .unwrap()
}
#[test]
fn dependencies_issued_by_another_cache_are_never_cacheable_here() {
    assert!(key_in(NS, result_dependency(NS, 5, 1)).is_some());
    assert!(
        key_in(NS, result_dependency(FOREIGN, 5, 1)).is_none(),
        "node 5 / version 1 of another cache is not this cache's node 5 / version 1"
    );
    let mut nested = result_dependency(NS, 6, 1);
    nested.lineage.push(DependencyVersion {
        id: DependencyId::Result(ResultNodeId(7)),
        version: 1,
        namespace: FOREIGN,
    });
    assert!(key_in(NS, nested).is_none(), "foreign lineage entries count too");
    // External dependencies are versioned by the host and valid in every cache.
    assert!(key_in(NS, ContentDependency::external(3, 1)).is_some());
    assert!(key_in(FOREIGN, ContentDependency::external(3, 1)).is_some());
}
#[test]
fn identical_requests_in_different_caches_have_different_keys() {
    let a = key_in(NS, ContentDependency::external(3, 1)).unwrap();
    let b = key_in(FOREIGN, ContentDependency::external(3, 1)).unwrap();
    assert_ne!(a, b);
    assert_eq!(a, key_in(NS, ContentDependency::external(3, 1)).unwrap());
}
#[test]
fn every_cache_gets_its_own_nonzero_namespace() {
    let (a, b) = (ResultCache::new(1, 1), ResultCache::new(1, 1));
    assert!(a.namespace != 0 && b.namespace != 0);
    assert_ne!(a.namespace, b.namespace);
}
#[test]
fn invalidation_only_reaches_dependencies_of_its_own_namespace() {
    let mut cache = ResultCache::new(8, 1 << 20);
    let own = cache.namespace;
    let store = |cache: &mut ResultCache, node: u64, namespace: u64| {
        let payload = Arc::new(ResultKeyPayload {
            namespace: own,
            node: ResultNodeId(node),
            device_generation: 9,
            inputs: vec![DependencyVersion {
                id: DependencyId::Result(ResultNodeId(5)),
                version: 1,
                namespace,
            }],
            parameters: Vec::new(),
            shader_implementation: 0,
            shader_interface: 0,
            descriptor: descriptor(),
            sharing: crate::tracking::ResourceSharing::Exclusive,
            final_access: 0x25,
        });
        let bytes = payload.bytes();
        let output = CachedOutput {
            owned: OwnedOutput {
                token: node,
                resource: None,
                retirement: cache.retired.clone(),
            },
            version: 1,
            produced: SyncSet::EMPTY,
        };
        assert!(matches!(
            cache.cache.insert(payload, output, bytes),
            Inserted::Stored(_)
        ));
    };
    store(&mut cache, 10, own);
    store(&mut cache, 11, FOREIGN);
    cache.invalidate(DependencyId::Result(ResultNodeId(5)), 2);
    assert_eq!(cache.cache.len(), 1);
    assert_eq!(cache.counters.invalidations, 1);
    assert!(
        cache.cache.iter().all(|(key, _)| key.node == ResultNodeId(11)),
        "only the entry depending on the foreign node 5 survives"
    );
}
