//! Known terminal loss must stop native object creation before touching Vulkan.
mod gpu_test_support;
use ash::vk;
use rhapsody::{
    device::{Device, DeviceConfig},
    raw::{
        buffer::{Buffer, BufferDesc},
        texture::{Texture, TextureDesc, TextureViewDesc},
    },
};
use std::sync::Arc;
#[test]
fn known_loss_rejects_buffer_image_and_view_creation_without_new_allocations() {
    let Some(device) = gpu_test_support::initialize(
        "resource lost guards",
        Device::new_headless(&DeviceConfig::headless()),
    ) else {
        return;
    };
    let desc =
        TextureDesc::new_2d(4, 4, vk::Format::R8G8B8A8_UNORM).usage(vk::ImageUsageFlags::SAMPLED);
    let image = Arc::new(Texture::new(device.clone(), &desc).unwrap());
    let managed = device
        .resources()
        .textures
        .allocate_shared(image.clone())
        .unwrap();
    let _cached_view = managed.view(None).unwrap();
    let allocations = device.allocator_stats().report().allocations.len();
    // No GPU work is pending. This exercises the sticky CPU state without
    // inducing a real driver fault or fabricating a successful completion point.
    device.timelines().mark_device_lost();
    assert!(matches!(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_only(16, vk::BufferUsageFlags::TRANSFER_DST)
        ),
        Err(rhapsody::Error::DeviceLost)
    ));
    assert!(matches!(
        Texture::new(device.clone(), &desc),
        Err(rhapsody::Error::DeviceLost)
    ));
    assert!(matches!(
        image.image_view(&TextureViewDesc::new(&desc).unwrap()),
        Err(rhapsody::Error::DeviceLost)
    ));
    assert!(matches!(
        managed.view(None),
        Err(rhapsody::Error::DeviceLost)
    ));
    assert_eq!(
        device.allocator_stats().report().allocations.len(),
        allocations
    );
}

