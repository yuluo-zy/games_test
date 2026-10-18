use super::*;
use crate::render::sparse_instance_buffer::SLOTS_PER_CHUNK;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct SmallBrick(u32);
impl StorageElement for SmallBrick {
    const NAME: &'static str = "GeometrySmallBrick";
    const BUFFER_LEN: usize = 64 * SLOTS_PER_CHUNK;
}
// SAFETY: 单个 u32 无填充，容量与 chunk 数一致。
unsafe impl SparseSsbo for SmallBrick {
    const CHUNK_COUNT: usize = 64;
}

#[test]
fn capacity_failure_preserves_old_geometry_and_retry_payload() {
    let device = crate::render::test_device::TestDevice(1);
    let mut storage = SparseInstanceBuffer::<SmallBrick>::try_new(&device).unwrap();
    let mut wall = InstancedWall::new(1, vec![SmallBrick(7); 33]);
    wall.upload_to_storage_buffer(&mut storage).unwrap();
    let mut other_wall = InstancedWall::new(2, vec![SmallBrick(8); 62 * 32]);
    other_wall.upload_to_storage_buffer(&mut storage).unwrap();
    wall.update(vec![SmallBrick(99); 65]);
    assert_eq!(
        wall.upload_to_storage_buffer(&mut storage),
        Err(WallUploadError::CapacityExceeded)
    );
    assert_eq!(storage.data[0], SmallBrick(7));
    assert_eq!(
        storage.allocation_to_draw_multi_instanced(wall.allocation().unwrap()),
        [(0, 33)]
    );
    assert_eq!(wall.pending_instances().len(), 65);
    assert!(wall.is_upload_pending());
    drop(other_wall);
    storage.take_out_waste();
    assert_eq!(wall.upload_to_storage_buffer(&mut storage), Ok(true));
    assert_eq!(storage.data[0], SmallBrick(99));
    assert!(!wall.is_upload_pending());
    assert_eq!(storage.free_chunks.read().len(), 61);
    let ranges = storage.allocation_to_draw_multi_instanced(wall.allocation().unwrap());
    assert_eq!(wall.upload_to_storage_buffer(&mut storage), Ok(false));
    assert_eq!(
        storage.allocation_to_draw_multi_instanced(wall.allocation().unwrap()),
        ranges
    );
    wall.update(vec![]);
    wall.upload_to_storage_buffer(&mut storage).unwrap();
    assert!(
        storage
            .allocation_to_draw_multi_instanced(wall.allocation().unwrap())
            .is_empty()
    );
    assert_eq!(storage.free_chunks.read().len(), 64);
}

#[test]
fn foreign_buffer_is_rejected_without_consuming_update() {
    let device = crate::render::test_device::TestDevice(1);
    let mut original = SparseInstanceBuffer::<SmallBrick>::try_new(&device).unwrap();
    let mut foreign = SparseInstanceBuffer::<SmallBrick>::try_new(&device).unwrap();
    let mut wall = InstancedWall::new(1, vec![SmallBrick(7)]);
    wall.upload_to_storage_buffer(&mut original).unwrap();
    wall.update(vec![SmallBrick(99)]);
    assert_eq!(
        wall.upload_to_storage_buffer(&mut foreign),
        Err(WallUploadError::ForeignStorage)
    );
    assert_eq!(wall.pending_instances(), [SmallBrick(99)]);
    assert_eq!(foreign.free_chunks.read().len(), 64);
    wall.upload_to_storage_buffer(&mut original).unwrap();
    assert_eq!(original.data[0], SmallBrick(99));
}

#[test]
fn only_the_five_transient_stair_ids_are_remapped() {
    for (d, expect) in [
        (0u32, 0x8000_0000u32),
        (1, 0x8000_0001),
        (2, 0x8000_0002),
        (3, 0x8000_0003),
        (11, 0x8000_000b),
    ] {
        assert_eq!(
            BrickSourceId::from_wall_id(WallId(100_000 + u64::from(d))).0,
            expect,
            "d {d}"
        );
        assert_eq!(
            BrickSourceId::from_color_id(ColorId(100_000 + d)).0,
            expect,
            "d {d}"
        );
    }
    for d in [4u32, 5, 6, 7, 8, 9, 10, 12] {
        let raw = 100_000 + d;
        assert_eq!(BrickSourceId::from_color_id(ColorId(raw)).0, raw, "d {d}");
    }
}

