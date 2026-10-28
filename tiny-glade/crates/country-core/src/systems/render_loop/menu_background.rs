//! `menu_background.rs` — menu background blit (boot/menu fullscreen).
//!
//! 证据（PDB 过程 + 机器码，O = 观测，I = 推断，U = 未知）：
//! - `MenuBackgroundPlugin::build`（0x1408c99c0，104 B）：分配 0x50 B 的 `Arc<Mutex<Option<ManagedTexture>>>`
//!   （strong=1、weak=1、mutex 字节 @+0x10 = 0、数据 @+0x18、数据内的 `None` niche 字节 @+0x4e = 7），
//!   然后 `insert_resource<MenuBackgroundResources>`。资源本体 0x40 B：`+0` 共享槽 Arc、`+8` 当前纹理
//!   `Option<ManagedTexture>`（0x38 B，`None` niche 字节 @+0x3e = 7）。（O）
//! - `render_menu_bg`（0x1408c9a30，1927 B）：见 [`render_menu_bg`]。
//! - 没有 PDB 过程的 `startup`：背景着色器 `blit_stretch_decode_srgb` 由 blit 模块在别处加载
//!   （`/blit/blit_stretch_decode_srgb.raster.hlsl`，O 字符串表），本模块不加载着色器。
//!
//! 外部调用（world 资源、锁外的 GPU 声明）经 [`MenuBackgroundHost`] 缝表达；调用点即 O。

use std::sync::Arc;

use parking_lot::Mutex;

/// 3 顶点全屏三角形（`draw_no_vao(3)`，O：`mov edx, 3`）。
pub const FULLSCREEN_TRIANGLE_VERTICES: u32 = 3;
/// 光栅着色器名（O：`get_raster_handle_by_name("blit_stretch_decode_srgb", len 0x18)`）。
pub const BLIT_SHADER_NAME: &str = "blit_stretch_decode_srgb";
/// pass 名（O：`RasterPass::new_impl(name = "Menu bg", len 7)`）。
pub const PASS_NAME: &str = "Menu bg";
/// 源纹理绑定名哈希（O：`bind_texture_to_draw_builder` 的 `r8`）。
pub const SOURCE_TEXTURE_BINDING: u64 = 0x11ba_5697_cc40_b3dd;
/// 源纹理访问类型（O：`r9b = 0x1d`，读采样）。
pub const SOURCE_TEXTURE_ACCESS: u8 = 0x1d;
/// `RasterState` 字节 0 的清除掩码（O：`and al, 0x7f`）。
pub const RASTER_STATE_BYTE0_MASK: u8 = 0x7f;

/// EscMenuState read projection. RNE@1408c9a4d uses the same native TypeId
/// d38dae617fb79c7a as session selection; the historical AppMode name was wrong.
/// Retained API name avoids changing unrelated adapters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AppModeView {
    /// `+0x00`：比较发生在 64 位（`== 2`）与 32 位（`== 0xa`、`== 7`）宽度上。
    pub state: u64,
    /// `+0x08`。
    pub flag: u8,
}

/// 受管 GPU 纹理句柄（`rhapsody::resource::managed` 的 0x38 B 值；字段布局 U，仅持有 id）。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ManagedTexture(pub u64);

/// 菜单背景资源（O 0x40 B）。
#[derive(bevy_ecs::resource::Resource)]
pub struct MenuBackgroundResources {
    /// `+0x00`：其它系统往里放新纹理的共享槽（`Arc<Mutex<Option<_>>>`）。
    pub shared: Arc<Mutex<Option<ManagedTexture>>>,
    /// `+0x08`：当前正在绘制的纹理。
    pub current: Option<ManagedTexture>,
}

impl Default for MenuBackgroundResources {
    /// `build` 里的初始化（O：Arc 内 strong=1 weak=1、mutex 空、数据 niche=7 即 `None`；`current` niche=7 即 `None`）。
    fn default() -> Self {
        Self {
            shared: Arc::new(Mutex::new(None)),
            current: None,
        }
    }
}

