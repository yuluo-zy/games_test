//! `tiny_glade::render_loop::render_world` (0x1404ec110, 4314 B) — 世界渲染图的整帧 pass 编排。
//!
//! 该函数属 **tiny_glade** 二进制 crate（`crates/tiny-glade/src/render_loop.rs`，PDB 名
//! `tiny_glade::render_loop::*`），不属于 country-core；因为 country-core 里的 pass 入口
//! （`compute_passes::*` / `draw_passes::*`）只有它在调用，所以放在 render_loop 旁边。
//!
//! 标注：O = 机器码观测，I = 推断，U = 未知。
//!
//! ## 形态
//! 原函数按调用顺序依次发出 54 个 pass；本文件把"调用顺序 + 分支条件 + 常量 + 资源查找/panic"逐条还原，
//! 每个 pass 入口通过 [`FramePasses`] 缝（trait）表达：方法名 = PDB 名，参数个数/顺序 = 机器码调用点
//! （rcx/rdx/r8/r9/栈）。`FramePasses` 的真实实现是 country-core 里 `compute_passes::*`/`draw_passes::*`
//! 的各入口 + rhapsody `RenderGraph`/`temporal`（见 `B_s_h.md` 的签名差异表）。
//!
//! ## 参数（O，MS x64：rcx 返回体、rdx rg、r8 world、r9 param_4、栈 param_5..8）
//! - `rg`（rdx）= `&mut RenderGraph`，`world`（r8）= `&World`：两者原样传给每个 pass，由实现者持有，
//!   所以在缝里不出现。
//! - `param_4` = 绘制列表 `&DrawLists`（I：传给 `shadow_pass`、`main_pass*`、`late_forward…`、透明 pass）。
//! - `param_5` = `world_render_dims: [u32;2]`（断言文本 O："assertion failed: world_render_dims[0] > 0 &&
//!   world_render_dims[1] > 0"，两维都必须非零）。
//! - `param_6` = 设置字节（O：`+0x9c` u32 → TAA/TGSR 末参；`+0xa9/+0xaa` 景深开关，下标 = `0xa9 + (dof_mode==0x13)`；
//!   `+0xab` TGSR 开关）。
//! - `param_7` = 深度纹理 `RgTexture`（9 个 u32：id + 32 B `TextureDesc`；I：深度，因为 `reprojection.prev_depth`
//!   取它的 desc 改 format=100、usage=0xc）。
//! - `param_8` = 场景颜色 `RgTexture`（I：`main_pass` 写入、TAA/DoF 读取、`extent_2d(desc)` 用于 TAA 历史尺寸）。

use core::fmt::Debug;

/// `rhapsody::resource::TextureDesc`（32 B，布局 O，取自 `new_2d` 0x140bd52a0 / `sampled` / `storage` / `format` /
/// `usage` / `extent_2d` 的字节写入）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TextureDesc {
    /// `+0x00` 用法位（`sampled` = `|= 0x4`，`storage` = `|= 0x8`，`usage(u)` 整体覆写）。
    pub usage: u32,
    /// `+0x04` 恒 0（`new_2d` 把 `[0..8)` 清零）。
    pub reserved: u32,
    /// `+0x08` 像素格式（`format(f)` 覆写）。
    pub format: u32,
    /// `+0x0c/+0x10` 宽高（`extent_2d` 读 `+0xc` 的 u64 = `(w,h)`）。
    pub extent: [u32; 2],
    /// `+0x14` = 1（O，`new_2d` 写 `0x1_0000_0001` 到 `+0x14`，另一半在 `+0x18`）。
    pub depth: u32,
    /// `+0x18` = 1。
    pub layers_or_mips: u32,
    /// `+0x1c` = 1、`+0x1d` = 1、`+0x1e` = 2（`new_2d` 写 `byte 1` + `word 0x0201`；语义 U）。
    pub tail: [u8; 4],
}

impl TextureDesc {
    /// `TextureDesc::new_2d(extent, format)`（0x140bd52a0）。
    pub fn new_2d(extent: [u32; 2], format: u32) -> Self {
        Self {
            usage: 0,
            reserved: 0,
            format,
            extent,
            depth: 1,
            layers_or_mips: 1,
            tail: [1, 1, 2, 0],
        }
    }
    /// `TextureDesc::sampled`（0x140bd5370）：`usage |= 4`。
    pub fn sampled(mut self) -> Self {
        self.usage |= 0x4;
        self
    }
    /// `TextureDesc::storage`（0x140bd5390）：`usage |= 8`。
    pub fn storage(mut self) -> Self {
        self.usage |= 0x8;
        self
    }
    /// `TextureDesc::format`（0x140bd5430）。
    pub fn with_format(mut self, format: u32) -> Self {
        self.format = format;
        self
    }
    /// `TextureDesc::usage`（0x140bd5340）：整体覆写。
    pub fn with_usage(mut self, usage: u32) -> Self {
        self.usage = usage;
        self
    }
    /// `TextureDesc::extent_2d`（0x140bd5790）：返回 `+0xc` 处的 `(w,h)`。
    pub fn extent_2d(&self) -> [u32; 2] {
        self.extent
    }
}

/// RG 纹理句柄（36 B：`id` + 32 B desc；O：`param_7/param_8` 与 `gbuffer`/`velocity` 的布局）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RgTexture {
    pub id: u32,
    pub desc: TextureDesc,
}

/// `get_or_create_temporal` 返回的单张历史纹理（40 B：`RgTexture` + `+0x24` 标志字节，O：1 = 历史有效，0 = 本帧新建）。
/// `+0x24 == 2` 是 `Result::Err` 的 niche（O：`cmp byte [rbp+0x364], 2; je err`）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TemporalTexture {
    pub tex: RgTexture,
    pub flag: u8,
}

/// `get_or_create_temporal_pair`（0x1404f7450）的结果：当前帧 + 上一帧（各 36 B）+ 末尾标志字节
/// （O：`+0x4c`，之后以 `& 1` 传给 `ray_trace_gi`、原样传给 deferred/透明 pass；1 表示历史有效）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TemporalPair {
    pub cur: RgTexture,
    pub prev: RgTexture,
    pub fresh: bool,
}

/// `extract_half_res_depth_normals` 返回的两张半分辨率纹理（O：sret 72 B = 两个 `RgTexture`，
/// 被分别拷到栈上的 `uStack_2d8`(A) 与 `auStack_358`(B)）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HalfResTargets {
    pub a: RgTexture,
    pub b: RgTexture,
}

/// 渲染图里 G-buffer 组（O：栈上 108 B = 三个 `RgTexture`：`gbuffer`、`param_7`(深度) 的拷贝、`velocity`）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GBufferTargets {
    pub gbuffer: RgTexture,
    pub depth: RgTexture,
    pub velocity: RgTexture,
}

