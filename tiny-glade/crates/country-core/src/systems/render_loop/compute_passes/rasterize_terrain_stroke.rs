//! `rasterize_terrain_stroke.rs` — W3 地形笔画栅格化 + 高斯模糊 + 地形导数。
//!
//! # 来源（O）
//! `rasterize_terrain_stroke` 0x1408c7120（10387 B）、dispatch 闭包 `closure_env$1` 0x1415fb550、
//! `GaussBlur::as_compute_pipeline` 0x1408ca1c0、`calculate_hash<(&str,&GaussBlur)>` 0x141459fc0、
//! `utils::pack_2x16f_uint` 0x140c90df0。HLSL：`shader-source/shaders/{terrain_editing/rasterize_terrain_stroke,
//! gauss_blur,terrain_derivative}.cs.hlsl`（绑定名由 FNV-1a 反解）。
//!
//! # 流程（O）
//! 1. 取三个资源（缺失 `Option::unwrap`）：`DynamicConstants`（type-id `0x47c94cb7…`）、
//!    活动笔画表（`0x1e38c167…`；**先写变更 tick** `row+0x5c = world+0x5b0`）、编辑状态（`0xcda6af2f…`）。
//!    若编辑状态里两个计数（`+0x10`、`+0x30`）都是 0 **且** 笔画表 `+0x40` 标志为 1（本帧已栅格化）→ 直接返回。
//! 2. puffin 作用域 `"upload terrain strokes"`（`puffin::are_scopes_on` 为真才开；见 `puffin_contract` stub）。
//! 3. 遍历活动笔画（hashbrown 表，每项 0x30 B）：过滤 `width > 0.01`（`.rdata 0x142960674`）且点数非 0。
//!    * 已收集的笔画数 `> 0x3fff` → 记一次性日志 **`"Ran out of live terrain strokes"`**（Error 级，模块
//!      `country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke`，经 `BTreeMap<String>` 去重）并跳过该笔画。
//!    * 否则 `DynamicConstants::allocate(min((len + 1) & !1, 0x400) * 8, align 1, 0x10)`，把前 `min(len, 0x400)` 个点
//!      （源 12 B：`x,y,_`）按 `x,y` 8 B 拷入；`count = min(len, 0x400)`，**断言 `point_count_packed != 0`**
//!      （原文 `"assertion failed: point_count_packed != 0"`，41 B）。
//!    * 16 B `StrokeInfo { device_address: u64, (id << 10) | (count & 0x3ff): u32, pack_2x16f(a, b): u32 }`，
//!      `a/b` 为笔画记录里的两个 f32；`into_gpu_lifetime_tracker` 保存供 dispatch 闭包 `track`。
//! 4. `"Rasterize Terrain Stroke"`（`/terrain_editing/rasterize_terrain_stroke.cs.hlsl`）：uniform `stroke_count`(i32)、
//!    `stroke_infos_device_address`([u32;2])、`stroke_width`(f32 = 1.5)；贴图预设 9 `img_output`(写)、0xb `stroke_indices`(写)、
//!    0x13 `water_mask`(读)、0xe `rocky_terrain`(写)、0xd `terrain_sdg`(写)；`write_storage_buffer<NormalDebugSsbo>
//!    (debug_normals_buffer)`；`dispatch_with`（闭包：对每个 tracker `track`，再 `dispatch_3d(512, 512, ..)`）。
//! 5. RG 纹理 `"blur temp"`（512×512，格式 `0x6d`）；两遍 `"Blur Terrain Stroke"`（`GaussBlur{fmt}`，
//!    `/gauss_blur.cs.hlsl`）：#1 `fmt="rgba32f"`、`is_horizontal=true`、`sigma=2.0`，`img_in`=预设 9 → `img_out`=temp；
//!    #2 `fmt="r16"`、`is_horizontal=false`、`sigma=2.0`，`img_in`=temp → `img_out`=预设 9；均 `dispatch_2d(512, 512)`
//!    （O：`.rdata 0x142aa2500 = {1, 512, 512, 1}`；#2 的描述符复用同一寄存器，I）。
//! 6. `"Terrain Derivative"`（`/terrain_derivative.cs.hlsl`）：预设 9 `terrain_heightmap`、10 `visual_terrain_heightmap`（读）、
//!    0xe `rocky_terrain`（写）。dispatch 同上（I）。
//! 7. 结束时把笔画表 `+0x40` 置 1（本帧已栅格化）。
//!
//! # 偏离
//! * 活动笔画的 hashbrown 遍历顺序由调用方给出（`&[RasterStroke]`）；`RasterStroke` 只含被读到的字段。
//! * `ActiveStrokes` 的 `device_address` 来自 `DynamicConstantsChunk::device_address`（rhapsody），经 [`StrokeAllocator`] 接缝。
//! * `GaussBlur` 第二个 define（`PERM_ty`）的值读不全（`floa..`），取 `"float"`（I）。

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::neg_cmp_op_on_partial_ord)]
#![allow(dead_code, unused_imports)]