/// EscMenuState gate（O：0x1408c9aca..0x1408c9fa3）。返回 `true` = 继续绘制。
///
/// 1. `state == 2`（64 位比较）→ 不画；
/// 2. `state == 0xa`（低 32 位）且 `flag != 1` → 不画；
/// 3. `k = state - 2`（无符号），`k >= 9` 取 4；
/// 4. `0xaf >> k & 1`（k ∈ {0,1,2,3,5,7}）：仅当 `state == 7 && flag == 1` 不画，否则画；
/// 5. 否则（k ∈ {4,6,8}）：`0x50 >> k & 1`（k ∈ {4,6}）不画；k = 8 仅当 `flag == 0` 才画。
pub fn menu_bg_mode_allows(mode: AppModeView) -> bool {
    let state = mode.state;
    if state == 2 {
        return false;
    }
    if (state as u32) == 0xa && mode.flag != 1 {
        return false;
    }
    let k = state.wrapping_sub(2);
    let k = if k < 9 { k } else { 4 };
    if (0xafu64 >> k) & 1 != 0 {
        !((state as u32) == 7 && mode.flag == 1)
    } else if (0x50u64 >> k) & 1 != 0 {
        false
    } else {
        mode.flag == 0
    }
}

/// `render_menu_bg` 调用的外部面（调用点 O；实现者是 bevy world / 资源库 / pass_builder）。
pub trait MenuBackgroundHost {
    /// `Components::get_resource_id(EscMenuState)` + value (missing panics).
    fn app_mode(&self) -> AppModeView;
    /// `MenuBackgroundResources`（hash `0xfb32e4dcc8287bca`，缺失 → unwrap panic，O @0x1408c9fe0）。
    fn menu_background(&mut self) -> &mut MenuBackgroundResources;
    /// 写资源的 changed tick（O：`[ticks+0x5c] = world.change_tick(+0x5b0)`；仅在取出新纹理时）。
    fn mark_menu_background_changed(&mut self);
    /// `Plugin::finish<MainCameraPlugin>(&current)`（O 调用点 0x1408c9c4b，0x14003bb80 是 1 字节 `ret`，
    /// 实为 ICF 合并的空函数，无副作用）+ 旧纹理的 `ManagedHandle::drop`。主相机并不在此释放。
    fn on_current_replaced(&mut self, old: ManagedTexture);
    /// `AssetShaderLibrary::get_raster_handle_by_name`（hash `0x85…`；未命中 → `Option::unwrap` panic）。
    fn raster_shader(&self, name: &str) -> u64;
    /// `RasterPass::new_impl("Menu bg", [ColorAttachment(target)], no depth)`；返回 pass 的视口 = `extent_2d(target.desc)`。
    fn begin_pass(&mut self, name: &str);
    /// `DrawPacket::new(shader)` + `DrawBuilder::new(variant 0)`。
    fn begin_draw(&mut self, shader: u64);
    /// `bind_texture_to_draw_builder(&Texture, binding, access)`。
    fn bind_texture(&mut self, tex: &ManagedTexture, binding_hash: u64, access: u8);
    /// `RasterState.byte0 &= mask`。
    fn clear_raster_state_byte0(&mut self, mask: u8);
    /// `DrawBuilder::draw_no_vao(vertices)`。
    fn draw_no_vao(&mut self, vertices: u32);
    /// `RasterPass::render`。
    fn render_pass(&mut self);
}

/// 把共享槽里的新纹理换进 `current`（O：0x1408c9b43..0x1408c9c97）。
///
/// 返回 `true` = 取到了新值（此时 world 的 changed tick 被写入，旧值被释放）。
/// 锁内 `mem::replace(slot, None)`（O：拷贝 0x38 B 并写 niche 7），随后解锁。
pub fn take_pending_into_current(
    res: &mut MenuBackgroundResources,
) -> (bool, Option<ManagedTexture>) {
    let taken = res.shared.lock().take();
    match taken {
        None => (false, None),
        Some(new) => {
            let old = res.current.replace(new);
            (true, old)
        }
    }
}

