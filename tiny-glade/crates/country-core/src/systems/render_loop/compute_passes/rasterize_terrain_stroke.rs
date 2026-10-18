//! Original B W3 terrain stroke rasterization, two Gauss passes and derivative.
//!
//! Full PDB-bounded RNE: rasterize@1408c7120/10387B,
//! GaussBlur::as_compute_pipeline@1408ca1c0/1435B. The host borrows strokes and
//! rasterized from ONE canonical TerrainEditingData; it marks the real Bevy
//! changed tick before inspecting the global RecalculateHeightMapCmd buffers.
//!
//! Native points copy XZ (76dd/76e3), round the allocation to an even count,
//! and retain actual page owners for 16-byte physical shader fetches. Padding
//! values were not established and are not newly initialized. Info size is
//! precisely count*16, including zero; count0's exact original SPIR-V exits
//! before any info/point BDA load, so only nonempty chunks declare reads.
//!
//! First Gauss source is (rgba32f,true)->PERM_ty=float4; second (r16,false)
//! ->float. Native source bool writes are 817a/8447; pipeline define writes
//! are a437..a48a. A constant float definition was an implementation error.
//! The original rasterized flag is set only after the four real pass records.
//!
//! Current-architecture adaptation: real DynamicConstantsChunk ranges enter
//! RecordedComputePass metadata and the same graph session's ResourceOwner /
//! submission-completion protocol, replacing the previous empty track call.

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::neg_cmp_op_on_partial_ord)]
#![allow(dead_code, unused_imports)]

use crate::render::pass_builder::compute_pass_builder::DispatchArgs;
use crate::render::pass_builder::shader_uri::{
    AsComputeShaderSource, ComputeShaderSource, ShaderDefValue, ShaderUri,
};
use crate::resources::texture_presets::TexturePreset;

use super::pass_kit::{PassEnv, binding_hash};
use crate::render::pass_builder::shader_uri::{ShaderDefine, perm_define};
use crate::rhapsody_types::TextureDesc;

// ---------------------------------------------------------------- GaussBlur