// Typed current-core outputs. These names describe real consumer values,
// never byte arrays intended to imitate an original compiler's sret ABI.
pub use super::compute_passes::{
    bin_lights::LightBinning as LightBins, dof::DofPrepared as DofTargets,
    generate_hzb::Hzb as HzbHandles, ray_trace_gi::RestirTextures as RayTracedGi,
};
pub type PassHandle = crate::render::pass_builder::pass_resource_access::RgTexture;

/// `param_6` 指向的设置字节（O 偏移；仅读取这几项）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WorldRenderSettings {
    /// VideoSettings `+0x9c`: original sharpen float bits forwarded to AA.
    pub sharpen_amount: f32,
    /// `+0xa9`、`+0xaa`：景深开关，下标 `(dof_mode == 0x13) as usize`。
    pub dof_enabled: [bool; 2],
    /// `+0xab`：非 0 → TGSR，否则 TAA。
    pub tgsr: bool,
}

/// `render_world` 的输入。
#[derive(Clone, Copy, Debug)]
pub struct RenderWorldArgs {
    /// `param_5`：`world_render_dims`（O：断言两维 > 0）。
    pub world_render_dims: [u32; 2],
    pub settings: WorldRenderSettings,
    /// `param_7`。
    pub depth: RgTexture,
    /// `param_8`。
    pub color: RgTexture,
}

/// 返回体（O：sret 0x6c B，三个 36 B 片段依次在 `+0x00/+0x24/+0x48`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RenderWorldOutputs {
    /// `+0x00`：深度纹理拷贝（`uStack_2a8`，与 `param_7` 相同）。
    pub depth: RgTexture,
    /// `+0x24`：`dof_prepare` 内部两张纹理中的第一张 CoC；tile 纹理仅在函数内传给 calc_dof。
    pub dof_prepare: PassHandle,
    /// `+0x48`：TAA/TGSR 的输出。
    pub taa: PassHandle,
}

/// 世界渲染图里用到的纹理格式（O：`new_2d` 的 `r8d`）。
pub mod formats {
    /// `gbuffer`（0x1404ec404）。
    pub const GBUFFER: u32 = 0x65;
    /// `velocity`（0x1404ec48e）。
    pub const VELOCITY: u32 = 0x61;
    /// `reprojection.prev_depth`：`param_7.desc.format(100)`（0x1404ec221）。
    pub const PREV_DEPTH: u32 = 100;
    /// 历史/输出/折射源：`new_2d(..., 0x7a)`（0x1404ec7a2、0x1404ecd31、0x1404eca95）。
    pub const COLOR_HDR: u32 = 0x7a;
}

/// `TextureDesc::usage(0xc)` = sampled | storage（0x1404ec239）。
pub const PREV_DEPTH_USAGE: u32 = 0xc;
/// Original AppMode::Photomode tag. It selects the photomode setting and
/// quality byte; calc_dof reads its separate global multires switch internally.
pub const DOF_PHOTOMODE_TAG: u32 = 0x13;

/// 名字常量（O，.rdata 0x1429ebf98…0x1429ec060）。
pub mod names {
    pub const PREV_DEPTH: &str = "reprojection.prev_depth";
    pub const GBUFFER: &str = "gbuffer";
    pub const VELOCITY: &str = "velocity";
    pub const HZB: &str = "main";
    pub const TAA_HISTORY: &str = "taa_history";
    pub const TGSR_HISTORY: &str = "tgsr_history";
    pub const REFRACTION_SOURCE: &str = "refraction source";
    pub const DOF_MIPS: &str = "dof_mips";
    pub const TAA_OUTPUT: &str = "taa_output";
}

/// `upload_raster_staging_to_texture<…>` 的 6 个单态（O：`r8b` = 光栅槽，依次 1/2/3/4/0x13/0x14）。
pub const UPLOAD_RASTER_SLOTS: [u8; 6] = [1, 2, 3, 4, 0x13, 0x14];

/// 位置：`crates/tiny-glade/src/render_loop.rs` 的 panic 行号（O，取自 `core::panic::Location`）。
pub mod panic_lines {
    pub const ASSERT_DIMS: u32 = 0x3c5;
    pub const WORLD_CONFIG: u32 = 0x3c7;
    pub const PREV_DEPTH_TEMPORAL: u32 = 0x3d2;
    pub const TGSR_DIMS_HISTORY: u32 = 0x459;
    pub const HISTORY_PAIR: u32 = 0x477;
    pub const DOF_MODE: u32 = 0x4e6;
    pub const TGSR_DIMS_OUTPUT: u32 = 0x516;
}

/// `render_world` 的外部调用面。方法顺序即 `render_world` 内的调用顺序；所有入口的 `rg`/`world` 由实现者持有。
///
/// 返回 `Option`/`Result` 的方法对应原函数里 `Components::get_resource_id`→`Option::unwrap`
/// 或 `Result::unwrap`；`render_world` 对它们 `unwrap`（panic 文本同原版）。
pub trait FramePasses {
    type Error: Debug;

    // ---- 资源/全局查找（O：Components::get_resource_id + 存储表） ----
    /// 世界配置资源（TypeId 0x74430dfcc6d8719b/0xca5591f7e464509c，原版未命中 → `Option::unwrap`，行 967）
    /// 上调 `BackupSaveFileKind::is_corrupted(res+0x58)`（0x140ae8320，PDB 名为 ICF 别名；契约称 `is_winter`）。
    fn is_winter(&mut self) -> Option<bool>;
    /// 分辨率资源（TypeId 0xa4c02661e67b048b/0xeae761e6aa2a67ec）的显示尺寸（`+8` 处 u64）。
    fn display_dims(&mut self) -> Option<[u32; 2]>;
    /// Canonical AppMode native tag (TypeId c2b8e941c91a6428/59f3c6d1d3da43a0).
    fn dof_mode(&mut self) -> Option<u32>;
    /// `*0x14362c0d8` → 0x143634da0：**`freeze_occlusion_culling` 调试开关**（O，grass `B_c2.md` 已命名该地址；`.data` 指针 →
    /// 零填充区字节，加载时 0）。非 0 → 跳过 `generate_wall_draw_lists_pass2` 与 `gpu_generate_draw_lists`（`render-gap/DIGEST.md §3.4`）。
    /// 为 false 时跑第二轮墙体剔除。
    fn flag_c0d8(&mut self) -> bool;
    /// `*0x14362c098` → 0x143634d90：进程级 **demo 开关**（O，`lut_gallery`/`gpu_lut`/`vermeer` 共用的同一进程布尔）。非 0 → 跳过
    /// 折射/水面 pass 链（"demo 下跳过水折射"的语义为 I）。为 false 时跑折射/水面。
    fn flag_c098(&mut self) -> bool;