/// O：0x1408c9a30（`render_menu_bg`，参数 `(pass_resources, world, target: &Texture)`）。
///
/// 顺序：AppMode 门控 → 取 `MenuBackgroundResources`、从共享槽取新纹理（有则 changed tick、释放旧句柄、换入）→
/// `current` 为 `None` 则结束 → 取 `blit_stretch_decode_srgb` 着色器 → "Menu bg" 光栅 pass（颜色附件=目标纹理、无深度）→
/// 单个全屏三角形，把 `current` 绑为源纹理（`0x11ba…`，访问 0x1d），清 raster state 字节 0 的最高位 → render。
pub fn render_menu_bg<H: MenuBackgroundHost>(host: &mut H) {
    if !menu_bg_mode_allows(host.app_mode()) {
        return;
    }
    let (got_new, old) = take_pending_into_current(host.menu_background());
    if got_new {
        host.mark_menu_background_changed();
        if let Some(old) = old {
            host.on_current_replaced(old);
        }
    }
    let Some(current) = host.menu_background().current.clone() else {
        return;
    };
    let shader = host.raster_shader(BLIT_SHADER_NAME);
    host.begin_pass(PASS_NAME);
    host.begin_draw(shader);
    host.bind_texture(&current, SOURCE_TEXTURE_BINDING, SOURCE_TEXTURE_ACCESS);
    host.clear_raster_state_byte0(RASTER_STATE_BYTE0_MASK);
    host.draw_no_vao(FULLSCREEN_TRIANGLE_VERTICES);
    host.render_pass();
}

/// MenuBackground bevy Plugin。
pub struct MenuBackgroundPlugin;

