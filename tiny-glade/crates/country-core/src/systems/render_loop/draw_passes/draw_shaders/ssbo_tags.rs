//! ssbo_tags.rs — draw_shaders 里被 `DrawBuilder::read_storage_buffer<T>` 读取的 SSBO 类型标签。
//!
//! 为什么需要：`read_storage_buffer<T>` 在机器码里有 25+ 个单态化实例（0x14162a190 起，`T` 各一份），实例体里的
//! 常量只有两样——**槽名**（`HashMap<&str, PassResourceSlot>::rustc_entry` 的键，即 `T::NAME`）与
//! **`SsboId`**（`get_rg_buffer<T>` 里的 `NameHash(xxh3_64(T::NAME))` 立即数）。这些 `T` 的真身分属别的模块
//! （compute_passes / resources / geometry / systems::ivy…）且多数还没有 `SsboType` 实现，所以这里给每个 `T`
//! 一个零大小标签：槽名与 id 全部来自 `ssbo_info.py` 对 EXE 的批量抽取（非手抄），测试用真实 `xxhash-rust`
//! 复算 `xxh3_64(NAME) == id`，把“槽名就是 NAME”这一推断钉死。
//!
//! 真实 SSBO 类型就绪并实现 `SsboType` 后，把调用点的标签换成真实类型即可（`rg_buffer_hash` 值必须相同，
//! 由测试保证）。O = 机器码观测，I = 推断。

use xxhash_rust::xxh3::xxh3_64;

use crate::render::pass_builder::draw_builder::DrawBuilder;
use crate::render::pass_builder::pass_resource_access::SsboType;
use crate::rhapsody_contract::ManagedHandle;

/// 能被 `read_storage_buffer` 读取的 SSBO：除 RG 哈希外还带槽名（O：`rustc_entry` 的键）与完整类型名（O：
/// `get_rg_buffer` 的 panic 文案 / `BufferState.name`）。
pub trait DrawSsbo: SsboType {
    /// 槽名 = `StorageElement::NAME`（O：各实例 `rustc_entry` 的字符串）。
    const SLOT_NAME: &'static str;
    /// 完整类型名（O：`get_rg_buffer<T>` 里的 `type_name::<T>()`）。
    const TYPE_NAME: &'static str;
}

/// `DrawBuilder::read_storage_buffer` 的按标签入口（槽名取自标签）。
pub trait ReadSsboExt<'p>: Sized {
    fn read_ssbo<T: DrawSsbo>(self, binding_name_hash: u64) -> Self;
}

impl<'p> ReadSsboExt<'p> for DrawBuilder<'p> {
    fn read_ssbo<T: DrawSsbo>(self, binding_name_hash: u64) -> Self {
        self.read_storage_buffer::<T>(T::SLOT_NAME, binding_name_hash)
    }
}

/// `AssetSsboLibrary` 虚表槽 +0x18 的返回值（I）：实例缓冲视图。
///
/// 证据（O）：`ivy_leaf_shader` / `draw_instanced_roof_spire_shader` / `…support_beam_shader` 里
/// `FrozenMap::get(SsboId)` → `assert_eq!(state.name, type_name::<T>())` → `call [vtable+0x18]` → 非空时 `mov rsi, [rax]`
/// 作为 `draw_instanced_w_gl_draw_flag` 的实例数；`indirect_draw_bricks` 则把同一个返回值 `+0x68` 当作
/// `&ManagedHandle` 交给 `bind_external_storage_buffer`。两者吻合 `InstanceBuffer<T> { instance_count, buffer }`
/// （`+0` 实例数、`+8` 缓冲，缓冲内句柄在 `+0x60`）。因此该槽是“取出 `InstanceBuffer`，非实例缓冲则为空”，而不是
/// A3 注释里推测的 `buffer()`（A3 `Ssbo::buffer` 返回 `&self.buffer`，对不上 `[rax]` 是实例数这一事实）。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstanceBufferView {
    pub instance_count: u64,
    pub handle: ManagedHandle,
}

/// 库侧查询边界：`FrozenMap<SsboId, Box<BufferState>>::get` + `BufferState.name` + 虚表槽 +0x18。
pub trait SsboInstanceView {
    /// 返回 `(state.name, 实例缓冲)`；表里没有该 id → `None`。
    fn lookup(&self, ssbo_id: u64) -> Option<(&str, Option<InstanceBufferView>)>;
}