use crate::asset_libraries::ssbo_library::SsboType as LibrarySsbo;
use crate::render::pass_builder::compute_pass_builder::{
    ComputeDispatchApi, DispatchArgs, RecordedComputePass,
};
use crate::render::pass_builder::pass_resource_access::RgTexture;
use crate::render::pass_builder::shader_uri::{
    AsComputeShaderSource, ComputeShaderSource, ShaderDefValue, ShaderUri,
};
use crate::resources::texture_presets::TexturePreset;
use crate::typed_gap::Gap;

use super::pass_kit::{PassEnv, binding_hash};
use crate::render::pass_builder::shader_uri::{ShaderDefine, perm_define};
use crate::rhapsody_types::TextureDesc;

// ---------------------------------------------------------------- GaussBlur

/// `/gauss_blur.cs.hlsl:cs_main` 源（O）。`fmt` = 输出图像格式名（`PERM_fmt` define）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GaussBlur {
    pub fmt: &'static str,
}

impl GaussBlur {
    pub const URI_WITH_ENTRY: &'static str = "/gauss_blur.cs.hlsl:cs_main";
}

impl AsComputeShaderSource for GaussBlur {
    /// O（0x141459fc0）：`calculate_hash(&("/gauss_blur.cs.hlsl:cs_main", &self))`，含 `fmt` 字符串。
    fn source_hash(&self) -> u64 {
        crate::render::utils::calculate_hash(&(Self::URI_WITH_ENTRY, self))
    }
    fn as_compute_pipeline(&self) -> ComputeShaderSource<'_> {
        ComputeShaderSource {
            uri: ShaderUri::parse(Self::URI_WITH_ENTRY),
            defs: ShaderDefValue::default(),
        }
    }
    /// `as_compute_pipeline` 的 defines（O：`PERM_fmt = {fmt}`（`format!("{}")`）、`PERM_ty = float`（I）），
    /// 经 `ComputePass::new` 进入 `ComputePipelineDesc.defines`。
    fn pipeline_defines(&self) -> Vec<ShaderDefine> {
        vec![
            perm_define("PERM_fmt", self.fmt),
            perm_define("PERM_ty", "float"),
        ]
    }
}

/// B 0x140a03ba0/0x14094e000 enqueue only the 8-byte EventId: payload is ZST.
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct RecalculateHeightMapCmd;

// ---------------------------------------------------------------- 笔画上传

/// 笔画宽度下限（O：`.rdata 0x142960674 = 0.01`）。
pub const MIN_STROKE_WIDTH: f32 = 0.01;
/// 单次上传的笔画数上限（O：`cmp uStack_98, 0x3fff`）。
pub const MAX_STROKES: usize = 0x3fff;
/// 每笔画最多上传的点数（O：`min(.., 0x400)`）。
pub const MAX_POINTS_PER_STROKE: usize = 0x400;
/// 超限日志（O 字符串，一次性）。
pub const OUT_OF_STROKES_MESSAGE: &str = "Ran out of live terrain strokes";
/// 断言文案（O，41 B）。
pub const POINT_COUNT_ASSERT: &str = "assertion failed: point_count_packed != 0";
/// puffin 作用域名（O）。
pub const PUFFIN_SCOPE: &str = "upload terrain strokes";

/// 笔画记录里被读到的字段（O：0x30 B 记录）。
#[derive(Clone, Debug, PartialEq)]
pub struct RasterStroke {
    pub id: u32,
    pub width: f32,
    /// 打进 `StrokeInfo` 第三个字的两个 f32（`pack_2x16f_uint(a, b)`）。
    pub params: [f32; 2],
    /// 点（源 12 B；只用 `x,y`）。
    pub points: Vec<[f32; 3]>,
}

