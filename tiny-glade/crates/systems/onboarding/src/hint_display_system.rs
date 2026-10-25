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
//! 位置、高亮、生命周期按 B 原指令恢复；绘制经生产模板和 Graphic 路径。

use bevy_ecs::prelude::*;
use country_core::resources::time::GameTime;
use country_core::systems::history::replay_mode::time_continuous_lerp;
use country_core::systems::ui::resources::widget_params::PlayUiSound;
use glam::Vec2;
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
    0.0, 0.3, 0.0, 0.0, 0.3, 0.3, 0.3, 0.3, 0.0, 0.0, 0.3, 0.3, 0.3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
];

/// hint_animation_system（O 0x141c4f400）：`value = clamp(value + dt/0.3*dir, 0, 1)`，
/// `eased = EasingFunction::apply(value)`，`time += dt`；dir 删除中 -1（有
/// HintMarkedForDeletion）否则 +1。原静态 142f144bc=1，即 CubicInOut。
pub fn hint_animation_system(
    time: Res<GameTime>,
    mut query: Query<(&mut HintAnimation, Option<&HintMarkedForDeletion>)>,
) {
    let dt = time.delta;
    for (mut anim, deleting) in &mut query {
        let step = dt / 0.3;
        let step = if deleting.is_some() { -step } else { step };
        let mut value = step + anim.value;
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

/// 原141c50120读取HintPlacement的subject与+0x11高亮标志，不读HintType。
pub fn hint_highlight_shape(
    walls: Res<country_core::resources::walls::public_walls::PublicWalls>,
    mut roof_cmds: EventWriter<country_core::resources::roofs::commands::HighlightRoofCmd>,
    mut wall_cmds: EventWriter<country_core::systems::wall::wall_highlight::HighlightWallCmd>,
    query: Query<(&HintAnimation, &HintPlacement)>,
) {
    use country_core::systems::mode_manager::EditSubject;
    for (animation, placement) in &query {
        if !(animation.eased > 0.9) {
            continue;
        }
        let HintPlacement::Subject { subject, word_0x10 } = *placement else {
            continue;
        };
        if word_0x10 >> 8 == 0 {
            continue;
        }
        let (EditSubject::Wall(id) | EditSubject::Roof(id)) = subject else {
            continue;
        };
        let wall = country_core::resources::walls::WallId(id);
        if walls.roof_entity(wall).is_some() {
            roof_cmds.write(
                country_core::resources::roofs::commands::HighlightRoofCmd::new(
                    wall,
                    country_core::resources::walls::HighlightColor(2),
                ),
            );
        }
        wall_cmds
            .write(country_core::systems::wall::wall_highlight::HighlightWallCmd { wall, ty: 2 });
    }
}

/// B @141c50510 的完整业务区（含PDB范围内的冷分支至141c517b8）。
/// 7/8直接读预览放置平面；1/2/3从墙、屋顶、杂物权威几何计算锚点。
#[allow(clippy::too_many_arguments)]
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
    walls: Res<country_core::resources::walls::public_walls::PublicWalls>,
    roofs: Query<&country_core::resources::roofs::roof_shape::Roof>,
    decorator_storage: Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,
    clutter_meshes: Res<country_core::resources::clutter_meshes::ClutterMeshes>,
    cached_transforms: Res<
        country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms,
    >,
    mut signifier_stream: ResMut<
        country_core::systems::ui::signifiers::signifier_create::UiSignifierStream,
    >,
    time: Res<GameTime>,
) {
    let dims = window.as_vec2();
    let margin20 = dims.y / 1000.0 * 20.0;
    let margin30 = dims.y / 1000.0 * 30.0;
    let project = |point| hint_project(point, dims, &main_camera);
    for (memo, mut spring, placement) in &mut query {
        let raw = match *placement {
            HintPlacement::Cursor => Vec2::new(cursor.0.x + margin20 + margin20, cursor.0.y + 0.0),
            HintPlacement::WorldPoint(_) => {
                let right = main_camera.camera_matrices().eye_right();
                let center = glam::Vec3::new(
                    preview_circle.center.x,
                    preview_circle.placement_y,
                    preview_circle.center.y,
                );
                project(center + right * (preview_circle.radius + 0.5)) + Vec2::splat(margin20)
            }
            HintPlacement::Bounds(_) => {
                let rect = utils::geometry::rectangle::Rectangle2d {
                    axis: preview_rectangle.orientation,
                    center: preview_rectangle.center,
                    size: preview_rectangle.dimensions,
                };
                let points = rect.as_points3(preview_rectangle.placement_y);
                let mut selected = project(points[0]);
                for point in &points[1..] {
                    let screen = project(*point);
                    if screen.x > selected.x {
                        selected = screen;
                    }
                }
                selected + Vec2::splat(margin20)
            }
            HintPlacement::ScreenCenter => dims * 0.5,
            HintPlacement::Subject { subject, word_0x10 } => {
                let Some(raw) = subject_hint_position(
                    subject,
                    word_0x10,
                    spring.pos,
                    cursor.0,
                    dims,
                    margin20,
                    &walls,
                    &roofs,
                    &decorator_storage,
                    &clutter_meshes,
                    &cached_transforms,
                    &mut signifier_stream,
                    &project,
                ) else {
                    continue;
                };
                raw
            }
            // 原0/4/5是unreachable；旧Other接口没有携带对应枚举负载，不能编造锚点。
            HintPlacement::Other(tag) => {
                panic!("unmodeled HintPlacement payload for native tag {tag}")
            }
        };
        update_hint_spring(memo, &mut spring, raw, dims, margin30, time.delta);
    }
}

fn hint_project(
    point: glam::Vec3,
    dims: Vec2,
    camera: &country_core::systems::camera_rig::main_camera::MainCamera,
) -> Vec2 {
    country_core::utils::world_space_to_screen_space(
        point,
        dims.x,
        dims.y,
        &camera.read_matrix(0x80),
        &camera.read_matrix(0),
    )
}

/// 原循环从0开始，仅采纳0<=x<窗口宽度的最大投影x，不按深度/屏幕y裁剪。
fn rightmost_subject_x(
    points: impl IntoIterator<Item = glam::Vec3>,
    width: f32,
    project: &impl Fn(glam::Vec3) -> Vec2,
) -> f32 {
    let mut rightmost = 0.0;
    for point in points {
        let x = project(point).x;
        if x >= 0.0 && x < width && x > rightmost {
            rightmost = x;
        }
    }
    rightmost
}

#[allow(clippy::too_many_arguments)]
fn subject_hint_position(
    subject: country_core::systems::mode_manager::EditSubject,
    word: u16,
    previous: Vec2,
    cursor: Vec2,
    dims: Vec2,
    margin: f32,
    walls: &country_core::resources::walls::public_walls::PublicWalls,
    roofs: &Query<&country_core::resources::roofs::roof_shape::Roof>,
    decorators: &country_core::resources::walls::decorator_storage::DecoratorStorage,
    meshes: &country_core::resources::clutter_meshes::ClutterMeshes,
    transforms: &country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms,
    stream: &mut country_core::systems::ui::signifiers::signifier_create::UiSignifierStream,
    project: &impl Fn(glam::Vec3) -> Vec2,
) -> Option<Vec2> {
    use country_core::resources::walls::{WallId, decorator::DecoratorId};
    use country_core::systems::mode_manager::{ClutterMeshTy, EditSubject};
    use country_core::systems::wall::valid_enclosure::ShapeParameters;
    let use_cursor_y = word as u8 != 0;
    let (rightmost, y) = match subject {
        EditSubject::Wall(id) => {
            let Some(wall) = walls.get(WallId(id)) else {
                return Some(previous);
            };
            let x = rightmost_subject_x(
                wall.geometry
                    .outline
                    .iter()
                    .copied()
                    .map(glam::Vec3::from_array),
                dims.x,
                project,
            );
            let y = if use_cursor_y {
                cursor.y
            } else {
                let center = if wall.is_premade_shape() {
                    ShapeParameters::from_snapshot(wall.shape()).center()
                } else {
                    Vec2::from_array(wall.simplified_curve_2d().pos_at_u(0.5))
                };
                let height = wall.max_y() - wall.building_projection().float_0x284();
                project(glam::Vec3::new(center.x, height, center.y)).y
            };
            (x, y)
        }
        EditSubject::Roof(id) => {
            let Some(entity) = walls.roof_entity(WallId(id)) else {
                return Some(previous);
            };
            let Ok(roof) = roofs.get(entity) else {
                return Some(previous);
            };
            let shape = ShapeParameters::from_observed(
                roof.observed()[8..36].try_into().expect("原屋顶形状28B"),
            );
            let curve = shape.as_curve2();
            let x = rightmost_subject_x(
                curve
                    .points
                    .iter()
                    .map(|p| glam::Vec3::new(p.x, roof.floor_y(), p.y)),
                dims.x,
                project,
            );
            let y = if use_cursor_y {
                cursor.y
            } else {
                // 原141c516f9：屋顶原高度的四分之一；不得改成顶部或半高。
                project(roof.pivot_3d() + glam::Vec3::Y * (roof.height() * 0.25)).y
            };
            (x, y)
        }
        EditSubject::DecoClutter { decorator_id } => {
            let Some(info) = decorators.decorator_info(DecoratorId(decorator_id)) else {
                return Some(previous);
            };
            assert!(
                info.ty < 56,
                "Hints around non-clutter decorators are not implemented"
            );
            // repr(u8)有连续0..55共56个真实变体，检查后才转换，保留原资产索引。
            let ty: ClutterMeshTy = unsafe { std::mem::transmute(info.ty) };
            let mesh = meshes.mesh(ty);
            let Some(transform) = transforms.try_transform(decorator_id) else {
                return None;
            };
            let axis3 = transform.rotation * glam::Vec3::X;
            let axis = Vec2::new(axis3.x, axis3.z);
            let inverse_length = 1.0 / (axis.x * axis.x + axis.y * axis.y).sqrt();
            let mut bounds = mesh.bounds;
            bounds.set_orientation_xz(axis * inverse_length);
            bounds.translate(transform.translation);
            // 原不把缓存scale再乘到资产bounds上；真实的附属调试工厂也必须提交。
            let f = bounds.footprint;
            country_core::utils::debug_viz::DebugRectangle3d::new([
                f.axis.x,
                f.axis.y,
                f.center.x,
                f.center.y,
                f.size.x,
                f.size.y,
                bounds.bottom,
                bounds.top,
            ])
            .dashed_line()
            .show(stream);
            assert!(
                f.size.x * f.size.y > 0.0,
                "original Rectangle3d::points area assertion"
            );
            let x = rightmost_subject_x(bounds.points(), dims.x, project);
            let y = if use_cursor_y {
                cursor.y
            } else {
                project(transform.translation).y
            };
            (x, y)
        }
        _ => panic!("internal error: entered unreachable code"),
    };
    Some(Vec2::new(rightmost + margin, y))
}

/// 原maxps/minps的源操作数平局与NaN选择顺序，不能由平台fmaxnm重排正负零。
fn hint_clamp(value: f32, lower: f32, upper: f32) -> f32 {
    let above_lower = if value.is_nan() || value < lower {
        lower
    } else {
        value
    };
    if above_lower.is_nan() || above_lower > upper {
        upper
    } else {
        above_lower
    }
}

fn update_hint_spring(
    memo: &crate::hint_orchestrator::HintLayoutMemo,
    spring: &mut HintSpring,
    raw: Vec2,
    window: Vec2,
    margin: f32,
    dt: f32,
) {
    let delta = memo.size - memo.position;
    let target = raw + delta;
    let upper = (window - memo.position_2) - Vec2::splat(margin);
    let final_pos = Vec2::new(
        hint_clamp(target.x, margin, upper.x),
        hint_clamp(target.y, margin, upper.y),
    ) - delta;
    spring.pos = final_pos;
    if !spring.initialized {
        spring.target = final_pos;
        spring.initialized = true;
    } else {
        spring.target.x = time_continuous_lerp(spring.target.x, final_pos.x, HINT_SPRING_RATE, dt);
        spring.target.y = time_continuous_lerp(spring.target.y, final_pos.y, HINT_SPRING_RATE, dt);
    }
}
const HINT_SPRING_RATE: f32 = 10.0;

/// 原141c51a60：提示实体进入生产MEI，同一Frame节点生命周期负责离开后的卸载。
#[allow(clippy::too_many_arguments)]
pub fn hint_draw(
    mut mei: ResMut<toolbar::MeiContext>,
    mut query: Query<(
        &mut crate::HintLayoutMemo,
        &HintAnimation,
        Option<&BindingHintDisplay>,
        &HintSpring,
        Option<&HintText>,
        Option<&HintProgressBar>,
        Option<&crate::InteractionPrompt>,
    )>,
    mode: Res<country_core::input::ActiveInputModeTracker>,
    steam_input: Option<Res<country_core::resources::steam::SteamInput>>,
    steam_client: Option<Res<country_core::systems::steam::SteamClientResource>>,
    window: Res<country_core::resources::window::WindowSize>,
    mut beta_warning: EventReader<country_core::resources::events::DisplayBetaWIPWarning>,
    cursor: Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,
) {
    let is_gamepad = mode.mode() == country_core::input::ActiveInputMode::Controller;
    let scale = window.as_vec2().y / 1000.;
    if beta_warning.read().last().is_some() {
        let root = mei.new_at_pos(
            "hint_beta_warning",
            cursor.0 / scale - Vec2::new(20., 0.),
            None,
            Vec2::new(1., -1.),
            false,
        );
        root.ignore_input();
        let template = root.template("other/hint");
        template.context.node_mut().local_opacity = 1.;
        hint_text(template.slot("text"), "⚠ Will be changed soon", 24.);
    }
    for (index, (mut memo, animation, display, spring, text, progress, interaction)) in
        query.iter_mut().enumerate()
    {
        // 原ordered compare：零、负值及NaN不生成节点。
        if !(animation.eased > 0.) {
            continue;
        }
        let anchor = if interaction.is_some() {
            Vec2::ZERO
        } else {
            Vec2::splat(-1.)
        };
        let root = mei.new_at_pos_string(
            format!("hint{index}"),
            spring.target / scale,
            None,
            anchor,
            false,
        );
        root.ignore_input();
        let template = root.template("other/hint");
        template.context.node_mut().local_opacity = animation.eased;
        let node = template.context.node();
        memo.position = spring.target;
        // 原prev_computed_layout的新节点为None：第二Vec2沿用当前锚点，不能清零。
        // 旧字段名size实际保存前帧origin；当前ComputedLayout顺序为(size,pos)。
        if node.created != 0 {
            let old = node.prev_computed_layout();
            memo.size = old.pos * scale;
            memo.position_2 = old.size * scale;
        } else {
            memo.size = spring.target;
            memo.position_2 = Vec2::ZERO;
        }
        if let Some(text) = text {
            hint_text(
                template.slot("text"),
                &text.text,
                if interaction.is_some() { 24. } else { 50. },
            );
        }
        if let Some(display) = display {
            let filtered = display.clone().filter_to_mode(is_gamepad);
            let resolved = if steam_client.is_some() {
                match steam_input.as_ref() {
                    Some(input) => filtered.resolve_steam(input),
                    None => filtered,
                }
            } else {
                filtered
            };
            render_hints(&resolved.cleanup(), animation, template.slot("icons"), 0);
        }
        if let Some(progress) = progress {
            let images = [
                "hint/progress_1",
                "hint/progress_2",
                "hint/progress_3",
                "hint/progress_4",
            ];
            let image = images[usize::from(progress.level)];
            template.slot("rows").image("progerss bar", image, 17.);
        }
    }
}
fn hint_text(node: toolbar::mei::core::NodeContext, text: &str, size: f32) {
    let label = node.label(text);
    let content = label.content_mut::<toolbar::mei::widgets::LabelContent>();
    content.text.set_weight(tg_text::Weight(700));
    content.text.set_metrics(size).expect("原提示字号");
    content.text.set_align(tg_text::Align::Center);
    content
        .text
        .set_family(tg_text::FontFamily::MenuButton)
        .expect("原提示字体");
}
fn hint_key_text(key: Option<crate::KeyCode>) -> &'static str {
    key.map_or("?", |key| {
        country_core::game_key_bindings::keycode_to_str(key).unwrap_or("???")
    })
}
fn hint_key_label(node: toolbar::mei::core::NodeContext, text: &str, size: f32) {
    let label = node.label(text);
    let content = label.content_mut::<toolbar::mei::widgets::LabelContent>();
    content.color = 0x505050ff;
    content.text.set_metrics(size).expect("原键帽字号");
    content
        .text
        .set_family(tg_text::FontFamily::MenuButton)
        .expect("原键帽字体");
    content.text.set_weight(tg_text::Weight(700));
}
fn hint_index_node(
    parent: toolbar::mei::core::NodeContext,
    index: u64,
) -> toolbar::mei::core::NodeContext {
    use toolbar::mei::core::{GlobalId, calculate_hash};
    let local = calculate_hash(&(index, 0xa79b7268a2a968d9u64));
    parent.child_with_global_id(GlobalId(calculate_hash(&(parent.id.0, local))))
}
fn hint_separator(parent: toolbar::mei::core::NodeContext, index: u64, text: &str) {
    use toolbar::mei::core::{GlobalId, calculate_hash};
    let local = calculate_hash(&(index, 0xce69dbf0aee13da2u64));
    let node = parent.child_with_global_id(GlobalId(calculate_hash(&(parent.id.0, local))));
    let content = node.content_mut::<toolbar::mei::widgets::LabelContent>();
    content.set_text(text.into());
    content.text.set_metrics(70.).expect("原组合符字号");
}
/// 原141c52700：完整递归组合；depth只控制首行和嵌套行模板，不创建另一渲染层。
pub fn render_hints(
    display: &BindingHintDisplay,
    animation: &HintAnimation,
    node: toolbar::mei::core::NodeContext,
    depth: usize,
) {
    match display {
        BindingHintDisplay::Empty => {}
        BindingHintDisplay::Graphic(graphic) => {
            node.image(
                "hint",
                graphic.image_at_time(animation.time),
                graphic.height,
            );
        }
        BindingHintDisplay::Key(key) => {
            let view = node.child("keys").template("other/hint_one_key");
            hint_key_label(view.slot("key"), hint_key_text(*key), 32.);
        }
        BindingHintDisplay::Mouse(button) => {
            let view = node.child("keys").template("other/hint_one_key");
            hint_key_label(
                view.slot("key"),
                &format!(
                    "🖱 {}",
                    country_core::game_key_bindings::mouse_button_to_string(*button)
                ),
                18.,
            );
        }
        BindingHintDisplay::Row(keys) => {
            let view = node.child("keys").template("other/hint_wasd");
            for (slot, key) in ["forward", "left", "back", "right"].into_iter().zip(keys) {
                hint_key_label(view.slot(slot), hint_key_text(*key), 18.);
            }
        }
        BindingHintDisplay::Steam(_) => {
            panic!("Call resolve_steam() and cleanup() on BindingHintDisplay before sending it in.")
        }
        BindingHintDisplay::Alternate(items)
        | BindingHintDisplay::Adjacent(items)
        | BindingHintDisplay::Together(items) => {
            let view = hint_index_node(node, 0).template(if depth == 0 {
                "other/hint_top_row"
            } else {
                "other/hint_row"
            });
            let body = view.slot("content");
            for (index, item) in items.iter().enumerate() {
                if index != 0 {
                    match display {
                        BindingHintDisplay::Alternate(_) => {
                            hint_separator(body, 1_000_000 + index as u64, "/")
                        }
                        BindingHintDisplay::Together(_) => {
                            hint_separator(body, 1_000_000 + index as u64, "+")
                        }
                        _ => {}
                    }
                }
                render_hints(
                    item,
                    animation,
                    hint_index_node(body, index as u64),
                    depth.wrapping_add(1),
                );
            }
        }
    }
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
        let mesh = country_core::resources::clutter_meshes::ClutterMesh {
            bounds: utils::geometry::rectangle::Rectangle3d::new(glam::Vec3::ZERO, glam::Vec3::ONE, Vec2::X),
            mesh_name: "cpu_hint_fixture", mesh: None,
            name_hash: utils::name_hash::NameHash::compute("cpu_hint_fixture"),
            radius_xz: 1., native_group: 0, solid_texture: None,
            native_flag_46: false, native_flag_47: false,
        };
        world.insert_resource(country_core::resources::clutter_meshes::ClutterMeshes::from_parts(
            std::array::from_fn(|_| mesh.clone()), vec![mesh],
        ));
        world.insert_resource(GameTime::default());
        world
    }

    fn run_position(world: &mut World) {
        let mut schedule = Schedule::default();
        schedule.add_systems(hint_position_update);
        schedule.run(world);
    }

    fn spawn_hint(
        world: &mut World,
        memo: HintLayoutMemo,
        spring: HintSpring,
        placement: HintPlacement,
    ) -> Entity {
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
            placement_y: 0.0,
            ..Default::default()
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

    #[test]
    fn original_lifecycle_opens_cools_closes_and_unloads() {
        let mut world = position_world();
        world.init_resource::<Events<PlayUiSound>>();
        let entity = world.spawn(HintAnimation::default()).id();
        let mut audio = Schedule::default(); audio.add_systems(hint_audio);
        audio.run(&mut world);
        assert_eq!(world.resource_mut::<Events<PlayUiSound>>().drain().map(|e|e.kind).collect::<Vec<_>>(), [20]);
        world.resource_mut::<GameTime>().delta = 0.3;
        let mut animation = Schedule::default(); animation.add_systems(hint_animation_system);
        animation.run(&mut world);
        assert_eq!(world.get::<HintAnimation>(entity).unwrap().eased, 1.);
        world.entity_mut(entity).insert(HintDeleteAfterCooldown {timer: 0.2});
        let mut cooldown = Schedule::default();cooldown.add_systems(hint_deletion_cooldown_system);cooldown.run(&mut world);
        assert!(world.get::<HintDeleteAfterCooldown>(entity).is_none());
        assert!(world.get::<HintMarkedForDeletion>(entity).is_some());
        audio.run(&mut world);
        assert_eq!(world.resource_mut::<Events<PlayUiSound>>().drain().map(|e|e.kind).collect::<Vec<_>>(), [21]);
        animation.run(&mut world);
        let mut deletion = Schedule::default();deletion.add_systems(hint_deletion_system);deletion.run(&mut world);
        assert!(world.get_entity(entity).is_err());
    }

    #[test]
    fn native_subject_highlight_uses_payload_and_flag_without_hint_type() {
        use country_core::{resources::roofs::commands::HighlightRoofCmd,systems::{mode_manager::EditSubject,wall::wall_highlight::HighlightWallCmd}};
        let mut world = position_world();
        world.init_resource::<Events<HighlightRoofCmd>>();world.init_resource::<Events<HighlightWallCmd>>();
        world.spawn((HintAnimation {eased:0.95,..Default::default()},HintPlacement::Subject {subject:EditSubject::Wall(37),word_0x10:0x100}));
        world.spawn((HintAnimation {eased:1.,..Default::default()},HintPlacement::Subject {subject:EditSubject::Wall(38),word_0x10:1}));
        let mut schedule=Schedule::default();schedule.add_systems(hint_highlight_shape);schedule.run(&mut world);
        let events:Vec<_>=world.resource_mut::<Events<HighlightWallCmd>>().drain().collect();
        assert_eq!(events.len(),1);assert_eq!(events[0].wall,country_core::resources::walls::WallId(37));
        assert_eq!(events[0].ty,2);assert_eq!(world.resource::<Events<HighlightRoofCmd>>().len(),0);
    }

    #[test]
    fn rectangle_anchor_reads_actual_placement_plane_and_ignores_legacy_payload() {
        let mut world = position_world();
        { let mut camera=world.resource_mut::<MainCamera>();camera.write_matrix(0x80,&glam::Mat4::IDENTITY);camera.write_matrix(0,&glam::Mat4::IDENTITY); }
        world.insert_resource(PreviewRectangle {center:Vec2::ZERO,dimensions:Vec2::splat(0.5),orientation:Vec2::X,placement_y:0.5,..Default::default()});
        let entity=spawn_hint(&mut world,HintLayoutMemo::default(),HintSpring::default(),HintPlacement::Bounds(utils::geometry::Rectangle2d::new_axis_aligned(Vec2::splat(99.),Vec2::splat(99.))));
        run_position(&mut world);
        assert_eq!(world.get::<HintSpring>(entity).unwrap().pos,Vec2::new(127.,27.));
    }

    #[test]
    fn missing_subject_retains_anchor_but_still_applies_original_clamp() {
        let mut world=position_world();
        let entity=spawn_hint(&mut world,HintLayoutMemo::default(),HintSpring {pos:Vec2::new(250.,-20.),..Default::default()},HintPlacement::Subject {subject:country_core::systems::mode_manager::EditSubject::Wall(99),word_0x10:0});
        run_position(&mut world);assert_eq!(world.get::<HintSpring>(entity).unwrap().pos,Vec2::new(197.,3.));
    }

}

