//! `upload_world_raster.rs` — W1 `upload_raster_staging_to_texture<T>` ×6（O：0x1403d8bd0 PathRaster、
//! 0x1403d90f0 Cobblestone3Raster、0x1403d9610 WaterRaster、0x1403d9b30 GardenRaster、0x1403da050 Cobblestone1Raster、
//! 0x1403da570 Cobblestone2Raster；六个实例机器码同形，各 1306 B，只差 TypeId 与暂存 SSBO 类型）。
//!
//! 参数（O）：`(ctx, world, preset: u8)`。流程：
//! 1. `get_resource_id(<R 的 TypeId>)`（缺失 → unwrap panic）；读 `world.change_tick(+0x5b0)`。
//! 2. 栅格资源 `+0x29a`（"上传待办"）为 0 → 直接返回。
//! 3. `TexturePreset::from(preset)` 取 desc；`desc.format`：`0x4c`(R16_SFLOAT) → 后缀 `"r16f"`，`0x61`(R16G16B16A16_SFLOAT) →
//!    `"rgba16f"`，其它 → `todo!`（panic "not yet implemented: {Format:?}"，**先于任何状态修改**）。
//! 4. 资源 `changed_tick(entry+0x5c) = world.change_tick`；`word [res+0x29a] = 0x0100`，即
//!    **`+0x29a = 0`、`+0x29b = 1`**（`+0x29b` 是下游 `compute_path_masks` / `garden_rasterize_from_cpu` 的"遮罩待模糊"标志）。
//! 5. `uri = format!("/upload_cpu_raster.cs.hlsl:{suffix}")`；`ComputePass::new<String>("Upload cpu raster", uri)`：
//!    - `read_storage_buffer<RStagingSsbo>`（`cpu_raster` = 0x2f52917136b0d5bb）；
//!    - `bind_texture(TexturePreset(preset), BindingNameHash::new(format!("output_tex_{suffix}")), 0x32)`；
//!    - `write_uniform<u32>`（`output_tex_width` = 0xb66175286d72dcb9，值 **0x200**）；
//!    - `dispatch_2d(0x80, 0x200)`（O：`rdx = 0x20000000080`）。
//!
//! 栅格资源的可见布局（O，各类型相同）：脏行表 `+0x98`（0x200 B，由 `compute_path_masks` / `garden_rasterize_from_cpu` 清零）、
//! 上传标志 `+0x29a`、遮罩标志 `+0x29b`。其余布局 U。
//!
//! 暂存 SSBO 泛型身份（O，B PDB `read_storage_buffer` 六实例 0x140588650..0x140588f60）均为
//! `resources::world_raster_defs::*RasterStagingSsbo`。直接复用真实 producer 的 512×512 u8 mapped buffer；
//! 不能用同 NAME 的零大小标记另建一份，否则库的完整类型名校验会拒绝。

use std::cell::Cell;

use crate::asset_libraries::ssbo_library::SsboType as LibrarySsbo;
use crate::render::pass_builder::pass_resources::TypeId128;
use crate::resources::texture_presets::TexturePreset;
use crate::systems::render_loop::compute_passes::pass_kit::{
    PassEnv, ResourceView, TEX_WRITE, binding_hash,
};

pub use crate::resources::world_raster_defs::{
    Cobblestone1RasterStagingSsbo as Cobblestone1RasterStagingBuf,
    Cobblestone2RasterStagingSsbo as Cobblestone2RasterStagingBuf,
    Cobblestone3RasterStagingSsbo as Cobblestone3RasterStagingBuf,
    GardenRasterStagingSsbo as GardenRasterStagingBuf,
    PathRasterStagingSsbo as PathRasterStagingBuf, WaterRasterStagingSsbo as WaterRasterStagingBuf,
};

