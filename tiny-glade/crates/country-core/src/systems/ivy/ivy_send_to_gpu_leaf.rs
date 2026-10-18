//! Current B Ivy leaf/flower GPU element identity and storage constructors.
//! Native full TypedStorageBuffer ranges141616120 /141615f50 prove48B,
//! NAME strings,10000/1000 capacities,usage0x22/memory1/alignment4, and
//! None with no upload. Original add_new prepends independent count0.
//!
//! The CPU packing/upload helpers below retain their previous implementation.
//! Its random selection, seed mapping and ECS schedule have not been fully
//! revalidated against the original master function; constructor recovery
//! does not certify that CPU producer or the complete72 registrations.
use crate::asset_libraries::ssbo_library::{AssetSsboLibrary, SsboType};
use crate::render::ssbo::rhapsody_boundary::GraphicsDevice;
use crate::render::ssbo::{InstanceBuffer, StorageElement};
use crate::resources::ivy::{IvyLeaf, IvyLeafStorage};
use crate::resources::walls::public_walls::PublicWalls;
use crate::resources::walls::WallId;
use crate::systems::ivy::ivy_grower::wall_arch_height;
use glam::Vec3;

/// Verified IvyLeafSsbo key (rne-read qword).
pub const IVY_LEAF_SSBO_ID: u64 = 0x9e3cfa_7f29_93f406;
/// Verified IvyFlowerSsbo key (rne-read qword).
pub const IVY_FLOWER_SSBO_ID: u64 = 0x1d6de5_e69a_c50d2a;
/// Original descriptor NAME strings, distinct from full type identity.
/// RNE142c7f8e0 (15B) /142c7f8cf (17B).
pub const IVY_LEAF_SSBO_NAME: &str = "Ivy Leaf Buffer";
pub const IVY_FLOWER_SSBO_NAME: &str = "Ivy Flower Buffer";

/// GPU leaf element, 48 B (O: `count * 0x30`; shader `instanced_ivy_leaf`).
/// The field order matches the shader struct; the CPU mapping for `seed` /
/// `arch_height` is 待取证.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct IvyLeafSsbo {
    pub position: Vec3,
    pub size: f32,
    /// The leaf's forward/facing direction (shader `forward_floattor`).
    pub forward: Vec3,
    /// Shader `seed` (int); the leaf spawner's `frame` is the candidate
    /// (待取证).
    pub seed: i32,
    /// The leaf's branch direction (shader `branch_dir`).
    pub branch_dir: Vec3,
    /// Shader `arch_height`; source 待取证 (needs the owning wall's arch).
    pub arch_height: f32,
}

pub use IvyLeafSsbo as IvyLeafGpu;

impl StorageElement for IvyLeafSsbo {
    const NAME: &'static str = IVY_LEAF_SSBO_NAME;
    const BUFFER_LEN: usize = 10_000;
}
impl SsboType for IvyLeafSsbo {
    type Storage = InstanceBuffer<IvyLeafSsbo>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        create_ivy_instance_storage::<IvyLeafSsbo>(device, initial)
    }
}

/// GPU flower element (same 48 B layout, separate buffer). The flower
/// collection is not yet represented in `IvyLeafStorage`; this type lets the
/// buffer be registered and uploaded once flowers exist (待取证).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct IvyFlowerSsbo {
    pub position: Vec3,
    pub size: f32,
    pub forward: Vec3,
    pub seed: i32,
    pub branch_dir: Vec3,
    pub arch_height: f32,
}
pub use IvyFlowerSsbo as IvyFlowerGpu;

impl StorageElement for IvyFlowerSsbo {
    const NAME: &'static str = IVY_FLOWER_SSBO_NAME;
    const BUFFER_LEN: usize = 1000;
}
impl SsboType for IvyFlowerSsbo {
    type Storage = InstanceBuffer<IvyFlowerSsbo>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        create_ivy_instance_storage::<IvyFlowerSsbo>(device, initial)
    }
}

/// Pack one CPU leaf into a 48 B GPU row. `size` is scaled by the
/// `[GraphicsDevice+0x1d]` setting bit (1.0 / 0.85; 0x1408c1631 / 0x1408c181d);
/// `forward` = `vector_2c`, `branch_dir` = `branch_direction`, `seed` = `frame`
/// (candidate, 待取证); `arch_height` = the owning wall's arch
/// (`leaf.source_18` = wall id), falling back to 0.0 when the wall is gone.
pub fn pack_leaf(leaf: &IvyLeaf, walls: &PublicWalls, scale: f32) -> IvyLeafSsbo {
    IvyLeafSsbo {
        position: leaf.position,
        size: leaf.size * scale,
        forward: leaf.vector_2c,
        seed: leaf.frame as i32,
        branch_dir: leaf.branch_direction,
        arch_height: walls
            .get(WallId(leaf.source_18))
            .map_or(0.0, wall_arch_height),
    }
}