#[test]
fn ordinary_and_out_of_range_ids_pass_through() {
    for raw in [0u32, 1, 99_999, 100_012, 5_000_000, u32::MAX] {
        assert_eq!(BrickSourceId::from_color_id(ColorId(raw)).0, raw);
        assert_eq!(BrickSourceId::from_wall_id(WallId(u64::from(raw))).0, raw);
    }
}

#[test]
fn wall_ids_use_only_the_low_32_bits() {
    let wall = WallId((7u64 << 32) | 100_001);
    assert_eq!(BrickSourceId::from_wall_id(wall).0, 0x8000_0001);
}

#[test]
fn transient_edge_ids_set_the_bit_30_family_flag() {
    assert_eq!(
        BrickSourceId(0x8000_0001).with_transient_stair_edge_id(3),
        0xC000_0031
    );
    assert_eq!(
        BrickSourceId(0).with_transient_stair_edge_id(0),
        0x4000_0000
    );
    assert_eq!(
        BrickSourceId(5).with_transient_stair_edge_id(0xf),
        0x4000_00f5
    );
}

#[test]
fn table_agrees_with_the_mask_for_every_index() {
    for d in 0..12u32 {
        let selected = (0x80f_u32 >> d) & 1 != 0;
        assert_eq!(selected, matches!(d, 0 | 1 | 2 | 3 | 11), "d {d}");
    }
}

// ---- BrickTransformSsbo（fresh 机器码锚定，2026-10-17 impl-brick） ----

#[test]
fn brick_transform_layout_matches_machine_code() {
    use core::mem::{align_of, offset_of, size_of};
    // grow_one ×0x60；push 逐 12 qword 整块拷贝。
    assert_eq!(size_of::<BrickTransformSsbo>(), 0x60);
    assert_eq!(align_of::<BrickTransformSsbo>(), 4); // 影子分配 align 4（0x1412e587b）
    assert_eq!(offset_of!(BrickTransformSsbo, affine_columns), 0x00);
    assert_eq!(offset_of!(BrickTransformSsbo, deformation), 0x30);
    assert_eq!(offset_of!(BrickTransformSsbo, surface), 0x40);
    assert_eq!(offset_of!(BrickTransformSsbo, seed), 0x4c);
    assert_eq!(offset_of!(BrickTransformSsbo, material), 0x50);
    assert_eq!(offset_of!(BrickTransformSsbo, wall_distance), 0x54);
    assert_eq!(offset_of!(BrickTransformSsbo, flags), 0x5c);
    // 类型/缓冲常量（0x141618890 / 0x142c7fd18 / 0x1412e5800）。
    assert_eq!(
        <BrickTransformSsbo as StorageElement>::NAME,
        "BrickTransforms"
    );
    assert_eq!(<BrickTransformSsbo as StorageElement>::NAME.len(), 0xf);
    assert_eq!(<BrickTransformSsbo as StorageElement>::BUFFER_LEN, 0x200000);
    assert_eq!(BrickTransformSsbo::CHUNK_COUNT, 0x10000);
    assert_eq!(
        <BrickTransformSsbo as StorageElement>::BUFFER_LEN,
        BrickTransformSsbo::CHUNK_COUNT * SLOTS_PER_CHUNK
    );
    assert_eq!(
        core::any::type_name::<BrickTransformSsbo>().len(),
        0x3a,
        "panic 文案里的完整类型名长度"
    );
}

#[test]
fn brick_transform_default_matches_0x1408587c0() {
    let d = BrickTransformSsbo::default();
    assert_eq!(d.deformation, [0.0; 4]);
    assert_eq!(d.surface, [-100.0; 3]);
    assert_eq!((d.seed, d.material), (0, 0));
    assert_eq!(d.wall_distance, [0.0; 2]);
    assert_eq!(d.flags, 0);
    // 逐字节核对 default 的观察模式：0x00..0x40 全零，0x40..0x4c = -100，0x4c..0x60 全零。
    // SAFETY: `BrickTransformSsbo` 是 0x60 B、无 padding 的 `repr(C)` 位模式类型。
    let bytes = bytemuck::bytes_of(&d);
    assert!(bytes[..0x40].iter().all(|&b| b == 0));
    let minus_100 = (-100.0f32).to_le_bytes();
    for off in [0x40, 0x44, 0x48] {
        assert_eq!(&bytes[off..off + 4], &minus_100, "offset {off:#x}");
    }
    assert!(bytes[0x4c..].iter().all(|&b| b == 0));
}