/// 16 B 笔画描述（O：`+0` u64 地址、`+8` `(id << 10) | count`、`+0xc` 打包 f16 对）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StrokeInfo {
    pub device_address: u64,
    pub id_and_count: u32,
    pub packed_params: u32,
}

/// `utils::pack_2x16f_uint`（O 0x140c90df0）：`f16(a) | f16(b) << 16`（`half::f16::from_f32`）。
pub fn pack_2x16f_uint(a: f32, b: f32) -> u32 {
    let lo = half::f16::from_f32(a).to_bits() as u32;
    let hi = half::f16::from_f32(b).to_bits() as u32;
    (hi << 16) | lo
}

/// `DynamicConstants::allocate` + `device_address` + `into_gpu_lifetime_tracker` 的外部接缝。
pub trait StrokeAllocator {
    /// 分配 `bytes` 字节（对齐 1，对齐上限 0x10），返回 `(设备地址, 可写内存, tracker 句柄)`。
    fn allocate(&mut self, bytes: usize) -> (u64, Vec<u8>, u64);
    /// 内存写回（原版直接写映射内存；这里在返回前交还）。
    fn commit(&mut self, tracker: u64, data: &[u8]);
}

/// 一次笔画上传的结果。
#[derive(Debug, Default, PartialEq)]
pub struct StrokeUpload {
    pub infos: Vec<StrokeInfo>,
    /// `into_gpu_lifetime_tracker` 句柄（每笔画一个）。
    pub trackers: Vec<u64>,
    /// 一次性错误日志（去重后）。
    pub errors: Vec<String>,
}

/// 步骤 3：遍历笔画、过滤、分配 + 拷点 + 生成 `StrokeInfo`。
pub fn upload_strokes(strokes: &[RasterStroke], alloc: &mut dyn StrokeAllocator) -> StrokeUpload {
    let mut out = StrokeUpload::default();
    for s in strokes {
        // O：`fVar14 < width && points.len != 0`。
        if !(MIN_STROKE_WIDTH < s.width) || s.points.is_empty() {
            continue;
        }
        if out.infos.len() > MAX_STROKES {
            if !out.errors.iter().any(|e| e == OUT_OF_STROKES_MESSAGE) {
                out.errors.push(OUT_OF_STROKES_MESSAGE.to_owned());
            }
            continue;
        }
        let len = s.points.len();
        // O：`(len + 1) & !1`，上限 0x400，×8 B。
        let alloc_points = ((len + 1) & !1).min(MAX_POINTS_PER_STROKE);
        let (address, mut mem, tracker) = alloc.allocate(alloc_points * 8);
        let n = len.min(MAX_POINTS_PER_STROKE);
        for (i, p) in s.points.iter().take(n.min(mem.len() / 8)).enumerate() {
            mem[i * 8..i * 8 + 4].copy_from_slice(&p[0].to_le_bytes());
            mem[i * 8 + 4..i * 8 + 8].copy_from_slice(&p[1].to_le_bytes());
        }
        alloc.commit(tracker, &mem);
        let count = n;
        assert!(count != 0, "{}", POINT_COUNT_ASSERT);
        out.infos.push(StrokeInfo {
            device_address: address,
            id_and_count: (s.id << 10) | (count as u32 & 0x3ff),
            packed_params: pack_2x16f_uint(s.params[0], s.params[1]),
        });
        out.trackers.push(tracker);
    }
    out
}

// ---------------------------------------------------------------- 绑定名 / 预设

pub const H_STROKE_COUNT: u64 = binding_hash("stroke_count");
pub const H_STROKE_INFOS_ADDR: u64 = binding_hash("stroke_infos_device_address");
pub const H_STROKE_WIDTH: u64 = binding_hash("stroke_width");
pub const H_IMG_OUTPUT: u64 = binding_hash("img_output");
pub const H_STROKE_INDICES: u64 = binding_hash("stroke_indices");
pub const H_WATER_MASK: u64 = binding_hash("water_mask");
pub const H_ROCKY_TERRAIN: u64 = binding_hash("rocky_terrain");
pub const H_TERRAIN_SDG: u64 = binding_hash("terrain_sdg");
pub const H_DEBUG_NORMALS_BUFFER: u64 = binding_hash("debug_normals_buffer");
pub const H_IS_HORIZONTAL: u64 = binding_hash("is_horizontal");
pub const H_SIGMA: u64 = binding_hash("sigma");
pub const H_IMG_IN: u64 = binding_hash("img_in");
pub const H_IMG_OUT: u64 = binding_hash("img_out");
pub const H_TERRAIN_HEIGHTMAP: u64 = binding_hash("terrain_heightmap");
pub const H_VISUAL_TERRAIN_HEIGHTMAP: u64 = binding_hash("visual_terrain_heightmap");