/// Maximum rows packed into the second IvyFlowerSsbo channel:
/// `Take(0x3e8)` = 1000 (0x1408c1580, closure `env_1`).
pub const FLOWER_LIMIT: usize = 0x3e8;

/// Pack one CPU leaf into the 48 B flower row. The native second channel
/// (`closure_env$1`, 0x1417d1940) reads `vector_58` (+0x58/+0x60) in place of
/// the leaf channel's `vector_2c` (+0x2c/+0x34), so `forward` is
/// `leaf.vector_58`; the remaining slots keep the shader layout.
pub fn pack_flower(leaf: &IvyLeaf, walls: &PublicWalls, scale: f32) -> IvyFlowerSsbo {
    IvyFlowerSsbo {
        position: leaf.position,
        size: leaf.size * scale,
        forward: leaf.vector_58,
        seed: leaf.frame as i32,
        branch_dir: leaf.branch_direction,
        arch_height: walls
            .get(WallId(leaf.source_18))
            .map_or(0.0, wall_arch_height),
    }
}

/// Second IvyFlowerSsbo channel of `ivy_send_to_gpu_leaf` (0x1408c1580,
/// 0x1408c1e.. path): only runs when `[GraphicsDevice+0x1d]` is set (the same
/// bit that selects scale 0.85). The native builds
/// `filter_map(closure_env$1).take(0x3e8)` and uploads 48 B rows. The captured
/// probability `*env == rand` split between the leaf and flower closures is
/// 待取证; `flower_rate` is supplied by the render owner. Returns the row
/// count; an unregistered buffer skips the upload (O pattern).
pub fn upload_ivy_flowers(
    leaves: &IvyLeafStorage,
    walls: &PublicWalls,
    ssbos: &mut AssetSsboLibrary,
    device: &dyn GraphicsDevice,
    flower_rate: f32,
) -> usize {
    let scale = device.ivy_leaf_scale();
    if scale == 1.0 {
        // `[device+0x1d] == 0` gate: the native skips the whole flower channel.
        return 0;
    }
    let elements: Vec<IvyFlowerSsbo> = leaves
        .all_leaves()
        .filter(|_| fastrand::f32() >= flower_rate)
        .take(FLOWER_LIMIT)
        .map(|leaf| pack_flower(leaf, walls, scale))
        .collect();
    let count = elements.len();
    let Some(buffer) = ssbos.get_mut::<IvyFlowerSsbo>() else {
        return count;
    };
    if count == 0 {
        buffer.instance_count = 0;
    } else {
        buffer
            .buffer
            .gpu_write(device, &elements)
            .unwrap_or_else(|e| panic!("called `Result::unwrap()` on an `Err` value: {e:?}"));
        buffer.instance_count = count;
    }
    count
}

