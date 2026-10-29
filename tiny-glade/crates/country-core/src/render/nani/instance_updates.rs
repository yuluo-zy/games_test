//! `instance_updates.rs` — nani 各 `nani_update_*` 系统的**实例载荷构建函数**（B 阶段按机器码恢复）。
//!
//! 来源：旧树 `render/nani/systems.rs`（Git `b5965593` 的 B 阶段成果）。该文件被整体排除，因为其中的 ECS 系统
//! 包装层（`nani_update_*`、`nani_maintain_instances`、`nani_flush_instance_updates`）依赖尚未恢复的游戏组件
//! （`NaniActiveInstance`/`NaniInstanceDesc`、`Wren`、`Plaster`、`StairsFloorMeshChunk` 等）。这里只迁**不依赖
//! 那些类型**的部分：每个系统对应的纯构建函数 `build_*`（输入普通值/视图，输出 `InstanceUpdate<N>` 字载荷）、
//! 打包辅助、常量，及其测试。系统层已在 `systems.rs`（R8）。
//!
//! 标注：**O**=机器码观测；**I**=推断；**U**=未知。原文的 O/I/U 注释与地址保持不动。
//! 与原文件一致，同名重复实现（`trs_words`/`trs_words12`/`gb_affine_words` 等）**没有合并**：它们的运算顺序
//! 各自对应机器码里不同的调用点，合并属于后续去重任务（见 `evidence/country-core-audit/R2_nani_instance_updates.md`）。

// 保留机器码的 `max/min` 链、取反比较（NaN 语义）、嵌套分支与证据注释的转录；不改可执行表达式。
#![allow(clippy::manual_clamp)]
#![allow(clippy::neg_cmp_op_on_partial_ord)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::doc_lazy_continuation)]

use glam::{Mat3, Quat, Vec2, Vec3, Vec4};

use ::utils::scalar::max_f32;
use crate::render::nani::gpu_world::{GpuWorldSubsetKind, InstanceCullData};
use crate::render::nani::instance_data::*;
use crate::resources::color_id::ColorId;
use crate::utils::components::transform::Transform;

/// 一次 `update_instance_data<T>(subset, key, &position, sort_key, &data)` 调用的全部参数（O）：
/// 写 16B 紧凑 key `{position: Vec3 @0, f16(sort_key) @0xc}`，并复制 `size_of::<T>()` 字节实例数据。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InstanceUpdate<const N: usize> {
    pub position: Vec3,
    pub sort_key: f32,
    /// 实例数据载荷（小端 32 位字；`N*4 == size_of::<T>()`）。
    pub words: [u32; N],
}

#[inline]
pub fn f32w(x: f32) -> u32 {
    x.to_bits()
}

/// 16B 紧凑 key（O：`extend_no_data` 的 items 元素；`update_instance_data` 写同一布局的前 14 字节）：
/// `{position: Vec3 @0, sort_key: f16 @0xc, mesh_part: u16 @0xe}` —— 即 gpu_world 的 [`InstanceCullData`]
/// （`radius` = f16 sort_key，`mesh_index` = mesh_part）。
pub type NaniCompactKey = InstanceCullData;

/// 把紧凑 key 展成 4 个小端 u32 字（测试用）。
pub fn compact_key_words(k: &InstanceCullData) -> [u32; 4] {
    [
        k.position.x.to_bits(),
        k.position.y.to_bits(),
        k.position.z.to_bits(),
        k.radius.to_bits() as u32 | ((k.mesh_index as u32) << 16),
    ]
}

/// `max(|sx|,|sy|,|sz|)`（O：`andps abs; maxss` 链，NaN 时取另一操作数）。
pub fn max_abs_scale(scale: Vec3) -> f32 {
    scale.z.abs().max(scale.y.abs()).max(scale.x.abs())
}

/// 新实例的紧凑 key（O，0x14206fb1c..0x14206fba1）：
/// `position = transform.translation`；`sort_key = f16::from_f32(max(|sx|,|sy|,|sz|))`；`mesh_part = part as u16`。
pub fn compact_key_for(transform: &Transform, mesh_part: u32) -> NaniCompactKey {
    InstanceCullData {
        position: transform.translation,
        radius: half::f16::from_f32(max_abs_scale(transform.scale)),
        mesh_index: mesh_part as u16,
    }
}

/// 植物（O）：12 字 = `Mat3::from_quat(rot)` 各列按 `scale.{x,y,z}` 缩放（列主序）+ 平移；
/// `position = translation`，`sort_key = max(|s|)`。
pub fn build_plant(translation: Vec3, rotation: Quat, scale: Vec3) -> InstanceUpdate<12> {
    let (words, sort_key) = trs_words12(translation, rotation, scale);
    InstanceUpdate {
        position: translation,
        sort_key,
        words,
    }
}

/// 所有 `nani_update_*` 共用的 12 字仿射打包（O，plant 0x142074740 起每个系统体内同一序列）。
pub fn trs_words12(translation: Vec3, rotation: Quat, scale: Vec3) -> ([u32; 12], f32) {
    let m = Mat3::from_quat(rotation);
    let c0 = m.x_axis * scale.x;
    let c1 = m.y_axis * scale.y;
    let c2 = m.z_axis * scale.z;
    let w = [
        c0.x,
        c0.y,
        c0.z,
        c1.x,
        c1.y,
        c1.z,
        c2.x,
        c2.y,
        c2.z,
        translation.x,
        translation.y,
        translation.z,
    ];
    let mut out = [0u32; 12];
    for (o, v) in out.iter_mut().zip(w) {
        *o = f32w(v);
    }
    (out, max_abs_scale(scale))
}

/// `utils::pack_2x16f_uint` (O 0x140c90df0): `f16(a) | f16(b) << 16`.
pub use ::utils::pack::pack_2x16f_uint;

/// 12-word packed affine shared by every `nani_update_*` (O; same code in
/// plant 0x142074740 and all of group A):
/// `Mat3::from_quat(rot)` columns scaled per-column by `scale.{x,y,z}` (words 0..9, column
/// major), then `translation` (words 9..12).  Second result = `sort_key` passed in xmm3 =
/// `max(|sx|, |sy|, |sz|)` (maxss chain with NaN-skipping select, equals `f32::max`).
pub fn trs_words(translation: Vec3, rotation: Quat, scale: Vec3) -> ([u32; 12], f32) {
    let m = Mat3::from_quat(rotation);
    let c0 = m.x_axis * scale.x;
    let c1 = m.y_axis * scale.y;
    let c2 = m.z_axis * scale.z;
    let w = [
        c0.x,
        c0.y,
        c0.z,
        c1.x,
        c1.y,
        c1.z,
        c2.x,
        c2.y,
        c2.z,
        translation.x,
        translation.y,
        translation.z,
    ];
    let mut out = [0u32; 12];
    for (o, v) in out.iter_mut().zip(w) {
        *o = f32w(v);
    }
    let sort_key = scale.z.abs().max(scale.y.abs()).max(scale.x.abs());
    (out, sort_key)
}

fn with_tail<const N: usize>(head: [u32; 12], tail: &[u32]) -> [u32; N] {
    let mut w = [0u32; N];
    w[..12].copy_from_slice(&head);
    w[12..12 + tail.len()].copy_from_slice(tail);
    w
}

// ---------------------------------------------------------------------------
// nani_update_solid_vertex_color  0x142070ac0  (kind 3, data 0x40)
// ---------------------------------------------------------------------------
/// O: words 12 = `Option<&SemiTransparent>.is_some()` (ptr!=0 & presence flag),
/// 13 = `Option<&FlatLighting>.is_some()`, 14/15 = 0.
pub fn build_solid_vertex_color(
    translation: Vec3,
    rotation: Quat,
    scale: Vec3,
    semi_transparent: bool,
    flat_lighting: bool,
) -> InstanceUpdate<16> {
    let (head, sort_key) = trs_words(translation, rotation, scale);
    InstanceUpdate {
        position: translation,
        sort_key,
        words: with_tail(head, &[semi_transparent as u32, flat_lighting as u32, 0, 0]),
    }
}

// ---------------------------------------------------------------------------
// nani_update_solid_vertex_color_window  0x142071590  (kind 4, data 0x40)
// ---------------------------------------------------------------------------
/// O: word 12 = `Option<&SemiTransparent>.is_some()`, word 13 = `ColorId.0` (raw, no range
/// check in this system), 14/15 = 0.
pub fn build_solid_vertex_color_window(
    translation: Vec3,
    rotation: Quat,
    scale: Vec3,
    color_id: u32,
    semi_transparent: bool,
) -> InstanceUpdate<16> {
    let (head, sort_key) = trs_words(translation, rotation, scale);
    InstanceUpdate {
        position: translation,
        sort_key,
        words: with_tail(head, &[semi_transparent as u32, color_id, 0, 0]),
    }
}

// ---------------------------------------------------------------------------
// nani_update_solid_albedo_map 0x142072010 (kind 0xf) / _emissive 0x142072b50 (kind 0x10)
// ---------------------------------------------------------------------------
/// O: `NaniSolidTexture` is a 1-byte enum (`movzx eax, byte [row]`); it indexes a u32 table
/// at 0x142fd38c8.  The table has 9 entries (I: = number of enum variants; the bytes after
/// entry 8 belong to unrelated data).  Out-of-range index is UB in the binary; here it panics.
pub const NANI_SOLID_TEXTURE_TABLE: [u32; 9] = [0, 1, 8, 2, 3, 4, 5, 6, 7];

/// O: words 12 = `TABLE[texture as u8]`, 13..15 = 0 (identical for emissive variant).
pub fn build_solid_albedo_map(
    translation: Vec3,
    rotation: Quat,
    scale: Vec3,
    texture: u8,
) -> InstanceUpdate<16> {
    let (head, sort_key) = trs_words(translation, rotation, scale);
    InstanceUpdate {
        position: translation,
        sort_key,
        words: with_tail(head, &[NANI_SOLID_TEXTURE_TABLE[texture as usize], 0, 0, 0]),
    }
}