pub const PRESET_TERRAIN_HEIGHT: TexturePreset = TexturePreset(9);
pub const PRESET_VISUAL_TERRAIN_HEIGHT: TexturePreset = TexturePreset(10);
pub const PRESET_STROKE_INDICES: TexturePreset = TexturePreset(0xb);
pub const PRESET_WATER_MASK: TexturePreset = TexturePreset(0x13);
pub const PRESET_ROCKY_TERRAIN: TexturePreset = TexturePreset(0xe);
pub const PRESET_TERRAIN_SDG: TexturePreset = TexturePreset(0xd);

pub const ACCESS_READ: u8 = 0x19;
pub const ACCESS_WRITE: u8 = 0x32;
/// 笔画宽度 uniform（O：`.rdata 0x1429258d0 = 1.5`）。
pub const STROKE_WIDTH_UNIFORM: f32 = 1.5;
/// 模糊 sigma（O：`.rdata 0x14292e6f0 = 2.0`）。
pub const BLUR_SIGMA: f32 = 2.0;
/// 栅格尺寸（O：`0x20000000200` → 512×512）。
pub const RASTER_SIZE: u32 = 512;
/// `"blur temp"` 格式（O：`new_2d(.., 0x6d)`）。
pub const BLUR_TEMP_FORMAT: u32 = 0x6d;

/// 笔画表 / 编辑状态（O：步骤 1 的两个资源里被读写的字段）。
#[derive(Clone, Debug, Default)]
pub struct ActiveStrokes {
    pub strokes: Vec<RasterStroke>,
    /// `+0x40`：本帧已栅格化。
    pub rasterized: bool,
    /// 变更 tick（`row+0x5c`）。
    pub changed_tick: u32,
}

/// 编辑状态的两个计数（O：`+0x10`、`+0x30`）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StrokeEditCounts {
    pub a: u64,
    pub b: u64,
}

/// 步骤 1 的早退条件。
pub fn should_skip(counts: StrokeEditCounts, rasterized: bool) -> bool {
    counts.b == 0 && counts.a == 0 && rasterized
}

// ---------------------------------------------------------------- 主体

