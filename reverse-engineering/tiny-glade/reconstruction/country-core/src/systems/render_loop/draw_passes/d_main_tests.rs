//! d_main 的 pass 级测试：用 A4 的 `MockHost` 记录每个 pass 提交的 `RasterPassSubmission`，
//! 断言机器码里观测到的名字 / 附件 tag / 访问类型 / 列表选择 / 状态位 / 变体 / 绘制种类 / 门控。

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::sync::Arc;
use core::cell::RefCell;
use core::time::Duration;

use bevy_ecs::world::World;

use crate::asset_libraries::color_library::ColorLibrarySsbo;
use crate::render::pass_builder::draw_builder::DrawBuilder;
use crate::render::pass_builder::draw_stream::test_host::{Log, MockHost};
use crate::render::pass_builder::draw_stream::{DrawKind, PassHost, ShaderReflection};
use crate::render::pass_builder::framebuffer_binding::AttachmentTexture;
use crate::render::pass_builder::pass_resource_access::SsboType;
use crate::render::shader::RasterShaderProgram;
use crate::rhapsody_contract::{Handle, ManagedHandle};
use crate::systems::render_loop::draw_passes::d_main_support::{
    DrawPassCtx, ShaderLookup, velocity_state, wireframe_state, with_stencil_write,
};
use crate::systems::render_loop::draw_passes::draw_lists::{DrawItem, DrawLists, EntityDrawParams, list};

fn tex(id: u32, w: u32, h: u32) -> AttachmentTexture {
    AttachmentTexture {
        id,
        extent: (w, h),
        format: 44,
        sample_count: 1,
    }
}

/// 所有测试共用：宿主工厂（共享日志；着色器句柄 1..=9 都有反射；`bind_common` 需要的预设与颜色库缓冲）。
struct Rig {
    log: Rc<RefCell<Log>>,
    shaders: Vec<u64>,
    extra_presets: Vec<(u8, AttachmentTexture)>,
    extra_files: Vec<u64>,
    extra_buffers: Vec<(u64, u32)>,
    extra_words: Vec<(u64, u32)>,
}

impl Rig {
    fn new() -> Rig {
        Rig {
            log: Rc::new(RefCell::new(Log::default())),
            shaders: (1..=9).collect(),
            extra_presets: Vec::new(),
            extra_files: Vec::new(),
            extra_buffers: Vec::new(),
            extra_words: Vec::new(),
        }
    }
    fn host(&self) -> Box<dyn PassHost> {
        let (mut host, _) = MockHost::new();
        host.log = self.log.clone();
        for id in &self.shaders {
            let mut r = ShaderReflection::default();
            r.program_name = "test";
            host.shaders.insert(*id, Arc::new(r));
        }
        host.rg_buffers.insert(ColorLibrarySsbo::rg_buffer_hash(), 1);
        host.rg_buffers.insert(
            crate::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo::rg_buffer_hash(),
            4,
        );
        for f in [
            crate::systems::render_loop::draw_passes::custom_cursor_pass::CURSOR_TEXTURE_FILE,
            crate::systems::render_loop::draw_passes::logo_overlay::LOGO_TEXTURE_FILE,
        ] {
            host.textures_by_name.insert(f, (ManagedHandle(1), tex(80, 16, 16)));
        }
        host.presets.insert(0x11, tex(900, 4, 4));
        host.presets.insert(0x12, tex(901, 4, 4));
        for (p, t) in &self.extra_presets {
            host.presets.insert(*p, *t);
        }
        for f in &self.extra_files {
            host.textures_by_name.insert(*f, (ManagedHandle(2), tex(81, 8, 8)));
        }
        for (h, r) in &self.extra_buffers {
            host.rg_buffers.insert(*h, *r);
        }
        for (h, w) in &self.extra_words {
            host.buffer_words.insert(*h, *w);
        }
        host.boxed()
    }
}

struct Shaders;
impl ShaderLookup for Shaders {
    fn raster_handle_by_name(&self, name: &str) -> Option<Handle<RasterShaderProgram>> {
        let id = match name {
            "custom_cursor" => 1,
            "blit_stretch_decode_srgb" => 2,
            "logo_overlay" => 3,
            "stairs_foliage_exclusion" => 4,
            "grass_object_push" => 5,
            "ui_2d" => 6,
            "ui_2d_color_button" => 7,
            "ui_2d_ws" => 8,
            "text" => 9,
            "mei_shadow" => 1,
            "raster_resolve_to_swapchain" => 2,
            "raster_resolve_msaa_to_swapchain" => 3,
            _ => return None,
        };
        Some(Handle(ManagedHandle(id), core::marker::PhantomData))
    }
}

fn run<R>(rig: &Rig, f: impl FnOnce(&mut DrawPassCtx<'_>) -> R) -> R {
    let mut world = World::new();
    let mut factory = || rig.host();
    let shaders = Shaders;
    let mut ctx = DrawPassCtx::new(&mut world, Duration::new(2, 500_000_000), &mut factory, &shaders);
    f(&mut ctx)
}

fn cb(b: DrawBuilder<'_>, _i: &EntityDrawParams, _w: &mut World) {
    b.draw_no_vao(3);
}

fn item(
    shader: u64,
    key: i32,
    stencil: Option<u8>,
) -> (DrawItem, crate::systems::render_loop::draw_passes::draw_lists::DrawFn) {
    let mut it = DrawItem::new(shader, key);
    it.vao = 0x77;
    it.stencil_ref = stencil;
    (it, cb)
}

fn names(rig: &Rig) -> Vec<&'static str> {
    rig.log.borrow().submissions.iter().map(|s| s.name).collect()
}

