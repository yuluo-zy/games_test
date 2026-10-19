//! Growable descriptor pool ring: growth on exhaustion, bounded pool count,
//! transient whole-pool reuse and persistent per-set reuse.
mod gpu_test_support;
use ash::vk;
use rhapsody::{
    device::{Device, DeviceConfig},
    raw::descriptor_set::{
        DescriptorPoolRing, DescriptorPoolRingConfig, LayoutBinding, LayoutPlan,
        VulkanDescriptorSetLayout,
    },
};
use std::sync::Arc;

fn device() -> Option<Arc<Device>> {
    gpu_test_support::initialize(
        "descriptor pool ring",
        Device::new_headless(&DeviceConfig::headless()),
    )
}
fn plan() -> LayoutPlan {
    LayoutPlan {
        flags: 0,
        binding_flags: vec![0],
        bindings: vec![LayoutBinding {
            binding: 0,
            descriptor_type: vk::DescriptorType::STORAGE_BUFFER.as_raw(),
            descriptor_count: 1,
            stage_flags: vk::ShaderStageFlags::COMPUTE.as_raw(),
            immutable_sampler: None,
        }],
    }
}

#[test]
fn transient_ring_grows_on_exhaustion_and_reuses_reset_pools() {
    let Some(device) = device() else {
        return;
    };
    let layout = VulkanDescriptorSetLayout::new(device.clone(), &plan()).unwrap();
    let ring = DescriptorPoolRing::new(
        device,
        &plan(),
        DescriptorPoolRingConfig {
            sets_per_pool: 2,
            max_pools: 3,
            transient: true,
        },
    )
    .unwrap();
    let sets: Vec<_> = (0..5)
        .map(|_| ring.allocate(layout.clone()).unwrap())
        .collect();
    assert_eq!(ring.stats().pools, 3);
    let sixth = ring.allocate(layout.clone()).unwrap();
    assert!(matches!(
        ring.allocate(layout.clone()),
        Err(rhapsody::Error::Exhausted(_))
    ));
    drop(sets);
    drop(sixth);
    // Dropped sets free whole pools lazily; no new pool is created.
    for _ in 0..6 {
        std::mem::forget(ring.allocate(layout.clone()).unwrap());
    }
    let stats = ring.stats();
    assert_eq!(stats.pools, 3);
    assert_eq!(stats.created, 3);
    assert!(stats.resets >= 3);
}

#[test]
fn persistent_ring_reuses_space_of_individually_freed_sets() {
    let Some(device) = device() else {
        return;
    };
    let layout = VulkanDescriptorSetLayout::new(device.clone(), &plan()).unwrap();
    let ring = DescriptorPoolRing::new(
        device,
        &plan(),
        DescriptorPoolRingConfig {
            sets_per_pool: 2,
            max_pools: 1,
            transient: false,
        },
    )
    .unwrap();
    let first = ring.allocate(layout.clone()).unwrap();
    let _second = ring.allocate(layout.clone()).unwrap();
    assert!(matches!(
        ring.allocate(layout.clone()),
        Err(rhapsody::Error::Exhausted(_))
    ));
    drop(first);
    let _third = ring.allocate(layout).unwrap();
    assert_eq!(ring.stats().pools, 1);
}