impl MenuBackgroundPlugin {
    /// O：0x1408c99c0。`insert_resource(MenuBackgroundResources::default())`（无系统注册）。
    pub fn build(&self) -> MenuBackgroundResources {
        MenuBackgroundResources::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct Rec {
        mode: Option<AppModeView>,
        res: Option<MenuBackgroundResources>,
        log: Vec<String>,
    }

    impl MenuBackgroundHost for Rec {
        fn app_mode(&self) -> AppModeView {
            self.mode.expect("AppMode")
        }
        fn menu_background(&mut self) -> &mut MenuBackgroundResources {
            self.res.as_mut().unwrap()
        }
        fn mark_menu_background_changed(&mut self) {
            self.log.push("changed".into());
        }
        fn on_current_replaced(&mut self, old: ManagedTexture) {
            self.log.push(format!("drop_old {}", old.0));
        }
        fn raster_shader(&self, name: &str) -> u64 {
            assert_eq!(name, "blit_stretch_decode_srgb");
            77
        }
        fn begin_pass(&mut self, name: &str) {
            self.log.push(format!("pass {name}"));
        }
        fn begin_draw(&mut self, shader: u64) {
            self.log.push(format!("draw_new {shader}"));
        }
        fn bind_texture(&mut self, t: &ManagedTexture, b: u64, a: u8) {
            self.log.push(format!("bind {} {b:#x} {a:#x}", t.0));
        }
        fn clear_raster_state_byte0(&mut self, m: u8) {
            self.log.push(format!("mask {m:#x}"));
        }
        fn draw_no_vao(&mut self, v: u32) {
            self.log.push(format!("no_vao {v}"));
        }
        fn render_pass(&mut self) {
            self.log.push("render".into());
        }
    }

    fn rec(mode: AppModeView) -> Rec {
        Rec {
            mode: Some(mode),
            res: Some(MenuBackgroundResources::default()),
            log: vec![],
        }
    }

    #[test]
    fn mode_gate_truth_table_matches_machine_code() {
        let m = |state, flag| menu_bg_mode_allows(AppModeView { state, flag });
        assert!(!m(2, 0) && !m(2, 1));
        assert!(!m(10, 0) && !m(10, 2));
        assert!(!m(10, 1)); // k=8：0xaf/0x50 位均未置 → 仅 flag==0 才画
    }

    #[test]
    fn mode_gate_exhaustive_against_reference_model() {
        // 逐条按反汇编控制流的独立实现。
        fn reference(state: u64, flag: u8) -> bool {
            if state == 2 {
                return false;
            }
            let low = state as u32;
            if low == 0xa && flag != 1 {
                return false;
            }
            let k = if state.wrapping_sub(2) < 9 {
                state - 2
            } else {
                4
            };
            let set_af = [0u64, 1, 2, 3, 5, 7].contains(&k);
            if set_af {
                if low == 7 && flag == 1 {
                    return false;
                }
                return true;
            }
            if k == 4 || k == 6 {
                return false;
            }
            flag == 0
        }
        for state in [
            0u64,
            1,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            9,
            10,
            11,
            12,
            100,
            0x1_0000_0007,
            u64::MAX,
        ] {
            for flag in [0u8, 1, 2] {
                assert_eq!(
                    menu_bg_mode_allows(AppModeView { state, flag }),
                    reference(state, flag),
                    "state={state} flag={flag}"
                );
            }
        }
    }

    #[test]
    fn state_10_with_flag_1_goes_through_k8_branch_requiring_flag_zero() {
        // state=10,flag=1：通过第 2 步；k=8 → 不在 0xaf，也不在 0x50 → 只有 flag==0 才画 → 不画。
        assert!(!menu_bg_mode_allows(AppModeView { state: 10, flag: 1 }));
        // state=7,flag=1 → 不画；state=7,flag=0 → 画。
        assert!(!menu_bg_mode_allows(AppModeView { state: 7, flag: 1 }));
        assert!(menu_bg_mode_allows(AppModeView { state: 7, flag: 0 }));
        // state=6 (k=4) 与 state=8 (k=6) 不画。
        assert!(!menu_bg_mode_allows(AppModeView { state: 6, flag: 0 }));
        assert!(!menu_bg_mode_allows(AppModeView { state: 8, flag: 0 }));
        // state=0/1 (k 下溢 → 4) 不画。
        assert!(!menu_bg_mode_allows(AppModeView { state: 0, flag: 0 }));
    }

    #[test]
    fn blocked_mode_does_nothing() {
        let mut h = rec(AppModeView { state: 2, flag: 0 });
        *h.res.as_mut().unwrap().shared.lock() = Some(ManagedTexture(5));
        render_menu_bg(&mut h);
        assert!(h.log.is_empty());
        // 共享槽未被取走。
        assert!(h.res.as_ref().unwrap().shared.lock().is_some());
    }

    #[test]
    fn no_texture_means_no_pass() {
        let mut h = rec(AppModeView { state: 3, flag: 0 });
        render_menu_bg(&mut h);
        assert!(h.log.is_empty());
    }

    #[test]
    fn pending_texture_is_swapped_in_and_blitted() {
        let mut h = rec(AppModeView { state: 3, flag: 0 });
        *h.res.as_mut().unwrap().shared.lock() = Some(ManagedTexture(5));
        render_menu_bg(&mut h);
        assert_eq!(
            h.log,
            [
                "changed",
                "pass Menu bg",
                "draw_new 77",
                "bind 5 0x11ba5697cc40b3dd 0x1d",
                "mask 0x7f",
                "no_vao 3",
                "render"
            ]
        );
        assert!(h.res.as_ref().unwrap().shared.lock().is_none());
    }

    #[test]
    fn replacing_current_drops_old_handle_first() {
        let mut h = rec(AppModeView { state: 3, flag: 0 });
        h.res.as_mut().unwrap().current = Some(ManagedTexture(1));
        *h.res.as_mut().unwrap().shared.lock() = Some(ManagedTexture(2));
        render_menu_bg(&mut h);
        assert_eq!(&h.log[..3], ["changed", "drop_old 1", "pass Menu bg"]);
        assert!(
            h.log
                .contains(&"bind 2 0x11ba5697cc40b3dd 0x1d".to_string())
        );
    }

    #[test]
    fn existing_current_is_redrawn_without_changed_tick() {
        let mut h = rec(AppModeView { state: 3, flag: 0 });
        h.res.as_mut().unwrap().current = Some(ManagedTexture(9));
        render_menu_bg(&mut h);
        assert_eq!(h.log.first().unwrap(), "pass Menu bg");
        assert!(!h.log.iter().any(|l| l == "changed"));
    }

    #[test]
    fn build_default_is_empty() {
        let r = MenuBackgroundPlugin.build();
        assert!(r.current.is_none());
        assert!(r.shared.lock().is_none());
        assert_eq!(Arc::strong_count(&r.shared), 1);
    }
}
