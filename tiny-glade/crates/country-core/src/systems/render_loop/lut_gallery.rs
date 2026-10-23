//! `lut_gallery.rs` — photomode LUT gallery thumbnails.
//!
//! 证据（O = 机器码观测，I = 推断，U = 未知）：
//! - `LutGalleryPlugin::build`（0x1408b04c0，149 B）：分配 0x48 B 的 `Arc<Mutex<LutGalleryInner>>`
//!   （strong=1、weak=1、mutex=0、metrics 默认值 @+0x14；Option::None 载荷未初始化），以 `LutGalleryState { state, preview: None }`
//!   （两个寄存器字：`rdx = Arc`、`r8 = 0`）调用 `insert_resource`，随后 `add_systems(PreStartup, startup)`。（O）
//! - `startup`（无独立 PDB 过程）：加载 `/lut_gallery/thumbnail.raster.hlsl` 为 `lut_gallery_thumbnail`（O：字符串表）。
//! - `render_photomode_lut_gallery`（0x1408b0560，6721 B）：见 [`render_photomode_lut_gallery`]。
//!
//! 外部调用经 [`LutGalleryHost`] 缝表达，调用点即 O。

use std::sync::Arc;

use parking_lot::Mutex;

use crate::resources::gui::{EscMenuState, LoadMenuThumbnailMetrics};

use super::menu_background::ManagedTexture;

/// 着色器 URI（O：.rdata 字符串表）。
pub const SHADER_URI: &str = "/lut_gallery/thumbnail.raster.hlsl";
/// 着色器名（O：`get_raster_handle_by_name(.., len 0x15)`）。
pub const SHADER_NAME: &str = "lut_gallery_thumbnail";
/// pass 名（O：`RasterPass::new_impl(name, len 0xd)`）。
pub const PASS_NAME: &str = "LUT thumbnail";
/// 每格 6 个顶点（O：`draw_no_vao(edx = 6)`）。
pub const VERTICES_PER_CELL: u32 = 6;
/// 摄影模式的 `EscMenuState` 判别式（O：`cmp dword [mode], 0xa` + `byte [mode+8] == 1`）。
pub const PHOTOGRAPHY_MODE_STATE: u32 = 0xa;

/// 纹理绑定名哈希与访问类型（均 O：`bind_texture_to_draw_builder` 的 `r8` / `r9b`）。
pub const PREVIEW_BINDING: u64 = 0x11ba_5697_cc40_b3dd;
pub const ICON_BINDING: u64 = -0x3f4f_ad49_ac53_624a_i64 as u64;
pub const LUT_BINDING: u64 = 0x0422_b14c_27be_4803;
pub const TEXTURE_ACCESS: u8 = 0x1d;
/// 图标纹理 `TextureFromDisk` 的名哈希（O：`movabs rax, -0x59b817f50b16c6a1`）。
pub const ICON_DISK_HASH: u64 = -0x59b8_17f5_0b16_c6a1_i64 as u64;

/// uniform 名哈希（O：各 `write_uniform` 调用的 `r8`）。
pub const U_TARGET_EXTENT: u64 = 0x531b_b931_8eea_4971; // optional_uniform<[u32;2]>
pub const U_ALPHA: u64 = -0x636e_51d8_0248_c905_i64 as u64; // first=config0 grading strength; metas=1
pub const U_CELL_POS: u64 = 0x1d06_ec44_e216_b191; // Vec2 = positions[i]
pub const U_CELL_SIZE: u64 = 0x7038_b90f_2875_5919; // Vec2 = (state.+0x10, state.+0x14)
pub const U_HIGHLIGHT: u64 = -0x012b_1270_98b2_24ef_i64 as u64; // bool = state.flag && state.selected == i
pub const U_IS_CURRENT: u64 = -0x01ba_97e0_6487_b428_i64 as u64; // bool = name == 当前 LUT 名
pub const U_IS_FIRST: u64 = -0x0567_db38_0922_48cc_i64 as u64; // bool = (i == 0)

/// `RasterState` 改写（O：byte0 &= 0x7f；byte3 = (byte3 & 0x9f) | 0x40）。
pub const RASTER_STATE_BYTE0_MASK: u8 = 0x7f;
pub const RASTER_STATE_BYTE3_KEEP: u8 = 0x9f;
pub const RASTER_STATE_BYTE3_SET: u8 = 0x40;

/// B uses the same gui metrics owner as the load menu (Arc data at +0x14).
pub type LutGalleryInner = LoadMenuThumbnailMetrics;