#[test]
fn known_loss_rejects_raw_pools_descriptors_and_host_access() {
    use rhapsody::raw::{
        binding::DescriptorSetBinding,
        command_buffer::VulkanCommandPool,
        descriptor_set::{
            LayoutBinding, LayoutPlan, VulkanDescriptorPool, VulkanDescriptorSetLayout,
        },
        sampler::Sampler,
        semaphore::TimelineSemaphore,
    };
    fn lost<T>(result: rhapsody::Result<T>) {
        assert!(matches!(result, Err(rhapsody::Error::DeviceLost)));
    }
    let Some(device) = gpu_test_support::initialize(
        "raw terminal entries",
        Device::new_headless(&DeviceConfig::headless()),
    ) else {
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
    let pool = VulkanDescriptorPool::new(device.clone(), &plan, 2).unwrap();
    let mut set = pool.allocate(layout.clone()).unwrap();
    let commands = VulkanCommandPool::new(device.clone()).unwrap();
    let mut idle = commands.allocate().unwrap();
    let mut recording = commands.allocate().unwrap();
    recording.begin().unwrap();
    let _cached_sampler = device.get_built_in_sampler(0).unwrap().unwrap();
    let semaphore = TimelineSemaphore::new(device.clone(), 0).unwrap();
    let desc = BufferDesc::new_cpu_to_gpu(64, vk::BufferUsageFlags::STORAGE_BUFFER);
    let mut host = Buffer::new(device.clone(), &desc).unwrap();
    let owner = Arc::new(Buffer::new(device.clone(), &desc).unwrap());
    let binding = DescriptorSetBinding::Buffer {
        ty: vk::DescriptorType::STORAGE_BUFFER,
        info: vk::DescriptorBufferInfo::default()
            .buffer(owner.handle)
            .range(64),
        owner,
    };
    let registered = device.allocator_stats().total_registered();
    let updates = device.descriptor_update_count();
    device.timelines().mark_device_lost(); // Idle driver; no actual device fault or pending GPU work.
    lost(Sampler::new(
        device.clone(),
        &vk::SamplerCreateInfo::default(),
    ));
    lost(device.get_built_in_sampler(0));
    lost(VulkanCommandPool::new(device.clone()));
    lost(commands.allocate());
    lost(idle.begin());
    lost(idle.reset());
    lost(unsafe { idle.reset_after_completion() });
    lost(recording.end());
    assert!(recording.is_recording());
    assert!(!idle.is_recording());
    lost(commands.reset_completed(&mut [&mut idle, &mut recording]));
    assert_eq!(commands.reset_count(), 0);
    lost(VulkanDescriptorSetLayout::new(device.clone(), &plan));
    lost(VulkanDescriptorPool::new(device.clone(), &plan, 2));
    lost(pool.allocate(layout.clone()));
    lost(unsafe { set.update(0, 0, &binding) });
    lost(unsafe { binding.write(&device, set.handle, 0, 0) });
    lost(TimelineSemaphore::new(device.clone(), 0));
    lost(semaphore.completed());
    lost(semaphore.wait(0, 0));
    lost(host.write_bytes(0, &[1]));
    lost(host.write_bytes_unsynchronized(0, &[1]));
    lost(host.read_bytes(0, &mut [0]));
    lost(unsafe { host.read_bytes_after_completion(0, &mut [0]) });
    lost(unsafe { host.mapped_slice_mut(0, 1) });
    assert!(unsafe { host.mapped_full_slice_mut() }.is_none());
    assert!(host.persistent_host_ptr().is_none());
    lost(host.device_address());
    assert_eq!(device.allocator_stats().total_registered(), registered);
    assert_eq!(device.descriptor_update_count(), updates);
}

#[test]
fn unsupported_descriptor_layout_is_rejected_before_native_creation() {
    use rhapsody::raw::descriptor_set::{LayoutBinding, LayoutPlan, VulkanDescriptorSetLayout};
    let Some(device) = gpu_test_support::initialize(
        "descriptor layout support",
        Device::new_headless(&DeviceConfig::headless()),
    ) else {
        return;
    };
    let mut maintenance = vk::PhysicalDeviceMaintenance3Properties::default();
    let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut maintenance);
    unsafe {
        device
            .instance()
            .instance
            .get_physical_device_properties2(device.physical_device(), &mut properties);
    }
    // The guaranteed minimum is not a maximum. Find a genuinely unsupported
    // native packing size, then require the production constructor to reject it.
    let mut rejected = None;
    for count in [
        maintenance.max_per_set_descriptors.saturating_add(1),
        8192,
        u32::MAX,
    ] {
        let bindings = [vk::DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .descriptor_count(count)
            .stage_flags(vk::ShaderStageFlags::COMPUTE)];
        let info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);
        let mut support = vk::DescriptorSetLayoutSupport::default();
        unsafe {
            device
                .raw_device()
                .get_descriptor_set_layout_support(&info, &mut support);
        }
        if support.supported == vk::FALSE {
            rejected = Some(count);
            break;
        }
    }
    let Some(count) = rejected else {
        eprintln!(
            "SKIP unsupported-layout rejection: implementation reports every queried layout supported"
        );
        return;
    };
    let error = match VulkanDescriptorSetLayout::new(
        device.clone(),
        &LayoutPlan {
            flags: 0,
            binding_flags: vec![0],
            bindings: vec![LayoutBinding {
                binding: 0,
                descriptor_type: vk::DescriptorType::STORAGE_BUFFER.as_raw(),
                descriptor_count: count,
                stage_flags: vk::ShaderStageFlags::COMPUTE.as_raw(),
                immutable_sampler: None,
            }],
        },
    ) {
        Ok(_) => panic!("production constructor created a natively unsupported descriptor layout"),
        Err(error) => error,
    };
    assert_eq!(error.kind(), rhapsody::ErrorKind::Unsupported);
    assert!(!device.is_device_lost());
    assert_eq!(device.garbage_stats().pending, 0);
}