// ---------------------------------------------------------------------------
// nani_update_solid_vertex_color_window_glass 0x142075070 (kind 5, data 0x40)
// ---------------------------------------------------------------------------
/// Panic text of `WindowGlassFlags::with_color_id_checked(..).unwrap()` (O: string at
/// 0x142fd3700, len 0x37; the `: OutOfBounds` suffix is the modular_bitfield error Debug, I).
pub const WINDOW_GLASS_COLOR_ID_OOB: &str =
    "value out of bounds for field WindowGlassFlags.color_id";

/// O: word 12 = `color_id | (semi_transparent << 31)` (color_id must be < 2^31 else
/// `Result::unwrap_failed(WINDOW_GLASS_COLOR_ID_OOB)`), word 13 = `window_light_value` f32
/// bits, word 14 = `albedo_map.is_some()`, word 15 = bindless index of the albedo texture (0 if none).
pub fn build_window_glass(
    translation: Vec3,
    rotation: Quat,
    scale: Vec3,
    color_id: u32,
    semi_transparent: bool,
    window_light_value: f32,
    albedo_bindless: Option<u32>,
) -> InstanceUpdate<16> {
    if color_id & 0x8000_0000 != 0 {
        panic!("{WINDOW_GLASS_COLOR_ID_OOB}: OutOfBounds");
    }
    let (head, sort_key) = trs_words(translation, rotation, scale);
    InstanceUpdate {
        position: translation,
        sort_key,
        words: with_tail(
            head,
            &[
                color_id | ((semi_transparent as u32) << 31),
                f32w(window_light_value),
                albedo_bindless.is_some() as u32,
                albedo_bindless.unwrap_or(0),
            ],
        ),
    }
}

// ---------------------------------------------------------------------------
// nani_update_solid_vertex_color_bricks 0x142076230 (kind 6, data 0x40)
// ---------------------------------------------------------------------------
/// `BrickColor` raw words (O: row stride 0x18): w0 u32, w1 u32, w2 f32, w3 f32, w4 f32,
/// w5 = dword whose bit 0 is a bool (`shl 31`).  Field names U.
/// Output (O): w12 = `pack_2x16f_uint(f32(w2), f32(w3))`, w13 = w0, w14 = `(w5 << 31) | w1`,
/// w15 = w4 (raw bits).
pub fn build_solid_vertex_color_bricks(
    translation: Vec3,
    rotation: Quat,
    scale: Vec3,
    brick: [u32; 6],
) -> InstanceUpdate<16> {
    let (head, sort_key) = trs_words(translation, rotation, scale);
    InstanceUpdate {
        position: translation,
        sort_key,
        words: with_tail(
            head,
            &[
                pack_2x16f_uint(f32::from_bits(brick[2]), f32::from_bits(brick[3])),
                brick[0],
                (brick[5] << 31) | brick[1],
                brick[4],
            ],
        ),
    }
}

#[cfg(test)]
mod group_a_tests {
    use super::*;