/// 取 `T` 的实例缓冲视图（O 的调用序列）：未注册 → `None`；名字不符 → `assert_failed<&str,&str>`
/// （`assertion `left == right` failed`）；非实例缓冲 → `None`。
pub fn instance_buffer<T: DrawSsbo>(lib: &dyn SsboInstanceView) -> Option<InstanceBufferView> {
    let (name, buffer) = lib.lookup(xxh3_64(T::SLOT_NAME.as_bytes()))?;
    assert_eq!(name, T::TYPE_NAME);
    buffer
}

/// 实例数：缺省 0（O：`xor esi, esi`）。
pub fn instance_count<T: DrawSsbo>(lib: &dyn SsboInstanceView) -> u64 {
    instance_buffer::<T>(lib).map_or(0, |b| b.instance_count)
}

macro_rules! ssbo_tag {
    ($(#[$m:meta])* $tag:ident, $ty:literal, $slot:literal, $id:expr) => {
        $(#[$m])*
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub struct $tag;
        impl SsboType for $tag {
            fn rg_buffer_hash() -> u64 {
                $id
            }
        }
        impl DrawSsbo for $tag {
            const SLOT_NAME: &'static str = $slot;
            const TYPE_NAME: &'static str = $ty;
        }
    };
}

ssbo_tag!(
    /// `resources::water::RippleSourceSsbo`（0x14162a190）。
    RippleSourceTag,
    "country_core::resources::water::RippleSourceSsbo",
    "RippleSourceSsbo",
    0x28351c5dbc9c80fb
);
ssbo_tag!(
    /// `resources::color_id_roof::RoofColorIdSsbo`（0x14162ad90）。
    RoofColorIdTag,
    "country_core::resources::color_id_roof::RoofColorIdSsbo",
    "RoofColorIds",
    0x8cce418b17d89f9b
);
ssbo_tag!(
    /// `compute_water::LilypadSsbo`（0x14162b390）。
    LilypadTag,
    "country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo",
    "Lilypads",
    0x321bc3dfb10d5554
);
ssbo_tag!(
    /// `compute_water::LilypadFlowerSsbo`（0x14162fe90）。
    LilypadFlowerTag,
    "country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo",
    "Lilypad Flowers",
    0xae37cbcafbe88da4
);
ssbo_tag!(
    /// `compute_water::ReedSsbo`（0x14162bf90）。
    ReedTag,
    "country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo",
    "Reeds",
    0xd6d72e989e1b8586
);
ssbo_tag!(
    /// `compute_passes::leaf_particles::LeafParticleSsbo`（0x14162b990）。
    LeafParticleTag,
    "country_core::systems::render_loop::compute_passes::leaf_particles::LeafParticleSsbo",
    "LeafParticles",
    0xda2386d830255aac
);
ssbo_tag!(
    /// `resources::walls::dormers::DormerHoleCountPerCellSsbo`（0x14162bc90）。
    DormerHoleCountTag,
    "country_core::resources::walls::dormers::DormerHoleCountPerCellSsbo",
    "Dormer Hole Count",
    0x96675839a35d7e8f
);
ssbo_tag!(
    /// `resources::walls::dormers::DormerHoleSsbo`（0x141631090）。
    DormerHoleTag,
    "country_core::resources::walls::dormers::DormerHoleSsbo",
    "Dormer Holes",
    0x13c9c799697a906c
);
ssbo_tag!(
    /// `compute_passes::stairs::StairsTransformSsbo`（0x14162cb90）。
    StairsTransformTag,
    "country_core::systems::render_loop::compute_passes::stairs::StairsTransformSsbo",
    "Stairs transforms",
    0xc405372d6efc32d3
);
ssbo_tag!(
    /// `geometry::instanced_wall::BrickDrawListSsbo`（0x14162d190）。
    BrickDrawListTag,
    "country_core::geometry::instanced_wall::BrickDrawListSsbo",
    "BrickDrawListSsbo",
    0x72c19119a2cdd0a1
);
ssbo_tag!(
    /// `geometry::instanced_wall::BrickTransformSsbo`（`indirect_draw_bricks` 里 `FrozenMap::get` 的键 0x142c2e2b8，
    /// 类型名 0x3a 字节；槽名 "BrickTransforms" 由 EXE 字符串全表按 `xxh3_64` 反查得到，见测试）。
    BrickTransformTag,
    "country_core::geometry::instanced_wall::BrickTransformSsbo",
    "BrickTransforms",
    0xeb57_5d06_1b90_ebf5
);
ssbo_tag!(
    /// `compute_passes::path_pebbles::PebblePlacementSsbo`（0x14162d490）。
    PebblePlacementTag,
    "country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo",
    "PebblePlacement",
    0x392675f5c9079eea
);
ssbo_tag!(
    /// `compute_passes::dof_autofocus::DofAutofocusSsbo`（0x14162d790）。
    DofAutofocusTag,
    "country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo",
    "DofAutofocusSsbo",
    0x530b5658f3632b30
);
ssbo_tag!(
    /// `systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo`（0x14162da90）。
    IvyLeafTag,
    "country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo",
    "Ivy Leaf Buffer",
    0x9e3cfa7f2993f406
);
ssbo_tag!(
    /// `systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo`（0x14162f590）。
    IvyFlowerTag,
    "country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo",
    "Ivy Flower Buffer",
    0x1d6de5e69ac50d2a
);
ssbo_tag!(
    /// `systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo`（0x141630490）。
    IvySegmentTag,
    "country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo",
    "Ivy Segments Buffer",
    0x2002d2fbdff3d80f
);
ssbo_tag!(
    /// `resources::rocky_terrain::RockyTerrainTriangleSSBO`（0x14162e990）。
    RockyTerrainTriangleTag,
    "country_core::resources::rocky_terrain::RockyTerrainTriangleSSBO",
    "Rocky Terrain Triangles",
    0x2a397285cdc5c9db
);
ssbo_tag!(
    /// `resources::rocky_terrain::RockyTerrainVertexSSBO`（0x14162f290）。
    RockyTerrainVertexTag,
    "country_core::resources::rocky_terrain::RockyTerrainVertexSSBO",
    "Rocky Terrain Vertices",
    0x1c5c4204bf6436bb
);
ssbo_tag!(
    /// `resources::roof_tile_ssbo::RoofTileSsbo`（0x14162ec90）。
    RoofTileTag,
    "country_core::resources::roof_tile_ssbo::RoofTileSsbo",
    "RoofTileSsbo",
    0x13d939590d705099
);
ssbo_tag!(
    /// `resources::roof_tile_ssbo::RoofSpireSsbo`（0x141630d90）。注意槽名在原版里就是 "RoofSpearTransforms"（拼写如此）。
    RoofSpireTag,
    "country_core::resources::roof_tile_ssbo::RoofSpireSsbo",
    "RoofSpearTransforms",
    0x3d1661290bab42e1
);
ssbo_tag!(
    /// `resources::roofs::roof_buffers::RoofSupportBeamSsbo`（0x14162ef90）。
    RoofSupportBeamTag,
    "country_core::resources::roofs::roof_buffers::RoofSupportBeamSsbo",
    "RoofSupportBeams",
    0x83773598f5ecfead
);
ssbo_tag!(
    /// `compute_passes::grass::GrassBladeDataSsbo`（0x141630790）。
    GrassBladeDataTag,
    "country_core::systems::render_loop::compute_passes::grass::GrassBladeDataSsbo",
    "GrassBlades",
    0x821b83720fc03446
);
// ---- indirect_params 的四个真实库侧 SSBO 类型（B 阶段 i_infra：不再用零大小标签）。
// `rg_buffer_hash` = `SsboId` 立即数（O：`get_rg_buffer<T>`，= xxh3_64(NAME)）；`DrawIndirectBuffer` /
// `DrawIndexedIndirectBuffer` 在 `multi_draw*indirect` 里只用 RG 哈希（O：0x1409076b0 / 0x140907990），没有槽名，
// 但同一个类型也被 compute 侧按库键使用，所以 `DrawSsbo` 也一并实现（槽名 = NAME，O：`AtomicCounterBuffer` 的
// `rustc_entry` 键 0x141631390）。
use crate::systems::render_loop::indirect_params::{
    AtomicCounterBuffer, DispatchIndirectBuffer, DrawIndexedIndirectBuffer, DrawIndirectBuffer,
    ssbo_ids,
};

macro_rules! indirect_ssbo_impl {
    ($ty:ident, $id:expr, $name:literal) => {
        impl SsboType for $ty {
            fn rg_buffer_hash() -> u64 {
                $id
            }
        }
        impl DrawSsbo for $ty {
            const SLOT_NAME: &'static str = $name;
            const TYPE_NAME: &'static str = concat!(
                "country_core::systems::render_loop::indirect_params::",
                $name
            );
        }
    };
}
indirect_ssbo_impl!(
    AtomicCounterBuffer,
    ssbo_ids::ATOMIC_COUNTER,
    "AtomicCounterBuffer"
);
indirect_ssbo_impl!(
    DrawIndirectBuffer,
    ssbo_ids::DRAW_INDIRECT,
    "DrawIndirectBuffer"
);
indirect_ssbo_impl!(
    DispatchIndirectBuffer,
    ssbo_ids::DISPATCH_INDIRECT,
    "DispatchIndirectBuffer"
);
indirect_ssbo_impl!(
    DrawIndexedIndirectBuffer,
    ssbo_ids::DRAW_INDEXED_INDIRECT,
    "DrawIndexedIndirectBuffer"
);

/// 旧标签名的别名（`draw_callbacks_b` 等调用方仍按旧名引用；类型即真实 SSBO）。
pub type AtomicCounterTag = AtomicCounterBuffer;
/// `multi_draw_indirect` 的缓冲（O：0x1409076b0）。
pub type DrawIndirectTag = DrawIndirectBuffer;
/// `multi_draw_indexed_indirect` 的缓冲（O：0x140907990）。
pub type DrawIndexedIndirectTag = DrawIndexedIndirectBuffer;

#[cfg(test)]
mod tests {
    use super::*;
    use xxhash_rust::xxh3::xxh3_64;

    fn check<T: DrawSsbo>() {
        assert_eq!(
            T::rg_buffer_hash(),
            xxh3_64(T::SLOT_NAME.as_bytes()),
            "{}: SsboId 必须等于 xxh3_64(槽名)",
            T::TYPE_NAME
        );
        assert!(T::TYPE_NAME.starts_with("country_core::"));
    }

    #[test]
    fn every_tag_id_is_xxh3_of_its_slot_name() {
        check::<RippleSourceTag>();
        check::<RoofColorIdTag>();
        check::<LilypadTag>();
        check::<LilypadFlowerTag>();
        check::<ReedTag>();
        check::<LeafParticleTag>();
        check::<DormerHoleCountTag>();
        check::<DormerHoleTag>();
        check::<StairsTransformTag>();
        check::<BrickDrawListTag>();
        check::<BrickTransformTag>();
        check::<PebblePlacementTag>();
        check::<DofAutofocusTag>();
        check::<IvyLeafTag>();
        check::<IvyFlowerTag>();
        check::<IvySegmentTag>();
        check::<RockyTerrainTriangleTag>();
        check::<RockyTerrainVertexTag>();
        check::<RoofTileTag>();
        check::<RoofSpireTag>();
        check::<RoofSupportBeamTag>();
        check::<GrassBladeDataTag>();
        check::<AtomicCounterBuffer>();
        check::<DispatchIndirectBuffer>();
        check::<DrawIndirectBuffer>();
        check::<DrawIndexedIndirectBuffer>();
    }

    #[test]
    fn indirect_buffer_ids_match_the_names_of_their_types() {
        // 原版没有给间接缓冲单独的槽名；用类型短名复算也能对上，说明 NAME == 类型短名。
        assert_eq!(
            DrawIndirectTag::rg_buffer_hash(),
            xxh3_64(b"DrawIndirectBuffer")
        );
        assert_eq!(
            DrawIndexedIndirectTag::rg_buffer_hash(),
            xxh3_64(b"DrawIndexedIndirectBuffer")
        );
    }

    struct Lib(Vec<(u64, &'static str, Option<InstanceBufferView>)>);
    impl SsboInstanceView for Lib {
        fn lookup(&self, id: u64) -> Option<(&str, Option<InstanceBufferView>)> {
            self.0
                .iter()
                .find(|e| e.0 == id)
                .map(|e| (e.1, e.2.clone()))
        }
    }

    #[test]
    fn instance_count_defaults_to_zero_when_unregistered_or_not_an_instance_buffer() {
        assert_eq!(
            instance_count::<IvyLeafTag>(&Lib(vec![])),
            0,
            "FrozenMap::get 为空 → xor esi, esi"
        );
        let lib = Lib(vec![(
            IvyLeafTag::rg_buffer_hash(),
            IvyLeafTag::TYPE_NAME,
            None,
        )]);
        assert_eq!(instance_count::<IvyLeafTag>(&lib), 0, "槽 +0x18 返回空 → 0");
        let view = InstanceBufferView {
            instance_count: 17,
            handle: ManagedHandle(5),
        };
        let lib = Lib(vec![(
            IvyLeafTag::rg_buffer_hash(),
            IvyLeafTag::TYPE_NAME,
            Some(view.clone()),
        )]);
        assert_eq!(instance_count::<IvyLeafTag>(&lib), 17);
        assert_eq!(instance_buffer::<IvyLeafTag>(&lib), Some(view));
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn instance_lookup_checks_the_registered_type_name() {
        let lib = Lib(vec![(
            IvyLeafTag::rg_buffer_hash(),
            "some::other::Type",
            None,
        )]);
        let _ = instance_count::<IvyLeafTag>(&lib);
    }

    #[test]
    fn roof_spire_slot_keeps_the_original_spelling() {
        assert_eq!(RoofSpireTag::SLOT_NAME, "RoofSpearTransforms");
    }
}