/// `LutGalleryState` 资源（O 16 B：`+0` 状态 Arc，`+8` 预览纹理共享槽，`build` 里为 `None`）。
#[derive(Clone, bevy_ecs::resource::Resource)]
pub struct LutGalleryState {
    pub state: Arc<Mutex<LutGalleryInner>>,
    /// `Option<Arc<Mutex<Option<ManagedTexture>>>>`：数据区 niche 字节 @Arc+0x4e == 7 即槽内 `None`。
    pub preview: Option<Arc<Mutex<Option<ManagedTexture>>>>,
}

impl Default for LutGalleryState {
    fn default() -> Self {
        Self {
            state: Arc::new(Mutex::new(LutGalleryInner::default())),
            preview: None,
        }
    }
}

/// B passes the String's borrowed pointer/length pair (+8/+0x10) to
/// Display at 0x1408b0a58, 0x1408b0b86 and 0x1408b0db0. These are two
/// machine words of `&str`, not a u128 name. Preserve the canonical LUT name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LutName(pub String);

impl core::fmt::Display for LutName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}

/// `render_photomode_lut_gallery` 调用的外部面。
pub trait LutGalleryHost {
    /// `_DEV_OVERRIDE_IS_DEMO_MODE`（静态字节 0x143634d90）。
    fn demo_mode_override(&self) -> bool;
    /// `Res<LutGalleryState>`（hash `0xaca0d6d443d3ae2e`）；缺失 → `Option::unwrap` panic（O @0x1408b150e）。
    fn gallery_state(&self) -> &LutGalleryState;
    /// `Res<EscMenuState>`（hash `0xd38dae617fb79c7a`）；缺失 → unwrap panic（O @0x1408b151a）。
    fn esc_menu_state(&self) -> &EscMenuState;
    /// 第三个资源（hash `0x4a142e09642375b9`，O @0x1408b0718）：`+8` 起 0x40 B 条目数组，选择字节 @+0xc0，
    /// 条目的 String ptr/len @+8 —— 当前生效的 LUT 名（条目 `[selected]`，O 0x1408b0b86）。
    fn active_lut_name(&self) -> LutName;
    /// Entry0 is prepended once before the metadata iterator (1408b0d3b).
    fn first_entry_name(&self) -> LutName;
    fn first_entry_strength(&self) -> f32;
    /// `AssetShaderLibrary::get_raster_handle_by_name`（缺失 → unwrap panic，O @0x1408b157b）。
    fn raster_shader(&self, name: &str) -> u64;
    /// `GradingLuts`（hash `0xd387f1aea6230aea`）：0x28 B meta 条目，+8/+0x10 为 String ptr/len。
    fn lut_names(&self) -> Vec<LutName>;
    /// `NameHash::compute(&str)`（O 0x1408b101d）。
    fn name_hash(&self, name: &str) -> u64;
    /// `RasterPass::new_impl(PASS_NAME, [ColorAttachment(target)], no depth)`。
    fn begin_pass(&mut self, name: &str);
    /// 目标纹理 `extent_2d`（`optional_uniform<[u32;2]>` 的值）。
    fn target_extent(&self) -> [u32; 2];
    fn begin_draw(&mut self, shader: u64);
    /// `bind_texture_to_draw_builder<&Texture>`。
    fn bind_preview_texture(&mut self, tex: &ManagedTexture, binding: u64, access: u8);
    /// `bind_texture_to_draw_builder<TextureFromDisk>`（`name_hash` 取自磁盘纹理表）。
    fn bind_disk_texture(&mut self, name_hash: u64, binding: u64, access: u8);
    fn optional_uniform_u32x2(&mut self, key: u64, v: [u32; 2]);
    fn uniform_f32(&mut self, key: u64, v: f32);
    fn uniform_vec2(&mut self, key: u64, v: [f32; 2]);
    fn uniform_bool(&mut self, key: u64, v: bool);
    fn edit_raster_state(&mut self, byte0_mask: u8, byte3_keep: u8, byte3_set: u8);
    fn draw_no_vao(&mut self, vertices: u32);
    fn render_pass(&mut self);
}

/// 一格的 uniform 值（O：0x1408b1065..0x1408b1274 的写入顺序）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CellUniforms {
    pub alpha: f32,
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub highlighted: bool,
    pub is_current: bool,
    pub is_first: bool,
}

/// 单格 uniform 的纯函数。`highlighted = (state.+0x14 低字节) & (state.selected == i)`（O：`sete r9b; and r9b, al`）。
pub fn cell_uniforms(
    state: &LutGalleryInner,
    i: u32,
    position: [f32; 2],
    is_current: bool,
) -> CellUniforms {
    CellUniforms {
        alpha: 1.0,
        position,
        size: state.cell_size.to_array(),
        highlighted: state.highlighted_index == Some(i),
        is_current,
        is_first: i == 0,
    }
}

