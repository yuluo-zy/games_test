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