/// `/gauss_blur.cs.hlsl:cs_main` 源（O）。`fmt` = 输出图像格式名（`PERM_fmt` define）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GaussBlur {
    pub fmt: &'static str,
    /// Original +0x10 byte, included in source identity and PERM_ty.
    pub vec4: bool,
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
    /// Native fmt has two literal quote pieces at 142ab5690/142ab5688;
    /// the Display callback is 140533930 -> Formatter::pad. PERM_ty is
    /// float4 when vec4, otherwise float.
    /// 经 `ComputePass::new` 进入 `ComputePipelineDesc.defines`。
    fn pipeline_defines(&self) -> Vec<ShaderDefine> {
        vec![
            perm_define("PERM_fmt", format!("\"{}\"", self.fmt)),
            perm_define("PERM_ty", if self.vec4 { "float4" } else { "float" }),
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

/// 16-byte original GPU descriptor. Source ID is the live map key, not an
/// invented sequential upload index. count1024 deliberately encodes low10=0.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StrokeInfo {
    pub device_address: u64,
    pub id_and_count: u32,
    pub packed_params: u32,
}
pub fn pack_2x16f_uint(a: f32, b: f32) -> u32 {
    half::f16::from_f32(a).to_bits() as u32 | ((half::f16::from_f32(b).to_bits() as u32) << 16)
}

/// Original BDA fetches are uint4/16B even when only one point is used.
/// The even allocation covers that read. Only meaningful XZ bytes are copied;
/// the original inactive padding value is not asserted or newly initialized.
fn point_upload_bytes(points: &[[f32; 3]]) -> (usize, Vec<u8>) {
    let n = points.len().min(MAX_POINTS_PER_STROKE);
    let allocated = ((points.len() + 1) & !1).min(MAX_POINTS_PER_STROKE) * 8;
    let mut bytes = Vec::with_capacity(n * 8);
    for point in points.iter().take(n) {
        bytes.extend_from_slice(&point[0].to_le_bytes());
        bytes.extend_from_slice(&point[2].to_le_bytes());
    }
    (allocated, bytes)
}

struct StrokeUpload {
    infos: Vec<StrokeInfo>,
    chunks: Vec<rhapsody::raw::dynamic_constants::DynamicConstantsChunk>,
}
fn upload_strokes(
    strokes: &std::collections::HashMap<
        crate::resources::terrain_editing::StrokeId,
        crate::resources::terrain_editing::TerrainStroke,
    >,
    constants: &mut rhapsody::raw::dynamic_constants::DynamicConstants,
) -> rhapsody::Result<StrokeUpload> {
    let mut result = StrokeUpload {
        infos: Vec::new(),
        chunks: Vec::new(),
    };
    for (id, stroke) in strokes {
        // MIN >= falloff skips; unordered original UCOMISS does not skip.
        if stroke.falloff <= MIN_STROKE_WIDTH || stroke.points.is_empty() {
            continue;
        }
        if result.infos.len() > MAX_STROKES {
            static WARNED: std::sync::atomic::AtomicBool =
                std::sync::atomic::AtomicBool::new(false);
            if !WARNED.swap(true, std::sync::atomic::Ordering::Relaxed) {
                log::error!(target: "country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke",
                    "{}", OUT_OF_STROKES_MESSAGE);
            }
            continue;
        }
        let (allocated, bytes) = point_upload_bytes(&stroke.points);
        let chunk = constants.allocate_aligned(allocated as u64, Some(1))?;
        chunk.write_bytes(&bytes)?;
        let address = chunk.device_address()?;
        let count = stroke.points.len().min(MAX_POINTS_PER_STROKE);
        assert!(count != 0, "{}", POINT_COUNT_ASSERT);
        result.infos.push(StrokeInfo {
            device_address: address,
            id_and_count: (id.0 << 10) | (count as u32 & 0x3ff),
            packed_params: pack_2x16f_uint(stroke.height_ws, stroke.falloff),
        });
        result.chunks.push(chunk);
    }
    Ok(result)
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

/// Original W3 has live TerrainEditingData plus the global Recalculate event
/// buffers. The caller borrows both fields from the ONE canonical resource via
/// TerrainEditingData::rasterization_parts; no layout ResourceView is involved.
pub fn rasterize_terrain_stroke(
    env: &mut PassEnv<'_>,
    constants: &mut rhapsody::raw::dynamic_constants::DynamicConstants,
    strokes: &std::collections::HashMap<
        crate::resources::terrain_editing::StrokeId,
        crate::resources::terrain_editing::TerrainStroke,
    >,
    rasterized: &mut bool,
    recalculation_requested: bool,
) -> rhapsody::Result<usize> {
    if !recalculation_requested && *rasterized {
        return Ok(0);
    }
    let upload = upload_strokes(strokes, constants)?;
    let mut bytes = Vec::with_capacity(upload.infos.len() * 16);
    for info in &upload.infos {
        bytes.extend_from_slice(&info.device_address.to_le_bytes());
        bytes.extend_from_slice(&info.id_and_count.to_le_bytes());
        bytes.extend_from_slice(&info.packed_params.to_le_bytes());
    }
    // The original requests exactly count*16, including size0. The current
    // allocator's original-supported zero-size path must be integrated first.
    let infos = constants.allocate(bytes.len() as u64)?;
    infos.write_bytes(&bytes)?;
    let infos_addr = infos.device_address()?;
    let mut chunks = upload.chunks;
    if infos.size != 0 {
        chunks.push(infos);
    }
    // count0's original SPIR-V exits its loop before loading the BDA pointer.
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
        .write_storage_buffer::<crate::render::normals_debug::NormalDebugSsbo>(
            H_DEBUG_NORMALS_BUFFER,
        )
        .dispatch_2d(RASTER_SIZE, RASTER_SIZE)
        .with_device_address_reads(chunks);

    let temp = env.create_texture(
        "blur temp",
        TextureDesc::new_2d([RASTER_SIZE, RASTER_SIZE], BLUR_TEMP_FORMAT),
    );
    let blur1 = env
        .new_pass(
            "Blur Terrain Stroke",
            &GaussBlur {
                fmt: "rgba32f",
                vec4: true,
            },
        )
        .uniform(H_IS_HORIZONTAL, true)
        .uniform(H_SIGMA, BLUR_SIGMA)
        .bind_texture(H_IMG_IN, &PRESET_TERRAIN_HEIGHT, ACCESS_READ)
        .bind_texture(H_IMG_OUT, &&temp, ACCESS_WRITE)
        .dispatch_2d(RASTER_SIZE, RASTER_SIZE);
    let blur2 = env
        .new_pass(
            "Blur Terrain Stroke",
            &GaussBlur {
                fmt: "r16",
                vec4: false,
            },
        )
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

    for pass in [rasterize, blur1, blur2, derivative] {
        env.frame.record(pass);
    }
    *rasterized = true;
    Ok(4)
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
    #[test]
    fn gauss_original_vector_flag_changes_define_and_source_hash() {
        let vector = GaussBlur {
            fmt: "rgba32f",
            vec4: true,
        };
        let scalar = GaussBlur {
            fmt: "rgba32f",
            vec4: false,
        };
        assert_eq!(
            vector.pipeline_defines(),
            vec![
                perm_define("PERM_fmt", "\"rgba32f\""),
                perm_define("PERM_ty", "float4")
            ]
        );
        assert_eq!(
            scalar.pipeline_defines()[1],
            perm_define("PERM_ty", "float")
        );
        assert_ne!(vector.source_hash(), scalar.source_hash());
        use rhapsody::shader_pipeline::shader_defines::ShaderDefines;
        assert_eq!(ShaderDefines(vector.pipeline_defines()).artifact_hash(), 0xb9dd0465835cd591);
        assert_eq!(ShaderDefines(GaussBlur { fmt: "r16", vec4: false }.pipeline_defines()).artifact_hash(),
            0xdc5fd7e4b9bf9935);
    }
    #[test]
    fn point_upload_uses_xz_and_retains_original_even_allocation() {
        let (allocated, bytes) = point_upload_bytes(&[[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]]);
        assert_eq!(allocated, 32);
        assert_eq!(bytes.len(), 24);
        let actual: Vec<f32> = bytes
            .chunks_exact(4)
            .map(|word| f32::from_le_bytes(word.try_into().unwrap()))
            .collect();
        assert_eq!(actual, [1., 3., 4., 6., 7., 9.]);
    }
    #[test]
    fn point_upload_caps_at_original_1024_and_retains_half_packing() {
        let (allocated, bytes) = point_upload_bytes(&vec![[1., 2., 3.]; 1025]);
        assert_eq!(allocated, 8192);
        assert_eq!(bytes.len(), 8192);
        assert_eq!(1024u32 & 0x3ff, 0);
        assert_eq!(pack_2x16f_uint(1., 2.), 0x4000_3c00);
    }
}