#[test]
fn wireframe_pass_matches_the_machine_code_sequence() {
    use super::wireframe_pass::wireframe_pass;
    let rig = Rig::new();
    let mut lists = DrawLists::new();
    lists.list_mut(list::WIREFRAME).push(item(1, 0, None));
    lists.list_mut(list::UI).push(item(1, 0, None)); // 不应被读取
    run(&rig, |c| wireframe_pass(c, &tex(7, 1280, 720), &lists));
    let l = rig.log.borrow();
    assert_eq!(l.submissions.len(), 1);
    let s = &l.submissions[0];
    assert_eq!(s.name, "Wireframe");
    assert_eq!(s.color_count, 1);
    // 颜色 tag0 → 0x36，无深度附件。
    assert_eq!(s.accesses[0].access, 0x36);
    assert_eq!(s.draws.len(), 1);
    let d = &s.draws[0];
    assert_eq!(d.head, [1, 0x77]);
    assert_eq!(d.variant, 0);
    assert_eq!(d.blend, 1.0);
    // MockHost 的默认状态 0x8000_0080：byte3 bit7 置位、byte0 bit7 清除。
    assert_eq!(d.state.0, wireframe_state(0x8000_0080));
    assert_eq!(d.state.0, 0x8000_0000);
    assert_eq!(d.kind, DrawKind::NoVao { vertex_count: 3 });
}

#[test]
fn ui_and_loading_screen_use_lists_19_and_18_and_are_named_ui() {
    use super::ui_pass::{loading_screen_pass, ui_pass};
    let rig = Rig::new();
    let mut lists = DrawLists::new();
    lists.list_mut(list::UI).push(item(1, 0, None));
    lists.list_mut(list::UI).push(item(2, 0, None));
    lists.list_mut(list::LOADING_SCREEN).push(item(3, 0, None));
    run(&rig, |c| ui_pass(c, &tex(7, 64, 64), &lists));
    run(&rig, |c| loading_screen_pass(c, &tex(7, 64, 64), &lists));
    assert_eq!(names(&rig), vec!["UI", "UI"]);
    let l = rig.log.borrow();
    assert_eq!(l.submissions[0].draws.len(), 2);
    assert_eq!(l.submissions[1].draws.len(), 1);
    // 无状态改写：状态保持宿主默认值。
    assert_eq!(l.submissions[0].draws[0].state.0, 0x8000_0080);
}

#[test]
fn state_edit_helpers_match_the_observed_masks() {
    // with_stencil_write: byte1 = (byte1 & 0x8e) | 0x21
    assert_eq!(with_stencil_write(0x0000_ff00), 0x0000_af00);
    assert_eq!(with_stencil_write(0xff00_00ff), 0xff00_21ff);
    // velocity: (state & 0xffffff8f) | 0x20
    assert_eq!(velocity_state(0x0000_00ff), 0x0000_00af);
    assert_eq!(velocity_state(0), 0x20);
    // wireframe
    assert_eq!(wireframe_state(0x0000_0080), 0x8000_0000);
}

fn gbuf() -> super::main_pass::MainPassTargets {
    super::main_pass::MainPassTargets {
        gbuffer: tex(10, 640, 360),
        depth: tex(11, 640, 360),
        velocity: tex(12, 640, 360),
    }
}

#[test]
fn main_pass_runs_six_subpasses_in_order_with_observed_attachments() {
    use super::main_pass::main_pass;
    let rig = Rig::new();
    let lists = DrawLists::new();
    run(&rig, |c| main_pass(c, &gbuf(), &tex(13, 640, 360), &lists, true));
    assert_eq!(
        names(&rig),
        vec![
            "G-buffer",
            "G-buffer emissive",
            "G-buffer velocity",
            "Forward",
            "ForwardVelocity",
            "Velocity"
        ]
    );
    let l = rig.log.borrow();
    // G-buffer：颜色 tag2 → 访问 0x2e；深度可写 0x2f；清除深度（描述里 clear_input）。
    let g = &l.submissions[0];
    assert_eq!(g.color_count, 1);
    assert_eq!(g.accesses[0].access, 0x2e);
    assert_eq!(g.accesses[1].access, 0x2f);
    assert_eq!(l.created_passes[0].colors[0].kind_tag, 2);
    assert!(l.created_passes[0].depth.clear_input);
    // emissive：tag0 (0x36) + tag1 (0x2e)；深度不再清除。
    let e = &l.submissions[1];
    assert_eq!(e.color_count, 2);
    assert_eq!((e.accesses[0].access, e.accesses[1].access), (0x36, 0x2e));
    assert!(!l.created_passes[1].depth.clear_input);
    // Velocity：深度只读 0x17。
    let v = &l.submissions[5];
    assert_eq!(v.accesses.last().unwrap().access, 0x17);
}

#[test]
fn main_pass_skips_forward_subpasses_when_shader_refactoring_is_not_disabled() {
    use super::main_pass::main_pass;
    let rig = Rig::new();
    run(&rig, |c| {
        main_pass(c, &gbuf(), &tex(13, 8, 8), &DrawLists::new(), false)
    });
    assert_eq!(
        names(&rig),
        vec!["G-buffer", "G-buffer emissive", "G-buffer velocity", "Velocity"]
    );
}

