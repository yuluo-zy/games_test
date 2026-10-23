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
        MappedStorageBuffer::from_mapping_with_access(mapped, buffer, device.mapped_upload_writer(&buffer))
    }
}
impl crate::render::pass_builder::pass_resource_access::SsboType for NormalDebugSsbo {
    fn rg_buffer_hash() -> u64 {
        crate::asset_libraries::ssbo_library::SsboId::of::<Self>()
            .0
             .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        asset_libraries::ssbo_library::AssetSsboLibrary,
        render::ssbo::rhapsody_boundary::{BackendError, Buffer},
        resources::compute_mouse_depth_value::MouseDepthValueSsbo,
    };
    use core::ops::Range;
    use std::cell::RefCell;

    #[derive(Default)]
    struct Recorder(RefCell<Vec<(String, BufferDesc, Option<Vec<u8>>)>>);
    impl GraphicsDevice for Recorder {
        fn create_buffer(
            &self,
            desc: &BufferDesc,
            name: &str,
            data: Option<&[u8]>,
        ) -> Result<Buffer, BackendError> {
            self.0
                .borrow_mut()
                .push((name.into(), *desc, data.map(<[u8]>::to_vec)));
            Ok(Buffer {
                byte_size: desc.size as u64,
                ..Default::default()
            })
        }
        fn update_buffer_bytes(
            &self,
            _: &Buffer,
            _: &[u8],
            _: &[Range<usize>],
            _: usize,
        ) -> Result<(), BackendError> {
            unreachable!("native constructors supply initialization to create_buffer")
        }
        fn mapped_slice_mut(&self, buffer: &Buffer) -> Option<&mut [u8]> {
            // Explicit CPU fixture: initialized, aligned pages; this test records
            // constructor calls and does not stand in for a GPU execution test.
            let page =
                Box::leak(vec![0u128; (buffer.byte_size as usize).div_ceil(16)].into_boxed_slice());
            Some(unsafe {
                core::slice::from_raw_parts_mut(page.as_mut_ptr().cast(), buffer.byte_size as usize)
            })
        }
    }

    #[test]
    fn original_mapped_ssbo_constructors_keep_distinct_none_policies() {
        let device = Recorder::default();
        let library = AssetSsboLibrary::new();
        library.add_new::<NormalDebugSsbo>(&device);
        library.add_new::<MouseDepthValueSsbo>(&device);
        let calls = device.0.borrow();
        assert_eq!(calls.len(), 2);
        let normal = &calls[0];
        assert_eq!(normal.0, "NormalDebug");
        assert_eq!(
            (
                normal.1.size,
                normal.1.usage,
                normal.1.memory,
                normal.1.alignment
            ),
            (320000, 0x22, 2, 4)
        );
        assert_eq!(normal.2, None);
        let mouse = &calls[1];
        assert_eq!(mouse.0, "MouseDepthValue");
        assert_eq!(
            (
                mouse.1.size,
                mouse.1.usage,
                mouse.1.memory,
                mouse.1.alignment
            ),
            (64, 0x22, 4, 4)
        );
        assert_eq!(mouse.2.as_deref(), Some([0u8; 64].as_slice()));
        assert_eq!(
            library.get::<NormalDebugSsbo>().unwrap().mapped.len(),
            10000
        );
        assert_eq!(
            library.get::<MouseDepthValueSsbo>().unwrap().mapped.len(),
            4
        );
        assert_eq!(
            (
                core::mem::size_of::<NormalDebugSsbo>(),
                core::mem::align_of::<NormalDebugSsbo>()
            ),
            (32, 4)
        );
        assert_eq!(
            (
                core::mem::size_of::<MouseDepthValueSsbo>(),
                core::mem::align_of::<MouseDepthValueSsbo>()
            ),
            (16, 4)
        );
    }
}
