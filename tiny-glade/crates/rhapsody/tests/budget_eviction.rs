//! Budget-pressure cache eviction: registered caches release idle entries
//! (never leased or in-flight ones) before an allocation fails.
mod gpu_test_support;
use ash::vk;
use rhapsody::{device::{Device, DeviceConfig}, transfer::{TransferConfig, TransferManager}};
use rhapsody::command::descriptor_set::{DescriptorPoolRing, DescriptorPoolRingConfig, LayoutBinding, LayoutPlan, VulkanDescriptorSetLayout};
use rhapsody::resource::buffer::{Buffer, BufferDesc};
use std::sync::Arc;

const MIB: u64 = 1 << 20;

fn device() -> Option<Arc<Device>> {
    gpu_test_support::initialize(
        "budget eviction",
        Device::new_headless(&DeviceConfig::headless()),
    )
}

#[test]
fn pressure_evicts_idle_staging_but_keeps_leased_slots_then_allocation_succeeds() {
    let Some(device) = device() else {
        return;
    };
    let mut manager = TransferManager::new(
        device.clone(),
        TransferConfig {
            slots: 2,
            chunk_bytes: 4 * MIB,
            max_bytes: 32 * MIB,
        },
    )
    .unwrap();
    let source = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_cpu_to_gpu(
                64,
                vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST,
            ),
        )
        .unwrap(),
    );
    source.write_bytes(0, &[7; 64]).unwrap();
    // A held readback ticket leases one slot: it must survive eviction.
    let ticket = manager.read_buffer(&source, 0, 64).unwrap();
    ticket.wait(10_000_000_000).unwrap();
    device.wait_idle().unwrap();
    let staged = manager.stats().unwrap().committed_bytes;
    assert!(staged >= 16 * MIB);
    let used = device.allocator_stats().device_memory_usage().total();
    // Room for 4 MiB more: a 10 MiB buffer only fits after idle staging goes.
    device
        .allocator_stats()
        .set_device_budget(used + 4 * MIB)
        .unwrap();
    let big = Buffer::new(
        device.clone(),
        &BufferDesc::new_gpu_only(10 * MIB, vk::BufferUsageFlags::STORAGE_BUFFER),
    );
    assert!(big.is_ok(), "eviction must make room: {:?}", big.err());
    let stats = manager.stats().unwrap();
    assert!(stats.evicted_slots >= 2, "{stats:?}");
    assert!(stats.committed_bytes < staged);
    assert_eq!(stats.leased_readbacks, 1);
    let mut bytes = [0u8; 64];
    assert_eq!(ticket.try_read(&mut bytes).unwrap(), Some(64));
    assert_eq!(bytes, [7; 64], "the leased slot was not evicted");
    // Without pressure, evicted slots are recreated on demand.
    drop(big);
    drop(ticket);
    device.allocator_stats().set_device_budget(0).unwrap();
    let destination = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_to_cpu(64, vk::BufferUsageFlags::TRANSFER_DST),
        )
        .unwrap(),
    );
    gpu_test_support::submit_upload(&mut manager, destination.clone(), 0, &[9; 64])
        .unwrap()
        .wait(10_000_000_000)
        .unwrap();
    destination.read_bytes(0, &mut bytes).unwrap();
    assert_eq!(bytes, [9; 64]);
    assert!(device.evictables().stats().0 >= 1);
}

#[test]
fn pressure_destroys_idle_descriptor_pools_but_not_pools_with_live_sets() {
    let Some(device) = device() else {
        return;
    };
    let plan = LayoutPlan {
        flags: 0,
        binding_flags: vec![0],
        bindings: vec![LayoutBinding {
            binding: 0,
            descriptor_type: vk::DescriptorType::STORAGE_BUFFER.as_raw(),
            descriptor_count: 1,
            stage_flags: vk::ShaderStageFlags::COMPUTE.as_raw(),
            immutable_sampler: None,
        }],
    };
    let layout = VulkanDescriptorSetLayout::new(device.clone(), &plan).unwrap();
    let ring = Arc::new(
        DescriptorPoolRing::new(
            device.clone(),
            &plan,
            DescriptorPoolRingConfig {
                sets_per_pool: 1,
                max_pools: 4,
                transient: true,
            },
        )
        .unwrap(),
    );
    ring.register_eviction();
    let kept = ring.allocate(&layout).unwrap();
    let dropped: Vec<_> = (0..2)
        .map(|_| ring.allocate(&layout).unwrap())
        .collect();
    assert_eq!(ring.stats().pools, 3);
    drop(dropped);
    assert!(device.evict_idle_caches(u64::MAX) > 0);
    assert_eq!(
        ring.stats().pools,
        1,
        "only the pool with a live set remains"
    );
    drop(kept);
    // The ring still grows on demand afterwards.
    let _again: Vec<_> = (0..2)
        .map(|_| ring.allocate(&layout).unwrap())
        .collect();
    assert_eq!(ring.stats().pools, 2);
}