    const ID: (Vec3, Quat, Vec3) = (Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
    const IDENT_WORDS: [f32; 12] = [1., 0., 0., 0., 1., 0., 0., 0., 1., 0., 0., 0.];

    fn f(u: &[u32]) -> Vec<f32> {
        u.iter().map(|w| f32::from_bits(*w)).collect()
    }

    #[test]
    fn trs_identity_and_scale_translation() {
        let (w, k) = trs_words(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
        assert_eq!(f(&w), IDENT_WORDS);
        assert_eq!(k, 1.0);
        let (w, k) = trs_words(
            Vec3::new(1., 2., 3.),
            Quat::IDENTITY,
            Vec3::new(2., -5., 3.),
        );
        assert_eq!(f(&w), [2., 0., 0., 0., -5., 0., 0., 0., 3., 1., 2., 3.]);
        assert_eq!(k, 5.0); // max(|sx|,|sy|,|sz|)
    }

    #[test]
    fn trs_rotation_90deg_z_scales_columns() {
        let q = Quat::from_rotation_z(core::f32::consts::FRAC_PI_2);
        let (w, _) = trs_words(Vec3::ZERO, q, Vec3::new(2., 3., 4.));
        let v = f(&w);
        assert!((v[0]).abs() < 1e-6 && (v[1] - 2.0).abs() < 1e-6); // col0 = (0,1,0)*2
        assert!((v[3] + 3.0).abs() < 1e-6 && v[4].abs() < 1e-6); // col1 = (-1,0,0)*3
        assert!((v[8] - 4.0).abs() < 1e-6);
    }

    #[test]
    fn solid_vertex_color_flags() {
        let u = build_solid_vertex_color(ID.0, ID.1, ID.2, true, false);
        assert_eq!(&u.words[12..], &[1, 0, 0, 0]);
        let u = build_solid_vertex_color(ID.0, ID.1, ID.2, false, true);
        assert_eq!(&u.words[12..], &[0, 1, 0, 0]);
        assert_eq!(u.position, Vec3::ZERO);
    }

    #[test]
    fn window_color_id_word13() {
        let u = build_solid_vertex_color_window(Vec3::X, ID.1, ID.2, 0xdead_beef, true);
        assert_eq!(&u.words[12..], &[1, 0xdead_beef, 0, 0]);
        assert_eq!(u.position, Vec3::X);
    }

    #[test]
    fn albedo_table_matches_binary() {
        assert_eq!(NANI_SOLID_TEXTURE_TABLE, [0, 1, 8, 2, 3, 4, 5, 6, 7]);
        let u = build_solid_albedo_map(ID.0, ID.1, ID.2, 2);
        assert_eq!(&u.words[12..], &[8, 0, 0, 0]);
        let u = build_solid_albedo_map(ID.0, ID.1, ID.2, 8);
        assert_eq!(u.words[12], 7);
    }

    #[test]
    #[should_panic]
    fn albedo_texture_out_of_range_panics() {
        let _ = build_solid_albedo_map(ID.0, ID.1, ID.2, 9);
    }

    #[test]
    fn glass_packs_color_semi_bit_and_albedo() {
        let u = build_window_glass(ID.0, ID.1, ID.2, 0x1234, true, 0.5, Some(77));
        assert_eq!(u.words[12], 0x1234 | 0x8000_0000);
        assert_eq!(f32::from_bits(u.words[13]), 0.5);
        assert_eq!((u.words[14], u.words[15]), (1, 77));
        let u = build_window_glass(ID.0, ID.1, ID.2, 7, false, 0.0, None);
        assert_eq!(&u.words[12..], &[7, 0, 0, 0]);
    }

    #[test]
    #[should_panic(expected = "value out of bounds for field WindowGlassFlags.color_id")]
    fn glass_color_id_bit31_panics() {
        let _ = build_window_glass(ID.0, ID.1, ID.2, 0x8000_0000, false, 0.0, None);
    }

    #[test]
    fn bricks_packing() {
        let brick = [
            0x11u32,
            0x22,
            1.0f32.to_bits(),
            2.0f32.to_bits(),
            0.25f32.to_bits(),
            1,
        ];
        let u = build_solid_vertex_color_bricks(ID.0, ID.1, ID.2, brick);
        assert_eq!(u.words[12], 0x4000_3c00); // f16(1.0)=0x3c00 low, f16(2.0)=0x4000 high
        assert_eq!(u.words[13], 0x11);
        assert_eq!(u.words[14], 0x8000_0022);
        assert_eq!(u.words[15], 0.25f32.to_bits());
        let brick0 = [0, 5, 0, 0, 0, 2]; // only bit0 of the dword at +0x14 matters? (shl 31 keeps bit0)
        assert_eq!(
            build_solid_vertex_color_bricks(ID.0, ID.1, ID.2, brick0).words[14],
            5
        );
    }

    #[test]
    fn pack_2x16f_order() {
        assert_eq!(pack_2x16f_uint(1.0, 0.0), 0x0000_3c00);
        assert_eq!(pack_2x16f_uint(0.0, 1.0), 0x3c00_0000);
    }
}

/// 48-byte packed affine used by Plant/Flower/Wren/Gothic payloads (O):
/// `[c0.x c0.y c0.z c1.x c1.y c1.z c2.x c2.y c2.z t.x t.y t.z]`, `c_i = R.col_i * scale_i`.
/// Quat->Mat3 follows the glam `from_quat` formula (verified against the SSE sequence).
pub fn gb_affine_words(rotation: Quat, translation: Vec3, scale: Vec3) -> [u32; 12] {
    let m = Mat3::from_quat(rotation);
    let c0 = m.x_axis * scale.x;
    let c1 = m.y_axis * scale.y;
    let c2 = m.z_axis * scale.z;
    [
        f32w(c0.x),
        f32w(c0.y),
        f32w(c0.z),
        f32w(c1.x),
        f32w(c1.y),
        f32w(c1.z),
        f32w(c2.x),
        f32w(c2.y),
        f32w(c2.z),
        f32w(translation.x),
        f32w(translation.y),
        f32w(translation.z),
    ]
}

/// `max(|sx|, max(|sy|, |sz|))` with maxss+cmpunord NaN blend == `f32::max` semantics (O).
/// This is the `sort_key` (xmm3) passed to update_instance_data by Flower/Wren/Gothic.
pub fn gb_max_abs_scale(scale: Vec3) -> f32 {
    scale.x.abs().max(scale.y.abs().max(scale.z.abs()))
}

/// `utils::pack_2x16f_uint` (O, 0x140c90df0): `f16(a) | f16(b) << 16` (half::f16::from_f32).
pub use ::utils::pack::pack_2x16f_uint as gb_pack_2x16f;

/// Reinterpret 10 raw words of a binary `Transform`-shaped row (stride 0x30: quat, translation
/// @+0x10, scale @+0x1c) as `(rotation, translation, scale)`.  Used for `PrevTransform` whose
/// Rust layout is a typed gap in components/drawable.rs (O: read with the same offsets as Transform).
pub fn gb_transform_from_binary_words(w: [u32; 10]) -> (Quat, Vec3, Vec3) {
    let f = |i: usize| f32::from_bits(w[i]);
    (
        Quat::from_xyzw(f(0), f(1), f(2), f(3)),
        Vec3::new(f(4), f(5), f(6)),
        Vec3::new(f(7), f(8), f(9)),
    )
}

/// Wren payload (O, 112 B):
/// `[0..12)` affine(current Transform), `[12..24)` affine(PrevTransform), `[24..26)` Wren+0x44 (8 B),
/// `[26]` Wren+0x4c, `[27]` = 0.  position = current translation, sort_key = max|scale| of current.
/// `wren_44` = words at Wren+0x44/+0x48, `wren_4c` = word at Wren+0x4c (field names U).
pub fn build_wren(
    cur: (Quat, Vec3, Vec3),
    prev: (Quat, Vec3, Vec3),
    wren_44: [u32; 2],
    wren_4c: u32,
) -> InstanceUpdate<28> {
    let mut words = [0u32; 28];
    words[0..12].copy_from_slice(&gb_affine_words(cur.0, cur.1, cur.2));
    words[12..24].copy_from_slice(&gb_affine_words(prev.0, prev.1, prev.2));
    words[24] = wren_44[0];
    words[25] = wren_44[1];
    words[26] = wren_4c;
    words[27] = 0;
    InstanceUpdate {
        position: cur.1,
        sort_key: gb_max_abs_scale(cur.2),
        words,
    }
}

/// Gothic payload (O, 112 B):
/// `[0..12)` affine(Transform);
/// `[12..14)` BrickColor+0x00 (8 B) , `[14]` pack_2x16f(BrickColor+0x08, +0x0c), `[15]` (BrickColor+0x14) as u8->u32;
/// `[16..19)` Deformation+0x18 (12 B), `[19]` pack_2x16f(Def+0x00,+0x04);
/// `[20..23)` Deformation+0x24 (12 B), `[23]` pack_2x16f(Def+0x08,+0x0c);
/// `[24..27)` Deformation+0x30 (12 B), `[27]` pack_2x16f(Def+0x10,+0x14).
/// `brick` = BrickColor row (0x18 B = 6 words), `def` = GothicWindowBrickDeformation row (0x3c B = 15 words).
pub fn build_gothic_window_bricks(
    rotation: Quat,
    translation: Vec3,
    scale: Vec3,
    brick: [u32; 6],
    def: [u32; 15],
) -> InstanceUpdate<28> {
    let f = |i: usize| f32::from_bits(def[i]);
    let mut w = [0u32; 28];
    w[0..12].copy_from_slice(&gb_affine_words(rotation, translation, scale));
    w[12] = brick[0];
    w[13] = brick[1];
    w[14] = gb_pack_2x16f(f32::from_bits(brick[2]), f32::from_bits(brick[3]));
    w[15] = brick[5] & 0xff; // movzx byte [BrickColor+0x14]
    w[16] = def[6];
    w[17] = def[7];
    w[18] = def[8];
    w[19] = gb_pack_2x16f(f(0), f(1));
    w[20] = def[9];
    w[21] = def[10];
    w[22] = def[11];
    w[23] = gb_pack_2x16f(f(2), f(3));
    w[24] = def[12];
    w[25] = def[13];
    w[26] = def[14];
    w[27] = gb_pack_2x16f(f(4), f(5));
    InstanceUpdate {
        position: translation,
        sort_key: gb_max_abs_scale(scale),
        words: w,
    }
}

/// Name asserted (`assert_eq!`, memcmp 0x3f bytes) on the BufferState fetched from AssetSsboLibrary (O).
pub const TERRAIN_ROCK_MASK_SSBO_NAME: &str =
    "country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo";

/// World extent used for the mask UV and passed as 3rd float (`0x43020000`) to
/// `bilinear_sample_at_world_xz` (O).
pub const FLOWER_WORLD_SIZE: f32 = 130.0;

/// Raster side (cells) of the rock mask (O: 512.0, clamp 0x1ff).
pub const FLOWER_MASK_SIDE: f32 = 512.0;

/// Hide when water sample is strictly greater than this (O: 0x3c23d70a).
pub const FLOWER_WATER_MAX: f32 = 0.01;

/// Hide when clamp01(sum of 4 path rasters) strictly greater than this (O: 0x3dcccccd).
pub const FLOWER_PATH_MAX: f32 = 0.1;

/// Hidden flowers are placed at this translation inside the payload (O: f64 bits
/// 0xc2c8000080000000 = (-0.0, -100.0) + 0x80000000 = -0.0), identity rotation, unit scale.
pub const FLOWER_HIDDEN_TRANSLATION: Vec3 = Vec3::new(-0.0, -100.0, -0.0);

/// The five rasters sampled in order (O): water first, then the four path rasters
/// (PathRastersResources fields at +0x00/+0x20/+0x40/+0x60).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowerRaster {
    Water,
    Path0,
    Path1,
    Path2,
    Path3,
}

/// `(word_index, bit)` of the rock-mask cell for world (x, z) (O):
/// `u = clamp01(x/130 + 0.5) * 512` (saturating cvttss2si, min 0x1ff), same for v with z;
/// `word = ((v << 9) | u) >> 5`, `bit = u & 31` (`bt ecx, eax` uses eax = u, mod 32).
pub fn flower_mask_cell(x: f32, z: f32) -> (usize, u32) {
    let cell = |c: f32| -> u32 {
        let t = (c / FLOWER_WORLD_SIZE + 0.5).max(0.0).min(1.0) * FLOWER_MASK_SIDE;
        (t as u32).min(0x1ff) // `as` saturates like cmov/cvttss2si sequence; NaN -> 0
    };
    let (u, v) = (cell(x), cell(z));
    ((((v << 9) | u) >> 5) as usize, u & 31)
}

/// Visibility verdict (O). Order and short-circuiting follow the machine code:
/// 1. rock-mask bit set -> hide (indexing the mask is bounds-checked: panic_bounds_check);
/// 2. water sample `> 0.01` -> hide;
/// 3. `s = clamp(0,1)(((p0 + p1) + p2) + p3)`, `s > 0.1` -> hide; otherwise show.
pub fn flower_visible(
    x: f32,
    z: f32,
    rock_mask: &[u32],
    mut sample: impl FnMut(FlowerRaster, f32, f32) -> f32,
) -> bool {
    let (word, bit) = flower_mask_cell(x, z);
    if (rock_mask[word] >> bit) & 1 != 0 {
        return false;
    }
    if sample(FlowerRaster::Water, x, z) > FLOWER_WATER_MAX {
        return false;
    }
    let p0 = sample(FlowerRaster::Path0, x, z);
    let p1 = sample(FlowerRaster::Path1, x, z);
    let p2 = sample(FlowerRaster::Path2, x, z);
    let p3 = sample(FlowerRaster::Path3, x, z);
    let s = (((p0 + p1) + p2) + p3).max(0.0).min(1.0);
    !(s > FLOWER_PATH_MAX)
}

/// Payload builder (O). Visible: affine(Transform), sort_key = max|scale|.
/// Hidden: affine(identity rot, scale 1, translation (-0,-100,-0)), sort_key = 0.0.
/// `position` passed to update_instance_data is always the real translation.
pub fn build_preplaced_flower(
    rotation: Quat,
    translation: Vec3,
    scale: Vec3,
    visible: bool,
) -> InstanceUpdate<12> {
    if visible {
        InstanceUpdate {
            position: translation,
            sort_key: gb_max_abs_scale(scale),
            words: gb_affine_words(rotation, translation, scale),
        }
    } else {
        InstanceUpdate {
            position: translation,
            sort_key: 0.0,
            words: gb_affine_words(Quat::IDENTITY, FLOWER_HIDDEN_TRANSLATION, Vec3::ONE),
        }
    }
}

/// Asserts the BufferState name (O: `assert_eq!(name, "...TerrainRockMaskSsbo")`).
pub fn check_rock_mask_name(name: &str) {
    assert_eq!(name, TERRAIN_ROCK_MASK_SSBO_NAME);
}

/// `CurvedHalfTimber` row (16 B, O): `key: u64 @+0`, `direct: u8 @+8`.
/// Colour selection (O):
///  - `direct != 0`           -> `ids[key]` (bounds checked);
///  - `direct == 0`           -> if PublicWalls (non-empty) maps `key` to a live roof entity whose
///    `Roof` has discriminant byte `@+0x40 == 2`, colour = `roof.u32 @+0x44`; otherwise `ids[key]`.
/// `roof_curved_colour` is that lookup already resolved by the caller.
/// Panics like the original (`panic_bounds_check`) when `ids` has no entry for `key`.
pub fn curved_halftimber_colour(
    key: u64,
    direct: bool,
    roof_curved_colour: Option<u32>,
    ids: impl Fn(u64) -> Option<u32>,
) -> u32 {
    if !direct {
        if let Some(c) = roof_curved_colour {
            return c;
        }
    }
    ids(key).unwrap_or_else(|| panic!("index out of bounds: HalftimberColorIds[{key}]"))
}

/// Roof discriminant check (O): `byte @+0x40 == 2` -> colour `u32 @+0x44` (words 16 / 17).
pub fn roof_curved_colour_from_words(roof: &[u32; 22]) -> Option<u32> {
    if (roof[16] & 0xff) == 2 {
        Some(roof[17])
    } else {
        None
    }
}

/// In-place write + upload payload (O): `data[3] = colour` (`mov [rbx+0xc], eax`), change tick set,
/// `position = data[0..3]` (as f32), `sort_key = 1.0` (xmm6), payload = whole 64 B.
pub fn build_curved_halftimber(mut data_words: [u32; 16], colour: u32) -> InstanceUpdate<16> {
    data_words[3] = colour;
    let pos = Vec3::new(
        f32::from_bits(data_words[0]),
        f32::from_bits(data_words[1]),
        f32::from_bits(data_words[2]),
    );
    InstanceUpdate {
        position: pos,
        sort_key: 1.0,
        words: data_words,
    }
}

/// Explicit field -> words view of `CurvedHalfTimberInstanceData` (offsets from instance_data.rs doc;
/// independent of Rust struct padding).
pub fn curved_halftimber_words(d: &CurvedHalfTimberInstanceData) -> [u32; 16] {
    [
        f32w(d.position.x),
        f32w(d.position.y),
        f32w(d.position.z),
        d.pad_0c,
        f32w(d.x0),
        f32w(d.x1),
        f32w(d.y0),
        f32w(d.y1),
        d.w20,
        d.w24,
        f32w(d.w28),
        d.w2c,
        d.w30,
        d.w34,
        d.w38,
        d.w3c,
    ]
}

/// Roof row words (0x58 B = 22 words); Floor row words (0x28 B = 10 words).
/// Field names are U; offsets are O (see build_stone_floor doc).
pub type RoofWords = [u32; 22];

pub type FloorWords = [u32; 10];

const SF_TALL_THRESHOLD: f32 = 0.1;

// 0x3dcccccd
const SF_RADIUS_SCALE: f32 = 0.5;

// 0x3f000000
const SF_RADIUS_BIAS: f32 = 0.63;

// 0x3f2147ae
const SF_LOW_Y_BIAS: f32 = -0.69;

/// `Circle2d::get_tangent_at_u(&circle, 0.0)` result (x, y); `circle` = row words starting at +0xc.
/// The adapter passes the canonical shared Circle2d projection below; callers
/// of the existing numerical API may still supply their own proven tangent.
pub type TangentAtU0<'a> = &'a dyn Fn(&[u32]) -> Vec2;

/// Stone floor payload (O).  Inputs: `translation` (Transform), optional Roof / RoofAnimation / Floor rows.
/// Neither Roof nor Floor -> `Option::unwrap_failed` panic (O @0x142079435).
///
/// Roof present (r14), `h = f32(roof[+0x30])`, `e = f32(roof[+0x2c])`, `tag = roof[+8]`:
///  * `x14 = (h <= 0.1) ? 0 : e*0.5 + 0.63`
///  * tag&1 (straight): A = f32x2(roof+0xc); T = (h<=0.1 || f32(+0x34)!=1.0) ? (1,1)
///    : (roof[+0x3c]&1 ? (1,0) : (0,1)); B = x14*T + f32x2(roof+0x1c); straight=1.
///  * else (curved): A = tangent(circle@+0xc, u=0); B = splat(x14 + 2*f32(roof+0x14)); straight=0.
///  * is_floor=0; position = (f32[roof+0xc+8*tag], f32(roof+0x4c) (+(-0.69) if h<=0.1), f32[roof+0x10+8*tag]);
///    tall(h>0.1): B = 0, flag=1 else flag=0.
///  * discriminant `d = byte roof[+0x40]`: d==2 -> w12 = byte[+0x41]|2, w13 = u32[+0x44], w14 = u32[+0x48];
///    d==1 -> (0,0,0); d==0 -> (byte[+0x54],0,0); d>2 -> (byte[+0x54], 0, u32[+0x48]).
/// Roof absent, Floor present: x14 = 0, is_floor=1, same straight/curved formulas using floor rows
/// (A=f32x2(floor+0xc), straight: B = f32x2(floor+0x1c); curved: tangent, B = splat(2*f32(floor+0x14))),
/// position = Transform.translation, flag = 0, w12..14 = 0.
/// Output words: `[x y z, src_u64_lo, A.x A.y B.x B.y, anim, straight, is_floor, flag, w12, w13, w14, 0]`.
/// sort_key = max(|B.x|, |B.y|).
pub fn build_stone_floor(
    translation: Vec3,
    roof: Option<&RoofWords>,
    anim: Option<f32>,
    floor: Option<&FloorWords>,
    tangent: TangentAtU0<'_>,
) -> InstanceUpdate<16> {
    let f = |w: &[u32], i: usize| f32::from_bits(w[i]);
    let anim = anim.unwrap_or(1.0);

    // (src_lo, A, B, straight, is_floor)
    let (src_lo, a, mut b, straight, is_floor, tall_info): (
        u32,
        Vec2,
        Vec2,
        u32,
        u32,
        Option<&RoofWords>,
    );
    if let Some(r) = roof {
        let h = f(r, 12);
        let e = f(r, 11);
        let x14 = if h <= SF_TALL_THRESHOLD {
            0.0
        } else {
            e * SF_RADIUS_SCALE + SF_RADIUS_BIAS
        };
        src_lo = r[0];
        if r[2] & 1 != 0 {
            a = Vec2::new(f(r, 3), f(r, 4));
            let t = if h <= SF_TALL_THRESHOLD || f(r, 13) != 1.0 {
                Vec2::new(1.0, 1.0)
            } else if r[15] & 1 != 0 {
                Vec2::new(1.0, 0.0)
            } else {
                Vec2::new(0.0, 1.0)
            };
            b = Vec2::new(x14 * t.x + f(r, 7), x14 * t.y + f(r, 8));
            straight = 1;
        } else {
            a = tangent(&r[3..]);
            let v = x14 + f(r, 5) * 2.0;
            b = Vec2::new(v, v);
            straight = 0;
        }
        is_floor = 0;
        tall_info = Some(r);
    } else {
        let fl = floor.expect("called `Option::unwrap()` on a `None` value");
        src_lo = fl[0];
        if fl[2] & 1 != 0 {
            a = Vec2::new(f(fl, 3), f(fl, 4));
            b = Vec2::new(0.0 * 1.0 + f(fl, 7), 0.0 * 1.0 + f(fl, 8));
            straight = 1;
        } else {
            a = tangent(&fl[3..]);
            let v = 0.0 + f(fl, 5) * 2.0;
            b = Vec2::new(v, v);
            straight = 0;
        }
        is_floor = 1;
        tall_info = None;
    }

    // position / tall flag / discriminant words
    let (pos, flag, w12, w13, w14);
    match tall_info {
        Some(r) => {
            let h = f(r, 12);
            let tag = r[2] as usize;
            let mut y = f(r, 19);
            if h > SF_TALL_THRESHOLD {
                b = Vec2::ZERO;
                flag = 1u32;
            } else {
                y += SF_LOW_Y_BIAS;
                flag = 0u32;
            }
            // x/z come from the (tag-selected) payload slot; indexing is bounds-checked here
            // (asm reads unchecked memory; tag is 0/1 in practice).
            pos = Vec3::new(f(r, 3 + 2 * tag), y, f(r, 4 + 2 * tag));
            let d = r[16] & 0xff;
            let (a12, a13, a14) = match d {
                2 => (((r[16] >> 8) & 0xff) | 2, r[17], r[18]),
                1 => (0, 0, 0),
                0 => (r[21] & 0xff, 0, 0),
                _ => (r[21] & 0xff, 0, r[18]),
            };
            w12 = a12;
            w13 = a13;
            w14 = a14;
        }
        None => {
            pos = translation;
            flag = 0;
            w12 = 0;
            w13 = 0;
            w14 = 0;
        }
    }

    let words = [
        f32w(pos.x),
        f32w(pos.y),
        f32w(pos.z),
        src_lo,
        f32w(a.x),
        f32w(a.y),
        f32w(b.x),
        f32w(b.y),
        f32w(anim),
        straight,
        is_floor,
        flag,
        w12,
        w13,
        w14,
        0,
    ];
    InstanceUpdate {
        position: pos,
        sort_key: b.x.abs().max(b.y.abs()),
        words,
    }
}

/// B140c97660 on the actual circle at component+0xc. The four words are
/// center.x, center.y, radius, rotation. Shared Circle2d already implements the
/// original remainder/angle order and signed tangent; no zero-vector fallback.
pub fn circle_tangent_u0(circle_words: &[u32]) -> Vec2 {
    ::utils::geometry::circle::Circle2d::new_rotated(
        Vec2::new(f32::from_bits(circle_words[0]), f32::from_bits(circle_words[1])),
        f32::from_bits(circle_words[2]),
        f32::from_bits(circle_words[3]),
    ).get_tangent_at_u(0.0)
}

// ===========================================================================
// tests (pure functions only)
// ===========================================================================
#[cfg(test)]
mod gb_tests {
    use super::*;

