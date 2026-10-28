//! BufferDesc 构造与 memory_location 内存属性映射。
use ash::vk;
use rhapsody::resource::buffer::BufferDesc;

#[test]
fn constructors_map_to_expected_memory_properties() {
    use vk::MemoryPropertyFlags as F;
    let u = vk::BufferUsageFlags::STORAGE_BUFFER;
    let cases = [
        (
            BufferDesc::new_gpu_only(64, u),
            F::DEVICE_LOCAL,
            F::DEVICE_LOCAL,
        ),
        (
            BufferDesc::new_cpu_to_gpu(64, u),
            F::HOST_VISIBLE,
            F::HOST_COHERENT,
        ),
        (
            BufferDesc::new_cpu_to_gpu_low_prio(64, u),
            F::HOST_VISIBLE,
            F::empty(),
        ),
        (
            BufferDesc::new_gpu_to_cpu(64, u),
            F::HOST_VISIBLE,
            F::HOST_CACHED | F::HOST_COHERENT,
        ),
    ];
    for (desc, required, preferred) in cases {
        assert_eq!(
            desc.memory_location.memory_properties(),
            (required, preferred)
        );
        assert_eq!(desc.alignment(), 0);
    }
    assert_eq!(
        BufferDesc::new_gpu_only(8, u)
            .with_alignment(256)
            .alignment(),
        256
    );
}