#[test]
fn main_pass_items_get_stencil_ref_and_stencil_write_state() {
    use super::main_pass::main_pass;
    let rig = Rig::new();
    let mut lists = DrawLists::new();
    lists.list_mut(list::GBUFFER).push(item(1, 0, Some(0x42)));
    lists.list_mut(list::VELOCITY).push(item(2, 0, None)); // Velocity 不读 stencil_ref
    run(&rig, |c| main_pass(c, &gbuf(), &tex(13, 8, 8), &lists, false));
    let l = rig.log.borrow();
    let d = &l.submissions[0].draws[0];
    assert_eq!(d.unknown_b1, 0x42);
    assert_eq!(d.state.0, with_stencil_write(0x8000_0080));
    let v = &l.submissions[3].draws[0];
    assert_eq!(v.state.0, velocity_state(0x8000_0080));
    assert_eq!(v.unknown_b1, 0);
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn main_pass_item_without_stencil_ref_panics() {
    use super::main_pass::main_pass;
    let rig = Rig::new();
    let mut lists = DrawLists::new();
    lists.list_mut(list::GBUFFER).push(item(1, 0, None));
    run(&rig, |c| main_pass(c, &gbuf(), &tex(13, 8, 8), &lists, false));
}

#[test]
fn oc2_has_five_subpasses_without_a_forward_gate_and_loads_everything() {
    use super::main_pass::main_pass_oc2;
    let rig = Rig::new();
    run(&rig, |c| main_pass_oc2(c, &gbuf(), &tex(13, 8, 8), &DrawLists::new()));
    assert_eq!(
        names(&rig),
        vec![
            "G-buffer OC/2",
            "G-buffer emissive OC/2",
            "G-buffer velocity OC/2",
            "Forward OC/2",
            "Velocity OC/2"
        ]
    );
    let l = rig.log.borrow();
    // 没有任何清除；第一个子 pass 的颜色是 tag0（0x36）。
    assert!(l.created_passes.iter().all(|p| !p.depth.clear_input));
    assert_eq!(l.submissions[0].accesses[0].access, 0x36);
    assert_eq!(l.submissions[4].accesses.last().unwrap().access, 0x17);
}

#[test]
fn late_passes_are_skipped_when_their_list_is_empty() {
    use super::main_pass::late_forward_and_deferred_pass;
    let rig = Rig::new();
    let mut lists = DrawLists::new();
    run(&rig, |c| late_forward_and_deferred_pass(c, &gbuf(), &lists));
    assert!(names(&rig).is_empty());
    lists.list_mut(list::LATE_FORWARD).push(item(1, 0, Some(1)));
    run(&rig, |c| late_forward_and_deferred_pass(c, &gbuf(), &lists));
    assert_eq!(names(&rig), vec!["Late Forward"]);
    lists.list_mut(list::LATE_DEFERRED).push(item(1, 0, Some(1)));
    let rig2 = Rig::new();
    run(&rig2, |c| late_forward_and_deferred_pass(c, &gbuf(), &lists));
    assert_eq!(names(&rig2), vec!["Late Deferred", "Late Forward"]);
}

#[test]
fn shadow_pass_runs_three_cascades_with_layer_clear_and_variant_one() {
    use super::shadow_pass::shadow_pass;
    let mut rig = Rig::new();
    rig.extra_presets.push((0x11, tex(50, 2048, 2048)));
    let mut lists = DrawLists::new();
    lists.list_mut(0).push(item(1, 0, None));
    run(&rig, |c| shadow_pass(c, &lists));
    assert_eq!(names(&rig), vec!["Shadows", "Shadows", "Shadows"]);
    let l = rig.log.borrow();
    for (i, s) in l.submissions.iter().enumerate() {
        assert_eq!(s.color_count, 0);
        assert_eq!(s.accesses[0].access, 0x2f);
        let d = &s.draws[0];
        assert_eq!(d.variant, 1);
        // `and byte0, 0xcf`：默认 0x8000_0080 → 0x8000_0080（bit4/5 本就为 0）。
        assert_eq!(d.state.0 & 0x30, 0);
        let _ = i;
    }
    assert!(l.created_passes.iter().all(|p| p.depth.clear_input));
}

#[test]
fn transparency_passes_names_lists_and_state() {
    use super::transparency_pass::*;
    let rig = Rig::new();
    let mut lists = DrawLists::new();
    lists.list_mut(REFRACTION_LIST).push(item(1, 0, None));
    lists.list_mut(PRE_TAA_LIST).push(item(1, 0, None));
    // post_taa 空列表 → 不创建 pass。
    run(&rig, |c| post_taa_transparency_pass(c, &tex(1, 8, 8), &lists));
    assert!(names(&rig).is_empty());
    run(&rig, |c| {
        refraction_copy_pass(c, &tex(1, 8, 8), &tex(2, 8, 8), &tex(3, 8, 8), &lists)
    });
    run(&rig, |c| {
        pre_taa_transparency_pass(c, &tex(1, 8, 8), &tex(3, 8, 8), &tex(4, 8, 8), &tex(5, 8, 8), &lists)
    });
    lists.list_mut(POST_TAA_LIST).push(item(1, 0, None));
    run(&rig, |c| post_taa_transparency_pass(c, &tex(1, 8, 8), &lists));
    assert_eq!(
        names(&rig),
        vec![
            "Copy refraction target",
            "Pre-TAA transparency",
            "Post-TAA transparency"
        ]
    );
    let l = rig.log.borrow();
    // refraction：颜色 tag1(0x2e) + 深度只读(0x17)；`byte0 &= 0xbf`，无 byte3 改写。
    assert_eq!(l.submissions[0].accesses[0].access, 0x2e);
    assert_eq!(l.submissions[0].accesses[1].access, 0x17);
    assert_eq!(l.submissions[0].draws[0].state.0, 0x8000_0080 & !0x40);
    // pre_taa / post_taa：同时 byte3 字段 = 0b10。
    let expect = (0x8000_0080u32 & !0x40 & !0x6000_0000) | 0x4000_0000;
    assert_eq!(l.submissions[1].draws[0].state.0, expect);
    assert_eq!(l.submissions[2].draws[0].state.0, expect);
    // post_taa 无深度：只有颜色访问。
    assert!(
        !l.submissions[2]
            .accesses
            .iter()
            .any(|a| a.access == 0x17 || a.access == 0x2f)
    );
}

#[test]
fn water_and_ice_plans_are_selected_by_season() {
    use super::transparency_pass::*;
    use crate::resources::session::GladeType;
    assert_eq!(WATER_PLAN.ops.len(), WATER_OPS.len());
    assert!(WATER_OPS.iter().any(|o| o.kind == PlanKind::DispatchIndirect));
    assert_eq!(WATER_PLAN.compute_passes.len(), 2);
    assert_eq!(ICE_PLAN.compute_passes.len(), 4);
    assert!(ICE_OPS.len() > WATER_OPS.len());
    assert_eq!(surface_plan_for(GladeType::Winter).kind, WaterSurfaceKind::Ice);
    assert_eq!(surface_plan_for(GladeType::Flowery).kind, WaterSurfaceKind::Water);
    // 机器码里水 pass 的首个哈希绑定：`0x2a333c3a28028230`（与 pre_taa 的 B 绑定同名）。
    assert!(WATER_OPS.iter().any(|o| o.key == Some(PRE_TAA_B_BINDING)));
}

#[test]
fn custom_cursor_gates_on_visibility_and_saturates_the_window_size() {
    use super::custom_cursor_pass::*;
    assert_eq!(window_uniform(1920, 1080), [1920, 1080]);
    // u32 → f32 → u32 的往返：2^24 以上会丢精度（f32 尾数 24 位）。
    assert_eq!(window_uniform(16_777_217, 0), [16_777_216, 0]);
    assert_eq!(window_uniform(u32::MAX, 1), [u32::MAX, 1]);
    let rig = Rig::new();
    run(&rig, |c| custom_cursor_pass(c, &tex(1, 64, 64), (64, 64), false));
    assert!(names(&rig).is_empty());
    run(&rig, |c| custom_cursor_pass(c, &tex(1, 64, 64), (64, 64), true));
    let l = rig.log.borrow();
    assert_eq!(l.submissions.len(), 1);
    assert_eq!(l.submissions[0].name, "Custom cursor");
    let d = &l.submissions[0].draws[0];
    assert_eq!(d.kind, DrawKind::NoVao { vertex_count: 6 });
    assert_eq!(d.head[0], 0);
    assert_eq!(
        d.state.0,
        overlay_state(crate::render::pass_builder::draw_stream::RasterState(0x8000_0080)).0
    );
}

#[test]
fn custom_cursor_without_shader_panics_like_expect() {
    use super::custom_cursor_pass::custom_cursor_pass;
    struct None_;
    impl ShaderLookup for None_ {
        fn raster_handle_by_name(&self, _: &str) -> Option<Handle<RasterShaderProgram>> {
            None
        }
    }
    let rig = Rig::new();
    let mut world = World::new();
    let mut factory = || rig.host();
    let s = None_;
    let r = std::panic::catch_unwind(core::panic::AssertUnwindSafe(|| {
        let mut ctx = DrawPassCtx::new(&mut world, Duration::ZERO, &mut factory, &s);
        custom_cursor_pass(&mut ctx, &tex(1, 8, 8), (8, 8), true);
    }));
    assert!(r.is_err());
}

#[test]
fn feedback_report_bg_gates_and_draws_one_fullscreen_triangle() {
    use super::feedback_report_bg::*;
    let rig = Rig::new();
    let report = FeedbackReportBg {
        visible: 1,
        page: 6,
        texture: ManagedHandle(5),
        texture_desc: tex(30, 32, 32),
    };
    run(&rig, |c| render_feedback_report_bg(c, &tex(1, 64, 64), 7, &report)); // 状态 != 8
    let mut hidden = report.clone();
    hidden.visible = 0;
    run(&rig, |c| render_feedback_report_bg(c, &tex(1, 64, 64), 8, &hidden));
    let mut late = report.clone();
    late.page = 7;
    run(&rig, |c| render_feedback_report_bg(c, &tex(1, 64, 64), 8, &late));
    assert!(names(&rig).is_empty());
    // 正常路径：Raw 纹理绑定需要 `get_vk_image_view`（MockHost 返回 Ok）。
    run(&rig, |c| render_feedback_report_bg(c, &tex(1, 64, 64), 8, &report));
    let l = rig.log.borrow();
    assert_eq!(l.submissions.len(), 1);
    let s = &l.submissions[0];
    assert_eq!(s.name, "Feedback report bg");
    assert_eq!(s.accesses[0].access, 0x2e); // tag2
    assert_eq!(s.draws[0].kind, DrawKind::NoVao { vertex_count: 3 });
    assert_eq!(s.draws[0].state.0 & 0x80, 0);
}

#[test]
fn logo_overlay_draws_six_vertices_with_overlay_state() {
    use super::logo_overlay::*;
    let mut rig = Rig::new();
    rig.shaders.push(3);
    run(&rig, |c| render_logo_overlay(c, &tex(1, 800, 600)));
    let l = rig.log.borrow();
    assert_eq!(l.submissions[0].name, "Logo overlay");
    assert_eq!(l.submissions[0].draws[0].kind, DrawKind::NoVao { vertex_count: 6 });
    LogoOverlayPlugin.build();
}

#[test]
fn foliage_exclusion_mask_clear_only_then_draw_paths() {
    use super::foliage_exclusion_mask::*;
    use core::sync::atomic::Ordering;
    let mut rig = Rig::new();
    rig.extra_presets.push((MASK_PRESET, tex(60, 256, 256)));
    rig.extra_presets.push((PRESET_A, tex(61, 4, 4)));
    rig.extra_presets.push((PRESET_B, tex(62, 4, 4)));
    FOLIAGE_EXCLUSION_DIRTY.store(false, Ordering::Relaxed);
    // 无资源 + 全局为 0 → 什么都不做。
    run(&rig, |c| rasterize_foliage_exclusion_mask(c, None));
    assert!(names(&rig).is_empty());
    // 有资源 → 画，全局置 1。
    let stairs = StairsFoliageExclusion {
        buffer: ManagedHandle(9),
        vertex_count: 24,
    };
    let mut host_vk = Rig::new();
    host_vk.extra_presets = rig.extra_presets.clone();
    // bind_external_storage_buffer 需要 vk 缓冲：给宿主登记（工厂里每个宿主都带）。
    let log = host_vk.log.clone();
    let mut world = World::new();
    let mut factory = || {
        let (mut h, _) = MockHost::new();
        h.log = log.clone();
        let mut r = ShaderReflection::default();
        r.program_name = "stairs";
        h.shaders.insert(4, Arc::new(r));
        h.rg_buffers.insert(ColorLibrarySsbo::rg_buffer_hash(), 1);
        h.vk_buffers.insert(9, 0xabc);
        h.presets.insert(MASK_PRESET, tex(60, 256, 256));
        h.presets.insert(PRESET_A, tex(61, 4, 4));
        h.presets.insert(PRESET_B, tex(62, 4, 4));
        h.boxed()
    };
    let shaders = Shaders;
    {
        let mut ctx = DrawPassCtx::new(&mut world, Duration::ZERO, &mut factory, &shaders);
        rasterize_foliage_exclusion_mask(&mut ctx, Some(&stairs));
        assert!(FOLIAGE_EXCLUSION_DIRTY.load(Ordering::Relaxed));
        // 资源消失 + 全局为 1 → 只清空（无 draw），全局回 0。
        rasterize_foliage_exclusion_mask(&mut ctx, None);
        assert!(!FOLIAGE_EXCLUSION_DIRTY.load(Ordering::Relaxed));
    }
    let l = log.borrow();
    assert_eq!(l.submissions.len(), 2);
    assert_eq!(l.submissions[0].draws[0].kind, DrawKind::NoVao { vertex_count: 24 });
    assert!(l.submissions[1].draws.is_empty());
    assert_eq!(l.submissions[1].name, "Foliage exclusion");
    assert_eq!(l.submissions[1].accesses[0].access, 0x2e);
}

#[test]
fn object_grass_push_draws_one_quad_per_vec4_and_caps_at_thirteen() {
    use super::object_grass_push::*;
    use glam::{Vec3, Vec4};
    let mut rig = Rig::new();
    rig.extra_presets.push((TARGET_PRESET, tex(70, 128, 128)));
    let objs = vec![
        Vec4::new(1.0, 2.0, 3.0, RADIUS_BASE),
        cursor_push(true, Vec3::ONE, RADIUS_9),
    ];
    run(&rig, |c| object_grass_push_pass(c, &objs));
    {
        let l = rig.log.borrow();
        assert_eq!(l.submissions[0].name, "Grass object push pass");
        assert_eq!(l.submissions[0].draws.len(), 2);
        assert_eq!(l.submissions[0].accesses[0].access, 0x2e);
    }
    assert_eq!(cursor_push(false, Vec3::ONE, 9.0), Vec4::new(0.0, 0.0, 0.0, 9.0));
    assert_eq!(RADIUS_NEAR, 7.125);
    assert_eq!(collect_push_objects(objs.clone(), [Vec4::ZERO]).len(), 3);
    let too_many = vec![Vec4::ZERO; MAX_PUSH_OBJECTS + 1];
    let r = std::panic::catch_unwind(core::panic::AssertUnwindSafe(|| {
        run(&rig, |c| object_grass_push_pass(c, &too_many))
    }));
    assert!(r.is_err());
}

#[test]
fn ui_2d_pass_sorts_by_order_picks_shader_by_tint_and_draws_ws_after() {
    use super::ui_2d_pass::*;
    let rig = Rig::new();
    let img = |n: &str| Ui2dImageRef {
        name: n.into(),
        layer: 2,
    };
    let el = |order: Option<u64>, tint: bool| Ui2dDrawElement {
        extent: [0.0, 0.0, 1.0, 1.0],
        image: img("a"),
        icon: tint.then_some(CustomizationIconRead { alpha: 0.5, flag: true }),
        interaction: UiInteractionStateRead { flag: true },
        order,
        inactive: false,
    };
    // 排序：None < Some；同键稳定。
    let elements = vec![
        el(Some(3), true),
        el(None, false),
        el(Some(3), false),
        el(Some(1), true),
    ];
    let ws = vec![Ui2dWsElement {
        extent: glam::Vec2::ONE,
        position: glam::Vec3::ZERO,
        image: img("b"),
    }];
    // 图像绑定是 `TextureFromDisk`：登记库里的纹理。
    let log = rig.log.clone();
    let mut world = World::new();
    let mut factory = || {
        let (mut h, _) = MockHost::new();
        h.log = log.clone();
        for id in 6..=8u64 {
            h.shaders.insert(id, Arc::new(ShaderReflection::default()));
        }
        h.rg_buffers.insert(ColorLibrarySsbo::rg_buffer_hash(), 1);
        for name in ["a", "b"] {
            h.textures_by_name.insert(
                crate::name_hash::NameHash::compute(name).0,
                (ManagedHandle(1), tex(80, 16, 16)),
            );
        }
        h.boxed()
    };
    let shaders = Shaders;
    let mut ctx = DrawPassCtx::new(&mut world, Duration::ZERO, &mut factory, &shaders);
    ui_2d_pass(&mut ctx, &tex(1, 1280, 720), (1280, 720), &elements, &ws);
    let l = log.borrow();
    assert_eq!(l.submissions.len(), 1);
    let s = &l.submissions[0];
    assert_eq!(s.name, "UI 2D");
    assert_eq!(s.draws.len(), 5);
    // 排序后顺序：None(color button=7)、Some(1)(ui_2d=6)、Some(3)×2（稳定：先 tint=6 再 button=7），最后 ws(8)。
    let shaders: Vec<u64> = s.draws.iter().map(|d| d.shader.0.0).collect();
    assert_eq!(shaders, vec![7, 6, 6, 7, 8]);
    assert!(s.draws.iter().all(|d| d.kind == DrawKind::NoVao { vertex_count: 6 }));
    // 没有 bind_common：不会绑定颜色库缓冲。
    assert!(s.buffer_slots.is_empty());
    assert_eq!(MEI_TEMP_FORMAT, 0x2b);
}

#[test]
fn constants_decoded_from_negative_immediates() {
    use super::ui_2d_pass::*;
    assert_eq!(U_POS_B, (-0x590372c7d4be463bi64) as u64);
    assert_eq!(U_ALPHA, (-0x154d5a253117dd2ai64) as u64);
    assert_eq!(U_WS_FLAG, (-0x3a5498c04f79ae0bi64) as u64);
    assert_eq!(MEI_FONT_BINDING, (-0x7ea40afc96996874i64) as u64);
    assert_eq!(MEI_TEXTPRIM_BINDING, (-0x284141834374fd41i64) as u64);
    assert_eq!(MEI_SHAPESTREAM_BINDING, (-0x6ed09d503dcccc4di64) as u64);
    assert_eq!(
        super::custom_cursor_pass::CURSOR_TEXTURE_FILE,
        (-0x60eb2eb433b1880i64) as u64
    );
    assert_eq!(
        super::custom_cursor_pass::CURSOR_TEXTURE_BINDING,
        (-0x60f2dd711c245bd7i64) as u64
    );
    assert_eq!(
        super::foliage_exclusion_mask::STAIRS_BUFFER_BINDING,
        (-0x6d6cf4f29e010e92i64) as u64
    );
    assert_eq!(
        super::transparency_pass::REFRACTION_BINDING,
        (-0x2b7415ec6ae3e224i64) as u64
    );
    assert_eq!(
        super::d_main_support::BIND_COMMON_FLAG_INT_UNIFORM,
        (-0x3a5498c04f79ae0bi64) as u64
    );
    use super::custom_cursor_pass::CustomCursorSsbo;
    assert_eq!(CustomCursorSsbo::rg_buffer_hash(), (-0x2f0f7d81800dd867i64) as u64);
    assert_eq!(
        super::ui_2d_pass::TextPrimSsbo::rg_buffer_hash(),
        (-0x1665dcb6219b5b0i64) as u64
    );
}

#[test]
fn mei_pass_draws_text_then_combines_into_the_target() {
    use super::ui_2d_pass::*;
    let rig = Rig::new();
    let text = MeiTextOutput {
        prim_count: 4,
        atlas: ManagedHandle(5),
        atlas_desc: tex(90, 512, 512),
    };
    let mut rig = rig;
    rig.shaders = (1..=9).collect();
    let log = rig.log.clone();
    let mut world = World::new();
    let mut factory = || {
        let (mut h, _) = MockHost::new();
        h.log = log.clone();
        for id in 1..=9u64 {
            h.shaders.insert(id, Arc::new(ShaderReflection::default()));
        }
        h.rg_buffers.insert(ColorLibrarySsbo::rg_buffer_hash(), 1);
        h.rg_buffers.insert(TextPrimSsbo::rg_buffer_hash(), 2);
        h.rg_buffers.insert(ShapeStreamSsbo::rg_buffer_hash(), 3);
        h.textures_by_name
            .insert(MEI_FONT_FILE, (ManagedHandle(1), tex(80, 16, 16)));
        h.boxed()
    };
    let shaders = Shaders;
    let mut ctx = DrawPassCtx::new(&mut world, Duration::ZERO, &mut factory, &shaders);
    mei_pass(&mut ctx, &tex(1, 1280, 720), &tex(2, 1280, 720), (1280, 720), &text);
    let l = log.borrow();
    assert_eq!(l.submissions.len(), 2);
    assert_eq!(l.submissions[0].name, "Mei 2D");
    assert_eq!(l.submissions[0].draws[0].kind, DrawKind::NoVao { vertex_count: 24 });
    assert_eq!(l.submissions[1].name, "UI shadow combine");
    assert_eq!(l.submissions[1].draws[0].kind, DrawKind::NoVao { vertex_count: 3 });
}

#[test]
fn resolve_to_swapchain_picks_msaa_shader_by_sample_count() {
    use super::final_post::*;
    assert_eq!(resolve_shader_name(1), "raster_resolve_to_swapchain");
    assert_eq!(resolve_shader_name(4), "raster_resolve_msaa_to_swapchain");
    assert_eq!("raster_resolve_to_swapchain".len(), 27);
    assert_eq!("raster_resolve_msaa_to_swapchain".len(), 32);
    let rig = Rig::new();
    let mut src = tex(5, 640, 360);
    src.sample_count = 4;
    run(&rig, |c| resolve_to_swapchain(c, &src, &tex(6, 640, 360), 1));
    let l = rig.log.borrow();
    assert_eq!(l.submissions[0].name, "Resolve to swapchain");
    assert_eq!(l.submissions[0].draws[0].kind, DrawKind::NoVao { vertex_count: 3 });
    assert_eq!(final_post_uniform_writes(), 12);
}

#[test]
fn deferred_raster_tables_match_the_material_ids() {
    use super::deferred_raster::*;
    use super::material_id::MaterialId;
    for m in MaterialId::DEFERRED {
        assert!(shader_uri(m).unwrap().starts_with("/deferred/"));
    }
    assert_eq!(shader_uri(MaterialId::Forward), None);
    assert_eq!(materials().len(), 4);
    assert_eq!(PASS_NAME.len(), 0x17);
    // 条目下标覆盖 0..=3 的四个材质（其中 Solid 出现两次：flag false / true）。
    let mut seen = ENTRIES.map(|e| e.0);
    seen.sort();
    assert_eq!(seen, [0, 0, 1, 2, 3]);
    assert_eq!(ENTRIES.iter().filter(|e| e.1).count(), 1);
}

#[test]
fn deferred_entry_and_final_state_masks_match_the_machine_code() {
    use super::deferred_raster::*;
    // flag false：byte0 &= 0xbf，byte3 不变，模板 = material
    assert_eq!(entry_state(0x8000_00c0, 1, false), (0x8000_0080, 1));
    // flag true：byte3 = (b3 & 0x9f) | 0x20，模板 = material | 0x80
    assert_eq!(entry_state(0x8000_0080, 0, true), (0xa000_0080, 0x80));
    assert_eq!(entry_state(0xffff_ffff, 2, true), (0xbfff_ffbf, 0x82));
    // 收尾：byte0 &= 0x7f，byte1 |= 1，byte2 = (b2 & 3) | 0xe8，byte3 = (b3 & 0xe0) | 0x10
    assert_eq!(final_state(0x8000_0080), 0x90e8_0100);
    assert_eq!(final_state(0xffff_ffff), 0xf0eb_ff7f);
}

fn deferred_inputs<'a>() -> super::deferred_raster::DeferredInputs<'a> {
    let h = |id| Some(Handle(ManagedHandle(id), core::marker::PhantomData));
    super::deferred_raster::DeferredInputs {
        shaders: [h(1), h(2), h(3), h(4), h(5)],
        lighting: [tex(10, 640, 360), tex(11, 640, 360)],
        depth: tex(12, 640, 360),
        arg5: tex(13, 4, 4),
        arg6: 1,
        arg7: tex(14, 4, 4),
        arg8: tex(15, 4, 4),
        gbuffer: [tex(16, 4, 4), tex(17, 4, 4), tex(18, 4, 4)],
        arg10: tex(19, 4, 4),
        arg11: tex(20, 4, 4),
        arg12: tex(21, 4, 4),
        rt_buffers: [30, 31],
        _marker: core::marker::PhantomData,
    }
}

fn deferred_rig() -> Rig {
    use super::deferred_raster::*;
    let mut rig = Rig::new();
    for p in [0u8, 8, 9, 0xa, 0xc] {
        rig.extra_presets.push((p, tex(100 + p as u32, 4, 4)));
    }
    rig.extra_files.push(DISK_FILE);
    rig.extra_buffers.push((LanternBufferHash::get(), 40));
    rig.extra_buffers.push((SpatialHash::get(), 41));
    for (h, id) in [(0xC001u64, 11u32), (0xC002, 12), (0xC003, 13), (0xC004, 14)] {
        rig.extra_buffers.push((h, id));
    }
    rig.extra_words.push((LanternBufferHash::get(), 7));
    rig.extra_words.push((0xC001, 0x11));
    rig.extra_words.push((0xC003, 0x33));
    rig
}

struct RtTri;
struct RtMeta;
struct RtBvhT;
struct WallT;
impl SsboType for RtTri {
    fn rg_buffer_hash() -> u64 {
        0xC001
    }
}
impl SsboType for RtMeta {
    fn rg_buffer_hash() -> u64 {
        0xC002
    }
}
impl SsboType for RtBvhT {
    fn rg_buffer_hash() -> u64 {
        0xC003
    }
}
impl SsboType for WallT {
    fn rg_buffer_hash() -> u64 {
        0xC004
    }
}

struct LanternBufferHash;
impl LanternBufferHash {
    fn get() -> u64 {
        use super::deferred_raster::LanternBuffer;
        LanternBuffer::rg_buffer_hash()
    }
}
struct SpatialHash;
impl SpatialHash {
    fn get() -> u64 {
        use super::deferred_raster::SpatialResolveOffsetsBuffer;
        SpatialResolveOffsetsBuffer::rg_buffer_hash()
    }
}

#[test]
fn deferred_raster_runs_five_entries_with_stencil_state_and_one_pass() {
    use super::deferred_raster::*;
    let rig = deferred_rig();
    run(&rig, |c| {
        deferred_raster::<RtTri, RtMeta, RtBvhT, WallT>(c, &deferred_inputs())
    });
    let l = rig.log.borrow();
    assert_eq!(l.submissions.len(), 1);
    let s = &l.submissions[0];
    assert_eq!(s.name, PASS_NAME);
    assert_eq!(s.color_count, 1);
    // 颜色 tag1（访问 0x2e）；深度可写在其后。
    assert_eq!(s.accesses[0].access, 0x2e);
    assert_eq!(s.draws.len(), 5);
    let shaders: Vec<u64> = s.draws.iter().map(|d| d.shader.0.0).collect();
    // 条目顺序 (1,3,0,0*,2) → 着色器数组 [1,2,3,4,5] 的 [1],[3],[0],[0],[2]
    assert_eq!(shaders, vec![2, 4, 1, 1, 3]);
    let stencils: Vec<u8> = s.draws.iter().map(|d| d.unknown_b1).collect();
    assert_eq!(stencils, vec![1, 3, 0, 0x80, 2]);
    for d in &s.draws {
        assert_eq!(d.head[0], 0);
        assert_eq!(d.variant, 0);
        assert_eq!(d.kind, DrawKind::NoVao { vertex_count: 3 });
    }
    // MockHost 默认状态 0x8000_0080 经 entry_state + final_state。
    for (d, (m, f)) in s.draws.iter().zip(ENTRIES) {
        assert_eq!(d.state.0, final_state(entry_state(0x8000_0080, m, f).0));
    }
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn deferred_raster_panics_when_a_material_shader_is_missing() {
    let rig = deferred_rig();
    let mut i = deferred_inputs();
    i.shaders[1] = None;
    run(&rig, |c| {
        super::deferred_raster::deferred_raster::<RtTri, RtMeta, RtBvhT, WallT>(c, &i)
    });
}

fn fp_args(src_w: u32, lut: super::final_post::LutSelect) -> super::final_post::FinalPostArgs {
    use super::final_post::*;
    FinalPostArgs {
        out_extent: (1000, 500),
        src0: tex(1, src_w, 10),
        src1: tex(2, 8, 8),
        flag94: 1,
        lut,
        arg_f0: 0.25,
        arg_f1: 0.5,
        shader: Some(Handle(ManagedHandle(3), core::marker::PhantomData)),
        cfg: GradingConfig {
            named: true,
            pair: [2.0, 3.0],
            floats: GradingFloats {
                f30: 0.1,
                f34: 0.2,
                f38: 0.3,
            },
        },
        theme: 19,
        custom_lut: None,
    }
}

#[test]
fn final_post_scale_class_and_env_packing() {
    use super::final_post::*;
    assert_eq!(scale_class(940, 1000), 4); // 0.94 < 0.95
    assert_eq!(scale_class(950, 1000), 1);
    assert_eq!(scale_class(1049, 1000), 1);
    assert_eq!(scale_class(1050, 1000), 2);
    let e = build_env(&fp_args(
        1000,
        LutSelect::Custom {
            name: "warm".into(),
            blend: 0.75,
            flag: 1,
        },
    ));
    assert_eq!((e.mode, e.blend, e.lut_flag), (0, 0.75, 1));
    assert_eq!(e.lut_name.as_deref(), Some("warm"));
    assert_eq!(e.grading, [2.0, 3.0]);
    assert!(e.theme_is_19);
    assert_eq!(e.extent, [1000, 500]);
    let k = build_env(&fp_args(1000, LutSelect::Keep));
    assert_eq!((k.mode, k.blend, k.lut_flag, k.lut_name.clone()), (1, 0.0, 0, None));
    assert_eq!(k.grading, [2.0, 3.0]);
    let d = build_env(&fp_args(1000, LutSelect::Default));
    assert_eq!((d.mode, d.grading), (3, [1.0, 1.0]));
    // 未命名的当前档 → (1, 1)
    let mut a = fp_args(1000, LutSelect::Keep);
    a.cfg.named = false;
    assert_eq!(build_env(&a).grading, [1.0, 1.0]);
    a.theme = 18;
    assert!(!build_env(&a).theme_is_19);
}

#[test]
fn final_post_runs_one_blit_pass_with_tag2_color_and_fullscreen_triangle() {
    use super::final_post::*;
    let mut rig = Rig::new();
    rig.extra_files.push(DISK_FILE);
    rig.extra_files.push(LUT_FIXED_FILE);
    rig.extra_files.push(lut_asset_hash(DEFAULT_LUT_NAME));
    rig.extra_files.push(lut_asset_hash("warm"));
    rig.extra_buffers.push((ShadowMaskMeanBuffer::rg_buffer_hash(), 50));
    let out = msaa_color_desc((1000, 500), 77);
    assert_eq!((out.format, out.sample_count), (0x2b, 4));
    run(&rig, |c| {
        final_post(
            c,
            &fp_args(
                1100,
                LutSelect::Custom {
                    name: "warm".into(),
                    blend: 0.5,
                    flag: 0,
                },
            ),
            &out,
        )
    });
    let l = rig.log.borrow();
    assert_eq!(l.submissions.len(), 1);
    let s = &l.submissions[0];
    assert_eq!(s.name, "Final blit");
    assert_eq!(s.accesses[0].access, 0x2e); // tag2：与 tag1 同访问类型
    assert_eq!(s.draws[0].shader.0.0, 3);
    assert_eq!(s.draws[0].kind, DrawKind::NoVao { vertex_count: 3 });
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn final_post_panics_without_the_shader_resource() {
    use super::final_post::*;
    let rig = Rig::new();
    let mut a = fp_args(1000, LutSelect::Keep);
    a.shader = None;
    run(&rig, |c| final_post(c, &a, &msaa_color_desc((1000, 500), 1)));
}

#[test]
fn common_params_time_follows_duration_as_secs_f32() {
    let rig = Rig::new();
    run(&rig, |c| {
        let p = c.common_params(true);
        assert_eq!(p.time, 2.5);
        assert!(p.flag.is_set());
    });
}