#[cfg(test)]
mod original_render_tests {
    use super::*;
    use crate::hint_orchestrator::{
        HintGraphicImages, HintGraphicParams, HintLayoutMemo, InteractionPrompt,
    };
    use bevy_input::keyboard::{KeyCode, NativeKeyCode};
    use toolbar::{
        MeiContext,
        mei::{
            core::{FrameKind, NodeContent, NodeDrawContext, NodeFlags},
            widgets::{ImageContent, LabelContent},
        },
    };

    fn real_mei() -> MeiContext {
        let root =
            std::path::PathBuf::from(std::env::var_os("TG_INSTALL_ROOT").expect("原资产根目录"));
        let mut mei = MeiContext::default();
        mei.load_font_table(&root.join("assets/fonts/table.ron"))
            .unwrap();
        for name in [
            "other/hint",
            "other/hint_top_row",
            "other/hint_row",
            "other/hint_one_key",
            "other/hint_wasd",
        ] {
            mei.configs.insert(
                name.into(),
                std::sync::Arc::new(
                    toolbar::mei::config::read_node_config_from_file(
                        &root.join(format!("assets/ui/{name}.ron")),
                    )
                    .unwrap(),
                ),
            );
        }
        mei
    }
    fn texts(mei: &MeiContext) -> Vec<String> {
        mei.nodes
            .values()
            .filter_map(|node| {
                let any: &dyn std::any::Any = node.content.as_ref();
                any.downcast_ref::<LabelContent>()
                    .and_then(|l| l.pending_text.clone())
            })
            .collect()
    }
    #[test]
    fn original_graphic_time_saturates_then_wraps_and_static_images_do_not_animate() {
        let animated =
            HintGraphicParams::animated(vec!["a".into(), "b".into(), "c".into()], 0.7, 100.);
        for time in [-1., 0., f32::NAN, f32::NEG_INFINITY] {
            assert_eq!(animated.image_at_time(time), "a");
        }
        assert_eq!(animated.image_at_time(0.7), "b");
        assert_eq!(animated.image_at_time(1.4), "c");
        assert_eq!(animated.image_at_time(2.1), "a");
        assert_eq!(
            animated.image_at_time(f32::INFINITY),
            ["a", "b", "c"][usize::MAX % 3]
        );
        let static_image = HintGraphicParams::image("static", 64.);
        assert!(matches!(static_image.images, HintGraphicImages::Image(_)));
        assert_eq!(static_image.image_at_time(f32::NAN), "static");
    }
    #[test]
    fn native_graphic_frame_selection_matches_f32_saturation_and_wrap_fixture() {
        let rows: Vec<(bool, usize, u32, u32, u32, usize, u32)> = ron::from_str(include_str!(
            "../../../../tests/fixtures/native/hint-graphic-frames.ron"
        ))
        .unwrap();
        assert_eq!(rows.len(), 1320);
        for (is_static, count, period, time, height, index, returned_height) in rows {
            let graphic = if is_static {
                HintGraphicParams::image("f0", f32::from_bits(height))
            } else {
                HintGraphicParams::animated(
                    (0..count).map(|i| format!("f{i}")).collect(),
                    f32::from_bits(period),
                    f32::from_bits(height),
                )
            };
            assert_eq!(
                graphic.image_at_time(f32::from_bits(time)),
                format!("f{index}"),
                "period={period:#x} time={time:#x} count={count}"
            );
            assert_eq!(graphic.height.to_bits(), returned_height);
        }
    }
    #[test]
    #[should_panic]
    fn original_empty_graphic_animation_is_not_silently_hidden() {
        HintGraphicParams::animated(Vec::new(), 0.7, 100.).image_at_time(0.);
    }
    #[test]
    #[ignore = "需TG_INSTALL_ROOT原模板和字体；仅CPU，不进行OS输入"]
    fn real_recursive_templates_preserve_typed_keys_mouse_rows_graphics_and_combinators() {
        let mut mei = real_mei();
        let root = mei.safe_frame("hint-test", false);
        let display =
            BindingHintDisplay::Key(Some(KeyCode::Unidentified(NativeKeyCode::Xkb(u32::MAX))))
                .alternate(BindingHintDisplay::Mouse(crate::MouseButton::Other(1)))
                .together(BindingHintDisplay::Row([
                    Some(KeyCode::KeyW),
                    Some(KeyCode::KeyA),
                    Some(KeyCode::KeyS),
                    None,
                ]))
                .adjacent(BindingHintDisplay::Graphic(HintGraphicParams::animated(
                    vec!["hint/mouse".into(), "hint/rmb".into()],
                    0.7,
                    100.,
                )));
        render_hints(
            &display,
            &HintAnimation {
                time: 0.7,
                ..Default::default()
            },
            root,
            0,
        );
        let values = texts(&mei);
        for value in ["???", "🖱 Mouse button 1", "W", "A", "S", "?", "/", "+"] {
            assert!(
                values.iter().any(|s| s == value),
                "缺少原显示{value:?}: {values:?}"
            );
        }
        assert!(mei.nodes.values().any(|node| {
            let any: &dyn std::any::Any = node.content.as_ref();
            any.downcast_ref::<ImageContent>()
                .is_some_and(|image| image.image == "hint/rmb" && image.height == 100.)
        }));
        // 使用原字体和生产 LabelContent 消费者产生真实文字图元，避免仅检查建树。
        let mut renderer = tg_text::TextRenderer::new();
        fn load_fonts(path: &std::path::Path, renderer: &mut tg_text::TextRenderer) {
            for entry in std::fs::read_dir(path).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() { load_fonts(&path, renderer); }
                else if matches!(path.extension().and_then(|x| x.to_str()), Some("ttf" | "otf")) {
                    renderer.load_font_from_file(path).unwrap();
                }
            }
        }
        let install = std::path::PathBuf::from(std::env::var_os("TG_INSTALL_ROOT").unwrap());
        load_fonts(&install.join("assets/fonts"), &mut renderer);
        let mut labels = 0;
        for node in mei.nodes.values_mut() {
            let any: &mut dyn std::any::Any = node.content.as_mut();
            let Some(content) = any.downcast_mut::<LabelContent>() else { continue; };
            let Some(text) = content.pending_text.take() else { continue; };
            if text.is_empty() { continue; }
            content.text.set_text(&mut renderer, text).unwrap();
            content.text.prepare(&mut renderer).unwrap();
            assert!(content.text.bounds()[0] > 0.);
            content.draw(&mut NodeDrawContext {
                text: &mut renderer, center: Vec2::new(100., 40.),
                extent: Vec2::new(200., 80.), opacity: 0.5,
                pixels_per_point: 1., cursor: Vec2::ZERO, elapsed: 0.7, state: 0,
            });
            assert!(!content.text.primitives().is_empty());
            let batch = renderer.consume().unwrap();
            assert!(!batch.primitives.is_empty());
            let expected = tg_text::scale_u32_packed_srgba_color(content.color, 0.5);
            assert!(batch.primitives.iter().all(|p| p.color == expected));
            labels += 1;
        }
        assert!(labels >= 8);
        let mut unresolved = std::panic::AssertUnwindSafe(&mut mei);
        assert!(
            std::panic::catch_unwind(move || {
                let node = unresolved.safe_frame("unresolved", false);
                render_hints(
                    &BindingHintDisplay::Steam(crate::hint_orchestrator::steam_query(0xf01)),
                    &HintAnimation::default(),
                    node,
                    0,
                );
            })
            .is_err(),
            "原Steam必须先resolve和cleanup"
        );
    }
    #[test]
    #[ignore = "需TG_INSTALL_ROOT原模板和字体；仅CPU，不进行OS输入"]
    fn real_draw_uses_frame_scale_anchor_memo_beta_and_entity_unload() {
        let mut world = World::new();
        world.insert_resource(real_mei());
        world.insert_resource(
            country_core::input::ActiveInputModeTracker::new_default_mouse_keyboard(
                std::time::Duration::ZERO,
            ),
        );
        world.insert_resource(country_core::resources::window::WindowSize::new(2400, 1500));
        world.insert_resource(
            country_core::resources::mouse_terrain_raycast::CursorPositionSS(Vec2::new(900., 600.)),
        );
        world.init_resource::<Events<country_core::resources::events::DisplayBetaWIPWarning>>();
        let entity = world
            .spawn((
                HintLayoutMemo::default(),
                HintAnimation {
                    eased: 0.5,
                    ..Default::default()
                },
                HintSpring {
                    target: Vec2::new(900., 600.),
                    ..Default::default()
                },
                BindingHintDisplay::Key(Some(KeyCode::KeyJ)),
                HintText {
                    text: "SNAP".into(),
                },
                HintProgressBar { level: 2 },
            ))
            .id();
        world.send_event(country_core::resources::events::DisplayBetaWIPWarning);
        let mut schedule = Schedule::default();
        schedule.add_systems(hint_draw);
        schedule.run(&mut world);
        {
            let mei = world.resource::<MeiContext>();
            assert!(texts(mei).contains(&"SNAP".into()));
            assert!(texts(mei).contains(&"J".into()));
            assert!(texts(mei).contains(&"⚠ Will be changed soon".into()));
            let event=mei.frame_events.iter().find(|event|matches!(event.kind,FrameKind::AtPos{position,..} if position==Vec2::new(600.,400.))).unwrap();
            assert!(matches!(event.kind,FrameKind::AtPos{anchor,..} if anchor==Vec2::splat(-1.)));
            assert!(mei.nodes.get(&event.id).unwrap().flags & NodeFlags::IGNORE_INPUT != 0);
            assert!(mei.nodes.values().any(|node| {
                let any: &dyn std::any::Any = node.content.as_ref();
                any.downcast_ref::<ImageContent>()
                    .is_some_and(|image| image.image == "hint/progress_3" && image.height == 17.)
            }));
        }
        assert_eq!(
            world.get::<HintLayoutMemo>(entity).unwrap().size,
            Vec2::new(900., 600.)
        );
        assert_eq!(
            world.get::<HintLayoutMemo>(entity).unwrap().position_2,
            Vec2::ZERO
        );
        // 真实MEI在下一帧移动旧节点。设置上一帧布局结果，检查原memo读数。
        for node in world.resource_mut::<MeiContext>().nodes.values_mut() {
            node.size = Vec2::new(220., 80.);
            node.pos = Vec2::new(500., 450.);
        }
        world.resource_mut::<MeiContext>().recycle_frame_nodes();
        world
            .entity_mut(entity)
            .insert(InteractionPrompt { active: false });
        schedule.run(&mut world);
        assert_eq!(
            world.get::<HintLayoutMemo>(entity).unwrap().position,
            Vec2::new(900., 600.)
        );
        assert_eq!(
            world.get::<HintLayoutMemo>(entity).unwrap().size,
            Vec2::new(750., 675.)
        );
        assert_eq!(
            world.get::<HintLayoutMemo>(entity).unwrap().position_2,
            Vec2::new(330., 120.)
        );
        assert!(
            world.resource::<MeiContext>().frame_events.iter().any(
                |event| matches!(event.kind,FrameKind::AtPos{anchor,..} if anchor==Vec2::ZERO)
            )
        );
        world.despawn(entity);
        world.resource_mut::<MeiContext>().recycle_frame_nodes();
        schedule.run(&mut world);
        assert!(
            world.resource::<MeiContext>().nodes.is_empty(),
            "实体卸载后不保留提示画面节点"
        );
    }
}