/// O：0x1408b0560，参数 `(pass_resources, world, target: &Texture)`。
///
/// 顺序（O）：演示模式开关 → 资源 → `EscMenuState` 须为 (0xa, 1) → 预览槽（`None` Arc 或槽内 `None` 都直接返回，
/// 锁先放）→ 第三资源 → 着色器 → LUT 列表 → 锁画廊状态 → 取 3 个名字 → "LUT thumbnail" 光栅 pass →
/// 逐格（`zip(luts, positions)`）绘制 → `render`。
pub fn render_photomode_lut_gallery<H: LutGalleryHost>(host: &mut H) {
    if host.demo_mode_override() {
        return;
    }
    let gallery = host.gallery_state();
    if !matches!(host.esc_menu_state(), EscMenuState::Photomode(true)) {
        return;
    }
    // 预览槽：Arc 缺失 / 槽空 → 返回。
    let Some(preview_arc) = gallery.preview.clone() else {
        return;
    };
    let preview_guard = preview_arc.lock();
    let Some(preview) = preview_guard.as_ref() else {
        return;
    };
    let active = host.active_lut_name();
    let first = host.first_entry_name();
    let first_strength = host.first_entry_strength();
    let shader = host.raster_shader(SHADER_NAME);
    let luts = host.lut_names();
    let state_arc = host.gallery_state().state.clone();
    let state = *state_arc.lock();
    let active_name = active.to_string();
    host.begin_pass(PASS_NAME);
    let positions = state.positions();
    let cells =
        std::iter::once((&first, first_strength)).chain(luts.iter().map(|name| (name, 1.0)));
    for (i, ((lut, strength), pos)) in cells.zip(positions).enumerate() {
        let i = i as u32;
        let name = lut.to_string();
        let is_current = name == active_name;
        host.begin_draw(shader);
        host.bind_preview_texture(preview, PREVIEW_BINDING, TEXTURE_ACCESS);
        host.bind_disk_texture(ICON_DISK_HASH, ICON_BINDING, TEXTURE_ACCESS);
        let asset = crate::resources::grading_lut::lut_name_to_asset_name(&name);
        let hash = host.name_hash(&asset);
        host.bind_disk_texture(hash, LUT_BINDING, TEXTURE_ACCESS);
        let extent = host.target_extent();
        host.optional_uniform_u32x2(U_TARGET_EXTENT, extent);
        let u = cell_uniforms(&state, i, pos.to_array(), is_current);
        host.uniform_f32(U_ALPHA, strength);
        host.uniform_vec2(U_CELL_POS, u.position);
        host.uniform_vec2(U_CELL_SIZE, u.size);
        host.uniform_bool(U_HIGHLIGHT, u.highlighted);
        host.uniform_bool(U_IS_CURRENT, u.is_current);
        host.uniform_bool(U_IS_FIRST, u.is_first);
        host.edit_raster_state(
            RASTER_STATE_BYTE0_MASK,
            RASTER_STATE_BYTE3_KEEP,
            RASTER_STATE_BYTE3_SET,
        );
        host.draw_no_vao(VERTICES_PER_CELL);
    }
    host.render_pass();
}

/// `startup` 的加载清单（O：字符串表；`load_shader_into_library(uri, name)`）。
pub const STARTUP_SHADERS: [(&str, &str); 1] = [(SHADER_URI, SHADER_NAME)];

/// LUT 画廊 bevy Plugin。
pub struct LutGalleryPlugin;

