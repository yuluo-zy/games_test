//! Original B BoatSsbo creation contract: Typed ctor@14161a020 (451B)
//! allocates64B ×4096, usage0x22/memory1/alignment16 with None uninitialized.
//! add_new@1412c23d0 prepends independent instance_count0 (InstanceBuffer48).
//! Original shader BoatXform is dense48B; do not shrink the64B native capacity
//! or infer unobserved CPU field meanings from that separate GPU schema.
//! This file restores type/factory identity, not the complete spawn pass.
use crate::asset_libraries::ssbo_library::SsboType as LibrarySsbo;
use crate::render::pass_builder::pass_resource_access::SsboType;
use crate::render::ssbo::{
    rhapsody_boundary::{BufferDesc, GraphicsDevice},
    InstanceBuffer, StorageElement, TypedStorageBuffer,
};
use crate::systems::render_loop::compute_passes::pass_kit::binding_hash;

/// Original constructor elements are64B even though the compute shader
/// writes dense48B BoatXform rows. Preserve that actual allocation contract;
/// the extra native creation words have no independently recovered CPU role.
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BoatSsbo(pub [u32; 16]);
impl StorageElement for BoatSsbo {
    const NAME: &'static str = "Boats";
    const BUFFER_LEN: usize = 4096;
}
impl LibrarySsbo for BoatSsbo {
    type Storage = InstanceBuffer<Self>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        let count = initial.map_or(0, |bytes| {
            assert_eq!(bytes.len() % 64, 0);
            assert_eq!(bytes.as_ptr() as usize % 4, 0);
            let count = bytes.len() / 64;
            assert!(
                count <= 4096,
                "Excessive data size supplied for buffer {}",
                core::any::type_name::<Self>()
            );
            count
        });
        let buffer = device
            .create_buffer(
                &BufferDesc::new(0x40000, 0x22, 1).alignment(16),
                Self::NAME,
                initial,
            )
            .expect("called `Result::unwrap()` on an `Err` value: create_buffer");
        InstanceBuffer {
            instance_count: count,
            buffer: TypedStorageBuffer::from_created_buffer(buffer),
        }
    }
}

/// 绑定名（O：`spawn_boats.cs.hlsl` 里的 SSBO 名；推断为 `boat_buffer`，立即数未在已恢复函数中出现 → U）。
pub const BOAT_BINDING: u64 = binding_hash("boat_buffer");