#[test]
fn brick_transform_ssbo_id_is_the_exe_immediate() {
    use crate::render::pass_builder::pass_resource_access::SsboType as PassSsboType;
    // O：add_new<BrickTransformSsbo> 0x1412bf670 的 FrozenMap 键立即数。
    assert_eq!(
        xxhash_rust::xxh3::xxh3_64(b"BrickTransforms"),
        0xeb57_5d06_1b90_ebf5
    );
    assert_eq!(
        <BrickTransformSsbo as PassSsboType>::rg_buffer_hash(),
        0xeb57_5d06_1b90_ebf5
    );
}

#[test]
fn brick_transform_element_packing_is_field_order() {
    let v = BrickTransformSsbo {
        affine_columns: [
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
            [10.0, 11.0, 12.0],
        ],
        deformation: [13.0, 14.0, 15.0, 16.0],
        surface: [-100.0; 3],
        seed: 0x1122_3344,
        material: 0x5566_7788,
        wall_distance: [0.1, 0.2],
        flags: 6,
    };
    // SAFETY: 0x60 B 的无 padding 位模式类型。
    let bytes = bytemuck::bytes_of(&v);
    // 0x00..0x30：12×f32 列主序。
    for (i, x) in [
        1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
    ]
    .into_iter()
    .enumerate()
    {
        assert_eq!(&bytes[i * 4..i * 4 + 4], &x.to_le_bytes(), "affine[{i}]");
    }
    // 0x30..0x40：deformation。
    for (i, x) in [13.0f32, 14.0, 15.0, 16.0].into_iter().enumerate() {
        let off = 0x30 + i * 4;
        assert_eq!(&bytes[off..off + 4], &x.to_le_bytes(), "deformation[{i}]");
    }
    // 0x4c/0x50/0x54/0x58/0x5c。
    assert_eq!(&bytes[0x4c..0x50], &0x1122_3344u32.to_le_bytes());
    assert_eq!(&bytes[0x50..0x54], &0x5566_7788u32.to_le_bytes());
    assert_eq!(&bytes[0x54..0x58], &0.1f32.to_le_bytes());
    assert_eq!(&bytes[0x58..0x5c], &0.2f32.to_le_bytes());
    assert_eq!(&bytes[0x5c..0x60], &6u32.to_le_bytes());
}

#[test]
fn brick_transform_registers_as_a_sparse_instance_buffer() {
    use crate::asset_libraries::ssbo_library::AssetSsboLibrary;
    use crate::render::pass_builder::pass_resource_access::SsboType as PassSsboType;
    use crate::render::test_device::TestDevice;
    let dev = TestDevice(1);
    let mut lib = AssetSsboLibrary::new();
    // O：resources::init_populate_ssbo_library 的 add_new<BrickTransformSsbo>
    // （0x1412bf670 → impl$10::new_storage_buffer 0x1412e5800）。
    lib.add_new::<BrickTransformSsbo>(&dev);
    assert!(lib.contains::<BrickTransformSsbo>());
    let s = lib.get_mut::<BrickTransformSsbo>().unwrap();
    // O：new_storage_buffer 0x1412e5800 的逐常量。
    assert_eq!(s.buffer.buffer.byte_size, 0xc000000); // BUFFER_LEN * 0x60
    assert_eq!(s.byte_map.len(), 0x10001);
    assert_eq!(s.byte_map[0x10000], 0x20, "哨兵 chunk 恒满");
    assert_eq!(s.data.len(), 0x200000);
    // 影子初值 = T::default()（surface = -100），不是全零。
    assert!(s.data.iter().all(|&v| v == BrickTransformSsbo::default()));
    assert_eq!(s.active_bitset.len(), 0x400);
    assert_eq!(s.dirty_bitset.len(), 0x400);
    let free = s.free_chunks.read();
    assert_eq!(free.len(), 0x10000);
    assert_eq!((free[0], free[0xffff]), (0xffff, 0));
    drop(free);
    // `push<BrickTransformSsbo>` 0x1412e4450：整块 0x60 B 写入影子 + dirty/active 位。
    let v = BrickTransformSsbo {
        seed: 9,
        material: 0x8000_0001,
        wall_distance: [0.1, 0.1],
        flags: 6,
        ..BrickTransformSsbo::default()
    };
    let a = s.new_allocation();
    {
        let mut w = s.writer(&a);
        w.push(&v).unwrap();
    }
    assert_eq!(s.data[0], v);
    assert_eq!(s.byte_map[0], 1);
    // 库键与 pass 侧 RG 哈希一致（同一 xxh3 短名）。
    assert_eq!(
        <BrickTransformSsbo as PassSsboType>::rg_buffer_hash(),
        xxhash_rust::xxh3::xxh3_64(<BrickTransformSsbo as StorageElement>::NAME.as_bytes())
    );
}
