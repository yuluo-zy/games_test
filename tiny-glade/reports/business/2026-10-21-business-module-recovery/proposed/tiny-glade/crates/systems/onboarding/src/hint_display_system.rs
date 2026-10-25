//! hint_display_system（O：动画/冷却/音效/删除/位置/高亮/绘制）。
//!
//! 恢复（evidence/batch2-display.txt）：
//! - `hint_animation_system` 0x141c4f400：逐提示逐属性 `value=clamp(value+dt/0.3*dir,
//!   0,1)`（dir=±1，删除中 -1）、`eased=EasingFunction::apply(value)`、`time+=dt`。
//! - `hint_deletion_cooldown_system` 0x141c4f790：`timer-=dt`，`<=0` →
//!   remove<HintDeleteAfterCooldown> + insert<HintMarkedForDeletion>（warn）。
//! - `hint_audio` 0x141c4fb00：动画 `+4`(eased)==1.0 → PlayUiSound tag 0x15；
//!   `+8`(time)==0.0 → tag 0x14。
//! - `hint_deletion_system` 0x141c4fea0：动画 `+4`(eased)==0.0 → despawn。
//! - `progress_bar_update_system` 0x141c530f0：按 id 0xa/0xb/0xc 取 CameraHintProgress
//!   量化档位（0 / 1 / 2 / 3，阈值 0.4/0.99），档位变 → PlayUiSound tag 0x16。
//! 未实现（分歧点见各函数）：`hint_highlight_shape`（HighlightRoofCmd/HighlightWallCmd
//! 未落库）、`hint_position_update`（HintPlacement 跳转表 7/8 依赖相机矩阵/几何）、
//! `hint_draw`/`render_hints`（mei/toolbar 批次 3）。

use bevy_ecs::prelude::*;
use glam::Vec2;
use country_core::resources::time::GameTime;
use country_core::systems::history::replay_mode::time_continuous_lerp;
use country_core::systems::ui::resources::widget_params::PlayUiSound;
use utils::easing_functions::EasingFunction;

use crate::{BindingHintDisplay, HintPlacement, HintProgressBar, HintType};

/// 提示动画（O：HintAnimation，每属性 {value f32@+0, eased f32@+4, time f32@+8}，
/// 步长 0xc；属性数组结构未恢复，本重建取单属性）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct HintAnimation {
    /// 归一化进度 [0,1]
    pub value: f32,
    /// 缓动值
    pub eased: f32,
    /// 存活时间
    pub time: f32,
}

/// 提示弹簧（O：HintSpring{pos, target, initialized}；position_update 写，
/// initialized@+8）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct HintSpring {
    pub pos: glam::Vec2,
    pub target: glam::Vec2,
    pub initialized: bool,
}

/// 提示文本（O：HintText；lang_lib lookup 载荷）。
#[derive(Component, Clone, Debug, Default, PartialEq)]
pub struct HintText {
    pub text: String,
}

/// 删除冷却标记（O：HintDeleteAfterCooldown，f32 计时）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct HintDeleteAfterCooldown {
    pub timer: f32,
}

/// 待删除标记（O：HintMarkedForDeletion，unit）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct HintMarkedForDeletion;

/// 冷却表 DAT_143038ae8（按提示 id 索引，O 读 80B）：[0]=0 [1]=0.3 [2]=0 [3]=0
/// 冷却表 DAT_143038ae8（按提示 id 索引，fresh read_memory 2026-10-17：前 20 个
/// f32 = 80B，id 0..19；+0x50 起是另一张 u64 表 [8,9,4,15,16,12]，非本表）：
/// [0]=0 [1]=0.3 [2]=0 [3]=0 [4]=0.3 [5]=0.3 [6]=0.3 [7]=0.3 [8]=0 [9]=0
/// [10]=0.3 [11]=0.3 [12]=0.3 [13..19]=0。
/// id 18 必须可索引：prompt_rollercoaster 写 id@+0x49 = 0x12，orchestrator 用
/// `DAT_143038ae8[id]` 取冷却，故表长至少 19。
pub const DELETION_COOLDOWN_TABLE: [f32; 19] = [
    0.0, 0.3, 0.0, 0.0, 0.3, 0.3, 0.3, 0.3, 0.0, 0.0, 0.3, 0.3, 0.3, 0.0, 0.0, 0.0, 0.0, 0.0,
    0.0,
];

