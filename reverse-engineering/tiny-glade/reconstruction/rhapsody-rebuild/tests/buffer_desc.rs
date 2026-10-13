//! BufferDesc 原 16 字节 ABI 与 memory_location 枚举映射。
use ash::vk;
use rhapsody::raw::buffer::{BufferDesc, MemoryLocation};
use std::mem::{align_of, size_of};

#[test]
fn layout_is_16_bytes_with_original_field_order() {
    assert_eq!(size_of::<BufferDesc>(), 16);
    assert_eq!(align_of::<BufferDesc>(), 8);
}

#[test]
fn memory_location_raw_values_match_original_enum() {
    for (raw, loc) in [
        (1, MemoryLocation::GpuOnly),
        (2, MemoryLocation::CpuToGpu),
        (3, MemoryLocation::CpuToGpuLowPrio),
        (4, MemoryLocation::GpuToCpu),
    ] {
        assert_eq!(MemoryLocation::from_raw(raw), Some(loc));
        assert_eq!(loc.as_raw(), raw);
    }
    assert_eq!(MemoryLocation::from_raw(0), None);
    assert_eq!(MemoryLocation::from_raw(5), None);
}

#[test]
fn constructors_map_to_expected_memory_properties() {
    use vk::MemoryPropertyFlags as F;
    let u = vk::BufferUsageFlags::STORAGE_BUFFER;
    let cases = [
        (BufferDesc::new_gpu_only(64, u), F::DEVICE_LOCAL, F::DEVICE_LOCAL),
        (BufferDesc::new_cpu_to_gpu(64, u), F::HOST_VISIBLE, F::HOST_COHERENT),
        (BufferDesc::new_cpu_to_gpu_low_prio(64, u), F::HOST_VISIBLE, F::empty()),
        (
            BufferDesc::new_gpu_to_cpu(64, u),
            F::HOST_VISIBLE,
            F::HOST_CACHED | F::HOST_COHERENT,
        ),
    ];
    for (desc, required, preferred) in cases {
        assert_eq!(desc.memory_location.memory_properties(), (required, preferred));
        assert_eq!(desc.alignment(), 0);
    }
    assert_eq!(BufferDesc::new_gpu_only(8, u).with_alignment(256).alignment(), 256);
}