    fn f(w: u32) -> f32 {
        f32::from_bits(w)
    }

    #[test]
    fn affine_identity_and_scale() {
        let w = gb_affine_words(
            Quat::IDENTITY,
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(2.0, 3.0, 4.0),
        );
        let v: Vec<f32> = w.iter().map(|x| f(*x)).collect();
        assert_eq!(
            v,
            vec![2.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 4.0, 1.0, 2.0, 3.0]
        );
    }

    #[test]
    fn max_abs_scale_nan_and_neg() {
        assert_eq!(gb_max_abs_scale(Vec3::new(-3.0, 2.0, 1.0)), 3.0);
        assert_eq!(gb_max_abs_scale(Vec3::new(f32::NAN, 2.0, -5.0)), 5.0);
    }

    #[test]
    fn pack_2x16f_matches_half() {
        assert_eq!(gb_pack_2x16f(1.0, 2.0), 0x4000_3c00);
    }

    #[test]
    fn wren_layout() {
        let cur = (Quat::IDENTITY, Vec3::new(1.0, 2.0, 3.0), Vec3::ONE * -2.0);
        let prev = (Quat::IDENTITY, Vec3::new(9.0, 8.0, 7.0), Vec3::ONE);
        let u = build_wren(cur, prev, [0xaa, 0xbb], 0xcc);
        assert_eq!(u.position, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(u.sort_key, 2.0);
        assert_eq!(f(u.words[0]), -2.0);
        assert_eq!(f(u.words[9]), 1.0);
        assert_eq!(f(u.words[12]), 1.0); // prev c0.x
        assert_eq!(f(u.words[21]), 9.0); // prev translation
        assert_eq!(&u.words[24..], &[0xaa, 0xbb, 0xcc, 0]);
    }

    #[test]
    fn gothic_layout() {
        let brick = [1, 2, f32w(1.0), f32w(2.0), 0, 0x1234_5607];
        let mut def = [0u32; 15];
        for (i, d) in def.iter_mut().enumerate() {
            *d = 100 + i as u32;
        }
        def[0] = f32w(1.0);
        def[1] = f32w(2.0);
        def[2] = f32w(0.5);
        def[3] = f32w(-1.0);
        def[4] = f32w(4.0);
        def[5] = f32w(8.0);
        let u =
            build_gothic_window_bricks(Quat::IDENTITY, Vec3::ZERO, Vec3::splat(-3.0), brick, def);
        assert_eq!(u.sort_key, 3.0);
        assert_eq!(&u.words[12..14], &[1, 2]);
        assert_eq!(u.words[14], 0x4000_3c00);
        assert_eq!(u.words[15], 7); // byte only
        assert_eq!(&u.words[16..19], &[106, 107, 108]);
        assert_eq!(u.words[19], gb_pack_2x16f(1.0, 2.0));
        assert_eq!(&u.words[20..23], &[109, 110, 111]);
        assert_eq!(u.words[23], gb_pack_2x16f(0.5, -1.0));
        assert_eq!(&u.words[24..27], &[112, 113, 114]);
        assert_eq!(u.words[27], gb_pack_2x16f(4.0, 8.0));
    }

    #[test]
    fn flower_cell_math() {
        assert_eq!(flower_mask_cell(0.0, 0.0), ((256 << 9 | 256) >> 5, 0)); // centre
        assert_eq!(flower_mask_cell(-1000.0, -1000.0), (0, 0)); // clamp low
        let (w, b) = flower_mask_cell(1000.0, 1000.0); // clamp high -> 0x1ff
        assert_eq!(w, (0x1ff << 9 | 0x1ff) >> 5);
        assert_eq!(b, 31);
        assert_eq!(flower_mask_cell(f32::NAN, 0.0).1, 0);
    }

    #[test]
    fn flower_visibility_branches() {
        let mut mask = vec![0u32; 512 * 512 / 32];
        let sampled = std::cell::Cell::new(0);
        let zero = |_r: FlowerRaster, _x: f32, _z: f32| {
            sampled.set(sampled.get() + 1);
            0.0
        };
        assert!(flower_visible(0.0, 0.0, &mask, zero));
        assert_eq!(sampled.get(), 5);
        // water > 0.01 hides after exactly one sample
        sampled.set(0);
        let water = |r: FlowerRaster, _x: f32, _z: f32| {
            sampled.set(sampled.get() + 1);
            if r == FlowerRaster::Water { 0.02 } else { 0.0 }
        };
        assert!(!flower_visible(0.0, 0.0, &mask, water));
        assert_eq!(sampled.get(), 1);
        // water == 0.01 does not hide (strict >)
        assert!(flower_visible(0.0, 0.0, &mask, |r, _, _| {
            if r == FlowerRaster::Water { 0.01 } else { 0.0 }
        }));
        // path sum clamp: 0.04*4 = 0.16 > 0.1 hides; 0.025*4 = 0.1 shows
        assert!(!flower_visible(0.0, 0.0, &mask, |r, _, _| {
            if r == FlowerRaster::Water { 0.0 } else { 0.04 }
        }));
        assert!(flower_visible(0.0, 0.0, &mask, |r, _, _| {
            if r == FlowerRaster::Water { 0.0 } else { 0.025 }
        }));
        // rock mask bit hides without sampling
        let (w, b) = flower_mask_cell(0.0, 0.0);
        mask[w] |= 1 << b;
        let boom = |_r: FlowerRaster, _x: f32, _z: f32| -> f32 { panic!("must not sample") };
        assert!(!flower_visible(0.0, 0.0, &mask, boom));
    }

    #[test]
    #[should_panic]
    fn flower_mask_bounds_checked() {
        let mask = vec![0u32; 4];
        flower_visible(0.0, 0.0, &mask, |_, _, _| 0.0);
    }

    #[test]
    fn flower_payloads() {
        let vis = build_preplaced_flower(
            Quat::IDENTITY,
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::splat(2.0),
            true,
        );
        assert_eq!(vis.sort_key, 2.0);
        assert_eq!(f(vis.words[9]), 1.0);
        let hid = build_preplaced_flower(
            Quat::IDENTITY,
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::splat(2.0),
            false,
        );
        assert_eq!(hid.sort_key, 0.0);
        assert_eq!(hid.position, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(f(hid.words[0]), 1.0);
        assert_eq!(f(hid.words[10]), -100.0);
        assert_eq!(hid.words[9], 0x8000_0000);
        assert_eq!(hid.words[11], 0x8000_0000);
        assert_eq!(TERRAIN_ROCK_MASK_SSBO_NAME.len(), 0x3f);
    }

    #[test]
    #[should_panic]
    fn rock_mask_name_assert() {
        check_rock_mask_name("nope");
    }

    #[test]
    fn halftimber_colour_selection() {
        let ids = |k: u64| if k < 3 { Some(10 + k as u32) } else { None };
        assert_eq!(curved_halftimber_colour(1, true, Some(99), ids), 11);
        assert_eq!(curved_halftimber_colour(1, false, Some(99), ids), 99);
        assert_eq!(curved_halftimber_colour(2, false, None, ids), 12);
        let mut roof = [0u32; 22];
        roof[16] = 2;
        roof[17] = 7;
        assert_eq!(roof_curved_colour_from_words(&roof), Some(7));
        roof[16] = 1;
        assert_eq!(roof_curved_colour_from_words(&roof), None);
    }

    #[test]
    #[should_panic]
    fn halftimber_missing_id_panics() {
        curved_halftimber_colour(5, true, None, |_| None);
    }

    #[test]
    fn halftimber_payload() {
        let mut w = [0u32; 16];
        w[0] = f32w(1.0);
        w[1] = f32w(2.0);
        w[2] = f32w(3.0);
        w[3] = 0xdead;
        w[8] = 77;
        let u = build_curved_halftimber(w, 5);
        assert_eq!(u.words[3], 5);
        assert_eq!(u.words[8], 77);
        assert_eq!(u.position, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(u.sort_key, 1.0);
    }

    fn roof_straight(h: f32) -> RoofWords {
        let mut r = [0u32; 22];
        r[0] = 0x1234;
        r[2] = 1;
        r[3] = f32w(5.0);
        r[4] = f32w(6.0);
        r[5] = f32w(7.0);
        r[6] = f32w(8.0);
        r[7] = f32w(0.5);
        r[8] = f32w(0.25);
        r[11] = f32w(2.0);
        r[12] = f32w(h);
        r[13] = f32w(1.0);
        r[15] = 1;
        r[19] = f32w(4.0);
        r[16] = 2 | (0x10 << 8);
        r[17] = 111;
        r[18] = 222;
        r
    }
    const NOTAN: &dyn Fn(&[u32]) -> Vec2 = &|_| Vec2::new(0.5, -0.5);

    #[test]
    fn circle_tangent_uses_actual_rotation_and_rejects_zero_vector_regression() {
        for rotation in [0.0f32, 0.73, -1.25, 2.0 * std::f32::consts::TAU + 0.47] {
            let words = [2.5f32.to_bits(), (-3.25f32).to_bits(), 4.5f32.to_bits(), rotation.to_bits()];
            let tangent = circle_tangent_u0(&words);
            let angle = rotation.rem_euclid(std::f32::consts::TAU) + std::f32::consts::TAU;
            assert!((tangent.x + angle.sin()).abs() < 1e-6);
            assert!((tangent.y - angle.cos()).abs() < 1e-6);
            assert!((tangent.length_squared() - 1.0).abs() < 1e-6);
        }
    }

    #[test]
    fn stone_floor_rotated_circle_uses_real_tangent_payload() {
        let mut roof = roof_straight(0.05);
        roof[2] = 0;
        roof[5] = 1.5f32.to_bits();
        roof[6] = 0.73f32.to_bits();
        let update = build_stone_floor(Vec3::new(12.0, 13.0, 14.0), Some(&roof),
            Some(0.375), None, &circle_tangent_u0);
        let angle = 0.73 + std::f32::consts::TAU;
        assert!((f(update.words[4]) + angle.sin()).abs() < 1e-6);
        assert!((f(update.words[5]) - angle.cos()).abs() < 1e-6);
        assert_eq!(&update.words[6..9], &[3.0f32.to_bits(), 3.0f32.to_bits(), 0.375f32.to_bits()]);
        assert_eq!(update.words[9], 0);
        assert_eq!(update.sort_key, 3.0);
    }

    #[test]
    fn stone_floor_roof_straight_tall() {
        let r = roof_straight(1.0);
        let u = build_stone_floor(Vec3::ZERO, Some(&r), None, None, NOTAN);
        // tall: B zeroed, flag 1, y unchanged, x/z from slot 5/6? tag=1 -> words 5,6
        assert_eq!(u.position, Vec3::new(7.0, 4.0, 8.0));
        assert_eq!(u.words[3], 0x1234);
        assert_eq!((f(u.words[4]), f(u.words[5])), (5.0, 6.0));
        assert_eq!((f(u.words[6]), f(u.words[7])), (0.0, 0.0));
        assert_eq!(f(u.words[8]), 1.0); // default anim
        assert_eq!(&u.words[9..12], &[1, 0, 1]);
        assert_eq!(&u.words[12..15], &[0x10 | 2, 111, 222]);
        assert_eq!(u.sort_key, 0.0);
    }

    #[test]
    fn stone_floor_roof_straight_low() {
        let r = roof_straight(0.05); // h <= 0.1
        let u = build_stone_floor(Vec3::ZERO, Some(&r), Some(0.5), None, NOTAN);
        assert_eq!(f(u.words[8]), 0.5);
        assert_eq!(u.words[11], 0); // not tall
        assert!((u.position.y - (4.0 - 0.69)).abs() < 1e-6);
        // x14 = 0 (h<=0.1); T=(1,1): B = (0+0.5, 0+0.25)
        assert_eq!((f(u.words[6]), f(u.words[7])), (0.5, 0.25));
        assert_eq!(u.sort_key, 0.5);
    }

    #[test]
    fn stone_floor_roof_straight_axis_select() {
        // h>0.1 but still B zeroed by tall; check T via a roof whose tag path differs: use h<=0.1 to force T=(1,1)
        let mut r = roof_straight(1.0);
        r[13] = f32w(2.0); // != 1.0 -> T=(1,1), x14 = 2*0.5+0.63
        let u = build_stone_floor(Vec3::ZERO, Some(&r), None, None, NOTAN);
        assert_eq!(f(u.words[6]), 0.0); // zeroed by tall anyway
        assert_eq!(u.words[11], 1);
    }

    #[test]
    fn stone_floor_discriminants() {
        let mut r = roof_straight(1.0);
        r[21] = 0x1ab;
        r[16] = 0;
        let u = build_stone_floor(Vec3::ZERO, Some(&r), None, None, NOTAN);
        assert_eq!(&u.words[12..15], &[0xab, 0, 0]);
        r[16] = 1;
        let u = build_stone_floor(Vec3::ZERO, Some(&r), None, None, NOTAN);
        assert_eq!(&u.words[12..15], &[0, 0, 0]);
        r[16] = 3;
        let u = build_stone_floor(Vec3::ZERO, Some(&r), None, None, NOTAN);
        assert_eq!(&u.words[12..15], &[0xab, 0, 222]);
    }

    #[test]
    fn stone_floor_roof_curved() {
        let mut r = roof_straight(1.0);
        r[2] = 0; // curved
        r[5] = f32w(1.5);
        let u = build_stone_floor(Vec3::ZERO, Some(&r), None, None, NOTAN);
        assert_eq!((f(u.words[4]), f(u.words[5])), (0.5, -0.5)); // tangent
        assert_eq!(u.words[9], 0); // straight flag
        assert_eq!(u.words[11], 1); // tall -> B zeroed
        assert_eq!(u.position, Vec3::new(5.0, 4.0, 6.0)); // tag 0 slot
    }

    #[test]
    fn stone_floor_floor_only() {
        let mut fl = [0u32; 10];
        fl[0] = 0x77;
        fl[2] = 1;
        fl[3] = f32w(1.0);
        fl[4] = f32w(2.0);
        fl[7] = f32w(-4.0);
        fl[8] = f32w(3.0);
        let u = build_stone_floor(Vec3::new(9.0, 8.0, 7.0), None, None, Some(&fl), NOTAN);
        assert_eq!(u.position, Vec3::new(9.0, 8.0, 7.0));
        assert_eq!(u.words[3], 0x77);
        assert_eq!((f(u.words[6]), f(u.words[7])), (-4.0, 3.0));
        assert_eq!(&u.words[9..12], &[1, 1, 0]);
        assert_eq!(&u.words[12..16], &[0, 0, 0, 0]);
        assert_eq!(u.sort_key, 4.0);
        // curved floor: B = splat(0 + 2*f32(+0x14))
        fl[2] = 0;
        fl[5] = f32w(1.25);
        let u = build_stone_floor(Vec3::ZERO, None, None, Some(&fl), NOTAN);
        assert_eq!((f(u.words[6]), f(u.words[7])), (2.5, 2.5));
        assert_eq!(u.words[9], 0);
    }

    #[test]
    #[should_panic(expected = "None")]
    fn stone_floor_requires_roof_or_floor() {
        build_stone_floor(Vec3::ZERO, None, None, None, NOTAN);
    }
}

/// Per-row Transform → ( packed 3x3(R*S) column-major + translation = 12 words, translation,
/// sort_key = max(|sx|,|sy|,|sz|) ) (O).
///
/// * words 0..9  = columns of `Mat3::from_quat(rotation) * diag(scale)` (col_i * scale_i),
///   stored contiguous (c0.xyz, c1.xyz, c2.xyz) — O: shuffles at 0x14207d3f6..0x14207d40a;
/// * words 9..12 = translation (O: `movaps xmm0,[row+0x10]` lanes 0..2);
/// * position passed to `update_instance_data` = translation (O: r8 = copy of row+0x10 12 B);
/// * sort_key (xmm3) = NaN-propagating max chain of |scale| (O: andps mask 0x7fffffff,
///   cmpunordss/maxss sequence order y=[+0x20], z=[+0x24], then x=[+0x1c]).
/// Binary Transform layout is `{rotation: Quat @+0, translation: Vec3 @+0x10, scale: Vec3 @+0x1c}`
/// (stride 0x30) — NOTE: crate `Transform` in utils/components/transform.rs declares
/// translation @+0, rotation @+0xC; field *names* are used here so layout is irrelevant.
pub fn trs_packed(rotation: Quat, translation: Vec3, scale: Vec3) -> ([u32; 12], Vec3, f32) {
    let r = Mat3::from_quat(rotation);
    let c0 = r.x_axis * scale.x;
    let c1 = r.y_axis * scale.y;
    let c2 = r.z_axis * scale.z;
    let w = [
        c0.x,
        c0.y,
        c0.z,
        c1.x,
        c1.y,
        c1.z,
        c2.x,
        c2.y,
        c2.z,
        translation.x,
        translation.y,
        translation.z,
    ];
    let mut words = [0u32; 12];
    for (o, v) in words.iter_mut().zip(w) {
        *o = v.to_bits();
    }
    // sort key: y = |s.y|, z = |s.z|; m1 = y.is_nan() ? z : max_f32(z, y); m2 = x.is_nan() ? m1 : max_f32(m1, x)
    let (ax, ay, az) = (scale.x.abs(), scale.y.abs(), scale.z.abs());
    let m1 = if ay.is_nan() { az } else { max_f32(az, ay) };
    let sort_key = if ax.is_nan() { m1 } else { max_f32(m1, ax) };
    (words, translation, sort_key)
}

/// 原36B LanternLightParams中Nani实际读取的字段；世界灯位和装饰ID属于另一消费者。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LanternParamsView {
    /// +0x0c f32 (packed into word12 low half).
    pub p0: f32,
    /// +0x10 f32 (word12 high half).
    pub p1: f32,
    /// +0x14 f32 (word13 low half).
    pub p2: f32,
    /// +0x18 f32 (word13 high half).
    pub p3: f32,
    /// +0x20 u8 (shifted into bit 30 of word14; `shl edi,0x1e`). O: bytes 0x00..0x0b / 0x1c..0x1f unread.
    pub flag: u8,
}

impl From<&crate::components::drawable::LanternLightParams> for LanternParamsView {
    fn from(params: &crate::components::drawable::LanternLightParams) -> Self {
        let field = |offset| f32::from_le_bytes(params.0[offset..offset + 4].try_into().unwrap());
        Self { p0: field(12), p1: field(16), p2: field(20), p3: field(24), flag: params.0[32] }
    }
}

/// O: `data = [mat3(9) , translation(3), pack2f16(p0,p1), pack2f16(p2,p3),
///            color_id | semi<<31 | flag<<30, 0]` (0x40 B `LanternEmissiveInstanceData`).
/// `color_id` None ⇒ 0 (O: `xor r12d,r12d` when Option<&ColorId> is None).
pub fn build_lantern_emissive(
    rotation: Quat,
    translation: Vec3,
    scale: Vec3,
    params: LanternParamsView,
    semi_transparent: bool,
    color_id: Option<u32>,
) -> InstanceUpdate<16> {
    let (t, position, sort_key) = trs_packed(rotation, translation, scale);
    let mut words = [0u32; 16];
    words[..12].copy_from_slice(&t);
    words[12] = pack_2x16f_uint(params.p0, params.p1);
    words[13] = pack_2x16f_uint(params.p2, params.p3);
    words[14] =
        color_id.unwrap_or(0) | ((semi_transparent as u32) << 31) | ((params.flag as u32) << 30);
    words[15] = 0;
    InstanceUpdate {
        position,
        sort_key,
        words,
    }
}

/// Fields of the `PublicWallState` value (hash bucket = `WallId u64` key + 0x2a0 value, stride 0x2a8)
/// read by the system (O). Offsets are relative to the *value* (bucket + 8).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PlasterWallInputs {
    /// value+0x210 u64: `simplified_curve_2d` point count (O: `cmp qword,2; jae`).
    pub simplified_point_count: u64,
    /// value+0x230 f32: `simplified_curve_2d` length (O: `ucomiss >0`).
    pub simplified_length: f32,
    /// value+0x258 u32: shape discriminant (0 Circle, 1 Rectangle, 2 Freehand); `!= 2` → word6.
    pub shape_tag: u32,
    /// value+0x290 f32: fallback peel value when `AnimatedPlasterPeel` has no entry for the wall.
    pub default_peel: f32,
}

impl PlasterWallInputs {
    /// Reads the observed offsets from the 0x2a0-byte `PublicWallState` record (O offsets).
    ///
    /// 仅用于原版完整记录或原指令夹具。当前 PublicWallState 拥有安全字段，生产 ECS
    /// 直接读取曲线、shape 与 authority；其局部 building_projection 不能当完整原ABI记录。
    pub fn from_bytes(b: &[u8]) -> Self {
        let u64_at = |o: usize| u64::from_le_bytes(b[o..o + 8].try_into().unwrap());
        let u32_at = |o: usize| u32::from_le_bytes(b[o..o + 4].try_into().unwrap());
        Self {
            simplified_point_count: u64_at(0x210),
            simplified_length: f32::from_bits(u32_at(0x230)),
            shape_tag: u32_at(0x258),
            default_peel: f32::from_bits(u32_at(0x290)),
        }
    }

