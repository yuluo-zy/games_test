//! `terrain_lighting_texture.rs` — W24 地形光照纹理计算 + mips（O：`calc_terrain_lighting_texture` 0x140a51440，2290 B）。
//!
//! 流程（逐条对照机器码）：
//! 1. 内联 `get_resource_id(TEXTURE_LIBRARY)` → 读锁 `AssetTextureLibrary` → `get_rg_texture(preset 8)`（缺失 / 句柄 `None` → panic）；
//!    返回里带 desc（`extent_2d` = dispatch 尺寸，同一份 desc 交给 `generate_mips`）。
//! 2. `ComputePass::new("Calculate Terrain Lighting Texture", "/terrain_lighting_texture.cs.hlsl")`：
//!    - 输出：preset **8**（`img_output` = 0xb981790202418bc8）访问码 **0x32**（写）；
//!    - `get_resource_id(0xbba451ee6a08a3ff, 0x8d008d15febcf29d)`（`CameraMatrices`，缺失 → unwrap panic）→
//!      `eye_position()` → `write_uniform<Vec3>`（`camera_position`）；
//!    - preset 9（`terrain_heightmap`）、preset 0xa（`visual_terrain_heightmap`），访问码 0x19；
//!    - `write_uniform<bool>`（`bool_render_shadows`，值 **true**）；
//!    - preset 0x11（`shadow_map`），访问码 0x19；
//!    - `dispatch_2d(w, h)`（模式 1，z=1；`(w, h)` = preset 8 的 extent）。
//! 3. `generate_mips<TexturePreset>(ctx, world, &desc, TexturePreset(8), "terrain")`。
//!
//! `CameraMatrices::eye_position`（O 0x140c96d40，58 B）：`inverse_view(+0xc0, 列主序 4×16 B)` 乘 `(0,0,0,1)` 取 xyz，
//! 即 `x*0 + y*0 + z*0 + w` 的 xyz（保留 `mulps` 零乘的 NaN 传播）。

use glam::{Mat4, Vec3, Vec4};

use crate::render::pass_builder::pass_resources::{PassResources, TypeId128};
use crate::resources::texture_presets::TexturePreset;
use crate::systems::render_loop::compute_passes::generate_mips::generate_mips;
use crate::systems::render_loop::compute_passes::pass_kit::{
    PassEnv, ResourceView, TEX_READ, TEX_WRITE, binding_hash,
};

pub const PASS_NAME: &str = "Calculate Terrain Lighting Texture";
pub const SHADER_URI: &str = "/terrain_lighting_texture.cs.hlsl";
/// `generate_mips` 的 SPD 名（O：rodata "terrain"，7 B）。
pub const MIPS_NAME: &str = "terrain";
pub const OUTPUT: (u64, u8, u8) = (binding_hash("img_output"), 8, TEX_WRITE); // 0xb981790202418bc8
pub const KEY_EYE_POSITION: u64 = binding_hash("camera_position"); // 0x4bbdb30b873d3990
pub const KEY_FLAG: u64 = binding_hash("bool_render_shadows"); // 0x418a98441f59a54c
/// 三个只读预设（O：`(绑定名哈希, preset)`；第三个在 bool uniform 之后）。
pub const INPUTS: [(u64, u8); 3] = [
    (binding_hash("terrain_heightmap"), 9),
    (binding_hash("visual_terrain_heightmap"), 0xa),
    (binding_hash("shadow_map"), 0x11), // 0x11651f4dc9841aa4
];

/// `utils::camera::CameraMatrices` 的可见部分：`+0xc0` 起的 4 个列向量（O：`eye_position` 读 `[rdx+0xc0..0x100]`）。
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CameraMatrices {
    _before: [u8; 0xc0],
    pub inverse_view: Mat4,
}

impl CameraMatrices {
    pub fn with_inverse_view(inverse_view: Mat4) -> Self {
        CameraMatrices {
            _before: [0; 0xc0],
            inverse_view,
        }
    }

    /// O 0x140c96d40：`((x_axis*0 + y_axis*0) + z_axis*0) + w_axis` 的 xyz。
    pub fn eye_position(&self) -> Vec3 {
        let z = Vec4::ZERO;
        let m = &self.inverse_view;
        let acc = m.y_axis * z + m.x_axis * z;
        let acc = z * m.z_axis + acc;
        (acc + m.w_axis).truncate()
    }
}

// SAFETY: `CameraMatrices` 布局按 `eye_position` 的读偏移（+0xc0）复刻。
unsafe impl ResourceView for CameraMatrices {
    const TYPE_ID: TypeId128 = TypeId128 {
        lo: 0xbba451ee6a08a3ff,
        hi: 0x8d008d15febcf29d,
    };
}