    // ---- rhapsody RenderGraph ----
    /// `rhapsody::rg::temporal::get_or_create_temporal<PassResources, &str>`（0x1404f6e50）。
    fn get_or_create_temporal(
        &mut self,
        name: &str,
        desc: &TextureDesc,
    ) -> Result<TemporalTexture, Self::Error>;
    /// `get_or_create_temporal_pair`（0x1404f7450）。
    fn get_or_create_temporal_pair(
        &mut self,
        name: &str,
        desc: &TextureDesc,
    ) -> Result<TemporalPair, Self::Error>;
    /// `RenderGraph::create_raw_resource`（0x140be94d0）；返回资源 id。
    fn create_raw_resource(&mut self, name: &str, desc: &TextureDesc) -> u32;

    // ---- W1-W24：地形/植被/水/剔除 ----
    fn upload_raster_staging_to_texture(&mut self, slot: u8);
    fn water_rasterize_from_cpu(&mut self);
    fn rasterize_terrain_stroke(&mut self);
    fn displace_rocky_terrain(&mut self);
    fn read_terrain_data(&mut self);
    fn mouse_read_terrain_stroke_index(&mut self);
    fn rasterize_foliage_exclusion_mask(&mut self);
    fn highlight_active_terrain_stroke(&mut self);
    fn object_grass_push_pass(&mut self);
    fn compute_path_masks(&mut self);
    fn rasterize_buildings(&mut self);
    fn garden_rasterize_from_cpu(&mut self);
    fn water_spawn_vegetation(&mut self);
    fn spawn_vegetation_around_buildings(&mut self);
    fn read_bush_positions_for_cpu(&mut self);
    fn garden_spawn_vegetation(&mut self);
    fn path_pebbles_compute(&mut self);
    fn leaf_particles_compute(&mut self);
    fn roof_tiles_gravity_compute(&mut self);
    fn stairs(&mut self);
    fn generate_wall_draw_lists(&mut self);
    /// `gpu_generate_draw_lists(rg, world, hzb)`：第一轮 `hzb = None`（O：r8 = 0），第二轮 `Some(hzb)`。
    fn gpu_generate_draw_lists(&mut self, hzb: Option<&HzbHandles>);
    fn shadow_pass(&mut self);
    fn calc_terrain_lighting_texture(&mut self);

    // ---- W25-W37：主 pass / HZB / GI 前置 ----
    fn main_pass(&mut self, gbuf: &GBufferTargets, color: &RgTexture);
    /// `generate_hzb(out, rg, world, &depth, name_ptr, name_len)`。
    fn generate_hzb(&mut self, depth: &RgTexture, name: &str) -> HzbHandles;
    fn generate_wall_draw_lists_pass2(&mut self, hzb: &HzbHandles);
    fn grass_compute(&mut self, hzb: &HzbHandles);
    fn main_pass_oc2(&mut self, gbuf: &GBufferTargets, color: &RgTexture);
    fn run_visibility_queries(&mut self, depth: &RgTexture);
    fn calculate_dof_autofocus(&mut self, depth: &RgTexture);
    fn mouse_depth_compute(&mut self, depth: &RgTexture);
    fn late_forward_and_deferred_pass(&mut self, gbuf: &GBufferTargets, color: &RgTexture);
    fn extract_half_res_depth_normals(&mut self, gbuf: &GBufferTargets) -> HalfResTargets;
    fn calculate_reprojection_map(
        &mut self,
        depth: &RgTexture,
        velocity: &RgTexture,
        prev_depth: &TemporalTexture,
    ) -> PassHandle;
    fn calculate_ssgi(
        &mut self,
        half: &HalfResTargets,
        gbuf: &GBufferTargets,
        reprojection: &PassHandle,
    ) -> PassHandle;

    // ---- W38-W43：GI / 光照 / 延迟光栅 ----
    #[allow(clippy::too_many_arguments)]
    fn ray_trace_gi(
        &mut self,
        gbuf: &GBufferTargets,
        half: &HalfResTargets,
        history: &RgTexture,
        history_fresh: bool,
        reprojection: &PassHandle,
        ssgi: &PassHandle,
    ) -> RayTracedGi;
    fn eval_gi_screen_radiance(
        &mut self,
        half: &HalfResTargets,
        ssgi: &PassHandle,
        rt_gi: &RayTracedGi,
        reprojection: &PassHandle,
    ) -> PassHandle;
    fn bin_lights(&mut self) -> LightBins;
    fn restir_sample_direct_lights(
        &mut self,
        half: &HalfResTargets,
        reprojection: &PassHandle,
        bins: &LightBins,
    ) -> PassHandle;
    fn apply_screen_shadows(
        &mut self,
        gbuf: &GBufferTargets,
        reprojection: &PassHandle,
    ) -> PassHandle;
    #[allow(clippy::too_many_arguments)]
    fn deferred_raster(
        &mut self,
        gbuf: &GBufferTargets,
        color: &RgTexture,
        history: &RgTexture,
        history_fresh: bool,
        ssgi: &PassHandle,
        half: &HalfResTargets,
        rt_gi: &RayTracedGi,
        gi_radiance: &PassHandle,
        shadows: &PassHandle,
        restir: &PassHandle,
        bins: &LightBins,
    );

    // ---- W44-W49：透明/体积/DoF 准备 ----
    fn refraction_copy_pass(
        &mut self,
        refraction: &RgTexture,
        color: &RgTexture,
        depth: &RgTexture,
    );
    #[allow(clippy::too_many_arguments)]
    fn water_surface_pass(
        &mut self,
        color: &RgTexture,
        depth: &RgTexture,
        half: &HalfResTargets,
        history: &RgTexture,
        history_fresh: bool,
        refraction: &RgTexture,
        reprojection: &PassHandle,
    );
    #[allow(clippy::too_many_arguments)]
    fn ice_surface_pass(
        &mut self,
        color: &RgTexture,
        depth: &RgTexture,
        half: &HalfResTargets,
        history: &RgTexture,
        history_fresh: bool,
        refraction: &RgTexture,
        reprojection: &PassHandle,
    );
    fn pre_taa_transparency_pass(
        &mut self,
        color: &RgTexture,
        depth: &RgTexture,
        half: &HalfResTargets,
        history: &RgTexture,
        history_fresh: bool,
    );
    fn compute_shadow_mask_mean(&mut self);
    fn apply_volumetrics(&mut self, color: &RgTexture, depth: &RgTexture);
    fn dof_prepare(&mut self, depth: &RgTexture) -> DofTargets;