    /// `w.simplified_curve_2d.is_valid()` (O: inlined; `count >= 2 && length > 0.0`,
    /// NaN length ⇒ invalid since `ucomiss; jbe`).
    pub fn simplified_curve_is_valid(&self) -> bool {
        self.simplified_point_count >= 2 && self.simplified_length > 0.0
    }
}

/// `Plaster` 记录视图（原 stride 0x28）；生产组件由 resources/walls/plaster.rs 持有。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PlasterView {
    /// +0x00..0x10: 16 B copied verbatim (O: 0x140dc0a90 is the ICF'd 16-byte memcpy; words 0..4).
    pub head: [u32; 4],
    /// +0x10 u64 (data words 8..10).
    pub qword_10: u64,
    /// +0x18 u32 (data word 7).
    pub dword_18: u32,
    /// +0x1c u32 (data word 10).
    pub dword_1c: u32,
    /// +0x20 u64: `WallId` — key into `PublicWalls` / `AnimatedPlasterPeel` (O).
    pub wall_id: u64,
}

impl PlasterView {
    pub fn from_words(w: [u32; 10]) -> Self {
        Self {
            head: [w[0], w[1], w[2], w[3]],
            qword_10: (w[4] as u64) | ((w[5] as u64) << 32),
            dword_18: w[6],
            dword_1c: w[7],
            wall_id: (w[8] as u64) | ((w[9] as u64) << 32),
        }
    }
}

/// O: `data = [head(4), wall_id as u32, simplified_length, (shape_tag != 2) as u32, p.dword_18,
///             p.qword_10 (2), p.dword_1c, peel]`; position = (0,0,0), sort_key = 1.0
/// (O: `[rbp+0x38]=0`, `movaps xmm3,xmm6(=1.0)`).
/// `peel` = `AnimatedPlasterPeel[wall_id]` if present else `wall.default_peel`.
/// Panics (O): "assertion failed: w.simplified_curve_2d.is_valid()".
pub fn build_plaster(
    p: PlasterView,
    wall: PlasterWallInputs,
    animated_peel: Option<f32>,
) -> InstanceUpdate<12> {
    assert!(
        wall.simplified_curve_is_valid(),
        "assertion failed: w.simplified_curve_2d.is_valid()"
    );
    let peel = animated_peel.unwrap_or(wall.default_peel);
    let words = [
        p.head[0],
        p.head[1],
        p.head[2],
        p.head[3],
        p.wall_id as u32,
        wall.simplified_length.to_bits(),
        (wall.shape_tag != 2) as u32,
        p.dword_18,
        p.qword_10 as u32,
        (p.qword_10 >> 32) as u32,
        p.dword_1c,
        peel.to_bits(),
    ];
    InstanceUpdate {
        position: Vec3::ZERO,
        sort_key: 1.0,
        words,
    }
}

/// O: `data = [mat3(9), translation(3), cloth[0], cloth[1], cloth[2], bindless_index]`.
/// `cloth` = the 12-byte `ClothlineCloth` row (O stride 12: u64 @+0 copied via movsd, u32 @+8).
/// `normal_map_bindless_index` = `AssetTextureLibrary::handle_to_bindless_index(handle of NormalMap name)`.
pub fn build_cloth(
    rotation: Quat,
    translation: Vec3,
    scale: Vec3,
    cloth: [u32; 3],
    normal_map_bindless_index: u32,
) -> InstanceUpdate<16> {
    let (t, position, sort_key) = trs_packed(rotation, translation, scale);
    let mut words = [0u32; 16];
    words[..12].copy_from_slice(&t);
    words[12] = cloth[0];
    words[13] = cloth[1];
    words[14] = cloth[2];
    words[15] = normal_map_bindless_index;
    InstanceUpdate {
        position,
        sort_key,
        words,
    }
}

/// 原16B Smoke的GPU读取投影；实体权威属于components::Smoke。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SmokeView {
    /// +0x00 u32 → data word 13.
    pub dword_0: u32,
    /// +0x08 f32 (bit pattern copied) → data word 14.
    pub float_8: f32,
    /// +0x0c u8 (zero-extended, `movzx r8d,byte`) → data word 12.
    pub byte_c: u8,
}