impl SsboType for BoatSsbo {
    /// 旧 trait（`rg_buffer_hash`）：原值 = 注册键 `xxh3_64("Boats")`（O）。
    fn rg_buffer_hash() -> u64 {
        0x4f69c51e83e1d670
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asset_libraries::ssbo_library::AssetSsboLibrary;
    use crate::render::ssbo::StorageElement;
    use crate::render::test_device::TestDevice;
    use crate::rhapsody_types::RenderGraph;

    #[test]
    fn library_key_name_is_boats() {
        assert_eq!(BoatSsbo::NAME, "Boats");
        assert_eq!(BoatSsbo::rg_buffer_hash(), 0x4f69c51e83e1d670);
    }

    #[test]
    fn register_then_import_into_the_render_graph() {
        let lib = AssetSsboLibrary::new();
        lib.add_new::<BoatSsbo>(&TestDevice(1));
        let mut rg = RenderGraph::default();
        let a = lib.get_rg_buffer::<BoatSsbo>(&mut rg);
        // 再取一次返回同一个导入句柄（O：已导入则复用）。
        assert_eq!(lib.get_rg_buffer::<BoatSsbo>(&mut rg), a);
    }

    #[test]
    #[should_panic(expected = "Tried to register the same storage buffer twice")]
    fn double_registration_panics() {
        let lib = AssetSsboLibrary::new();
        lib.add_new::<BoatSsbo>(&TestDevice(1));
        lib.add_new::<BoatSsbo>(&TestDevice(1));
    }
}

#[cfg(test)]
mod native_constructor_tests {
    use super::*;
    use crate::render::ssbo::rhapsody_boundary::{BackendError, Buffer};
    use crate::systems::render_loop::compute_passes::stairs::StairsTransformSsbo;
    use std::{cell::RefCell, ops::Range};
    #[derive(Default)]
    struct Recorder(RefCell<Vec<(String, BufferDesc, Option<usize>)>>);
    impl GraphicsDevice for Recorder {
        fn create_buffer(
            &self,
            desc: &BufferDesc,
            name: &str,
            data: Option<&[u8]>,
        ) -> Result<Buffer, BackendError> {
            self.0
                .borrow_mut()
                .push((name.into(), *desc, data.map(<[u8]>::len)));
            Ok(Buffer {
                id: 987,
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
            unreachable!()
        }
    }
    #[test]
    fn native_boat_and_stair_constructors_preserve_distinct_storage_and_capacity() {
        use crate::asset_libraries::ssbo_library::{AssetSsboLibrary, SsboId};
        let device = Recorder::default();
        let library = AssetSsboLibrary::new();
        library.add_new::<BoatSsbo>(&device);
        library.add_new::<StairsTransformSsbo>(&device);
        assert_eq!(library.instance_count::<BoatSsbo>(), 0);
        assert_eq!(
            core::mem::size_of_val(library.get::<BoatSsbo>().unwrap()),
            48
        );
        assert_eq!(
            core::mem::size_of_val(library.get::<StairsTransformSsbo>().unwrap()),
            40
        );
        assert_eq!(
            (
                core::mem::size_of::<BoatSsbo>(),
                core::mem::size_of::<StairsTransformSsbo>()
            ),
            (64, 48)
        );
        let calls = device.0.borrow();
        assert_eq!(
            (
                calls[0].0.as_str(),
                calls[0].1.size,
                calls[0].1.usage,
                calls[0].1.memory,
                calls[0].1.alignment,
                calls[0].2
            ),
            ("Boats", 0x40000, 0x22, 1, 16, None)
        );
        assert_eq!(
            (
                calls[1].0.as_str(),
                calls[1].1.size,
                calls[1].1.usage,
                calls[1].1.memory,
                calls[1].1.alignment,
                calls[1].2
            ),
            ("Stairs transforms", 0x1e0780, 0x22, 1, 4, None)
        );
        assert_eq!(SsboId::of::<BoatSsbo>().0 .0, 0x4f69c51e83e1d670);
        assert_eq!(SsboId::of::<StairsTransformSsbo>().0 .0, 0xc405372d6efc32d3);
    }
    #[test]
    fn native_boat_and_stair_initial_data_keep_inclusive_bounds_and_some_empty() {
        let device = Recorder::default();
        let boats = vec![BoatSsbo::default(); 4096];
        let storage = <BoatSsbo as LibrarySsbo>::new_storage_buffer(
            &device,
            Some(bytemuck::cast_slice(&boats)),
        );
        assert_eq!(storage.instance_count, 4096);
        let rows = vec![[0u32; 12]; 41000];
        let typed = <StairsTransformSsbo as LibrarySsbo>::new_storage_buffer(
            &device,
            Some(bytemuck::cast_slice(&rows)),
        );
        assert_eq!(typed.byte_size(), 0x1e0780);
        let empty = <BoatSsbo as LibrarySsbo>::new_storage_buffer(
            &device,
            Some(bytemuck::cast_slice::<BoatSsbo, u8>(&[])),
        );
        assert_eq!(empty.instance_count, 0);
        assert_eq!(device.0.borrow()[2].2, Some(0));
        let excessive = vec![BoatSsbo::default(); 4097];
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            <BoatSsbo as LibrarySsbo>::new_storage_buffer(
                &device,
                Some(bytemuck::cast_slice(&excessive)),
            )
        }));
        assert!(result.is_err());
        let excessive = vec![[0u32; 12]; 41001];
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            <StairsTransformSsbo as LibrarySsbo>::new_storage_buffer(
                &device,
                Some(bytemuck::cast_slice(&excessive)),
            )
        }));
        assert!(result.is_err());
        assert_eq!(
            device.0.borrow().len(),
            3,
            "excess must fail before creation"
        );
    }
}