/// `ivy_send_to_gpu_leaf` body: collect all leaves, pack 48 B rows and upload
/// to the IvyLeafSsbo buffer, updating the instance count. Returns the packed
/// count. An unregistered buffer skips the upload silently (O pattern).
pub fn upload_ivy_leaves(
    leaves: &IvyLeafStorage,
    walls: &PublicWalls,
    ssbos: &mut AssetSsboLibrary,
    device: &dyn GraphicsDevice,
) -> usize {
    let scale = device.ivy_leaf_scale();
    let elements: Vec<IvyLeafSsbo> = leaves
        .all_leaves()
        .map(|leaf| pack_leaf(leaf, walls, scale))
        .collect();
    let count = elements.len();
    let Some(buffer) = ssbos.get_mut::<IvyLeafSsbo>() else {
        return count;
    };
    if count == 0 {
        buffer.instance_count = 0;
    } else {
        buffer
            .buffer
            .gpu_write(device, &elements)
            .unwrap_or_else(|e| panic!("called `Result::unwrap()` on an `Err` value: {e:?}"));
        buffer.instance_count = count;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leaf_packs_to_48_bytes() {
        assert_eq!(core::mem::size_of::<IvyLeafSsbo>(), 48);
        let leaf = IvyLeaf {
            point_a: 0,
            point_b: 1,
            ivy: 7,
            source_18: 3,
            position: Vec3::new(1.0, 2.0, 3.0),
            vector_2c: Vec3::Y,
            branch_direction: Vec3::X,
            size: 1.0,
            branch_thickness: 0.5,
            growth: 0.0,
            growth_delay: 0.1,
            frame: 5,
            vector_58: Vec3::Z,
        };
        let gpu = pack_leaf(&leaf, &PublicWalls::default(), 1.0);
        assert_eq!(gpu.position, leaf.position);
        assert_eq!(gpu.size, leaf.size);
        // The scale bit scales the size (0x1408c181d: 0.85 / 0x3f800000: 1.0).
        let scaled = pack_leaf(&leaf, &PublicWalls::default(), 0.85);
        assert!((scaled.size - leaf.size * 0.85).abs() < 1e-6);
        // Verified GPU mapping (shader `instanced_ivy_leaf.raster.hlsl`).
        assert_eq!(gpu.forward, leaf.vector_2c);
        assert_eq!(gpu.branch_dir, leaf.branch_direction);
        assert_eq!(gpu.seed, 5);
        // A missing owning wall leaves arch_height at zero.
        assert_eq!(gpu.arch_height, 0.0);
    }

    #[test]
    fn flower_rows_use_vector_58_and_the_take_limit() {
        assert_eq!(FLOWER_LIMIT, 0x3e8);
        assert_eq!(core::mem::size_of::<IvyFlowerSsbo>(), 48);
        let leaf = IvyLeaf {
            point_a: 0,
            point_b: 1,
            ivy: 7,
            source_18: 3,
            position: Vec3::new(1.0, 2.0, 3.0),
            vector_2c: Vec3::Y,
            branch_direction: Vec3::X,
            size: 1.0,
            branch_thickness: 0.5,
            growth: 0.0,
            growth_delay: 0.1,
            frame: 5,
            vector_58: Vec3::Z * 2.0,
        };
        let flower = pack_flower(&leaf, &PublicWalls::default(), 0.85);
        assert_eq!(flower.forward, leaf.vector_58);
        assert_eq!(flower.branch_dir, leaf.branch_direction);
        assert!((flower.size - 0.85).abs() < 1e-6);
        assert_eq!(flower.seed, 5);
    }

    #[test]
    fn verified_keys_are_the_documented_constants() {
        use crate::asset_libraries::ssbo_library::SsboId;
        assert_eq!(SsboId::of::<IvyLeafSsbo>().0 .0, IVY_LEAF_SSBO_ID);
        assert_eq!(SsboId::of::<IvyFlowerSsbo>().0 .0, IVY_FLOWER_SSBO_ID);
    }
}

impl crate::render::pass_builder::pass_resource_access::SsboType for IvyLeafSsbo {
    fn rg_buffer_hash() -> u64 {
        crate::asset_libraries::ssbo_library::SsboId::of::<Self>()
            .0
             .0
    }
}

impl crate::render::pass_builder::pass_resource_access::SsboType for IvyFlowerSsbo {
    fn rg_buffer_hash() -> u64 {
        crate::asset_libraries::ssbo_library::SsboId::of::<Self>()
            .0
             .0
    }
}

/// Safe current-library byte adapter for the three independently verified
/// fixed48B Ivy typed constructors. None forwards no data; Some(empty)
/// remains Some, and count is independent of the buffer handle.
pub(super) fn create_ivy_instance_storage<T: StorageElement>(
    device: &dyn GraphicsDevice,
    initial: Option<&[u8]>,
) -> InstanceBuffer<T> {
    use crate::render::ssbo::{rhapsody_boundary::BufferDesc, TypedStorageBuffer};
    let stride = core::mem::size_of::<T>();
    let count = initial.map_or(0, |bytes| {
        assert_eq!(bytes.len() % stride, 0);
        assert_eq!(bytes.as_ptr() as usize % 4, 0);
        let count = bytes.len() / stride;
        assert!(
            count <= T::BUFFER_LEN,
            "Excessive data size supplied for buffer {}",
            core::any::type_name::<T>()
        );
        count
    });
    let desc = BufferDesc::new(stride * T::BUFFER_LEN, 0x22, 1).alignment(4);
    let buffer = device
        .create_buffer(&desc, T::NAME, initial)
        .expect("called `Result::unwrap()` on an `Err` value: create_buffer");
    InstanceBuffer {
        instance_count: count,
        buffer: TypedStorageBuffer::from_created_buffer(buffer),
    }
}

#[cfg(test)]
mod native_constructor_tests {
    use super::*;
    use crate::render::ssbo::rhapsody_boundary::{BackendError, Buffer, BufferDesc};
    use crate::systems::ivy::ivy_send_to_gpu_segments::{IvySegmentGpu, IvySegmentSsbo};
    use core::ops::Range;
    use std::cell::RefCell;

    #[derive(Default)]
    struct Recorder(RefCell<Vec<(String, BufferDesc, Option<usize>)>>);
    impl GraphicsDevice for Recorder {
        fn create_buffer(
            &self,
            desc: &BufferDesc,
            name: &str,
            initial: Option<&[u8]>,
        ) -> Result<Buffer, BackendError> {
            self.0
                .borrow_mut()
                .push((name.into(), *desc, initial.map(<[u8]>::len)));
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
            unreachable!("constructor test records creation, not GPU execution")
        }
    }

    #[test]
    fn native_ivy_constructors_keep_names_identity_fixed_sizes_and_none_policy() {
        let device = Recorder::default();
        let library = AssetSsboLibrary::new();
        library.add_new::<IvyLeafSsbo>(&device);
        library.add_new::<IvyFlowerSsbo>(&device);
        library.add_new::<IvySegmentSsbo>(&device);
        assert_eq!(library.instance_count::<IvyLeafSsbo>(), 0);
        assert_eq!(library.instance_count::<IvyFlowerSsbo>(), 0);
        assert_eq!(library.instance_count::<IvySegmentSsbo>(), 0);
        let calls = device.0.borrow();
        for (i, (name, size)) in [
            ("Ivy Leaf Buffer", 480000),
            ("Ivy Flower Buffer", 48000),
            ("Ivy Segments Buffer", 1920000),
        ]
        .into_iter()
        .enumerate()
        {
            assert_eq!(
                (
                    calls[i].0.as_str(),
                    calls[i].1.size,
                    calls[i].1.usage,
                    calls[i].1.memory,
                    calls[i].1.alignment,
                    calls[i].2
                ),
                (name, size, 0x22, 1, 4, None)
            );
        }
        assert_eq!(
            core::any::type_name::<IvyLeafSsbo>(),
            "country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo"
        );
        assert_eq!(
            core::any::type_name::<IvyFlowerSsbo>(),
            "country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo"
        );
        assert_eq!(
            core::any::type_name::<IvySegmentSsbo>(),
            "country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo"
        );
        assert_eq!(
            core::any::TypeId::of::<IvyLeafSsbo>(),
            core::any::TypeId::of::<IvyLeafGpu>()
        );
        assert_eq!(
            core::any::TypeId::of::<IvyFlowerSsbo>(),
            core::any::TypeId::of::<IvyFlowerGpu>()
        );
        assert_eq!(
            core::any::TypeId::of::<IvySegmentSsbo>(),
            core::any::TypeId::of::<IvySegmentGpu>()
        );
    }

    fn check_data_boundary<
        T: SsboType<Storage = InstanceBuffer<T>> + bytemuck::NoUninit + Copy + Default,
    >() {
        let device = Recorder::default();
        let rows = vec![T::default(); T::BUFFER_LEN];
        let library = AssetSsboLibrary::new();
        library.add_new_with_data::<T>(&device, &rows);
        assert_eq!(library.instance_count::<T>(), T::BUFFER_LEN);
        assert_eq!(device.0.borrow()[0].2, Some(rows.len() * 48));
        let empty = T::new_storage_buffer(&device, Some(bytemuck::cast_slice::<T, u8>(&[])));
        assert_eq!(empty.instance_count, 0);
        assert_eq!(
            device.0.borrow()[1].2,
            Some(0),
            "Some(empty) must not turn into None"
        );
        let excessive = vec![T::default(); T::BUFFER_LEN + 1];
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            T::new_storage_buffer(&device, Some(bytemuck::cast_slice(&excessive)))
        }));
        assert!(result.is_err());
        assert_eq!(
            device.0.borrow().len(),
            2,
            "over capacity must fail before device creation"
        );
    }

    #[test]
    fn native_ivy_initial_data_has_inclusive_capacity_and_independent_counts() {
        check_data_boundary::<IvyLeafSsbo>();
        check_data_boundary::<IvyFlowerSsbo>();
        check_data_boundary::<IvySegmentSsbo>();
    }
}