impl From<&crate::components::Smoke> for SmokeView {
    fn from(smoke: &crate::components::Smoke) -> Self {
        Self { dword_0: smoke.key as u32, float_8: smoke.thickness, byte_c: smoke.kind }
    }
}

/// O: `data = [mat3(9), translation(3), byte_c as u32, dword_0, float_8, 0]`.
pub fn build_smoke(
    rotation: Quat,
    translation: Vec3,
    scale: Vec3,
    smoke: SmokeView,
) -> InstanceUpdate<16> {
    let (t, position, sort_key) = trs_packed(rotation, translation, scale);
    let mut words = [0u32; 16];
    words[..12].copy_from_slice(&t);
    words[12] = smoke.byte_c as u32;
    words[13] = smoke.dword_0;
    words[14] = smoke.float_8.to_bits();
    words[15] = 0;
    InstanceUpdate {
        position,
        sort_key,
        words,
    }
}

/// O: `data = [mat3(9), translation(3), semi_transparent as u32 (`and cl,1`), color_id, 0, 0]`.
pub fn build_balcony_door(
    rotation: Quat,
    translation: Vec3,
    scale: Vec3,
    color_id: u32,
    semi_transparent: bool,
) -> InstanceUpdate<16> {
    let (t, position, sort_key) = trs_packed(rotation, translation, scale);
    let mut words = [0u32; 16];
    words[..12].copy_from_slice(&t);
    words[12] = semi_transparent as u32;
    words[13] = color_id;
    InstanceUpdate {
        position,
        sort_key,
        words,
    }
}