pub const PASS_NAME: &str = "Upload cpu raster";
pub const URI_PREFIX: &str = "/upload_cpu_raster.cs.hlsl:";
pub const BINDING_PREFIX: &str = "output_tex_";
pub const HASH_CPU_RASTER: u64 = binding_hash("cpu_raster"); // 0x2f52917136b0d5bb
pub const KEY_WIDTH: u64 = binding_hash("output_tex_width"); // 0xb66175286d72dcb9
pub const WIDTH_VALUE: u32 = 0x200;
pub const DISPATCH: (u32, u32) = (0x80, 0x200);
/// `ash::vk::Format::R16_SFLOAT` / `R16G16B16A16_SFLOAT`（O：`cmp eax, 0x4c` / `cmp eax, 0x61`）。
pub const FORMAT_R16F: u32 = 0x4c;
pub const FORMAT_RGBA16F: u32 = 0x61;

/// 栅格资源的可见部分。
#[repr(C)]
pub struct RasterFlags {
    _head: [u8; 0x98],
    /// `+0x98`：0x200 B 脏行表。
    pub dirty_rows: [Cell<u8>; 0x200],
    _gap: [u8; 2],
    /// `+0x29a`：上传待办。
    pub upload_pending: Cell<bool>,
    /// `+0x29b`：遮罩待模糊 / 待处理。
    pub mask_dirty: Cell<bool>,
}

impl RasterFlags {
    pub fn new(upload_pending: bool, mask_dirty: bool) -> Self {
        RasterFlags {
            _head: [0; 0x98],
            dirty_rows: core::array::from_fn(|_| Cell::new(0xff)),
            _gap: [0; 2],
            upload_pending: Cell::new(upload_pending),
            mask_dirty: Cell::new(mask_dirty),
        }
    }

    /// `memset(res + 0x98, 0, 0x200)`（O：下游两个 pass 函数里的清脏行）。
    pub fn clear_dirty_rows(&self) {
        for b in &self.dirty_rows {
            b.set(0);
        }
    }

    /// 下游 pass 的"消费"动作（O：`byte [res+0x29b] = 0` + `memset(+0x98, 0, 0x200)`）。
    pub fn consume_mask_dirty(&self) {
        self.mask_dirty.set(false);
        self.clear_dirty_rows();
    }
}

/// 一个可上传的世界栅格（`world_raster_defs::*Raster`）。
pub trait RasterKind: ResourceView {
    /// 暂存 SSBO（`read_storage_buffer<…RasterStagingSsbo>`）。
    type Staging: LibrarySsbo;
    fn flags(&self) -> &RasterFlags;
}

/// A source-build ECS resource and its native upload-view identity. The pair
/// fixes the staging type without claiming that the two Rust layouts match.
pub trait LiveRasterKind:
    bevy_ecs::resource::Resource
    + core::ops::DerefMut<Target = crate::resources::world_raster::WorldRaster<512>>
{
    type View: RasterKind;
}

macro_rules! raster_kinds {
    ($($raster:ident, $staging:ident, $name:literal, $lo:literal, $hi:literal;)*) => {$(
        /// `resources::world_raster_defs` 同名栅格资源的可见头部（布局见 [`RasterFlags`]）。
        #[repr(transparent)]
        pub struct $raster(pub RasterFlags);

        // SAFETY: 布局按 `+0x98` / `+0x29a` / `+0x29b` 的读写偏移复刻。
        unsafe impl ResourceView for $raster {
            const TYPE_ID: TypeId128 = TypeId128 { lo: $lo, hi: $hi };
        }

        impl RasterKind for $raster {
            type Staging = $staging;
            fn flags(&self) -> &RasterFlags {
                &self.0
            }
        }

        impl LiveRasterKind for crate::resources::world_raster_defs::$raster {
            type View = $raster;
        }
    )*};
}

