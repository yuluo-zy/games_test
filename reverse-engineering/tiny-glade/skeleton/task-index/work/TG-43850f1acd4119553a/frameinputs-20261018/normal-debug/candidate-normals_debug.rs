//! Canonical B NormalDebugSsbo from ctor14161e5a0. Only the proven
//! 32B record shape is restored; individual native fields remain unresolved.
use crate::asset_libraries::ssbo_library::SsboType;
use crate::render::ssbo::rhapsody_boundary::{BufferDesc, GraphicsDevice};
use crate::render::ssbo::{MappedStorageBuffer, StorageElement};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct NormalDebugSsbo(pub [u32; 8]);
impl StorageElement for NormalDebugSsbo {
    const NAME: &'static str = "NormalDebug";
    const BUFFER_LEN: usize = 10000;
}
impl SsboType for NormalDebugSsbo {
    type Storage = MappedStorageBuffer<'static, Self>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        if let Some(bytes) = initial {
            assert_eq!(bytes.len() % 32, 0);
            assert_eq!(bytes.as_ptr() as usize % 4, 0);
            assert!(
                bytes.len() < 320032,
                "Excessive data size supplied for buffer {}",
                core::any::type_name::<Self>()
            );
        }
        // Native None edge@14161e665 does not allocate or upload zero data.
        let desc = BufferDesc::new(320000, 0x22, 2).alignment(4);
        let buffer = device
            .create_buffer(&desc, Self::NAME, initial)
            .expect("called `Result::unwrap()` on an `Err` value: create_buffer");
        let mapped = device
            .mapped_slice_mut(&buffer)
            .expect("called `Option::unwrap()` on a `None` value");
        assert!(
            mapped.len() >= 320000,
            "assertion failed: data.len() >= buffer.buffer.desc.size"
        );
        assert_eq!(mapped.len() % 32, 0);
        assert_eq!(mapped.as_ptr() as usize % 4, 0);
        // SAFETY: original opaque32B/align4 range, persistent GPU mapping
        // owned by the caller's real device. No read or fake init is added.
        let mapped = unsafe {
            core::slice::from_raw_parts_mut(mapped.as_mut_ptr().cast::<Self>(), mapped.len() / 32)
        };
        MappedStorageBuffer { mapped, buffer }
    }
}
impl crate::render::pass_builder::pass_resource_access::SsboType for NormalDebugSsbo {
    fn rg_buffer_hash() -> u64 {
        crate::asset_libraries::ssbo_library::SsboId::of::<Self>()
            .0
             .0
    }
}