/// W24 系统（O 0x140a51440）。
pub fn calc_terrain_lighting_texture(env: &mut PassEnv<'_>) {
    let preset8 = TexturePreset(OUTPUT.1);
    // 内联的纹理库查找：`get_rg_texture(8)`（缺失 / 无句柄 → panic）。
    let imported = PassResources::new_from(env.world)
        .texture_library()
        .get_rg_texture(preset8);
    let desc = imported.desc;

    let pass = env
        .pass(PASS_NAME, SHADER_URI)
        .bind_texture(OUTPUT.0, &preset8, OUTPUT.2);
    let eye = env.resource::<CameraMatrices>().eye_position();
    let pass = pass
        .uniform(KEY_EYE_POSITION, eye)
        .bind_texture(INPUTS[0].0, &TexturePreset(INPUTS[0].1), TEX_READ)
        .bind_texture(INPUTS[1].0, &TexturePreset(INPUTS[1].1), TEX_READ)
        .uniform(KEY_FLAG, true)
        .bind_texture(INPUTS[2].0, &TexturePreset(INPUTS[2].1), TEX_READ);
    let recorded = pass.dispatch_2d(desc.extent[0], desc.extent[1]);
    env.record(recorded);
    generate_mips(env, &desc, &preset8, MIPS_NAME);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::compute_pass_builder::DispatchMode;
    use crate::rhapsody_types::TextureDesc;
    use crate::systems::render_loop::compute_passes::generate_mips as gm;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::*;

    fn world(with_camera: bool) -> TestWorld {
        // 预设 8 的 desc：256×128、B10G11R11（0x7a）、5 个 mip（`get_rg_texture(8).desc`，O 0x14093cf30）。
        let preset_desc = |p: TexturePreset| {
            if p.0 == 8 {
                TextureDesc::new_2d([256, 128], 0x7a).mip_levels(5)
            } else {
                DEFAULT_PRESET_DESC
            }
        };
        let mut hashes = vec![OUTPUT.0];
        hashes.extend(INPUTS.iter().map(|i| i.0));
        hashes.extend([
            gm::HASH_MIP0,
            gm::HASH_MIPS,
            gm::HASH_MIP_LAST,
            gm::HASH_ATOMIC,
        ]);
        let w = TestWorld::with_presets(
            Reflection::new()
                .bind(&hashes)
                .uniforms(&[KEY_EYE_POSITION, KEY_FLAG]),
            preset_desc,
        );
        if with_camera {
            let m = Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0));
            w.with_resource(CameraMatrices::with_inverse_view(m))
        } else {
            w
        }
    }

    #[test]
    fn eye_position_is_translation_column() {
        let m = Mat4::from_cols(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(4.0, 5.0, 6.0, 1.0),
        );
        assert_eq!(
            CameraMatrices::with_inverse_view(m).eye_position(),
            Vec3::new(4.0, 5.0, 6.0)
        );
        // 零乘保留 NaN 传播（`mulps` 对 inf 得 NaN）。
        let mut n = m;
        n.x_axis.x = f32::INFINITY;
        assert!(
            CameraMatrices::with_inverse_view(n)
                .eye_position()
                .x
                .is_nan()
        );
        assert_eq!(core::mem::offset_of!(CameraMatrices, inverse_view), 0xc0);
    }

    #[test]
    fn records_lighting_pass_then_spd_mips() {
        let w = world(true);
        let mut f = TestFrame::default();
        with_env(&w, &mut f, calc_terrain_lighting_texture);
        assert_eq!(
            f.order,
            ["Calculate Terrain Lighting Texture", "Generate mips"]
        );
        let p = &f.passes[0];
        assert!(matches!(
            p.dispatch,
            DispatchMode::Threads {
                x: 256,
                y: 128,
                z: 1
            }
        ));
        // 访问：color library 读，输出写(0x32)，9/0xa 读，0x11 读。
        let acc: Vec<u8> = p.builder.accesses.iter().map(|a| a.access).collect();
        assert_eq!(acc, [0x1a, 0x32, 0x19, 0x19, 0x19]);
        // uniform：eye 位置 @0（3×f32），flag @16 = 1。
        let d = p.uniform_binder.as_ref().unwrap().data();
        let f32_at = |o: usize| f32::from_le_bytes(d[o..o + 4].try_into().unwrap());
        assert_eq!((f32_at(0), f32_at(4), f32_at(8)), (1.0, 2.0, 3.0));
        assert_eq!(uniform_u32(p, 16), 1);
        // generate_mips 用同一份 desc：mips-1 = 4，groups = 4×2。
        assert_eq!(
            f.passes[1].push_constants,
            gm::spd_push_constants(4, [4, 2])
        );
        assert_eq!(f.temporal_buffers[0].0, "SPD Atomic: terrain");
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn missing_camera_resource_unwraps() {
        let w = world(false);
        let mut f = TestFrame::default();
        with_env(&w, &mut f, calc_terrain_lighting_texture);
    }

    #[test]
    fn constants_match_machine_code() {
        assert_eq!(OUTPUT.0, (-0x467e86fdfdbe7438_i64) as u64);
        assert_eq!(KEY_EYE_POSITION, 0x4bbdb30b873d3990);
        assert_eq!(KEY_FLAG, 0x418a98441f59a54c);
        assert_eq!(INPUTS[2].0, 0x11651f4dc9841aa4);
        assert_eq!(
            <CameraMatrices as ResourceView>::TYPE_ID.lo,
            (-0x445bae1195f75c01_i64) as u64
        );
        assert_eq!(
            <CameraMatrices as ResourceView>::TYPE_ID.hi,
            (-0x72ff72ea01430d63_i64) as u64
        );
        assert_eq!(OUTPUT.2, 0x32);
    }
}