raster_kinds! {
    PathRaster, PathRasterStagingBuf, "PathRasterStagingSsbo", 0x85c987d23d58d2de, 0x2ec6ab231f77203b;
    Cobblestone1Raster, Cobblestone1RasterStagingBuf, "Cobblestone1RasterStagingSsbo", 0xbb8761bcc7907109, 0xdc4ef498cea8673a;
    Cobblestone2Raster, Cobblestone2RasterStagingBuf, "Cobblestone2RasterStagingSsbo", 0x86486f3e82eb6bbe, 0x4d7091c2f9bfa189;
    Cobblestone3Raster, Cobblestone3RasterStagingBuf, "Cobblestone3RasterStagingSsbo", 0x6dc90dcfd23786fa, 0xed952bd348f66c04;
    WaterRaster, WaterRasterStagingBuf, "WaterRasterStagingSsbo", 0x1ad182936286a0ee, 0xac04e3b6f9127e6f;
    GardenRaster, GardenRasterStagingBuf, "GardenRasterStagingSsbo", 0x979ffa6288aa259a, 0x09791d5b46ed449d;
}

/// 着色器入口 / 绑定名后缀（O：`format` 0x4c → "r16f"，0x61 → "rgba16f"，其它 → `todo!`）。
pub fn format_suffix(format: u32) -> &'static str {
    match format {
        FORMAT_R16F => "r16f",
        FORMAT_RGBA16F => "rgba16f",
        other => panic!("not yet implemented: Format({other})"),
    }
}

/// W1 `upload_raster_staging_to_texture<R>`（O，见文件头）。返回 `Some(world_change_tick)`：栅格资源的
/// `changed_tick` 被刷新为它；无待办上传返回 `None`。
pub fn upload_world_raster<'a, R: RasterKind + 'a>(
    env: &mut PassEnv<'a>,
    world_change_tick: u32,
    preset: u8,
) -> Option<u32> {
    let raster = env.resource::<R>();
    let flags = raster.flags();
    if !flags.upload_pending.get() {
        return None;
    }
    declare_upload::<R::Staging>(env, preset, || {
        flags.upload_pending.set(false);
        flags.mask_dirty.set(true);
    });
    Some(world_change_tick)
}

/// Source-build host adapter for the actual ECS raster producer. The caller
/// resolves the mandatory, correctly named Path/Cobblestone/Water/Garden
/// resource and supplies its actual Bevy Mut, never a pointer cast to the old
/// compiler's RasterFlags layout. The first pending read does not mark it
/// changed; original 0x1403d8cd7 marks changed only after format validation,
/// immediately before 0x1403d8ce3 writes +29a/+29b. Keep that order even when
/// subsequent pass construction fails.
pub fn upload_live_world_raster<R: LiveRasterKind>(
    env: &mut PassEnv<'_>,
    raster: &mut bevy_ecs::change_detection::Mut<'_, R>,
    world_change_tick: u32,
    preset: u8,
) -> Option<u32> {
    use bevy_ecs::change_detection::DetectChangesMut;
    if !raster.bypass_change_detection().published() {
        return None;
    }
    declare_upload::<<R::View as RasterKind>::Staging>(env, preset, || {
        raster.set_changed();
        raster.bypass_change_detection().mark_staging_uploaded();
    });
    Some(world_change_tick)
}

