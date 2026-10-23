//! B MouseDepthValue readback carrier. Native constructor14161e8f0 and
//! original read_mouse_depth_value SPIR-V define four 16B float4 records.
//! Host schema uses four f32s with original alignment4, without Vec4's
//! target-specific SIMD alignment. Canonical type identity is this module.
use crate::asset_libraries::ssbo_library::SsboType;
use crate::render::ssbo::rhapsody_boundary::{BufferDesc, GraphicsDevice};
use crate::render::ssbo::{MappedStorageBuffer, StorageElement};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MouseDepthValueSsbo(pub [f32; 4]);

impl MouseDepthValueSsbo {
    /// Representation adapter only; original four components remain intact.
    pub fn as_vec4(self) -> glam::Vec4 {
        glam::Vec4::from_array(self.0)
    }
}
impl From<glam::Vec4> for MouseDepthValueSsbo {
    fn from(value: glam::Vec4) -> Self {
        Self(value.to_array())
    }
}
impl StorageElement for MouseDepthValueSsbo {
    const NAME: &'static str = "MouseDepthValue";
    const BUFFER_LEN: usize = 4;
    const MAPPED_MEMORY: u8 = crate::render::ssbo::MAPPED_MEMORY_READBACK;
}
impl SsboType for MouseDepthValueSsbo {
    type Storage = MappedStorageBuffer<'static, Self>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        // Native byte input guards precede selecting real zero defaults.
        if let Some(bytes) = initial {
            assert_eq!(bytes.len() % 16, 0);
            assert_eq!(bytes.as_ptr() as usize % 4, 0);
            assert!(
                bytes.len() < 80,
                "Excessive data size supplied for buffer {}",
                core::any::type_name::<Self>()
            );
        }
        let zero_records = [0u32; 16];
        let data = initial.unwrap_or_else(|| bytemuck::cast_slice(&zero_records));
        let desc = BufferDesc::new(64, 0x22, 4).alignment(4);
        let buffer = device
            .create_buffer(&desc, Self::NAME, Some(data))
            .expect("called `Result::unwrap()` on an `Err` value: create_buffer");
        let mapped = device
            .mapped_slice_mut(&buffer)
            .expect("called `Option::unwrap()` on a `None` value");
        assert!(
            mapped.len() >= 64,
            "assertion failed: data.len() >= buffer.buffer.desc.size"
        );
        assert_eq!(mapped.len() % 16, 0);
        assert_eq!(mapped.as_ptr() as usize % 4, 0);
        // SAFETY: exact f32[4]/align4 layout, checked complete record range.
        // The actual host owns this persistent mapping; as with the current
        // MappedStorageBuffer factory, its live GPU owner must outlive storage.
        let mapped = unsafe {
            core::slice::from_raw_parts_mut(mapped.as_mut_ptr().cast::<Self>(), mapped.len() / 16)
        };
        MappedStorageBuffer::from_mapping_with_access(mapped, buffer, device.mapped_upload_writer(&buffer))
    }
}
impl crate::render::pass_builder::pass_resource_access::SsboType for MouseDepthValueSsbo {
    fn rg_buffer_hash() -> u64 {
        crate::asset_libraries::ssbo_library::SsboId::of::<Self>()
            .0
             .0
    }
}