    // ---- W50-W54：DoF / TAA|TGSR / 收尾 ----
    /// Original final stack8 is `name.len()`, not a separate mip count.
    /// Actual mip levels always come from color.desc.
    fn generate_mips(&mut self, color: &RgTexture, name: &str);
    /// `calc_dof(out, rg, world, &color, &depth, &dof_prepare, photomode)`；返回的 36 B 即后续 TAA 的颜色输入。
    fn calc_dof(
        &mut self,
        color: &RgTexture,
        depth: &RgTexture,
        dof_prepare: &DofTargets,
        photomode: bool,
    ) -> RgTexture;
    #[allow(clippy::too_many_arguments)]
    fn calculate_taa(
        &mut self,
        color: &RgTexture,
        output: &RgTexture,
        reprojection: &PassHandle,
        cur: &RgTexture,
        prev: &RgTexture,
        sharpen_amount: f32,
    ) -> PassHandle;
    #[allow(clippy::too_many_arguments)]
    fn calculate_tgsr(
        &mut self,
        color: &RgTexture,
        depth: &RgTexture,
        output: &RgTexture,
        reprojection: &PassHandle,
        cur: &RgTexture,
        prev: &RgTexture,
        sharpen_amount: f32,
    ) -> PassHandle;
    fn post_taa_transparency_pass(&mut self, taa: &PassHandle);
    fn render_shader_refactoring(&mut self, taa: &PassHandle);
}

/// 建立一张 RG 纹理资源：`new_2d` + `RenderGraph::create_raw_resource(name, desc)`，返回 `{id, desc}`。
fn create_texture<P: FramePasses>(p: &mut P, name: &str, desc: TextureDesc) -> RgTexture {
    let id = p.create_raw_resource(name, &desc);
    RgTexture { id, desc }
}