fn declare_upload<Staging: LibrarySsbo>(
    env: &mut PassEnv<'_>,
    preset: u8,
    mark_uploaded: impl FnOnce(),
) {
    let suffix = format_suffix(env.preset_desc(preset).format);
    mark_uploaded();
    let uri = format!("{URI_PREFIX}{suffix}");
    let name_hash = binding_hash(&format!("{BINDING_PREFIX}{suffix}"));
    let recorded = env
        .pass(PASS_NAME, &uri)
        .read_storage_buffer::<Staging>(HASH_CPU_RASTER)
        .bind_texture(name_hash, &TexturePreset(preset), TEX_WRITE)
        .uniform(KEY_WIDTH, WIDTH_VALUE)
        .dispatch_2d(DISPATCH.0, DISPATCH.1);
    env.record(recorded);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::compute_pass_builder::DispatchMode;
    use crate::rhapsody_types::TextureDesc;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::*;

    /// 预设 3 的格式 = `format`（`get_rg_texture(3).desc.format`，决定入口点后缀）。
    fn world<R: RasterKind + 'static>(raster: R, suffix: &str, format: u32) -> TestWorld {
        TestWorld::with_presets(
            Reflection::new()
                .bind(&[
                    HASH_CPU_RASTER,
                    binding_hash(&format!("output_tex_{suffix}")),
                ])
                .uniforms(&[KEY_WIDTH]),
            move |p| {
                if p.0 == 3 {
                    TextureDesc::new_2d([512, 512], format).mip_levels(1)
                } else {
                    DEFAULT_PRESET_DESC
                }
            },
        )
        .with_ssbo::<R::Staging>()
        .with_resource(raster)
    }

    #[test]
    fn not_pending_returns_without_touching_anything() {
        let w = world(
            PathRaster(RasterFlags::new(false, false)),
            "r16f",
            FORMAT_R16F,
        );
        let mut f = TestFrame::default();
        assert_eq!(
            with_env(&w, &mut f, |env| upload_world_raster::<PathRaster>(
                env, 9, 3
            )),
            None
        );
        assert!(f.passes.is_empty());
        assert_eq!(w.backend.loads.get(), 0);
    }

    #[test]
    fn live_ecs_producer_and_upload_consumer_share_flags_and_precise_change_tick() {
        use crate::resources::{
            world_raster::DirtyTilePublisher, world_raster_defs::PathRaster as LivePathRaster,
        };
        use bevy_ecs::{change_detection::DetectChanges, world::World};
        let valid = world(
            PathRaster(RasterFlags::new(false, false)),
            "r16f",
            FORMAT_R16F,
        );
        let invalid = world(PathRaster(RasterFlags::new(false, false)), "r16f", 0);
        let mut ecs = World::new();
        ecs.insert_resource(LivePathRaster::new(DirtyTilePublisher::unique()));
        ecs.clear_trackers();
        let mut frame = TestFrame::default();
        assert_eq!(
            with_env(&valid, &mut frame, |env| upload_live_world_raster(
                env,
                &mut ecs.resource_mut::<LivePathRaster>(),
                7,
                3
            )),
            None
        );
        assert!(
            !ecs.get_resource_ref::<LivePathRaster>()
                .unwrap()
                .is_changed()
        );
        assert!(frame.passes.is_empty());

        // This is the actual CPU producer, not a copied RasterFlags object.
        ecs.resource_mut::<LivePathRaster>()
            .copy_to_staging(0, &mut vec![0; 512 * 512]);
        ecs.clear_trackers();
        let bad = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            with_env(&invalid, &mut frame, |env| {
                upload_live_world_raster(env, &mut ecs.resource_mut::<LivePathRaster>(), 8, 3)
            });
        }));
        assert!(bad.is_err());
        let live = ecs.get_resource_ref::<LivePathRaster>().unwrap();
        assert!(
            !live.is_changed(),
            "unsupported format fails before original changed_tick write"
        );
        assert!(live.published());
        assert!(!live.reupload_pending());
        assert!(frame.passes.is_empty());

        assert_eq!(
            with_env(&valid, &mut frame, |env| upload_live_world_raster(
                env,
                &mut ecs.resource_mut::<LivePathRaster>(),
                9,
                3
            )),
            Some(9)
        );
        let live = ecs.get_resource_ref::<LivePathRaster>().unwrap();
        assert!(live.is_changed());
        assert!(!live.published());
        assert!(live.reupload_pending());
        assert_eq!(frame.passes.len(), 1);
    }

    #[test]
    fn r16f_upload_records_one_pass_and_flips_flags() {
        let w = world(
            PathRaster(RasterFlags::new(true, false)),
            "r16f",
            FORMAT_R16F,
        );
        let mut f = TestFrame::default();
        assert_eq!(
            with_env(&w, &mut f, |env| upload_world_raster::<PathRaster>(
                env, 1234, 3
            )),
            Some(1234)
        );
        let r = cx_raster::<PathRaster>(&w);
        assert!(!r.0.upload_pending.get(), "+0x29a 清零");
        assert!(r.0.mask_dirty.get(), "+0x29b 置位");
        let p = &f.passes[0];
        assert_eq!(p.name, "Upload cpu raster");
        assert_eq!(
            w.backend.paths.borrow()[0],
            "/upload_cpu_raster.cs.hlsl:r16f"
        );
        assert!(matches!(
            p.dispatch,
            DispatchMode::Threads {
                x: 0x80,
                y: 0x200,
                z: 1
            }
        ));
        assert_eq!(uniform_u32(p, 0), 0x200);
        // 访问：color library 读、cpu_raster 读(0x1a)、输出写(0x32)。
        let acc: Vec<u8> = p.builder.accesses.iter().map(|a| a.access).collect();
        assert_eq!(acc, [0x1a, 0x1a, 0x32]);
    }

    #[test]
    fn rgba16f_uses_the_other_entry_point() {
        let w = world(
            WaterRaster(RasterFlags::new(true, false)),
            "rgba16f",
            FORMAT_RGBA16F,
        );
        let mut f = TestFrame::default();
        with_env(&w, &mut f, |env| {
            upload_world_raster::<WaterRaster>(env, 0, 3)
        });
        assert_eq!(
            w.backend.paths.borrow()[0],
            "/upload_cpu_raster.cs.hlsl:rgba16f"
        );
    }

    #[test]
    #[should_panic(expected = "not yet implemented: Format(37)")]
    fn other_formats_are_todo_and_leave_state_untouched() {
        let w = world(GardenRaster(RasterFlags::new(true, false)), "r16f", 37);
        let mut f = TestFrame::default();
        with_env(&w, &mut f, |env| {
            upload_world_raster::<GardenRaster>(env, 0, 3)
        });
    }

    #[test]
    fn format_panic_precedes_flag_mutation() {
        let w = world(GardenRaster(RasterFlags::new(true, false)), "r16f", 37);
        let mut f = TestFrame::default();
        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            with_env(&w, &mut f, |env| {
                upload_world_raster::<GardenRaster>(env, 0, 3)
            })
        }));
        assert!(res.is_err());
        let r = cx_raster::<GardenRaster>(&w);
        assert!(r.0.upload_pending.get() && !r.0.mask_dirty.get());
    }

    fn cx_raster<R: ResourceView>(w: &TestWorld) -> &R {
        use crate::render::pass_builder::pass_resources::ResourceLookup;
        unsafe { &*(w.get_resource_ptr(R::TYPE_ID).unwrap() as *const R) }
    }

    #[test]
    fn six_instances_have_distinct_type_ids_and_staging_names() {
        use crate::render::ssbo::StorageElement;
        let ids = [
            PathRaster::TYPE_ID,
            Cobblestone1Raster::TYPE_ID,
            Cobblestone2Raster::TYPE_ID,
            Cobblestone3Raster::TYPE_ID,
            WaterRaster::TYPE_ID,
            GardenRaster::TYPE_ID,
        ];
        for i in 0..6 {
            for j in i + 1..6 {
                assert_ne!(ids[i], ids[j]);
            }
        }
        assert_eq!(PathRaster::TYPE_ID.lo, (-0x7a36782dc2a72d22_i64) as u64);
        assert_eq!(WaterRasterStagingBuf::NAME, "WaterRasterStagingSsbo");
        assert_eq!(HASH_CPU_RASTER, 0x2f52917136b0d5bb);
        assert_eq!(KEY_WIDTH, (-0x499e8ad7928d2347_i64) as u64);
        assert_eq!(
            binding_hash("output_tex_r16f"),
            binding_hash("output_tex_r16f")
        );
    }

    #[test]
    fn flag_layout_matches_machine_code_offsets() {
        assert_eq!(core::mem::offset_of!(RasterFlags, dirty_rows), 0x98);
        assert_eq!(core::mem::offset_of!(RasterFlags, upload_pending), 0x29a);
        assert_eq!(core::mem::offset_of!(RasterFlags, mask_dirty), 0x29b);
    }

    #[test]
    fn consume_clears_flag_and_rows() {
        let f = RasterFlags::new(false, true);
        assert!(f.dirty_rows.iter().all(|b| b.get() == 0xff));
        f.consume_mask_dirty();
        assert!(!f.mask_dirty.get());
        assert!(f.dirty_rows.iter().all(|b| b.get() == 0));
    }
}