/// hint_animation_system（O 0x141c4f400）：`value = clamp(value + dt/0.3*dir, 0, 1)`，
/// `eased = EasingFunction::apply(value)`，`time += dt`；dir 删除中 -1（有
/// HintMarkedForDeletion）否则 +1。EasingFunction 变体未从反编译解析（I：取
/// CubicInOut，与 utils 恢复一致）。
pub fn hint_animation_system(
    time: Res<GameTime>,
    mut query: Query<(&mut HintAnimation, Option<&HintMarkedForDeletion>)>,
) {
    let dt = time.delta;
    for (mut anim, deleting) in &mut query {
        let dir = if deleting.is_some() { -1.0 } else { 1.0 };
        let mut value = anim.value + dt / 0.3 * dir;
        value = value.clamp(0.0, 1.0);
        anim.value = value;
        anim.eased = EasingFunction::CubicInOut.apply(value);
        anim.time += dt;
    }
}

/// hint_deletion_cooldown_system（O 0x141c4f790）：timer -= dt；<=0 →
/// remove<HintDeleteAfterCooldown> + insert<HintMarkedForDeletion>（warn）。
pub fn hint_deletion_cooldown_system(
    time: Res<GameTime>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut HintDeleteAfterCooldown)>,
) {
    let dt = time.delta;
    for (entity, mut cooldown) in &mut query {
        cooldown.timer -= dt;
        if cooldown.timer <= 0.0 {
            let mut cmd = commands.entity(entity);
            cmd.remove::<HintDeleteAfterCooldown>();
            cmd.insert(HintMarkedForDeletion);
        }
    }
}

/// hint_audio（O 0x141c4fb00）：**有 HintMarkedForDeletion 且** eased(+4)==1.0 →
/// PlayUiSound tag 0x15(21)；time(+8)==0.0 → tag 0x14(20)（无门控）。
/// 使用 core 的 24B PlayUiSound 业务载荷；实例 EventId 由 Bevy 管理。
pub fn hint_audio(
    mut sounds: EventWriter<PlayUiSound>,
    query: Query<(&HintAnimation, Option<&HintMarkedForDeletion>)>,
) {
    for (anim, deleting) in &query {
        if deleting.is_some() && anim.eased == 1.0 {
            // O：PlayUiSound tag 0x15（21），仅在满显后被标记删除（播放"关闭"音）。
            let _ = sounds.write(PlayUiSound {
                kind: 0x15,
                _payload: [0; 20],
            });
        }
        if anim.time == 0.0 {
            // O：PlayUiSound tag 0x14（20）。
            let _ = sounds.write(PlayUiSound {
                kind: 0x14,
                _payload: [0; 20],
            });
        }
    }
}