impl LutGalleryPlugin {
    /// O：0x1408b04c0。`insert_resource(LutGalleryState::default())` +
    /// `add_systems(PreStartup, startup)`（`startup` 加载 [`STARTUP_SHADERS`]）。
    pub fn build(&self) -> LutGalleryState {
        LutGalleryState::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct Rec {
        demo: bool,
        mode: Option<EscMenuState>,
        state: Option<LutGalleryState>,
        active: u128,
        luts: Vec<u128>,
        log: Vec<String>,
    }

    impl LutGalleryHost for Rec {
        fn demo_mode_override(&self) -> bool {
            self.demo
        }
        fn gallery_state(&self) -> &LutGalleryState {
            self.state.as_ref().expect("LutGalleryState")
        }
        fn esc_menu_state(&self) -> &EscMenuState {
            self.mode.as_ref().expect("EscMenuState")
        }
        fn active_lut_name(&self) -> LutName {
            LutName(self.active.to_string())
        }
        fn first_entry_name(&self) -> LutName {
            LutName("0".to_owned())
        }
        fn first_entry_strength(&self) -> f32 {
            0.625
        }
        fn raster_shader(&self, name: &str) -> u64 {
            assert_eq!(name, "lut_gallery_thumbnail");
            9
        }
        fn lut_names(&self) -> Vec<LutName> {
            self.luts.iter().map(|&v| LutName(v.to_string())).collect()
        }
        fn name_hash(&self, name: &str) -> u64 {
            crate::name_hash::NameHash::compute(name).0
        }
        fn begin_pass(&mut self, n: &str) {
            self.log.push(format!("pass {n}"));
        }
        fn target_extent(&self) -> [u32; 2] {
            [1920, 1080]
        }
        fn begin_draw(&mut self, s: u64) {
            self.log.push(format!("draw {s}"));
        }
        fn bind_preview_texture(&mut self, t: &ManagedTexture, b: u64, a: u8) {
            self.log.push(format!("preview {} {b:#x} {a:#x}", t.0));
        }
        fn bind_disk_texture(&mut self, h: u64, b: u64, a: u8) {
            self.log.push(format!("disk {h:#x} {b:#x} {a:#x}"));
        }
        fn optional_uniform_u32x2(&mut self, k: u64, v: [u32; 2]) {
            self.log.push(format!("u2 {k:#x} {v:?}"));
        }
        fn uniform_f32(&mut self, k: u64, v: f32) {
            self.log.push(format!("f32 {k:#x} {v}"));
        }
        fn uniform_vec2(&mut self, k: u64, v: [f32; 2]) {
            self.log.push(format!("v2 {k:#x} {v:?}"));
        }
        fn uniform_bool(&mut self, k: u64, v: bool) {
            self.log.push(format!("b {k:#x} {v}"));
        }
        fn edit_raster_state(&mut self, a: u8, b: u8, c: u8) {
            self.log.push(format!("state {a:#x} {b:#x} {c:#x}"));
        }
        fn draw_no_vao(&mut self, v: u32) {
            self.log.push(format!("vao {v}"));
        }
        fn render_pass(&mut self) {
            self.log.push("render".into());
        }
    }

    fn ready(cells: u32, luts: &[u128]) -> Rec {
        let st = LutGalleryState::default();
        *st.state.lock() = LutGalleryInner {
            highlighted_index: Some(1),
            origin: glam::Vec2::new(0.0, 10.0),
            step: glam::Vec2::new(0.5, 0.75),
            cell_size: glam::Vec2::new(0.25, 0.5),
            cell_count: cells,
            cells_per_line: 2,
            ..Default::default()
        };
        let mut r = Rec {
            mode: Some(EscMenuState::Photomode(true)),
            active: luts.first().copied().unwrap_or(0),
            luts: luts.to_vec(),
            ..Default::default()
        };
        let mut st = st;
        st.preview = Some(Arc::new(Mutex::new(Some(ManagedTexture(42)))));
        r.state = Some(st);
        r
    }

    #[test]
    fn lut_name_preserves_actual_utf8_asset_stem() {
        for name in ["default", "golden-hour", "雨后"] {
            assert_eq!(LutName(name.to_owned()).to_string(), name);
        }
    }

    #[test]
    fn build_matches_original_alloc() {
        let s = LutGalleryPlugin.build();
        assert!(s.preview.is_none());
        assert_eq!(*s.state.lock(), LutGalleryInner::default());
        assert_eq!(core::mem::size_of::<LutGalleryInner>(), 0x34);
    }

    #[test]
    fn demo_mode_override_short_circuits_before_any_lookup() {
        let mut r = Rec {
            demo: true,
            ..Default::default()
        };
        render_photomode_lut_gallery(&mut r); // 若越过守卫会在 expect 上 panic
        assert!(r.log.is_empty());
    }

    #[test]
    fn only_photography_mode_with_flag_draws() {
        for state in [
            EscMenuState::Credits,
            EscMenuState::Closed,
            EscMenuState::Photomode(false),
        ] {
            let mut r = ready(1, &[1]);
            r.mode = Some(state);
            render_photomode_lut_gallery(&mut r);
            assert!(r.log.is_empty());
        }
    }

    #[test]
    fn missing_or_empty_preview_returns_without_a_pass() {
        let mut r = ready(1, &[1]);
        r.state.as_mut().unwrap().preview = None;
        render_photomode_lut_gallery(&mut r);
        assert!(r.log.is_empty());
        let mut r = ready(1, &[1]);
        *r.state.as_ref().unwrap().preview.as_ref().unwrap().lock() = None;
        render_photomode_lut_gallery(&mut r);
        assert!(r.log.is_empty());
    }

    #[test]
    fn draws_one_quad_per_cell_with_exact_uniform_order() {
        let mut r = ready(2, &[0xa0, 0xb1]);
        render_photomode_lut_gallery(&mut r);
        let l = &r.log;
        assert_eq!(l[0], "pass LUT thumbnail");
        // cell 0
        assert_eq!(l[1], "draw 9");
        assert_eq!(l[2], "preview 42 0x11ba5697cc40b3dd 0x1d");
        assert!(l[3].starts_with("disk 0xa647e80af4e9395f 0x") && l[3].ends_with(" 0x1d"));
        assert!(l[4].ends_with("0x422b14c27be4803 0x1d"));
        assert_eq!(l[5], "u2 0x531bb9318eea4971 [1920, 1080]");
        assert_eq!(l[6], format!("f32 {:#x} 0.625", U_ALPHA));
        assert_eq!(l[7], format!("v2 {:#x} [0.0, 10.0]", U_CELL_POS));
        assert_eq!(l[8], format!("v2 {:#x} [0.25, 0.5]", U_CELL_SIZE));
        assert_eq!(l[9], format!("b {:#x} false", U_HIGHLIGHT)); // 选中的是 1
        assert_eq!(l[10], format!("b {:#x} false", U_IS_CURRENT));
        assert_eq!(l[11], format!("b {:#x} true", U_IS_FIRST));
        assert_eq!(l[12], "state 0x7f 0x9f 0x40");
        assert_eq!(l[13], "vao 6");
        // cell 1：高亮 + 当前
        assert!(l.iter().any(|s| *s == format!("b {:#x} true", U_HIGHLIGHT)));
        assert!(
            l.iter()
                .any(|s| *s == format!("b {:#x} true", U_IS_CURRENT))
        );
        assert_eq!(l.last().unwrap(), "render");
        assert_eq!(l.iter().filter(|s| *s == "vao 6").count(), 2);
    }

    #[test]
    fn zip_stops_at_the_shorter_side() {
        let mut r = ready(5, &[1, 2]);
        render_photomode_lut_gallery(&mut r);
        assert_eq!(r.log.iter().filter(|s| *s == "vao 6").count(), 3);
        let mut r = ready(1, &[1, 2, 3]);
        render_photomode_lut_gallery(&mut r);
        assert_eq!(r.log.iter().filter(|s| *s == "vao 6").count(), 1);
    }

    #[test]
    fn empty_gallery_still_opens_and_renders_the_pass() {
        let mut r = ready(0, &[]);
        render_photomode_lut_gallery(&mut r);
        assert_eq!(r.log, ["pass LUT thumbnail", "render"]);
    }

    #[test]
    fn highlight_needs_a_present_matching_index() {
        let s = LutGalleryInner {
            highlighted_index: None,
            ..Default::default()
        };
        assert!(!cell_uniforms(&s, 3, [0.0; 2], false).highlighted);
        let s = LutGalleryInner {
            highlighted_index: Some(3),
            ..Default::default()
        };
        assert!(cell_uniforms(&s, 3, [0.0; 2], false).highlighted);
        assert!(!cell_uniforms(&s, 2, [0.0; 2], false).highlighted);
        assert!(cell_uniforms(&s, 0, [0.0; 2], false).is_first);
    }

    #[test]
    fn real_gui_metrics_reach_the_draw_uniforms_in_both_grid_orders() {
        for column_major in [false, true] {
            let mut r = ready(5, &[1, 2, 3, 4, 5]);
            r.state.as_ref().unwrap().state.lock().column_major = column_major;
            render_photomode_lut_gallery(&mut r);
            let positions: Vec<_> = r
                .log
                .iter()
                .filter(|entry| entry.starts_with(&format!("v2 {U_CELL_POS:#x} ")))
                .cloned()
                .collect();
            let expected = if column_major {
                [
                    [0.0, 10.0],
                    [0.0, 10.75],
                    [0.5, 10.0],
                    [0.5, 10.75],
                    [1.0, 10.0],
                ]
            } else {
                [
                    [0.0, 10.0],
                    [0.5, 10.0],
                    [0.0, 10.75],
                    [0.5, 10.75],
                    [0.0, 11.5],
                ]
            };
            assert_eq!(
                positions,
                expected.map(|pos| format!("v2 {U_CELL_POS:#x} {pos:?}"))
            );
        }
    }

    #[test]
    fn startup_loads_the_thumbnail_shader() {
        assert_eq!(
            STARTUP_SHADERS,
            [(
                "/lut_gallery/thumbnail.raster.hlsl",
                "lut_gallery_thumbnail"
            )]
        );
    }
}