// ---------------------------------------------------------------------------
// tests (pure functions only)
// ---------------------------------------------------------------------------
#[cfg(test)]
mod group_c_tests {
    use super::*;

    fn w(x: f32) -> u32 {
        x.to_bits()
    }

    #[test]
    fn pack_2x16f_low_is_first_arg() {
        // f16(1.0)=0x3c00, f16(2.0)=0x4000, f16(-0.5)=0xb800
        assert_eq!(pack_2x16f_uint(1.0, 2.0), 0x4000_3c00);
        assert_eq!(pack_2x16f_uint(-0.5, 0.0), 0x0000_b800);
    }

    #[test]
    fn trs_identity_layout() {
        let (words, pos, key) = trs_packed(
            Quat::IDENTITY,
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(2.0, 3.0, 4.0),
        );
        // columns: (2,0,0)(0,3,0)(0,0,4) then translation
        let expect = [2.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 4.0, 1.0, 2.0, 3.0];
        for i in 0..12 {
            assert_eq!(words[i], w(expect[i]), "word {i}");
        }
        assert_eq!(pos, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(key, 4.0);
    }

    #[test]
    fn trs_sort_key_is_max_abs_scale() {
        let (_, _, k) = trs_packed(Quat::IDENTITY, Vec3::ZERO, Vec3::new(-5.0, 2.0, 3.0));
        assert_eq!(k, 5.0);
        let (_, _, k) = trs_packed(Quat::IDENTITY, Vec3::ZERO, Vec3::new(1.0, -7.0, 3.0));
        assert_eq!(k, 7.0);
    }

    #[test]
    fn trs_rotation_90_about_z() {
        let q = Quat::from_rotation_z(core::f32::consts::FRAC_PI_2);
        let (words, _, _) = trs_packed(q, Vec3::ZERO, Vec3::ONE);
        // x axis -> +y, y axis -> -x
        assert!((f32::from_bits(words[0]) - 0.0).abs() < 1e-6);
        assert!((f32::from_bits(words[1]) - 1.0).abs() < 1e-6);
        assert!((f32::from_bits(words[3]) + 1.0).abs() < 1e-6);
    }

    #[test]
    fn lantern_flags_and_packing() {
        let p = LanternParamsView {
            p0: 1.0,
            p1: 2.0,
            p2: 0.5,
            p3: 4.0,
            flag: 1,
        };
        let u = build_lantern_emissive(Quat::IDENTITY, Vec3::ZERO, Vec3::ONE, p, true, Some(7));
        assert_eq!(u.words[12], 0x4000_3c00);
        assert_eq!(u.words[13], pack_2x16f_uint(0.5, 4.0));
        assert_eq!(u.words[14], 7 | (1 << 31) | (1 << 30));
        assert_eq!(u.words[15], 0);
        // no color id and not semi-transparent: only flag bit
        let u = build_lantern_emissive(Quat::IDENTITY, Vec3::ZERO, Vec3::ONE, p, false, None);
        assert_eq!(u.words[14], 1 << 30);
        let p0 = LanternParamsView { flag: 0, ..p };
        let u = build_lantern_emissive(Quat::IDENTITY, Vec3::ZERO, Vec3::ONE, p0, false, Some(3));
        assert_eq!(u.words[14], 3);
    }

    fn wall(count: u64, len: f32, tag: u32, def: f32) -> PlasterWallInputs {
        PlasterWallInputs {
            simplified_point_count: count,
            simplified_length: len,
            shape_tag: tag,
            default_peel: def,
        }
    }

    #[test]
    fn plaster_layout_and_constants() {
        let view = PlasterView {
            head: [1, 2, 3, 4],
            qword_10: 0x1111_2222_3333_4444,
            dword_18: 0x18,
            dword_1c: 0x1c,
            wall_id: 0xAAAA_BBBB_CCCC_DDDD,
        };
        let u = build_plaster(view, wall(2, 3.5, 1, 0.25), None);
        assert_eq!(u.position, Vec3::ZERO);
        assert_eq!(u.sort_key, 1.0);
        assert_eq!(
            u.words,
            [
                1,
                2,
                3,
                4,
                0xCCCC_DDDD,
                w(3.5),
                1,
                0x18,
                0x3333_4444,
                0x1111_2222,
                0x1c,
                w(0.25)
            ]
        );
    }

    #[test]
    fn plaster_peel_override_and_freehand() {
        let view = PlasterView::default();
        let u = build_plaster(view, wall(5, 1.0, 2, 0.25), Some(0.75));
        assert_eq!(u.words[6], 0); // shape_tag == 2 (Freehand) -> 0
        assert_eq!(u.words[11], w(0.75));
        let u = build_plaster(view, wall(5, 1.0, 0, 0.25), None);
        assert_eq!(u.words[6], 1);
        assert_eq!(u.words[11], w(0.25));
    }

    #[test]
    #[should_panic(expected = "assertion failed: w.simplified_curve_2d.is_valid()")]
    fn plaster_invalid_curve_few_points() {
        build_plaster(PlasterView::default(), wall(1, 1.0, 0, 0.0), None);
    }

    #[test]
    #[should_panic(expected = "assertion failed: w.simplified_curve_2d.is_valid()")]
    fn plaster_invalid_curve_zero_length() {
        build_plaster(PlasterView::default(), wall(4, 0.0, 0, 0.0), None);
    }

    #[test]
    #[should_panic(expected = "is_valid")]
    fn plaster_invalid_curve_nan_length() {
        build_plaster(PlasterView::default(), wall(4, f32::NAN, 0, 0.0), None);
    }

    #[test]
    fn plaster_view_from_words() {
        let v = PlasterView::from_words([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(v.head, [1, 2, 3, 4]);
        assert_eq!(v.qword_10, 5 | (6u64 << 32));
        assert_eq!((v.dword_18, v.dword_1c), (7, 8));
        assert_eq!(v.wall_id, 9 | (10u64 << 32));
    }

    #[test]
    fn cloth_layout() {
        let u = build_cloth(
            Quat::IDENTITY,
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::ONE,
            [10, 11, 12],
            99,
        );
        assert_eq!(&u.words[12..], &[10, 11, 12, 99]);
        assert_eq!(u.sort_key, 1.0);
    }

    #[test]
    fn smoke_layout() {
        let s = SmokeView {
            dword_0: 0xdead,
            float_8: 1.5,
            byte_c: 0xff,
        };
        let u = build_smoke(Quat::IDENTITY, Vec3::ZERO, Vec3::ONE, s);
        assert_eq!(&u.words[12..], &[0xff, 0xdead, w(1.5), 0]);
    }

    #[test]
    fn balcony_layout() {
        let u = build_balcony_door(Quat::IDENTITY, Vec3::ZERO, Vec3::ONE, 42, true);
        assert_eq!(&u.words[12..], &[1, 42, 0, 0]);
        let u = build_balcony_door(Quat::IDENTITY, Vec3::ZERO, Vec3::ONE, 42, false);
        assert_eq!(&u.words[12..], &[0, 42, 0, 0]);
    }
}

/// 48B packed `Mat3 (col-major, rot*scale) ++ translation` as 12 f32 words (O: identical tail in
/// plant/uber/velocity_only). Layout: [c0.xyz, c1.xyz, c2.xyz, t.xyz]; c_i = rot_axis_i * scale_i.
pub fn pack_affine3x4(t: &Transform) -> [u32; 12] {
    let m = Mat3::from_quat(t.rotation);
    let c0 = m.x_axis * t.scale.x;
    let c1 = m.y_axis * t.scale.y;
    let c2 = m.z_axis * t.scale.z;
    let tr = t.translation;
    [
        f32w(c0.x),
        f32w(c0.y),
        f32w(c0.z),
        f32w(c1.x),
        f32w(c1.y),
        f32w(c1.z),
        f32w(c2.x),
        f32w(c2.y),
        f32w(c2.z),
        f32w(tr.x),
        f32w(tr.y),
        f32w(tr.z),
    ]
}

/// Uber flags word (O): `bit0 = has AlbedoMap`, `bit1 = SemiTransparent present`, `bit2 = always set`
/// (asm: `lea ecx,[rcx + 2*r8 + 4]` with ecx = 1 when albedo present else 0, r8 = (semi != null)).
pub const UBER_FLAG_ALBEDO: u32 = 1;

pub const UBER_FLAG_SEMI_TRANSPARENT: u32 = 2;

pub const UBER_FLAG_BASE: u32 = 4;

/// O: words[0..12] = affine3x4(transform); words[12] = flags; words[13] = bindless index of albedo
/// texture (0 if no AlbedoMap); words[14..16] = 0. position = translation, sort_key = max |scale| (O).
pub fn build_uber(
    transform: &Transform,
    semi_transparent: bool,
    albedo_bindless_index: Option<u32>,
) -> InstanceUpdate<16> {
    let mut w = [0u32; 16];
    w[..12].copy_from_slice(&pack_affine3x4(transform));
    let has_albedo = albedo_bindless_index.is_some() as u32;
    w[12] = has_albedo * UBER_FLAG_ALBEDO
        + (semi_transparent as u32) * UBER_FLAG_SEMI_TRANSPARENT
        + UBER_FLAG_BASE;
    w[13] = albedo_bindless_index.unwrap_or(0);
    InstanceUpdate {
        position: transform.translation,
        sort_key: max_abs_scale(transform.scale),
        words: w,
    }
}

/// O: words[0..12] = affine3x4(current); words[12..24] = affine3x4(prev). position = current translation,
/// sort_key = max |current scale| (the prev transform's scale is not used for the key).
pub fn build_velocity_only(current: &Transform, prev: &Transform) -> InstanceUpdate<24> {
    let mut w = [0u32; 24];
    w[..12].copy_from_slice(&pack_affine3x4(current));
    w[12..].copy_from_slice(&pack_affine3x4(prev));
    InstanceUpdate {
        position: current.translation,
        sort_key: max_abs_scale(current.scale),
        words: w,
    }
}

/// O: data words[0..9] = 36 bytes copied from `StairsFloorMeshChunk + 8` (movups +8, movups +0x18, mov dword +0x28),
/// words[9..12] = 0; position = Vec3::ZERO; sort_key = 1.0 (xmm6 = 1.0f). `chunk_words` = those 9 words.
pub fn build_stairs_floor(chunk_words: [u32; 9]) -> InstanceUpdate<12> {
    let mut w = [0u32; 12];
    w[..9].copy_from_slice(&chunk_words);
    InstanceUpdate {
        position: Vec3::ZERO,
        sort_key: 1.0,
        words: w,
    }
}

#[cfg(test)]
mod group_d_tests {
    use super::*;

    fn tf(t: Vec3, r: Quat, s: Vec3) -> Transform {
        Transform {
            translation: t,
            rotation: r,
            scale: s,
        }
    }
    fn f(w: u32) -> f32 {
        f32::from_bits(w)
    }

    #[test]
    fn affine_identity_layout() {
        let w = pack_affine3x4(&tf(
            Vec3::new(1.0, 2.0, 3.0),
            Quat::IDENTITY,
            Vec3::new(2.0, 3.0, 4.0),
        ));
        let v: Vec<f32> = w.iter().map(|x| f(*x)).collect();
        assert_eq!(
            v,
            vec![2.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 4.0, 1.0, 2.0, 3.0]
        );
    }

    #[test]
    fn affine_rot_z_90() {
        let w = pack_affine3x4(&tf(
            Vec3::ZERO,
            Quat::from_rotation_z(core::f32::consts::FRAC_PI_2),
            Vec3::ONE,
        ));
        assert!((f(w[0])).abs() < 1e-6 && (f(w[1]) - 1.0).abs() < 1e-6); // x axis -> +y
        assert!((f(w[3]) + 1.0).abs() < 1e-6); // y axis -> -x
    }

    #[test]
    fn max_abs_uses_abs_and_ignores_nan() {
        assert_eq!(max_abs_scale(Vec3::new(-5.0, 2.0, 3.0)), 5.0);
        assert_eq!(max_abs_scale(Vec3::new(1.0, f32::NAN, -2.0)), 2.0);
    }

    #[test]
    fn uber_flags_and_index() {
        let t = tf(Vec3::new(1.0, 2.0, 3.0), Quat::IDENTITY, Vec3::splat(2.0));
        let a = build_uber(&t, false, None);
        assert_eq!(
            (a.words[12], a.words[13], a.words[14], a.words[15]),
            (4, 0, 0, 0)
        );
        let b = build_uber(&t, true, None);
        assert_eq!(b.words[12], 6);
        let c = build_uber(&t, false, Some(77));
        assert_eq!((c.words[12], c.words[13]), (5, 77));
        let d = build_uber(&t, true, Some(9));
        assert_eq!((d.words[12], d.words[13]), (7, 9));
        assert_eq!(d.sort_key, 2.0);
        assert_eq!(d.position, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn velocity_only_two_affines_key_from_current() {
        let cur = tf(Vec3::new(1.0, 0.0, 0.0), Quat::IDENTITY, Vec3::splat(3.0));
        let prev = tf(Vec3::new(9.0, 8.0, 7.0), Quat::IDENTITY, Vec3::splat(100.0));
        let u = build_velocity_only(&cur, &prev);
        assert_eq!(f(u.words[9]), 1.0);
        assert_eq!(f(u.words[12]), 100.0);
        assert_eq!(f(u.words[21]), 9.0);
        assert_eq!(u.sort_key, 3.0);
        assert_eq!(u.position, cur.translation);
    }

    #[test]
    fn stairs_floor_zero_tail_unit_key() {
        let u = build_stairs_floor([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(&u.words[..9], &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(&u.words[9..], &[0, 0, 0]);
        assert_eq!(u.position, Vec3::ZERO);
        assert_eq!(u.sort_key, 1.0);
    }
}