/// W3 — `rasterize_terrain_stroke`（O 0x1408c7120）。返回 5 个 pass：栅格化、两遍模糊、地形导数（早退为空）。
pub fn rasterize_terrain_stroke<N: LibrarySsbo>(
    env: &mut PassEnv<'_>,
    alloc: &mut dyn StrokeAllocator,
    active: &mut ActiveStrokes,
    counts: StrokeEditCounts,
    world_tick: u32,
) -> (Vec<RecordedComputePass>, StrokeUpload) {
    active.changed_tick = world_tick;
    if should_skip(counts, active.rasterized) {
        return (Vec::new(), StrokeUpload::default());
    }
    let upload = upload_strokes(&active.strokes, alloc);

    // 笔画信息数组本身另行分配 → `stroke_infos_device_address`。
    let mut bytes = Vec::with_capacity(upload.infos.len() * 16);
    for i in &upload.infos {
        bytes.extend_from_slice(&i.device_address.to_le_bytes());
        bytes.extend_from_slice(&i.id_and_count.to_le_bytes());
        bytes.extend_from_slice(&i.packed_params.to_le_bytes());
    }
    let (infos_addr, mut mem, infos_tracker) = alloc.allocate(bytes.len().max(16));
    mem[..bytes.len()].copy_from_slice(&bytes);
    alloc.commit(infos_tracker, &mem);

    let mut trackers = upload.trackers.clone();
    trackers.push(infos_tracker);
    let count = upload.infos.len() as i32;
    let rasterize = env
        .new_pass(
            "Rasterize Terrain Stroke",
            &"/terrain_editing/rasterize_terrain_stroke.cs.hlsl",
        )
        .uniform(H_STROKE_COUNT, count)
        .uniform(
            H_STROKE_INFOS_ADDR,
            [infos_addr as u32, (infos_addr >> 32) as u32],
        )
        .uniform(H_STROKE_WIDTH, STROKE_WIDTH_UNIFORM)
        .bind_texture(H_IMG_OUTPUT, &PRESET_TERRAIN_HEIGHT, ACCESS_WRITE)
        .bind_texture(H_STROKE_INDICES, &PRESET_STROKE_INDICES, ACCESS_WRITE)
        .bind_texture(H_WATER_MASK, &PRESET_WATER_MASK, ACCESS_READ)
        .bind_texture(H_ROCKY_TERRAIN, &PRESET_ROCKY_TERRAIN, ACCESS_WRITE)
        .bind_texture(H_TERRAIN_SDG, &PRESET_TERRAIN_SDG, ACCESS_WRITE)
        .write_storage_buffer::<N>(H_DEBUG_NORMALS_BUFFER)
        .dispatch_with(move |api| {
            // O（0x1415fb550）：每个 tracker `Handle::track`，然后 `dispatch_3d`。
            for _ in &trackers {
                api.track_gpu_lifetime();
            }
            api.dispatch_3d(dispatch_args());
        });

    let temp = env.create_texture(
        "blur temp",
        TextureDesc::new_2d([RASTER_SIZE, RASTER_SIZE], BLUR_TEMP_FORMAT),
    );
    let blur1 = env
        .new_pass("Blur Terrain Stroke", &GaussBlur { fmt: "rgba32f" })
        .uniform(H_IS_HORIZONTAL, true)
        .uniform(H_SIGMA, BLUR_SIGMA)
        .bind_texture(H_IMG_IN, &PRESET_TERRAIN_HEIGHT, ACCESS_READ)
        .bind_texture(H_IMG_OUT, &&temp, ACCESS_WRITE)
        .dispatch_2d(RASTER_SIZE, RASTER_SIZE);
    let blur2 = env
        .new_pass("Blur Terrain Stroke", &GaussBlur { fmt: "r16" })
        .uniform(H_IS_HORIZONTAL, false)
        .uniform(H_SIGMA, BLUR_SIGMA)
        .bind_texture(H_IMG_IN, &&temp, ACCESS_READ)
        .bind_texture(H_IMG_OUT, &PRESET_TERRAIN_HEIGHT, ACCESS_WRITE)
        .dispatch_2d(RASTER_SIZE, RASTER_SIZE);
    let derivative = env
        .new_pass("Terrain Derivative", &"/terrain_derivative.cs.hlsl")
        .bind_texture(H_TERRAIN_HEIGHTMAP, &PRESET_TERRAIN_HEIGHT, ACCESS_READ)
        .bind_texture(
            H_VISUAL_TERRAIN_HEIGHTMAP,
            &PRESET_VISUAL_TERRAIN_HEIGHT,
            ACCESS_READ,
        )
        .bind_texture(H_ROCKY_TERRAIN, &PRESET_ROCKY_TERRAIN, ACCESS_WRITE)
        .dispatch_2d(RASTER_SIZE, RASTER_SIZE);

    active.rasterized = true;
    (vec![rasterize, blur1, blur2, derivative], upload)
}