/// `tiny_glade::render_loop::render_world`（0x1404ec110）。
///
/// 返回 `RenderWorldOutputs`（O：写入 `param_1` 的 0x6c B）。panic 与原版一致：
/// - `world_render_dims` 任一维为 0 → `assert!`（行 965）；
/// - 资源查找失败 → `Option::unwrap`（行 967/1113/1254/1302）；
/// - temporal 创建失败 → `Result::unwrap`（行 978/1143）。
pub fn render_world<P: FramePasses>(p: &mut P, args: &RenderWorldArgs) -> RenderWorldOutputs {
    let dims = args.world_render_dims;
    assert!(
        dims[0] > 0 && dims[1] > 0,
        "assertion failed: world_render_dims[0] > 0 && world_render_dims[1] > 0"
    );
    let depth = args.depth;
    let color = args.color;
    let settings = args.settings;

    // 0x1404ec154..1eb：世界配置 → is_winter（0x140ae8320）
    let is_winter = p.is_winter().unwrap();

    // 0x1404ec1f6..260：`reprojection.prev_depth` = temporal(param_7.desc.format(100).usage(0xc))
    let prev_depth_desc = depth
        .desc
        .with_format(formats::PREV_DEPTH)
        .with_usage(PREV_DEPTH_USAGE);
    let prev_depth = p
        .get_or_create_temporal(names::PREV_DEPTH, &prev_depth_desc)
        .unwrap();

    // W1：6 个 world-raster 上传
    for slot in UPLOAD_RASTER_SLOTS {
        p.upload_raster_staging_to_texture(slot);
    }
    // W2-W20
    p.water_rasterize_from_cpu();
    p.rasterize_terrain_stroke();
    p.displace_rocky_terrain();
    p.read_terrain_data();
    p.mouse_read_terrain_stroke_index();
    p.rasterize_foliage_exclusion_mask();
    p.highlight_active_terrain_stroke();
    p.object_grass_push_pass();
    p.compute_path_masks();
    p.rasterize_buildings();
    p.garden_rasterize_from_cpu();
    p.water_spawn_vegetation();
    p.spawn_vegetation_around_buildings();
    p.read_bush_positions_for_cpu();
    p.garden_spawn_vegetation();
    p.path_pebbles_compute();
    p.leaf_particles_compute();
    p.roof_tiles_gravity_compute();
    p.stairs();
    // W21-W24
    p.generate_wall_draw_lists();
    p.gpu_generate_draw_lists(None);
    p.shadow_pass();
    p.calc_terrain_lighting_texture();

    // gbuffer / velocity（0x1404ec3fa..4f2）
    let gbuffer = create_texture(
        p,
        names::GBUFFER,
        TextureDesc::new_2d(dims, formats::GBUFFER),
    );
    let velocity = create_texture(
        p,
        names::VELOCITY,
        TextureDesc::new_2d(dims, formats::VELOCITY),
    );
    let gbuf = GBufferTargets {
        gbuffer,
        depth,
        velocity,
    };

    // W25-W34
    p.main_pass(&gbuf, &color);
    let hzb = p.generate_hzb(&depth, names::HZB);
    if !p.flag_c0d8() {
        p.generate_wall_draw_lists_pass2(&hzb);
        p.gpu_generate_draw_lists(Some(&hzb));
    }
    p.grass_compute(&hzb);
    p.main_pass_oc2(&gbuf, &color);
    p.run_visibility_queries(&depth);
    p.calculate_dof_autofocus(&depth);
    p.mouse_depth_compute(&depth);
    p.late_forward_and_deferred_pass(&gbuf, &color);

    // W35-W37
    let half = p.extract_half_res_depth_normals(&gbuf);
    let reprojection = p.calculate_reprojection_map(&depth, &velocity, &prev_depth);
    let ssgi = p.calculate_ssgi(&half, &gbuf, &reprojection);

    // GI/TAA 历史对：`tgsr` 取显示尺寸，否则取 `param_8.desc` 的 extent
    let tgsr = settings.tgsr;
    let (history_name, history_dims) = if tgsr {
        (names::TGSR_HISTORY, p.display_dims().unwrap())
    } else {
        (names::TAA_HISTORY, color.desc.extent_2d())
    };
    let history_desc = TextureDesc::new_2d(history_dims, formats::COLOR_HDR)
        .sampled()
        .storage();
    let pair = p
        .get_or_create_temporal_pair(history_name, &history_desc)
        .unwrap();

    // W38-W43
    let rt_gi = p.ray_trace_gi(&gbuf, &half, &pair.prev, pair.fresh, &reprojection, &ssgi);
    let gi_radiance = p.eval_gi_screen_radiance(&half, &ssgi, &rt_gi, &reprojection);
    let bins = p.bin_lights();
    let restir = p.restir_sample_direct_lights(&half, &reprojection, &bins);
    let shadows = p.apply_screen_shadows(&gbuf, &reprojection);
    p.deferred_raster(
        &gbuf,
        &color,
        &pair.prev,
        pair.fresh,
        &ssgi,
        &half,
        &rt_gi,
        &gi_radiance,
        &shadows,
        &restir,
        &bins,
    );

    // W44/W45：折射 + 水面/冰面
    if !p.flag_c098() {
        let refraction_desc = TextureDesc::new_2d(color.desc.extent_2d(), formats::COLOR_HDR);
        let refraction = create_texture(p, names::REFRACTION_SOURCE, refraction_desc);
        p.refraction_copy_pass(&refraction, &color, &depth);
        if is_winter {
            p.ice_surface_pass(
                &color,
                &depth,
                &half,
                &pair.prev,
                pair.fresh,
                &refraction,
                &reprojection,
            );
        } else {
            p.water_surface_pass(
                &color,
                &depth,
                &half,
                &pair.prev,
                pair.fresh,
                &refraction,
                &reprojection,
            );
        }
    }

    // W46-W49
    p.pre_taa_transparency_pass(&color, &depth, &half, &pair.prev, pair.fresh);
    p.compute_shadow_mask_mean();
    p.apply_volumetrics(&color, &depth);
    let dof_prepare = p.dof_prepare(&depth);

    // W50/W51：DoF（设置字节下标 = 0xa9 + (mode == 0x13)）
    let dof_mode = p.dof_mode().unwrap();
    let photomode = dof_mode == DOF_PHOTOMODE_TAG;
    let taa_input = if settings.dof_enabled[photomode as usize] {
        p.generate_mips(&color, names::DOF_MIPS);
        p.calc_dof(&color, &depth, &dof_prepare, photomode)
    } else {
        color
    };

    // W52：TAA / TGSR（输出纹理 "taa_output"，同 format 0x7a、无用法位）
    let taa = if !tgsr {
        let output_desc = TextureDesc::new_2d(taa_input.desc.extent_2d(), formats::COLOR_HDR);
        let output = create_texture(p, names::TAA_OUTPUT, output_desc);
        p.calculate_taa(
            &taa_input,
            &output,
            &reprojection,
            &pair.cur,
            &pair.prev,
            settings.sharpen_amount,
        )
    } else {
        let output_dims = p.display_dims().unwrap();
        let output_desc = TextureDesc::new_2d(output_dims, formats::COLOR_HDR);
        let output = create_texture(p, names::TAA_OUTPUT, output_desc);
        p.calculate_tgsr(
            &taa_input,
            &depth,
            &output,
            &reprojection,
            &pair.cur,
            &pair.prev,
            settings.sharpen_amount,
        )
    };

    // W53/W54
    p.post_taa_transparency_pass(&taa);
    p.render_shader_refactoring(&taa);

    RenderWorldOutputs {
        depth,
        dof_prepare: dof_prepare.coc,
        taa,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 录制型实现：把每次调用按序记成字符串。
    #[derive(Default)]
    struct Rec {
        log: Vec<String>,
        winter: Option<bool>,
        display: Option<[u32; 2]>,
        dof_mode: Option<u32>,
        c0d8: bool,
        c098: bool,
        next_id: u32,
        temporal_err: bool,
        pair_err: bool,
    }

    impl Rec {
        fn new() -> Self {
            Self {
                winter: Some(false),
                display: Some([1920, 1080]),
                dof_mode: Some(0),
                next_id: 100,
                ..Default::default()
            }
        }
        fn p(&mut self, s: impl Into<String>) {
            self.log.push(s.into());
        }
    }

    fn core_tex(id: u32) -> PassHandle {
        PassHandle {
            resource: id,
            desc: crate::rhapsody_types::TextureDesc::new_2d([16, 16], 0x4c),
        }
    }

    impl FramePasses for Rec {
        type Error = &'static str;
        fn is_winter(&mut self) -> Option<bool> {
            self.winter
        }
        fn display_dims(&mut self) -> Option<[u32; 2]> {
            self.display
        }
        fn dof_mode(&mut self) -> Option<u32> {
            self.dof_mode
        }
        fn flag_c0d8(&mut self) -> bool {
            self.c0d8
        }
        fn flag_c098(&mut self) -> bool {
            self.c098
        }
        fn get_or_create_temporal(
            &mut self,
            name: &str,
            desc: &TextureDesc,
        ) -> Result<TemporalTexture, &'static str> {
            self.p(format!(
                "temporal {name} fmt={} usage={:#x}",
                desc.format, desc.usage
            ));
            if self.temporal_err {
                return Err("boom");
            }
            Ok(TemporalTexture {
                tex: RgTexture { id: 7, desc: *desc },
                flag: 0,
            })
        }
        fn get_or_create_temporal_pair(
            &mut self,
            name: &str,
            desc: &TextureDesc,
        ) -> Result<TemporalPair, &'static str> {
            self.p(format!(
                "pair {name} {:?} usage={:#x}",
                desc.extent, desc.usage
            ));
            if self.pair_err {
                return Err("pair");
            }
            Ok(TemporalPair {
                cur: RgTexture {
                    id: 21,
                    desc: *desc,
                },
                prev: RgTexture {
                    id: 22,
                    desc: *desc,
                },
                fresh: true,
            })
        }
        fn create_raw_resource(&mut self, name: &str, desc: &TextureDesc) -> u32 {
            self.next_id += 1;
            self.p(format!(
                "create {name} fmt={:#x} {:?} usage={:#x}",
                desc.format, desc.extent, desc.usage
            ));
            self.next_id
        }
        fn upload_raster_staging_to_texture(&mut self, slot: u8) {
            self.p(format!("upload {slot:#x}"));
        }
        fn water_rasterize_from_cpu(&mut self) {
            self.p("water_rasterize_from_cpu");
        }
        fn rasterize_terrain_stroke(&mut self) {
            self.p("rasterize_terrain_stroke");
        }
        fn displace_rocky_terrain(&mut self) {
            self.p("displace_rocky_terrain");
        }
        fn read_terrain_data(&mut self) {
            self.p("read_terrain_data");
        }
        fn mouse_read_terrain_stroke_index(&mut self) {
            self.p("mouse_read_terrain_stroke_index");
        }
        fn rasterize_foliage_exclusion_mask(&mut self) {
            self.p("rasterize_foliage_exclusion_mask");
        }
        fn highlight_active_terrain_stroke(&mut self) {
            self.p("highlight_active_terrain_stroke");
        }
        fn object_grass_push_pass(&mut self) {
            self.p("object_grass_push_pass");
        }
        fn compute_path_masks(&mut self) {
            self.p("compute_path_masks");
        }
        fn rasterize_buildings(&mut self) {
            self.p("rasterize_buildings");
        }
        fn garden_rasterize_from_cpu(&mut self) {
            self.p("garden_rasterize_from_cpu");
        }
        fn water_spawn_vegetation(&mut self) {
            self.p("water_spawn_vegetation");
        }
        fn spawn_vegetation_around_buildings(&mut self) {
            self.p("spawn_vegetation_around_buildings");
        }
        fn read_bush_positions_for_cpu(&mut self) {
            self.p("read_bush_positions_for_cpu");
        }
        fn garden_spawn_vegetation(&mut self) {
            self.p("garden_spawn_vegetation");
        }
        fn path_pebbles_compute(&mut self) {
            self.p("path_pebbles_compute");
        }
        fn leaf_particles_compute(&mut self) {
            self.p("leaf_particles_compute");
        }
        fn roof_tiles_gravity_compute(&mut self) {
            self.p("roof_tiles_gravity_compute");
        }
        fn stairs(&mut self) {
            self.p("stairs");
        }
        fn generate_wall_draw_lists(&mut self) {
            self.p("generate_wall_draw_lists");
        }
        fn gpu_generate_draw_lists(&mut self, hzb: Option<&HzbHandles>) {
            self.p(format!("gpu_generate_draw_lists hzb={}", hzb.is_some()));
        }
        fn shadow_pass(&mut self) {
            self.p("shadow_pass");
        }
        fn calc_terrain_lighting_texture(&mut self) {
            self.p("calc_terrain_lighting_texture");
        }
        fn main_pass(&mut self, g: &GBufferTargets, _c: &RgTexture) {
            self.p(format!(
                "main_pass gbuf={} vel={}",
                g.gbuffer.id, g.velocity.id
            ));
        }
        fn generate_hzb(&mut self, depth: &RgTexture, name: &str) -> HzbHandles {
            self.p(format!("generate_hzb depth={} name={name}", depth.id));
            HzbHandles {
                half_extent: [8, 8],
                texture: core_tex(9),
            }
        }
        fn generate_wall_draw_lists_pass2(&mut self, _h: &HzbHandles) {
            self.p("generate_wall_draw_lists_pass2");
        }
        fn grass_compute(&mut self, _h: &HzbHandles) {
            self.p("grass_compute");
        }
        fn main_pass_oc2(&mut self, _g: &GBufferTargets, _c: &RgTexture) {
            self.p("main_pass_oc2");
        }
        fn run_visibility_queries(&mut self, _d: &RgTexture) {
            self.p("run_visibility_queries");
        }
        fn calculate_dof_autofocus(&mut self, _d: &RgTexture) {
            self.p("calculate_dof_autofocus");
        }
        fn mouse_depth_compute(&mut self, _d: &RgTexture) {
            self.p("mouse_depth_compute");
        }
        fn late_forward_and_deferred_pass(&mut self, _g: &GBufferTargets, _c: &RgTexture) {
            self.p("late_forward_and_deferred_pass");
        }
        fn extract_half_res_depth_normals(&mut self, _g: &GBufferTargets) -> HalfResTargets {
            self.p("extract_half_res_depth_normals");
            HalfResTargets {
                a: RgTexture {
                    id: 31,
                    ..Default::default()
                },
                b: RgTexture {
                    id: 32,
                    ..Default::default()
                },
            }
        }
        fn calculate_reprojection_map(
            &mut self,
            _d: &RgTexture,
            v: &RgTexture,
            pd: &TemporalTexture,
        ) -> PassHandle {
            self.p(format!(
                "calculate_reprojection_map vel={} prev_depth={}",
                v.id, pd.tex.id
            ));
            core_tex(1)
        }
        fn calculate_ssgi(
            &mut self,
            h: &HalfResTargets,
            _g: &GBufferTargets,
            _r: &PassHandle,
        ) -> PassHandle {
            self.p(format!("calculate_ssgi a={} b={}", h.a.id, h.b.id));
            core_tex(2)
        }
        fn ray_trace_gi(
            &mut self,
            _g: &GBufferTargets,
            _h: &HalfResTargets,
            hist: &RgTexture,
            fresh: bool,
            _r: &PassHandle,
            _s: &PassHandle,
        ) -> RayTracedGi {
            self.p(format!("ray_trace_gi hist={} fresh={fresh}", hist.id));
            RayTracedGi {
                radiance: core_tex(3),
                ray_dir: core_tex(30),
                reservoir: core_tex(31),
            }
        }
        fn eval_gi_screen_radiance(
            &mut self,
            _h: &HalfResTargets,
            _s: &PassHandle,
            _t: &RayTracedGi,
            _r: &PassHandle,
        ) -> PassHandle {
            assert_eq!(_t.radiance, core_tex(3));
            assert_eq!(_t.ray_dir, core_tex(30));
            assert_eq!(_t.reservoir, core_tex(31));
            self.p("eval_gi_screen_radiance");
            core_tex(4)
        }
        fn bin_lights(&mut self) -> LightBins {
            self.p("bin_lights");
            LightBins {
                grid: crate::render::pass_builder::compute_pass_builder::RgBuffer { resource: 5 },
                indices: crate::render::pass_builder::compute_pass_builder::RgBuffer {
                    resource: 6,
                },
            }
        }
        fn restir_sample_direct_lights(
            &mut self,
            _h: &HalfResTargets,
            _r: &PassHandle,
            _b: &LightBins,
        ) -> PassHandle {
            assert_eq!(_b.grid.resource, 5);
            assert_eq!(_b.indices.resource, 6);
            self.p("restir_sample_direct_lights");
            core_tex(6)
        }
        fn apply_screen_shadows(&mut self, _g: &GBufferTargets, _r: &PassHandle) -> PassHandle {
            self.p("apply_screen_shadows");
            core_tex(7)
        }
        fn deferred_raster(
            &mut self,
            _g: &GBufferTargets,
            _c: &RgTexture,
            _h: &RgTexture,
            _f: bool,
            _s: &PassHandle,
            _half: &HalfResTargets,
            _t: &RayTracedGi,
            _gi: &PassHandle,
            _sh: &PassHandle,
            _re: &PassHandle,
            _b: &LightBins,
        ) {
            assert_eq!(_t.radiance, core_tex(3));
            assert_eq!(_t.ray_dir, core_tex(30));
            assert_eq!(_t.reservoir, core_tex(31));
            self.p("deferred_raster");
        }
        fn refraction_copy_pass(&mut self, r: &RgTexture, _c: &RgTexture, _d: &RgTexture) {
            self.p(format!("refraction_copy_pass id={}", r.id));
        }
        fn water_surface_pass(
            &mut self,
            _c: &RgTexture,
            _d: &RgTexture,
            _h: &HalfResTargets,
            _hi: &RgTexture,
            _f: bool,
            _r: &RgTexture,
            _p: &PassHandle,
        ) {
            self.p("water_surface_pass");
        }
        fn ice_surface_pass(
            &mut self,
            _c: &RgTexture,
            _d: &RgTexture,
            _h: &HalfResTargets,
            _hi: &RgTexture,
            _f: bool,
            _r: &RgTexture,
            _p: &PassHandle,
        ) {
            self.p("ice_surface_pass");
        }
        fn pre_taa_transparency_pass(
            &mut self,
            _c: &RgTexture,
            _d: &RgTexture,
            _h: &HalfResTargets,
            _hi: &RgTexture,
            _f: bool,
        ) {
            self.p("pre_taa_transparency_pass");
        }
        fn compute_shadow_mask_mean(&mut self) {
            self.p("compute_shadow_mask_mean");
        }
        fn apply_volumetrics(&mut self, _c: &RgTexture, _d: &RgTexture) {
            self.p("apply_volumetrics");
        }
        fn dof_prepare(&mut self, _d: &RgTexture) -> DofTargets {
            self.p("dof_prepare");
            DofTargets {
                coc: core_tex(8),
                coc_tiles: core_tex(80),
            }
        }
        fn generate_mips(&mut self, _c: &RgTexture, name: &str) {
            self.p(format!("generate_mips {name}"));
        }
        fn calc_dof(
            &mut self,
            c: &RgTexture,
            _d: &RgTexture,
            _p: &DofTargets,
            photomode: bool,
        ) -> RgTexture {
            assert_eq!(_p.coc, core_tex(8));
            assert_eq!(_p.coc_tiles, core_tex(80));
            self.p(format!("calc_dof photomode={photomode}"));
            RgTexture {
                id: c.id + 1000,
                desc: c.desc,
            }
        }
        fn calculate_taa(
            &mut self,
            c: &RgTexture,
            o: &RgTexture,
            _r: &PassHandle,
            cur: &RgTexture,
            prev: &RgTexture,
            param: f32,
        ) -> PassHandle {
            self.p(format!(
                "calculate_taa in={} out={} cur={} prev={} param={param}",
                c.id, o.id, cur.id, prev.id
            ));
            core_tex(10)
        }
        fn calculate_tgsr(
            &mut self,
            c: &RgTexture,
            _d: &RgTexture,
            o: &RgTexture,
            _r: &PassHandle,
            cur: &RgTexture,
            prev: &RgTexture,
            param: f32,
        ) -> PassHandle {
            self.p(format!(
                "calculate_tgsr in={} out={} {:?} cur={} prev={} param={param}",
                c.id, o.id, o.desc.extent, cur.id, prev.id
            ));
            core_tex(11)
        }
        fn post_taa_transparency_pass(&mut self, t: &PassHandle) {
            self.p(format!("post_taa_transparency_pass {}", t.resource));
        }
        fn render_shader_refactoring(&mut self, t: &PassHandle) {
            self.p(format!("render_shader_refactoring {}", t.resource));
        }
    }

    fn args() -> RenderWorldArgs {
        RenderWorldArgs {
            world_render_dims: [1280, 720],
            settings: WorldRenderSettings {
                sharpen_amount: 0.375,
                dof_enabled: [true, false],
                tgsr: false,
            },
            depth: RgTexture {
                id: 1,
                desc: TextureDesc::new_2d([1280, 720], 0x30),
            },
            color: RgTexture {
                id: 2,
                desc: TextureDesc::new_2d([1280, 720], 0x7a),
            },
        }
    }

    fn names_of(log: &[String]) -> Vec<&str> {
        log.iter().map(|s| s.split(' ').next().unwrap()).collect()
    }

    #[test]
    fn texture_desc_matches_rhapsody_bytes() {
        let d = TextureDesc::new_2d([640, 480], 0x7a);
        assert_eq!(
            (d.usage, d.format, d.extent, d.depth, d.layers_or_mips),
            (0, 0x7a, [640, 480], 1, 1)
        );
        assert_eq!(d.tail, [1, 1, 2, 0]);
        assert_eq!(d.sampled().storage().usage, 0xc);
        assert_eq!(d.sampled().with_usage(0).usage, 0);
        assert_eq!(d.extent_2d(), [640, 480]);
        assert_eq!(
            PREV_DEPTH_USAGE,
            TextureDesc::default().sampled().storage().usage
        );
    }

    #[test]
    fn full_world_frame_order_taa_path() {
        let mut p = Rec::new();
        let out = render_world(&mut p, &args());
        let n = names_of(&p.log);
        let expected = [
            "temporal",
            "upload",
            "upload",
            "upload",
            "upload",
            "upload",
            "upload",
            "water_rasterize_from_cpu",
            "rasterize_terrain_stroke",
            "displace_rocky_terrain",
            "read_terrain_data",
            "mouse_read_terrain_stroke_index",
            "rasterize_foliage_exclusion_mask",
            "highlight_active_terrain_stroke",
            "object_grass_push_pass",
            "compute_path_masks",
            "rasterize_buildings",
            "garden_rasterize_from_cpu",
            "water_spawn_vegetation",
            "spawn_vegetation_around_buildings",
            "read_bush_positions_for_cpu",
            "garden_spawn_vegetation",
            "path_pebbles_compute",
            "leaf_particles_compute",
            "roof_tiles_gravity_compute",
            "stairs",
            "generate_wall_draw_lists",
            "gpu_generate_draw_lists",
            "shadow_pass",
            "calc_terrain_lighting_texture",
            "create",
            "create",
            "main_pass",
            "generate_hzb",
            "generate_wall_draw_lists_pass2",
            "gpu_generate_draw_lists",
            "grass_compute",
            "main_pass_oc2",
            "run_visibility_queries",
            "calculate_dof_autofocus",
            "mouse_depth_compute",
            "late_forward_and_deferred_pass",
            "extract_half_res_depth_normals",
            "calculate_reprojection_map",
            "calculate_ssgi",
            "pair",
            "ray_trace_gi",
            "eval_gi_screen_radiance",
            "bin_lights",
            "restir_sample_direct_lights",
            "apply_screen_shadows",
            "deferred_raster",
            "create",
            "refraction_copy_pass",
            "water_surface_pass",
            "pre_taa_transparency_pass",
            "compute_shadow_mask_mean",
            "apply_volumetrics",
            "dof_prepare",
            "generate_mips",
            "calc_dof",
            "create",
            "calculate_taa",
            "post_taa_transparency_pass",
            "render_shader_refactoring",
        ];
        assert_eq!(n, expected);
        assert_eq!(out.depth.id, 1);
        assert_eq!(out.dof_prepare, core_tex(8));
        assert_eq!(out.taa, core_tex(10));
    }

    #[test]
    fn upload_slots_and_temporal_constants() {
        let mut p = Rec::new();
        render_world(&mut p, &args());
        let ups: Vec<_> = p
            .log
            .iter()
            .filter(|s| s.starts_with("upload"))
            .cloned()
            .collect();
        assert_eq!(
            ups,
            [
                "upload 0x1",
                "upload 0x2",
                "upload 0x3",
                "upload 0x4",
                "upload 0x13",
                "upload 0x14"
            ]
        );
        assert_eq!(
            p.log[0],
            "temporal reprojection.prev_depth fmt=100 usage=0xc"
        );
        assert!(
            p.log
                .contains(&"create gbuffer fmt=0x65 [1280, 720] usage=0x0".to_string())
        );
        assert!(
            p.log
                .contains(&"create velocity fmt=0x61 [1280, 720] usage=0x0".to_string())
        );
        assert!(
            p.log
                .contains(&"generate_hzb depth=1 name=main".to_string())
        );
        assert!(
            p.log
                .contains(&"pair taa_history [1280, 720] usage=0xc".to_string())
        );
        assert!(p.log.contains(&"generate_mips dof_mips".to_string()));
        assert!(
            p.log
                .iter()
                .any(|s| s.starts_with("create taa_output fmt=0x7a [1280, 720]"))
        );
        assert!(
            p.log
                .contains(&"create refraction source fmt=0x7a [1280, 720] usage=0x0".to_string())
        );
    }

    #[test]
    fn second_cull_round_skipped_when_flag_c0d8_set() {
        let mut p = Rec::new();
        p.c0d8 = true;
        render_world(&mut p, &args());
        assert!(!p.log.iter().any(|s| s == "generate_wall_draw_lists_pass2"));
        assert_eq!(
            p.log
                .iter()
                .filter(|s| s.starts_with("gpu_generate_draw_lists"))
                .count(),
            1
        );
        assert!(
            p.log
                .contains(&"gpu_generate_draw_lists hzb=false".to_string())
        );
        // grass_compute 仍无条件执行
        assert!(p.log.contains(&"grass_compute".to_string()));
    }

    #[test]
    fn refraction_and_surface_skipped_when_flag_c098_set() {
        let mut p = Rec::new();
        p.c098 = true;
        render_world(&mut p, &args());
        for s in [
            "refraction_copy_pass",
            "water_surface_pass",
            "ice_surface_pass",
        ] {
            assert!(!p.log.iter().any(|l| l.starts_with(s)), "{s}");
        }
        assert!(p.log.contains(&"pre_taa_transparency_pass".to_string()));
    }

    #[test]
    fn winter_selects_ice_surface() {
        let mut p = Rec::new();
        p.winter = Some(true);
        render_world(&mut p, &args());
        assert!(p.log.contains(&"ice_surface_pass".to_string()));
        assert!(!p.log.contains(&"water_surface_pass".to_string()));
    }

    #[test]
    fn dof_flag_index_follows_actual_photomode_tag() {
        // dof_enabled = [true, false]：mode != 0x13 → 用下标 0（开）；mode == 0x13 → 下标 1（关）
        let mut p = Rec::new();
        p.dof_mode = Some(0x12);
        render_world(&mut p, &args());
        assert!(p.log.contains(&"calc_dof photomode=false".to_string()));
        let mut p = Rec::new();
        p.dof_mode = Some(DOF_PHOTOMODE_TAG);
        render_world(&mut p, &args());
        assert!(
            !p.log
                .iter()
                .any(|l| l.starts_with("generate_mips") || l.starts_with("calc_dof"))
        );
        // 未做 DoF 时 TAA 输入 = 原颜色（id 2），而不是 calc_dof 的结果
        assert!(p.log.iter().any(|l| l.starts_with("calculate_taa in=2 ")));
        // 打开 photomode 下标
        let mut a = args();
        a.settings.dof_enabled = [false, true];
        let mut p = Rec::new();
        p.dof_mode = Some(DOF_PHOTOMODE_TAG);
        render_world(&mut p, &a);
        assert!(p.log.contains(&"calc_dof photomode=true".to_string()));
        assert!(
            p.log
                .iter()
                .any(|l| l.starts_with("calculate_taa in=1002 "))
        );
    }

    #[test]
    fn tgsr_path_uses_display_dims_and_other_pass() {
        let mut a = args();
        a.settings.tgsr = true;
        let mut p = Rec::new();
        render_world(&mut p, &a);
        assert!(
            p.log
                .contains(&"pair tgsr_history [1920, 1080] usage=0xc".to_string())
        );
        assert!(
            p.log
                .iter()
                .any(|l| l.starts_with("create taa_output fmt=0x7a [1920, 1080]"))
        );
        let tgsr = p
            .log
            .iter()
            .find(|l| l.starts_with("calculate_tgsr"))
            .unwrap();
        assert!(
            tgsr.contains("[1920, 1080]") && tgsr.contains("param=0.375"),
            "{tgsr}"
        );
        assert!(!p.log.iter().any(|l| l.starts_with("calculate_taa")));
    }

    #[test]
    fn taa_output_extent_follows_dof_result() {
        let mut a = args();
        a.color.desc = TextureDesc::new_2d([800, 600], 0x7a);
        let mut p = Rec::new();
        render_world(&mut p, &a);
        assert!(
            p.log
                .contains(&"pair taa_history [800, 600] usage=0xc".to_string())
        );
        assert!(
            p.log
                .iter()
                .any(|l| l.starts_with("create taa_output fmt=0x7a [800, 600]"))
        );
    }

    #[test]
    #[should_panic(
        expected = "assertion failed: world_render_dims[0] > 0 && world_render_dims[1] > 0"
    )]
    fn zero_width_dims_panic_before_any_pass() {
        let mut a = args();
        a.world_render_dims = [0, 720];
        render_world(&mut Rec::new(), &a);
    }

    #[test]
    #[should_panic(
        expected = "assertion failed: world_render_dims[0] > 0 && world_render_dims[1] > 0"
    )]
    fn zero_height_dims_panic() {
        let mut a = args();
        a.world_render_dims = [1280, 0];
        render_world(&mut Rec::new(), &a);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn missing_world_config_resource_panics() {
        let mut p = Rec::new();
        p.winter = None;
        render_world(&mut p, &args());
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn prev_depth_temporal_error_panics_before_uploads() {
        let mut p = Rec::new();
        p.temporal_err = true;
        render_world(&mut p, &args());
    }

    #[test]
    fn prev_depth_error_emits_no_pass() {
        let mut p = Rec::new();
        p.temporal_err = true;
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            render_world(&mut p, &args())
        }));
        assert!(r.is_err());
        assert_eq!(p.log.len(), 1);
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn history_pair_error_panics() {
        let mut p = Rec::new();
        p.pair_err = true;
        render_world(&mut p, &args());
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn missing_dof_resource_panics() {
        let mut p = Rec::new();
        p.dof_mode = None;
        render_world(&mut p, &args());
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn tgsr_without_display_resource_panics() {
        let mut a = args();
        a.settings.tgsr = true;
        let mut p = Rec::new();
        p.display = None;
        render_world(&mut p, &a);
    }

    #[test]
    fn panic_line_table_matches_location_statics() {
        // 0x1429ebf68 / 0x1429ebf80 / 0x1429ebfb0 / 0x1429ebfe8 / 0x1429ec010 / 0x1429ec040 / 0x1429ec070
        assert_eq!(
            [
                panic_lines::ASSERT_DIMS,
                panic_lines::WORLD_CONFIG,
                panic_lines::PREV_DEPTH_TEMPORAL,
                panic_lines::TGSR_DIMS_HISTORY,
                panic_lines::HISTORY_PAIR,
                panic_lines::DOF_MODE,
                panic_lines::TGSR_DIMS_OUTPUT,
            ],
            [965, 967, 978, 1113, 1143, 1254, 1302]
        );
    }
}