/// hint_deletion_system（O 0x141c4fea0）：有 HintMarkedForDeletion 且 eased==0.0 →
/// despawn。
pub fn hint_deletion_system(
    mut commands: Commands,
    mut query: Query<(Entity, &HintAnimation), With<HintMarkedForDeletion>>,
) {
    for (entity, anim) in &mut query {
        if anim.eased == 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

/// hint_highlight_shape（O 0x141c50120）：动画 eased(+4)>0.9 且 HintType∈{1,2}
/// （`*piVar1-1U<2` 与 `*piVar1-6` 的 `4<uVar6||uVar6==3` 对 {1,2}（u32 回绕）同时成立）
/// 且组件 +0x11!=0 → `PublicWalls::roof_entity` 有效 → 向 HighlightRoofCmd /
/// HighlightWallCmd 各写 tag 2（颜色判别 2）命令。
/// **重建（2026-10-17，HighlightRoofCmd/HighlightWallCmd 落库）**；分歧点：+0x11 字节
/// 语义 U（本重建省略该门控）、WallId 源 U（本重建取实体 id 近似）。
pub fn hint_highlight_shape(
    walls: Res<country_core::resources::walls::public_walls::PublicWalls>,
    mut roof_cmds: EventWriter<country_core::resources::roofs::commands::HighlightRoofCmd>,
    mut wall_cmds: EventWriter<country_core::systems::wall::wall_highlight::HighlightWallCmd>,
    query: Query<(&HintType, &HintAnimation, &HintPlacement)>,
) {
    for (hint_type, anim, _placement) in &query {
        if anim.eased > 0.9
            && matches!(hint_type, HintType::FocusedEdit | HintType::Undo)
        {
            // O：写面 {head=事件序号, wall, color=2}（0x18B 步长）。
            let wall = country_core::resources::walls::WallId(0); // U：WallId 源（实体 id）待补证
            if walls.roof_entity(wall).is_some() {
                roof_cmds.write(country_core::resources::roofs::commands::HighlightRoofCmd::new(
                    wall,
                    country_core::resources::walls::HighlightColor(2),
                ));
            }
            wall_cmds.write(country_core::systems::wall::wall_highlight::HighlightWallCmd {
                wall,
                ty: 2,
            });
        }
    }
}

/// hint_position_update（O 0x141c50510，1538B；fresh 反编译 + B 机器码 2026-10-17）。
///
/// SystemParam 顺序（PDB 单态化名）：`Query<(&HintLayoutMemo, &mut HintSpring,
/// &HintPlacement)>`, `Res<CursorPositionSS>`, `Res<WindowSize>`,
/// `Res<PreviewCircle>`, `Res<PreviewRectangle>`, `Res<MainCamera>`,
/// `Res<PublicWalls>`, `Query<&Roof>`, `Res<DecoratorStorage>`,
/// `Res<ClutterMeshes>`, `Res<CachedDecoratorTransforms>`,
/// `Res<UiSignifierStream>`, `Res<GameTime>`。
/// **`Res<ClutterMeshes>` 未列入**：country-core 尚未落 `resources::clutter_meshes`
/// 类型（登记 U）；该参数在函数体内未被读取，接入后不影响算术。
///
/// 逐语句：
/// * 边距 `margin20 = window.height / 1000 * 20`、`margin30 = window.height / 1000 * 30`
///   （`WindowSize::as_vec2` 返回 XMM0=width、XMM1=height；入口 DIVSS/MULSS 用 **XMM1**；
///   常量 1000.0/20.0/30.0 @0x14295b630/0x142a7fa30/0x1429cd034）；
/// * switch placement（6/7/8/10；其余走第二级跳转表 0x142f14844，U）：
///   - 6 Cursor：`(cursor.x + 2*margin20, cursor.y)`；
///   - 7 WorldPoint：`p = (preview_circle.center.x, Y, preview_circle.center.y)
///     + eye_right * (preview_circle.radius + 0.5)` → `world_space_to_screen_space`
///     → `+ (margin20, margin20)`（0.5 @0x142925870）；
///   - 8 Bounds：`Rectangle2d { axis: orientation, center, size: dimensions }
///     .as_points3(Y)` 四角逐个投影 → **取屏幕 x 最大的角（`ucomiss/seta` 严格大于，
///     并列保留先者）并携带该角的 y** → `+ (margin20, margin20)`；
///   - 10 ScreenCenter：`(width*0.5, height*0.5)`（0.5 广播 @0x1429258e0）；
/// * `delta = memo.size - memo.position`；`target = raw + delta`；
///   `upper = (window - memo.position_2) - margin30`；NaN-aware
///   `clamp(target, margin30, upper)`（maxps/minps + cmpunordps 选择）；
///   `final = clamped - delta`；
/// * `spring.pos = final`；未初始化 → `spring.target = final; initialized = true`；
///   已初始化 → `target.x/y = time_continuous_lerp(target, final, 10.0, dt)`
///   （0x140c904e0；rate 10.0 @0x1429345a8，dt = `GameTime.delta`@+0x28）。
///
/// **未实现（Y 载荷，reviewer FIX-3）**：7 读 `PreviewCircle+0x20`、8 读
/// `PreviewRectangle+0x28` 的放置点 Y（ui_create_circle 0x14165bed0 /
/// ui_create_rectangle 0x14155d410 把射线命中点 y 写入该偏移）；country-core 的 typed
/// preview 资源没有该字段（`height` 是 calc_shape_height 的 +0x40 计算高度）。本实现以
/// 0.0 占位，故 **case 7/8 标未实现**（6/10、case 8 的 x 最大角选择、夹取与 lerp 判等）；
/// 补字段后仅需把 0.0 换成该 Y。
/// 目标位置的四分量中后两分量在本函数内不被消费（只写 spring 的 Vec2 两路）。
pub fn hint_position_update(
    mut query: Query<(
        &crate::hint_orchestrator::HintLayoutMemo,
        &mut HintSpring,
        &HintPlacement,
    )>,
    cursor: Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,
    window: Res<country_core::resources::window::WindowSize>,
    preview_circle: Res<country_core::systems::wall::shapes::PreviewCircle>,
    preview_rectangle: Res<country_core::systems::wall::shapes::PreviewRectangle>,
    main_camera: Res<country_core::systems::camera_rig::main_camera::MainCamera>,
    _walls: Res<country_core::resources::walls::public_walls::PublicWalls>,
    _roofs: Query<&country_core::resources::roofs::roof_shape::Roof>,
    _decorator_storage: Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,
    _cached_transforms:
        Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,
    _signifier_stream:
        Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,
    time: Res<GameTime>,
) {
    let dims = window.as_vec2();
    // O（reviewer FIX-1）：`WindowSize::as_vec2` 返回 XMM0=width、XMM1=height；
    // 入口 `DIVSS XMM1,1000; MULSS {20,30}` 用的是 **height**。
    let margin20 = dims.y / 1000.0 * 20.0;
    let margin30 = dims.y / 1000.0 * 30.0;

    for (memo, mut spring, placement) in &mut query {
        let raw = match *placement {
            // O case 6（0x141c50acb）：`cursor + (2*margin20, 0)`。
            HintPlacement::Cursor => Vec2::new(cursor.0.x + margin20 + margin20, cursor.0.y),
            // O case 7（0x141c50dc6）：圆预览中心抬高 + 半径外推的 billboard 世界点。
            HintPlacement::WorldPoint(_) => {
                let camera = main_camera.camera_matrices();
                let right = camera.eye_right();
                // U：native `PreviewCircle+0x20` 放置点 Y（country-core 未建模）→ 0.0。
                let center = glam::Vec3::new(preview_circle.center.x, 0.0, preview_circle.center.y);
                let world = center + right * (preview_circle.radius + 0.5);
                country_core::utils::world_space_to_screen_space(
                    world,
                    dims.x,
                    dims.y,
                    &main_camera.read_matrix(0x80),
                    &main_camera.read_matrix(0x00),
                ) + Vec2::splat(margin20)
            }
            // O case 8（0x141c50af9）：矩形四角投影后取 **屏幕 x 最大**的角
            // （`ucomiss new.x, acc.x; seta` 严格大于，并列保留先者），并携带该角的 y。
            HintPlacement::Bounds(_) => {
                let rect = utils::geometry::rectangle::Rectangle2d {
                    axis: preview_rectangle.orientation,
                    center: preview_rectangle.center,
                    size: preview_rectangle.dimensions,
                };
                // U：native `PreviewRectangle+0x28` 放置点 Y → 0.0（同上）。
                let first = main_camera.read_matrix(0x80);
                let second = main_camera.read_matrix(0x00);
                let mut corners = rect.as_points3(0.0).into_iter().map(|point| {
                    country_core::utils::world_space_to_screen_space(
                        point, dims.x, dims.y, &first, &second,
                    )
                });
                let mut acc = corners.next().expect("Rectangle2d::as_points3 4 角");
                for screen in corners {
                    if screen.x > acc.x {
                        acc = screen;
                    }
                }
                acc + Vec2::splat(margin20)
            }
            // O case 10（0x141c50ae0）：`(width*0.5, height*0.5)`。
            HintPlacement::ScreenCenter => dims * 0.5,
            // O default / 第二级跳转表 0x142f14844（tag 0..5,7..9,11..）语义 U。
            // 已恢复写面只产生 6/7/8/10；其余保持不伪造（跳过该实体）。
            HintPlacement::Other(_) | HintPlacement::Subject { .. } => continue,
        };

        // O 公共块 0x141c515e0。
        let delta = memo.size - memo.position;
        let target = raw + delta;
        let upper = Vec2::new(
            dims.x - memo.position_2.x - margin30,
            dims.y - memo.position_2.y - margin30,
        );
        // maxps(下界, target) 对 NaN 取**下界**；minps(upper, .) 对 NaN 取前者。
        // Rust 的 f32::max/min 正是“返回非 NaN 操作数”，与机器码选择一致。
        let clamped = Vec2::splat(margin30).max(target).min(upper);
        let final_pos = clamped - delta;

        spring.pos = final_pos;
        if !spring.initialized {
            spring.target = final_pos;
            spring.initialized = true;
        } else {
            // O 0x140c904e0：a + (b-a)*(1-exp(-rate*dt))，rate = 10.0。
            spring.target.x = time_continuous_lerp(
                spring.target.x,
                final_pos.x,
                HINT_SPRING_RATE,
                time.delta,
            );
            spring.target.y = time_continuous_lerp(
                spring.target.y,
                final_pos.y,
                HINT_SPRING_RATE,
                time.delta,
            );
        }
    }
}

/// hint_position_update 平滑速率（O `MOVSS XMM7,[0x1429345a8]` = 10.0）。
const HINT_SPRING_RATE: f32 = 10.0;

/// hint_draw（O 0x141c51a60，mei IMGUI）：动画>0 → new_at_pos + 模板 + 尺寸 +
/// with_slot 分槽渲染绑定/文本/进度条/交互。
/// **未实现**：mei/toolbar（批次 3）未落库；不伪造 UI 渲染。
pub fn hint_draw(
    _query: Query<(Entity, &HintSpring, &HintAnimation, &HintPlacement, &HintType)>,
) {
    // 未实现：见模块注释。
}

/// render_hints（O 0x141c52700）：按判别 switch：Empty 无操作；Graphic →
/// me_image(floorf(t/frames))；Key → keycode_to_str（0xc3 未绑→占位
/// DAT_142f147af）+ 模板；Mouse → mouse_button_to_string 闭包；0xc8 → 4 键
/// keycode_to_str + 4×with_slot；Steam → panic_fmt("Call resolve_steam() and
/// cleanup() on BindingHintDisplay before sending it in.")；列表 → child_with_global_id
/// + 递归 render_hints。
/// **未实现**：mei/toolbar（批次 3）未落库；switch/panic 串已核对，渲染 DSL 不伪造。
pub fn render_hints(_display: &BindingHintDisplay) {
    // 未实现：见模块注释。
}

/// progress_bar_update_system（O 0x141c530f0）：Query<(HintType,HintProgressBar)> +
/// Res<CameraHintProgress> + EventWriter<PlayUiSound>。取 id 0xa/0xb/0xc 的进度，
/// 量化：p==0→0；0<p≤0.4→1；0.4<p≤0.99→2；p<0 或 p>0.99→3。档位变化 →
/// PlayUiSound tag 0x16(22)。`core::panicking::panic()` 对非相机 id。
pub fn progress_bar_update_system(
    mut query: Query<(&crate::HintType, &mut HintProgressBar)>,
    progress: Res<crate::hint_camera_progress::CameraHintProgress>,
    mut sounds: EventWriter<PlayUiSound>,
) {
    for (hint_type, mut bar) in &mut query {
        // 非 0xa/0xb/0xc 的提示：原 panic。
        let value = match hint_type {
            crate::HintType::CameraPan => progress.progress_pan(),
            crate::HintType::CameraRotate => progress.progress_rotate(),
            crate::HintType::CameraZoom => progress.progress_zoom(),
            _ => panic!("progress_bar_update_system: non-camera hint"),
        };
        let level = quantize(value);
        if level != bar.level {
            let _ = sounds.write(PlayUiSound {
                kind: 0x16,
                _payload: [0; 20],
            }); // O：tag 0x16(22)
            bar.level = level;
        }
    }
}

/// 进度量化（O 0x141c530f0）：p==0→0；0<p≤0.4→1；0.4<p≤0.99→2；否则→3。
pub fn quantize(p: f32) -> u8 {
    if p == 0.0 {
        0
    } else if p < 0.0 || p > 0.99 {
        3
    } else if p > 0.4 {
        2
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::quantize;
    use super::*;
    use crate::hint_orchestrator::HintLayoutMemo;
    use country_core::resources::mouse_terrain_raycast::CursorPositionSS;
    use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
    use country_core::resources::walls::decorator_storage::DecoratorStorage;
    use country_core::resources::walls::public_walls::PublicWalls;
    use country_core::resources::window::WindowSize;
    use country_core::systems::camera_rig::main_camera::MainCamera;
    use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
    use country_core::systems::wall::shapes::{PreviewCircle, PreviewRectangle};

    #[test]
    fn quantize_thresholds() {
        // O 0x141c530f0：0→0；(0,0.4]→1；(0.4,0.99]→2；<0 或 >0.99→3。
        assert_eq!(quantize(0.0), 0);
        assert_eq!(quantize(0.2), 1);
        assert_eq!(quantize(0.4), 1);
        assert_eq!(quantize(0.5), 2);
        assert_eq!(quantize(0.99), 2);
        assert_eq!(quantize(1.2), 3);
        assert_eq!(quantize(-1.0), 3);
    }

    fn position_world() -> World {
        let mut world = World::new();
        world.insert_resource(CursorPositionSS(Vec2::new(10.0, 20.0)));
        world.insert_resource(WindowSize::new(200, 100));
        world.insert_resource(PreviewCircle::default());
        world.insert_resource(PreviewRectangle::default());
        world.insert_resource(MainCamera::default());
        world.insert_resource(PublicWalls::default());
        world.insert_resource(DecoratorStorage::default());
        world.insert_resource(CachedDecoratorTransforms::default());
        world.insert_resource(UiSignifierStream::default());
        world.insert_resource(GameTime::default());
        world
    }

    fn run_position(world: &mut World) {
        let mut schedule = Schedule::default();
        schedule.add_systems(hint_position_update);
        schedule.run(world);
    }

    fn spawn_hint(world: &mut World, memo: HintLayoutMemo, spring: HintSpring, placement: HintPlacement) -> Entity {
        world.spawn((memo, spring, placement)).id()
    }

    #[test]
    fn position_screen_center_is_half_window() {
        // O case 10：`(width*0.5, height*0.5)`；零布局记忆 → 无位移/无夹取。
        let mut world = position_world();
        let entity = spawn_hint(
            &mut world,
            HintLayoutMemo::default(),
            HintSpring::default(),
            HintPlacement::ScreenCenter,
        );
        run_position(&mut world);
        let spring = *world.get::<HintSpring>(entity).unwrap();
        assert_eq!(spring.pos, Vec2::new(100.0, 50.0));
        assert_eq!(spring.target, Vec2::new(100.0, 50.0));
        assert!(spring.initialized);
    }

    #[test]
    fn position_cursor_adds_double_margin20() {
        // O case 6 + FIX-1：margin20 = height(100)/1000*20 = 2 → cursor + (4, 0)。
        let mut world = position_world();
        let entity = spawn_hint(
            &mut world,
            HintLayoutMemo::default(),
            HintSpring::default(),
            HintPlacement::Cursor,
        );
        run_position(&mut world);
        let spring = *world.get::<HintSpring>(entity).unwrap();
        assert_eq!(spring.pos, Vec2::new(14.0, 20.0));
    }

    #[test]
    fn position_clamp_uses_margin30_and_layout_memo() {
        // O：delta = size-position；target = raw+delta；
        // upper = (window - field_c) - margin30；final = clamp - delta。
        // FIX-1：margin30 = height(100)/1000*30 = 3。
        // raw=(100,50)，delta=(300,300) → target=(400,350)；upper=(197,97) →
        // clamped=(197,97) → final=(-103,-203)。
        let mut world = position_world();
        let entity = spawn_hint(
            &mut world,
            HintLayoutMemo {
                position: Vec2::ZERO,
                size: Vec2::new(300.0, 300.0),
                position_2: Vec2::ZERO,
            },
            HintSpring::default(),
            HintPlacement::ScreenCenter,
        );
        run_position(&mut world);
        let spring = *world.get::<HintSpring>(entity).unwrap();
        assert_eq!(spring.pos, Vec2::new(-103.0, -203.0));
    }

    #[test]
    fn position_bounds_takes_max_screen_x_corner_with_its_y() {
        // O case 8（FIX-2）：四角投影后取屏幕 x 最大的角（并列保留先者），携带其 y。
        // 200×100 窗口；第一级矩阵置 x_axis.y=1（view.y = world.x），第二级单位阵：
        // 角 x=+1 → 屏幕 (200,0)，x=-1 → (0,100)。max-x 角 = (200,0)（并列保留先者）→
        // + margin20(2,2) = (202,2) → 夹取到 [3,197]x[3,97] → (197,3)。
        // 若误取分量 min 会得到 (3,97)，若误取分量 max-y 会得到 y=97，均不同。
        let mut world = position_world();
        {
            let mut camera = world.resource_mut::<MainCamera>();
            let mut first = glam::Mat4::IDENTITY;
            first.x_axis.y = 1.0;
            camera.write_matrix(0x80, &first);
            camera.write_matrix(0x00, &glam::Mat4::IDENTITY);
        }
        world.insert_resource(PreviewRectangle {
            center: Vec2::ZERO,
            dimensions: Vec2::new(2.0, 2.0),
            orientation: Vec2::X,
            height_ty: 0,
            height: 0.0,
            old_height: 0.0,
        });
        let entity = spawn_hint(
            &mut world,
            HintLayoutMemo::default(),
            HintSpring::default(),
            HintPlacement::Bounds(utils::geometry::rectangle::Rectangle2d {
                axis: Vec2::X,
                center: Vec2::ZERO,
                size: Vec2::new(2.0, 2.0),
            }),
        );
        run_position(&mut world);
        let spring = *world.get::<HintSpring>(entity).unwrap();
        assert_eq!(spring.pos, Vec2::new(197.0, 3.0));
    }

    #[test]
    fn position_smooths_target_with_time_continuous_lerp() {
        // O：已初始化 → target = lerp(target, final, rate=10, dt)。
        let mut world = position_world();
        world.resource_mut::<GameTime>().delta = 0.1;
        let entity = spawn_hint(
            &mut world,
            HintLayoutMemo::default(),
            HintSpring {
                pos: Vec2::ZERO,
                target: Vec2::ZERO,
                initialized: true,
            },
            HintPlacement::ScreenCenter,
        );
        run_position(&mut world);
        let spring = *world.get::<HintSpring>(entity).unwrap();
        // pos 直接写 final；target 指数逼近。
        assert_eq!(spring.pos, Vec2::new(100.0, 50.0));
        let factor = 1.0 - (-10.0_f32 * 0.1).exp();
        assert!((spring.target.x - 100.0 * factor).abs() < 1e-4);
        assert!((spring.target.y - 50.0 * factor).abs() < 1e-4);
    }
}