/// 兼容旧入口名（O：`closure$1` 的 `dispatch_3d` 里的线程数）。
pub fn dispatch_args() -> DispatchArgs {
    DispatchArgs {
        x: RASTER_SIZE,
        y: RASTER_SIZE,
        z: 1,
        indirect_index: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::compute_pass_builder::DispatchMode;
    use crate::systems::render_loop::compute_passes::pass_kit::imm;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::*;

    #[test]
    fn hashes_are_exe_immediates() {
        assert_eq!(H_STROKE_COUNT, 0x5efaa1884a48985d);
        assert_eq!(H_STROKE_INFOS_ADDR, 0x497d31a6f59bd1e5);
        assert_eq!(H_STROKE_WIDTH, imm(-0x75a3691b801e70c6));
        assert_eq!(H_STROKE_INDICES, imm(-0x1c9a63f0421840fd));
        assert_eq!(H_TERRAIN_SDG, imm(-0x54dbd78cbb54573f));
        assert_eq!(H_DEBUG_NORMALS_BUFFER, imm(-0x299dcb9f05af72c2));
        assert_eq!(H_IMG_IN, imm(-0x27d285f5246e9b3e));
        assert_eq!(H_IMG_OUT, 0x1ca51a73373208db);
        assert_eq!(POINT_COUNT_ASSERT.len(), 0x29);
        assert_eq!(PUFFIN_SCOPE.len(), 0x16);
    }

    #[test]
    fn pack_two_f16() {
        // f16(1.0) = 0x3c00, f16(2.0) = 0x4000
        assert_eq!(pack_2x16f_uint(1.0, 2.0), 0x4000_3c00);
        assert_eq!(pack_2x16f_uint(0.0, -0.0), 0x8000_0000);
    }

    #[derive(Default)]
    struct Alloc {
        next: u64,
        sizes: Vec<usize>,
        written: Vec<Vec<u8>>,
    }
    impl StrokeAllocator for Alloc {
        fn allocate(&mut self, bytes: usize) -> (u64, Vec<u8>, u64) {
            self.next += 0x1000;
            self.sizes.push(bytes);
            (self.next, vec![0; bytes], self.next)
        }
        fn commit(&mut self, _t: u64, data: &[u8]) {
            self.written.push(data.to_vec());
        }
    }

    fn stroke(id: u32, width: f32, n: usize) -> RasterStroke {
        RasterStroke {
            id,
            width,
            params: [1.0, 2.0],
            points: (0..n).map(|i| [i as f32, i as f32 + 0.5, 99.0]).collect(),
        }
    }

    #[test]
    fn upload_filters_narrow_and_empty_strokes() {
        let mut a = Alloc::default();
        let up = upload_strokes(
            &[stroke(1, 0.01, 3), stroke(2, 0.5, 0), stroke(3, 0.011, 3)],
            &mut a,
        );
        assert_eq!(up.infos.len(), 1, "width 必须严格大于 0.01，点数非 0");
        assert_eq!(up.infos[0].id_and_count, (3 << 10) | 3);
        assert_eq!(up.infos[0].packed_params, 0x4000_3c00);
        assert_eq!(up.infos[0].device_address, 0x1000);
    }

    #[test]
    fn upload_allocation_size_and_point_layout() {
        let mut a = Alloc::default();
        let up = upload_strokes(&[stroke(7, 1.0, 3)], &mut a);
        // (3 + 1) & !1 = 4 点 × 8 B。
        assert_eq!(a.sizes, [32]);
        let mem = &a.written[0];
        assert_eq!(f32::from_le_bytes(mem[8..12].try_into().unwrap()), 1.0);
        assert_eq!(f32::from_le_bytes(mem[12..16].try_into().unwrap()), 1.5);
        assert_eq!(up.trackers, [0x1000]);
    }

    #[test]
    fn upload_caps_points_at_0x400_and_wraps_count_into_ten_bits() {
        let mut a = Alloc::default();
        let up = upload_strokes(&[stroke(5, 1.0, 5000)], &mut a);
        assert_eq!(a.sizes, [0x400 * 8]);
        // count = 0x400 → `& 0x3ff` = 0（O：`and ebx, 0x3ff`）。
        assert_eq!(up.infos[0].id_and_count, 5 << 10);
    }

    #[test]
    fn too_many_strokes_logs_once_and_skips() {
        let mut a = Alloc::default();
        let many: Vec<_> = (0..(MAX_STROKES as u32 + 10))
            .map(|i| stroke(i, 1.0, 1))
            .collect();
        let up = upload_strokes(&many, &mut a);
        // 条件是 `len > 0x3fff` 才拒绝 → 允许收到 0x4000 个。
        assert_eq!(up.infos.len(), MAX_STROKES + 1);
        assert_eq!(up.errors, [OUT_OF_STROKES_MESSAGE.to_owned()]);
    }

    #[test]
    fn skip_only_when_nothing_pending_and_already_rasterized() {
        assert!(should_skip(StrokeEditCounts::default(), true));
        assert!(!should_skip(StrokeEditCounts::default(), false));
        assert!(!should_skip(StrokeEditCounts { a: 1, b: 0 }, true));
        assert!(!should_skip(StrokeEditCounts { a: 0, b: 2 }, true));
    }

    #[test]
    fn gauss_blur_source_identity() {
        let a = GaussBlur { fmt: "rgba32f" };
        assert_eq!(
            a.pipeline_defines()[0],
            ("PERM_fmt".to_owned(), "rgba32f".to_owned())
        );
        assert_ne!(a.source_hash(), GaussBlur { fmt: "r16" }.source_hash());
        assert_eq!(a.as_compute_pipeline().uri.path, "/gauss_blur.cs.hlsl");
    }

    struct Api(Vec<String>);
    impl ComputeDispatchApi for Api {
        fn dispatch_1d(&mut self, x: u32) {
            self.0.push(format!("1d {x}"));
        }
        fn dispatch_2d(&mut self, x: u32, y: u32) {
            self.0.push(format!("2d {x} {y}"));
        }
        fn dispatch_3d(&mut self, a: DispatchArgs) {
            self.0.push(format!("3d {} {} {}", a.x, a.y, a.z));
        }
        fn push_constants(&mut self, _: &[u8]) {}
        fn track_gpu_lifetime(&mut self) {
            self.0.push("track".into());
        }
    }

    #[test]
    fn full_pass_chain() {
        let mut e = env_with(
            &[
                H_IMG_OUTPUT,
                H_STROKE_INDICES,
                H_WATER_MASK,
                H_ROCKY_TERRAIN,
                H_TERRAIN_SDG,
                H_DEBUG_NORMALS_BUFFER,
                H_IMG_IN,
                H_IMG_OUT,
                H_TERRAIN_HEIGHTMAP,
                H_VISUAL_TERRAIN_HEIGHTMAP,
            ],
            &[
                H_STROKE_COUNT,
                H_STROKE_INFOS_ADDR,
                H_STROKE_WIDTH,
                H_IS_HORIZONTAL,
                H_SIGMA,
            ],
            |lib, dev| {
                lib.add_new::<SsboA>(dev);
            },
        );
        let mut a = Alloc::default();
        let mut active = ActiveStrokes {
            strokes: vec![stroke(1, 1.0, 2), stroke(2, 1.0, 4)],
            ..Default::default()
        };
        let (passes, up) = e.run(|penv| {
            rasterize_terrain_stroke::<SsboA>(
                penv,
                &mut a,
                &mut active,
                StrokeEditCounts { a: 1, b: 0 },
                9,
            )
        });
        assert_eq!(up.infos.len(), 2);
        assert!(active.rasterized);
        assert_eq!(active.changed_tick, 9);
        let names: Vec<_> = passes.iter().map(|p| p.name).collect();
        assert_eq!(
            names,
            [
                "Rasterize Terrain Stroke",
                "Blur Terrain Stroke",
                "Blur Terrain Stroke",
                "Terrain Derivative"
            ]
        );
        assert_eq!(e.frame.textures[0].0, "blur temp");
        assert_eq!(
            (e.frame.textures[0].1.extent, e.frame.textures[0].1.format),
            ([512, 512, 1], 0x6d)
        );
        assert_eq!(e.uniform_u32(&passes[0], H_STROKE_COUNT), 2);
        assert_eq!(e.uniform_f32(&passes[0], H_STROKE_WIDTH), 1.5);
        assert_eq!(e.uniform_u32(&passes[1], H_IS_HORIZONTAL), 1);
        assert_eq!(e.uniform_u32(&passes[2], H_IS_HORIZONTAL), 0);
        assert_eq!(e.uniform_f32(&passes[2], H_SIGMA), 2.0);
        assert!(matches!(
            passes[1].dispatch,
            DispatchMode::Threads {
                x: 512,
                y: 512,
                z: 1
            }
        ));
        // 栅格化 pass 的 dispatch 闭包：3 个 tracker（2 笔画 + 信息数组）后 dispatch。
        let mut api = Api(vec![]);
        match passes.into_iter().next().unwrap().dispatch {
            DispatchMode::With(f) => f(&mut api),
            _ => panic!("expected dispatch_with"),
        }
        assert_eq!(api.0, ["track", "track", "track", "3d 512 512 1"]);
        // 已栅格化且无待处理编辑 → 第二次调用什么也不做。
        let (again, _) = e.run(|penv| {
            rasterize_terrain_stroke::<SsboA>(
                penv,
                &mut a,
                &mut active,
                StrokeEditCounts::default(),
                10,
            )
        });
        assert!(again.is_empty());
        assert_eq!(active.changed_tick, 10, "变更 tick 先于早退写入");
    }
}
