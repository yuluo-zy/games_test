//! B1407a4850/12790B：装饰移动主交互。按原16组参数、完整四相位跳表及四目的跳表恢复。
//! 原输入2=悬停、3=开始、4=持续、0/1=结束；不把持续拖动误当释放。
//! 原Local状态与完整四元数缓存属于本系统，不借用旧全局三字段兼容资源。
use bevy_ecs::prelude::*;

use crate::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher;
use country_core::resources::walls::{
    cached_decorator_transforms::{CachedDecoratorTransform, CachedDecoratorTransforms},
    decorator::{DecoratorId, DecoratorInfo, DecoratorType},
    decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
    public_walls::PublicWalls,
};
use glam::{Quat, Vec2, Vec3};

/// 原58d0写完整16B：抓取姿态在相机水平旋转的逆空间中保存。
#[derive(Default)]
pub struct CacheDecoclutterRotationOnGrab {
    pub rotation: Quat,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MovingDecoratorPart {
    Whole,
    Part(usize),
}

/// 原Local<Option<...>>的活跃字段；无交互的niche判别2由Option拥有。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MoveDecorationInteractionState {
    part: MovingDecoratorPart,
    height_constraint: Option<f32>,
    grab_offset: Vec2,
    decorator_id: DecoratorId,
    vertical_offset: f32,
    latest_position: Vec3,
    original_base_y: f32,
}

/// 原40B：移动/放置分别保存上帧有效点和速度；不是音频事件的Vec队列。
/// Move原1407a4bad/72e5/7314，Place原1407a9ef8/ac653/ac695。
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct MoveDecoratorDeltaPositions {
    pub move_previous_position: Option<glam::Vec3>,
    pub previous_position: Option<glam::Vec3>,
    pub move_velocity: f32,
    pub velocity: f32,
}

/// 原1407a84d0：Raycasting展开为World/Terrain，再借用Camera/Window/Depth。
/// 五者必须来自同一个World；不能读取另行持有私有碰撞世界的旧Resource。
#[derive(bevy_ecs::system::SystemParam)]
pub struct RaycastResources<'w> {
    /// 射线世界（core collision world；O：0x1408d99b0 SystemParam init）。
    pub raycast: Res<'w, country_core::systems::collision::world::RaycastWorld>,
    /// 地形高度（O：0x1412773f0 intersect_ray 消费）。
    pub terrain: Res<'w, country_core::resources::terrain_height::TerrainHeightsData<
        country_core::resources::terrain_height::TerrainHeightSsbo,
    >>,
    pub camera: Res<'w, country_core::systems::camera_rig::MainCamera>,
    pub window: Res<'w, country_core::resources::window::WindowSize>,
    pub depth: Res<'w, country_core::resources::read_mouse_depth_value::MouseDepthValue>,
}

/// 原1407a4850。保持原16组SystemParam，只有原只读stream改成安全ResMut。
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn ui_move_decorator(
    time: Res<bevy_time::Time<()>>,
    (mut local, modifiers, mut init_place, demo): (
        Local<Option<MoveDecorationInteractionState>>,
        Res<country_core::systems::backend::input::mod_key::Modifiers>,
        EventWriter<super::ui_place_decorator::EvInitPlaceDecoratorInteractionState>,
        Res<country_core::systems::backend::input::translate_input::DemoModeEnabled>,
    ),
    (cursor_ray, raycasts, cursor): (
        Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,
        RaycastResources,
        Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,
    ),
    (
        mode,
        dispatcher,
        mut storage,
        mut archivist,
        operations,
        derived,
        blueprints,
        archival,
        mut rotation_cache,
        mut stairs_state,
        mut request_mode,
    ): (
        Res<country_core::systems::mode_manager::AppMode>,
        Res<DecoratorAffordanceDispatcher>,
        ResMut<DecoratorStorage>,
        ResMut<country_core::resources::walls::decorator_archivist::LiveDecoratorArchivist>,
        Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,
        country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,
        Res<crate::blueprint::DecoratorBlueprints>,
        country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,
        Local<CacheDecoclutterRotationOnGrab>,
        ResMut<country_core::resources::stairs::stairs_state::StairsState>,
        EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,
    ),
    (walls, wall_entities, roofs, roof_volumes, wall_volumes, terrain, stair_requests): (
        Res<PublicWalls>,
        Res<country_core::resources::walls::public_walls::WallEntities>,
        Query<&country_core::resources::roofs::Roof>,
        Query<&country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,
        Query<&country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,
        Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo>>,
        Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,
    ),
    mut history: ResMut<crate::deferred_decorator_history::DeferredDecoratorEditHistory>,
    mut deltas: ResMut<MoveDecoratorDeltaPositions>,
    mut ids: ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,
    mut cached: ResMut<CachedDecoratorTransforms>,
    (ui, mut stream, mut window_hint): (
        Res<country_core::systems::ui::resources::ui_state::UiState>,
        ResMut<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,
        EventWriter<country_core::resources::onboarding::NotifyHintSystemWindow>,
    ),
    mut audio: EventWriter<country_core::systems::audio::PlayDecoratorSoundCmd>,
    mut backup: ResMut<crate::backup::DecoratorBackup>,
    (border, mut show_border, mut border_feedback): (
        Res<country_core::resources::glade_border::GladeBorder>,
        EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,
        EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,
    ),
    children: Res<country_core::resources::clutter_meshes::DecoratorAutoClutterChildren>,
    world: Res<country_core::systems::collision::world::RaycastWorld>,
    (mut moved_hint, mut continue_hint, mut detached_hint, mut can_detach_hint): (
        EventWriter<country_core::resources::onboarding::NotifyHintSystemClutterWasMoved>,
        EventWriter<country_core::resources::onboarding::NotifyHintSystemStartedStairsContinue>,
        EventWriter<country_core::resources::onboarding::NotifyHintSystemStairsDetachedFromWall>,
        EventWriter<country_core::resources::onboarding::NotifyHintSystemStairsCanDetachFromWall>,
    ),
) {
    use super::calculate_decorator_dst::RaycastDecoratorDst;
    use super::decorator_interaction_intent::{
        calculate_decorator_interaction_intent, DecoratorInteractionIntent,
    };
    use country_core::resources::onboarding::*;
    use country_core::resources::walls::decorator::{DecoratorDst, DecoratorRank};
    use country_core::systems::audio::PlayDecoratorSoundCmd;
    // 原4bad..bc5：无论本帧是否有移动请求，先take上帧有效点；速度不在此清零。
    let previous_position = deltas.move_previous_position.take();
    let Some(affordance) = dispatcher.movement() else {
        return;
    };
    let request = &affordance.request;
    let widget = ui.widget_context(request.widget);
    if !widget.is_active() {
        return;
    }
    let source_pose = cached.transform(request.decorator_id.0);
    let phase = widget.dragged();
    if phase == 2 {
        if let Some(children) = children.parent_of(request.decorator_id.0) {
            for child in children {
                highlight_child_autoclutter(child, &mut stream);
            }
        }
    }
    if phase < 2 {
        history.pending = true;
    }
    if phase == 2
        || local
            .as_ref()
            .is_some_and(|state| state.decorator_id != request.decorator_id)
    {
        *local = None;
    }
    let source_address = storage.lookup_storage(request.decorator_id).unwrap();
    let source_info = *storage.decorator_info(request.decorator_id).unwrap();
    let grip_y = decorator_grip_y(
        request.ty,
        &source_info,
        source_pose.translation.y,
        request.hit.position.y,
    );
    let grab = |pose: CachedDecoratorTransform, info: &DecoratorInfo, address| {
        move_grab_offset(
            &raycasts,
            cursor.0,
            &walls,
            &wall_entities,
            &wall_volumes,
            pose,
            info,
            address,
        )
    };
    if local.is_none() {
        backup.clear();
        let mut grip_pose = source_pose;
        let lift = grip_y - source_pose.translation.y;
        grip_pose.translation += Vec3::new(0.0 * lift, lift, 0.0 * lift);
        let Some(grab_offset) = grab(grip_pose, &source_info, source_address) else {
            return;
        };
        let mut state = MoveDecorationInteractionState {
            part: MovingDecoratorPart::Whole,
            height_constraint: None,
            grab_offset,
            decorator_id: request.decorator_id,
            vertical_offset: 0.0,
            latest_position: Vec3::ZERO,
            original_base_y: source_pose.translation.y,
        };
        if source_info.rank != 1 {
            window_hint.write(NotifyHintSystemWindow { flag: 0 });
        }
        let subtype = derived.as_view().subtype(request.decorator_id);
        if let Some(blueprint) = blueprints.get(request.decorator_id.0) {
            let visual_pose = blueprint.transform.as_ref().unwrap();
            if source_info.rank != 1 && modifiers.disable_snapping() {
                state.part = match source_info.dst_tag() {
                    0 | 1 => MovingDecoratorPart::Part(super::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id(
                        &source_pose, &request.hit, &source_info, visual_pose, derived.as_view(), request.decorator_id, &mut stream,
                    )),
                    2 => super::handle_splitting_roof_decorators::try_display_roof_decorator_parts_and_get_hovered_part_id(
                        &request.hit, &source_info, &source_pose, request.decorator_id, subtype, source_address,
                        &mut stream, &walls, &roofs, &roof_volumes,
                    ).map_or(MovingDecoratorPart::Whole, MovingDecoratorPart::Part),
                    _ => unreachable!("复合装饰拆分只允许墙或屋顶"),
                };
            }
            state.vertical_offset = grip_y - source_pose.translation.y;
            if affordance.native_flag_0x5d && request.ty.0 != 0x3f {
                let mut outline_pose = *visual_pose;
                outline_pose.translation.y = grip_y;
                let transform = country_core::utils::components::transform::Transform {
                    rotation: outline_pose.rotation,
                    translation: outline_pose.translation,
                    scale: outline_pose.scale,
                };
                let forward = source_info.dst_tag() < 2
                    && walls
                        .get(source_address.as_wall())
                        .is_some_and(|wall| wall.shape().tag() < 2);
                derived
                    .as_view()
                    .mesh_library()
                    .get_outline(
                        country_core::resources::walls::decorator_subtype::DecoratorLibKey::new(
                            subtype.0,
                            source_info.rank,
                        ),
                    )
                    .display(
                        &glam::Affine3A::from_mat4(transform.compute_affine()),
                        &mut stream,
                        request.decorator_id,
                        DecoratorRank(source_info.rank),
                        forward,
                        true,
                    );
            }
        }
        *local = Some(state);
    }
    let mut finished_state;
    let state = match phase {
        2 => return,
        3 => {
            let state = local.as_mut().unwrap();
            state.latest_position = request.hit.position;
            rotation_cache.rotation = move_quaternion_product(
                raycasts.camera.xz_projective_rotation().conjugate(),
                source_pose.rotation,
            );
            if matches!(state.part, MovingDecoratorPart::Part(_)) {
                window_hint.write(NotifyHintSystemWindow { flag: 1 });
            }
            if is_move_clutter(request.ty) {
                audio.write(PlayDecoratorSoundCmd::at(0x21, state.latest_position));
            }
            if !backup.created {
                backup.state = 1;
                return;
            }
            state
        }
        4 => local.as_mut().unwrap(),
        _ => {
            if is_move_clutter(request.ty) {
                moved_hint.write(NotifyHintSystemClutterWasMoved);
            }
            finished_state = local.take().unwrap();
            backup.consume_after_processing();
            if finished_state.part == MovingDecoratorPart::Whole
                && move_pixel_distance(
                    finished_state.latest_position,
                    request.hit.position,
                    &raycasts,
                ) < 2.0
            {
                if request.ty.can_change_seed() {
                    let mut patch = crate::apply_decorator_op::DecoratorChangePatch::rotation(
                        request.decorator_id,
                        source_address,
                        source_info.destination(),
                        source_pose.translation,
                    );
                    patch.destination = None;
                    patch.word_2c_delta = Some(1);
                    patch.seed_demo_mode = demo.0;
                    archive_move_address(source_address, &storage, &mut archivist, &archival);
                    crate::apply_decorator_op::apply_decorator_op(
                        &crate::apply_decorator_op::DecoratorOp::from_change(source_address, patch),
                        0,
                        source_address,
                        &mut storage,
                        Some(&mut archivist),
                        Some(&mut history),
                        &operations,
                    );
                    return;
                }
                if request.ty.0 != 0x3f {
                    return;
                }
                stairs_state
                    .try_set_endpoint(move_stair_endpoint(source_address, request.decorator_id));
                let fixed_height = finished_state.vertical_offset == 0.0
                    && storage
                        .decorator_info(request.decorator_id)
                        .is_some_and(|info| move_stair_height(info) >= 1.5);
                init_place.write(
                    super::ui_place_decorator::EvInitPlaceDecoratorInteractionState(
                        super::ui_place_decorator::PlaceDecoratorInteractionState {
                            height_constraint: Some(source_pose.translation.y),
                            height_offset: finished_state.vertical_offset,
                            last_height: source_pose.translation.y,
                            native_flag_10: false,
                            fixed_height,
                            progress: 0,
                        },
                    ),
                );
                let degree = stairs_state
                    .graph()
                    .neighbors(request.decorator_id.0)
                    .count();
                if degree < 2
                    && *mode
                        != country_core::systems::mode_manager::AppMode::Decorator(
                            country_core::systems::mode_manager::DecoratorType::Stairs,
                        )
                {
                    request_mode.write(country_core::systems::mode_manager::RequestAppModeChange(
                        country_core::systems::mode_manager::AppMode::Decorator(
                            country_core::systems::mode_manager::DecoratorType::Stairs,
                        ),
                    ));
                    continue_hint.write(NotifyHintSystemStartedStairsContinue);
                }
                return;
            }
            &mut finished_state
        }
    };
    // 原拆分在预览使用瞬时ID，释放时才直写正式生成器。失败不能反向消耗正式ID。
    let direct_ids = phase < 2;
    let demo = demo.0;
    let mut moving_id = request.decorator_id;
    let mut remaining_ids = Vec::new();
    let mut split_destination = None;
    if let MovingDecoratorPart::Part(part) = state.part {
        match source_info.dst_tag() {
            0 | 1 => {
                let split = {
                    let mut transient = ids.transient(direct_ids);
                    super::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part(
                        part,
                        &mut storage,
                        source_address,
                        request.decorator_id,
                        &walls,
                        &mut cached,
                        derived.as_view(),
                        &mut transient,
                        &mut archivist,
                        &mut history,
                        &operations,
                        &archival,
                        u8::from(phase >= 2),
                        demo,
                    )
                };
                moving_id = split.moving_id;
                remaining_ids = split.remaining_ids;
            }
            2 => {
                let roof = roofs
                    .get(walls.roof_entity(source_address.as_wall()).unwrap())
                    .unwrap();
                let split = {
                    let mut transient = ids.transient(direct_ids);
                    super::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part(
                        part,
                        &mut storage,
                        source_address,
                        request.decorator_id,
                        &source_pose,
                        derived.as_view(),
                        &mut transient,
                        &mut archivist,
                        Some(&mut history),
                        &operations,
                        &archival,
                        u8::from(phase >= 2),
                        roof,
                        demo,
                    )
                };
                moving_id = split.moving_id;
                remaining_ids = split.remaining_ids;
                split_destination = Some(split.destination);
            }
            _ => unreachable!("移动部件目的必须是墙或屋顶"),
        }
    }
    let moving_pose = if let Some(DecoratorDst::Roof(destination)) = split_destination {
        let entity = walls.roof_entity(source_address.as_wall()).unwrap();
        let roof = roofs.get(entity).unwrap();
        let volume = roof_volumes.get(entity).unwrap();
        CachedDecoratorTransform {
            rotation: source_pose.rotation,
            translation: volume
                .get_position_from_mesh_uv(destination.coord, roof.floor_y())
                .unwrap_or(source_pose.translation),
            scale: Vec3::ONE,
        }
    } else if state.part == MovingDecoratorPart::Whole {
        source_pose
    } else {
        cached.transform(moving_id.0)
    };
    let info = *storage.decorator_info(moving_id).unwrap();
    if phase == 3 && !remaining_ids.is_empty() {
        let mut lifted = moving_pose;
        lifted.translation += Vec3::new(
            0.0 * state.vertical_offset,
            state.vertical_offset,
            0.0 * state.vertical_offset,
        );
        if let Some(offset) = grab(lifted, &info, source_address) {
            state.grab_offset = offset;
            audio.write(PlayDecoratorSoundCmd::at(10, state.latest_position));
        }
        // 原637d失败也返回；成功只校正抓取并发拆分音，本帧不再执行Intent。
        return;
    }
    let (mut candidate, offset_hit) = move_candidate(
        &raycasts,
        cursor.0,
        state.grab_offset,
        request.ty,
        &info,
        moving_pose,
        rotation_cache.rotation,
    );
    let flat_only = move_hit_faces_away(
        raycasts.camera.eye_position(),
        offset_hit.position,
        offset_hit.normal,
    );
    if state.height_constraint.is_none() {
        state.height_constraint = Some(state.vertical_offset + moving_pose.translation.y);
    }
    let pixels = move_pixel_distance(state.latest_position, request.hit.position, &raycasts);
    let mut stair_height = None;
    if request.ty.0 == 0x3f {
        adjust_moving_stair_candidate(
            state,
            &mut candidate,
            &mut stair_height,
            &info,
            modifiers.disable_snapping(),
            &raycasts,
            cursor.0,
            &walls,
            &terrain,
            &offset_hit,
            moving_pose.translation.y,
        );
    }
    let excluded_ids: Vec<u32> = remaining_ids.into_iter().map(|id| id.0).collect();
    let result = match candidate.destination {
        RaycastDecoratorDst::Invalid => DecoratorInteractionIntent::UnsupportedTerrain {
            position: candidate.position,
        },
        _ => calculate_decorator_interaction_intent(
            &walls,
            &roofs,
            &stair_requests,
            candidate,
            moving_id,
            &info,
            &excluded_ids,
            &storage,
            derived.as_view(),
            &cached,
            source_address,
            !modifiers.disable_snapping(),
            flat_only,
            &mut stream,
            &world,
        ),
    };
    if !backup.created {
        return;
    }
    match result {
        DecoratorInteractionIntent::Accepted(accepted) => {
            if request.ty.0 == 0x3f {
                let from_wall = info.dst_tag() < 2;
                let to_wall = matches!(
                    accepted.destination,
                    super::move_ops::MoveDstCandidate::Wall { .. }
                );
                if from_wall && !to_wall {
                    detached_hint.write(NotifyHintSystemStairsDetachedFromWall);
                } else if from_wall && to_wall {
                    can_detach_hint.write(NotifyHintSystemStairsCanDetachFromWall);
                }
            }
            if matches!(
                accepted.destination,
                super::move_ops::MoveDstCandidate::Terrain(_)
            ) && !border.is_pos_inside(Vec2::new(accepted.position.x, accepted.position.z))
            {
                show_border.write(country_core::resources::glade_border::DisplayOutOfBorder);
                border_feedback
                    .write(country_core::resources::glade_border::FeedbackInputOutOfBorder);
                backup.state = 2;
                super::ui_place_decorator::show_decorator_icon(request.ty.0, cursor.0, &mut stream);
                deltas.move_velocity = 0.0;
                return;
            }
            if pixels < 2.0 {
                return;
            }
            backup.state = 1;
            apply_accepted_move(
                &accepted,
                stair_height,
                u8::from(phase >= 2),
                phase,
                &walls,
                &mut storage,
                &mut archivist,
                &archival,
                &mut history,
                &operations,
                &mut audio,
            );
            deltas.move_velocity = previous_position.map_or(0.0, |previous| {
                move_distance(accepted.position, previous) / time.delta_secs()
            });
            deltas.move_previous_position = Some(accepted.position);
        }
        DecoratorInteractionIntent::UnsupportedTerrain { position } => {
            if phase >= 2 {
                super::move_ops::show_decorator_remove_signifier(
                    moving_id,
                    &super::move_ops::RemoveSignifierSource {
                        position: cursor_ray.origin,
                        direction: cursor_ray.dir,
                    },
                    &position,
                    &mut stream,
                );
            }
            archive_move_address(source_address, &storage, &mut archivist, &archival);
            let op =
                crate::apply_decorator_op::DecoratorOp::remove_at(moving_id, info, position, false);
            crate::apply_decorator_op::apply_decorator_op(
                &op,
                u8::from(phase >= 2),
                source_address,
                &mut storage,
                Some(&mut archivist),
                Some(&mut history),
                &operations,
            );
        }
        DecoratorInteractionIntent::Rejected { .. } => {
            backup.state = 2;
            super::ui_place_decorator::show_decorator_icon(request.ty.0, cursor.0, &mut stream);
        }
    }
}

fn is_move_clutter(ty: DecoratorType) -> bool {
    let family = ty.0.wrapping_sub(0x38);
    family >= 8 || family == 6
}

fn move_stair_height(info: &DecoratorInfo) -> f32 {
    f32::from_le_bytes(info.extra().as_stair().0[..4].try_into().unwrap())
}

/// 原SSE按(y²+x²)+z²求距离；不滤除NaN、负零或零dt。
fn move_distance(a: Vec3, b: Vec3) -> f32 {
    let d = a - b;
    ((d.y * d.y + d.x * d.x) + d.z * d.z).sqrt()
}
fn move_pixel_distance(a: Vec3, b: Vec3, resources: &RaycastResources<'_>) -> f32 {
    // 原finish5ebf先(y²+x²)，再z²；原72xx速度也保同一求和次序。
    move_distance(a, b)
        * utils::geometry::ws_to_ss(
            resources.depth.view_depth,
            resources.camera.read_matrix(0),
            resources.window.as_vec2().y,
        )
}

/// 原5881..58cd/6662..66b1的相同Hamilton乘法，保两个部分和再相加。
fn move_quaternion_product(left: Quat, right: Quat) -> Quat {
    let a = left.to_array();
    let b = right.to_array();
    let x = [a[3], -a[2], a[1], -a[0]];
    let y = [a[2], a[3], -a[0], -a[1]];
    let z = [-a[1], a[0], a[3], -a[2]];
    Quat::from_array(std::array::from_fn(|i| {
        (z[i] * b[2] + y[i] * b[1]) + (x[i] * b[0] + a[i] * b[3])
    }))
}

fn decorator_grip_y(ty: DecoratorType, info: &DecoratorInfo, base_y: f32, cursor_y: f32) -> f32 {
    if ty.0 != 0x3f {
        return base_y;
    }
    match info.dst_tag() {
        0 | 1 => base_y,
        2 | 3 => {
            let height = move_stair_height(info);
            if 0.01 > height {
                return base_y + -0.01;
            }
            let bottom = base_y - height;
            // 原CMPNLESS：无序时也选top，不改成abs距离或min/max。
            if !(cursor_y <= (base_y + bottom) * 0.5) {
                base_y
            } else {
                bottom
            }
        }
        _ => unreachable!("原楼梯抓取只接受墙、屋顶或地形"),
    }
}

/// 原140ac8780/174B：两级仿射点投影；第二级忽略第一级w，不能合并矩阵。
fn move_project_to_screen(point: Vec3, resources: &RaycastResources<'_>) -> Vec2 {
    move_project_with_matrices(
        point,
        resources.window.as_vec2(),
        resources.camera.read_matrix(0x80),
        resources.camera.read_matrix(0),
    )
}

fn move_project_with_matrices(
    point: Vec3,
    size: Vec2,
    world_to_view: glam::Mat4,
    view_to_clip: glam::Mat4,
) -> Vec2 {
    let transform = |matrix: glam::Mat4, point: Vec3| {
        (matrix.z_axis * point.z + (matrix.y_axis * point.y + matrix.x_axis * point.x))
            + matrix.w_axis
    };
    let view = transform(world_to_view, point);
    let clip = transform(view_to_clip, view.truncate());
    Vec2::new(
        (clip.x / clip.w * 0.5 + 0.5) * size.x,
        (1.0 - (clip.y / clip.w * 0.5 + 0.5)) * size.y,
    )
}

#[allow(clippy::too_many_arguments)]
fn move_grab_offset(
    resources: &RaycastResources<'_>,
    cursor: Vec2,
    walls: &PublicWalls,
    entities: &country_core::resources::walls::public_walls::WallEntities,
    volumes: &Query<&country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,
    pose: CachedDecoratorTransform,
    info: &DecoratorInfo,
    address: DecoratorStorageAddr,
) -> Option<Vec2> {
    super::calculate_decorator_grab_offset::try_get_grab_offset_ss_if_valid(
        info,
        &country_core::utils::components::transform::Transform {
            rotation: pose.rotation,
            translation: pose.translation,
            scale: pose.scale,
        },
        address,
        walls,
        cursor,
        |position| move_project_to_screen(position, resources),
        |offset| {
            super::calculate_decorator_grab_offset::raycast_with_ss_offset(
                cursor,
                offset,
                resources.window.as_vec2(),
                resources.camera.read_matrix(0x40),
                resources.camera.read_matrix(0xc0),
                info.ty,
                &resources.raycast,
                &resources.terrain,
            )
        },
        |wall, coordinate| {
            let volume = volumes.get(entities.wall_entity(wall).unwrap()).unwrap();
            let (triangle, weights) =
                utils::geometry::position_from_mesh_uvs::get_triangle_coords_from_mesh_uv(
                    volume.triangle_uvs(),
                    Vec2::new(coordinate.u, coordinate.y),
                )?;
            let p = volume.default_tris()[triangle].points;
            Some(p[1] * weights.y + p[0] * weights.x + p[2] * weights.z)
        },
    )
}

fn move_hit_faces_away(eye: Vec3, point: Vec3, normal: Vec3) -> bool {
    let x = eye.x - point.x;
    let z = eye.z - point.z;
    let reciprocal = 1.0 / (z * z + x * x).sqrt();
    0.0 >= (z * reciprocal) * normal.z + (x * reciprocal) * normal.x
}

#[allow(clippy::too_many_arguments)]
fn move_candidate(
    resources: &RaycastResources<'_>,
    cursor: Vec2,
    offset: Vec2,
    ty: DecoratorType,
    info: &DecoratorInfo,
    pose: CachedDecoratorTransform,
    relative_rotation: Quat,
) -> (
    super::decorator_interaction_intent::DecoratorIntentCandidate,
    country_core::systems::collision::world::RaycastWorldResult,
) {
    use super::calculate_decorator_dst::{
        convert_raycast_hit_to_decorator_dst, RaycastDecoratorDst, RaycastHitDisc,
    };
    let hit = super::calculate_decorator_grab_offset::raycast_with_ss_offset(
        cursor,
        offset,
        resources.window.as_vec2(),
        resources.camera.read_matrix(0x40),
        resources.camera.read_matrix(0xc0),
        info.ty,
        &resources.raycast,
        &resources.terrain,
    );
    let tag = hit.handle[0] as u8;
    let source = match tag {
        0 => RaycastHitDisc::Wall(country_core::resources::walls::WallId(hit.handle[1])),
        2 => RaycastHitDisc::Roof {
            flag: (hit.handle[0] >> 8) as u8,
            sub: (hit.handle[0] >> 16) as u8,
            payload: hit.handle[1],
        },
        0xb => RaycastHitDisc::Terrain,
        0xd => RaycastHitDisc::StairTop((hit.handle[0] >> 32) | (hit.handle[1] << 32)),
        3 => RaycastHitDisc::Unsupported,
        5 | 9 | 0xc | 0xf | 0x10 => RaycastHitDisc::Collider,
        _ => RaycastHitDisc::Other,
    };
    let destination = convert_raycast_hit_to_decorator_dst(source, info);
    let forward_rotation = || {
        let forward =
            country_core::resources::walls::decorator::DecoratorRotationXZ::negative_z_from_quat(
                pose.rotation,
            );
        utils::xz_projective_rotation_from_forward(forward)
    };
    let rotation = match destination {
        RaycastDecoratorDst::Wall(_) => utils::xz_projective_rotation_from_forward(hit.normal),
        RaycastDecoratorDst::Roof { .. } if is_move_clutter(ty) => {
            move_quaternion_product(resources.camera.xz_projective_rotation(), relative_rotation)
        }
        RaycastDecoratorDst::Roof { .. } if ty.0 != 0x3f => {
            utils::xz_projective_rotation_from_forward(hit.normal)
        }
        RaycastDecoratorDst::Terrain if info.ty == 0x3d && info.dst_tag() != 3 => {
            forward_rotation()
        }
        RaycastDecoratorDst::Terrain if is_move_clutter(DecoratorType(info.ty)) => {
            move_quaternion_product(resources.camera.xz_projective_rotation(), relative_rotation)
        }
        RaycastDecoratorDst::Terrain => forward_rotation(),
        _ => pose.rotation,
    };
    (
        super::decorator_interaction_intent::DecoratorIntentCandidate {
            rotation,
            destination,
            position: hit.position,
            roof_coordinate: (hit.discriminant == 1).then(|| Vec2::from_array(hit.extra)),
        },
        hit,
    )
}

#[allow(clippy::too_many_arguments)]
fn adjust_moving_stair_candidate(
    state: &mut MoveDecorationInteractionState,
    candidate: &mut super::decorator_interaction_intent::DecoratorIntentCandidate,
    height: &mut Option<f32>,
    info: &DecoratorInfo,
    disable_snapping: bool,
    resources: &RaycastResources<'_>,
    cursor: Vec2,
    walls: &PublicWalls,
    terrain: &country_core::resources::terrain_height::TerrainHeightsData<
        country_core::resources::terrain_height::TerrainHeightSsbo,
    >,
    hit: &country_core::systems::collision::world::RaycastWorldResult,
    current_base_y: f32,
) {
    use super::calculate_decorator_dst::RaycastDecoratorDst;
    let bottom_grab = state.vertical_offset != 0.0 && move_stair_height(info) >= 1.5;
    let detach = if info.dst_tag() < 2 {
        disable_snapping
    } else {
        disable_snapping || state.original_base_y > candidate.position.y || bottom_grab
    };
    let free_plane = if hit.handle[0] as u8 == 2 {
        detach || (hit.handle[0] >> 8) as u8 != 1
    } else {
        detach
    };
    let on_roof_or_ground = matches!(
        candidate.destination,
        RaycastDecoratorDst::Roof { .. } | RaycastDecoratorDst::Terrain
    );
    if free_plane || on_roof_or_ground {
        state.height_constraint = None;
    } else {
        state.height_constraint = Some(candidate.position.y);
    }
    let desired_height = if disable_snapping {
        state.original_base_y + state.vertical_offset
    } else {
        state
            .height_constraint
            .unwrap_or(current_base_y + state.vertical_offset)
    };
    if !(bottom_grab || free_plane || on_roof_or_ground) {
        return;
    }
    let ray = utils::geometry::Ray::from_mouse_ss(
        cursor + state.grab_offset,
        resources.window.as_vec2(),
        resources.camera.read_matrix(0x40),
        resources.camera.read_matrix(0xc0),
    );
    let old = *candidate;
    let mut point = candidate.position;
    if let Some(found) = find_stair_placement_candidate(
        resources,
        walls,
        terrain,
        state.vertical_offset,
        (!matches!(candidate.destination, RaycastDecoratorDst::Invalid)).then_some(&old),
        free_plane,
        &ray,
        desired_height,
        &mut point,
    ) {
        *candidate = found.candidate;
        *height = Some(found.height);
    }
}

fn move_stair_endpoint(
    address: DecoratorStorageAddr,
    id: DecoratorId,
) -> country_core::resources::stairs::stairs_state::ObservedStairEndpoint {
    let mut bytes = [0; 24];
    let (tag, wall) = match address {
        DecoratorStorageAddr::Wall(wall) => (0u64, wall.0),
        DecoratorStorageAddr::Terrain => (1u64, 0),
        DecoratorStorageAddr::Stair => (2u64, 0),
    };
    bytes[..8].copy_from_slice(&tag.to_le_bytes());
    bytes[8..16].copy_from_slice(&wall.to_le_bytes());
    bytes[16..20].copy_from_slice(&id.0.to_le_bytes());
    country_core::resources::stairs::stairs_state::ObservedStairEndpoint::from_observed(bytes)
}

fn archive_move_address(
    address: DecoratorStorageAddr,
    storage: &DecoratorStorage,
    archivist: &mut country_core::resources::walls::decorator_archivist::LiveDecoratorArchivist,
    resources: &country_core::resources::walls::decorator_archivist::DecoratorArchivalResources<'_>,
) {
    archivist.archive_current_decorators_at_with_resources(
        address,
        storage,
        resources,
        "ui_move_decorator",
    );
}

/// 原1407a7a50：合并先移除被移动ID，再改目标ID；迁移须归档源与目标。
#[allow(clippy::too_many_arguments)]
fn apply_accepted_move(
    accepted: &super::decorator_interaction_intent::AcceptedDecoratorIntent,
    stair_height: Option<f32>,
    kind: u8,
    phase: u8,
    walls: &PublicWalls,
    storage: &mut DecoratorStorage,
    archivist: &mut country_core::resources::walls::decorator_archivist::LiveDecoratorArchivist,
    archival: &country_core::resources::walls::decorator_archivist::DecoratorArchivalResources<'_>,
    history: &mut crate::deferred_decorator_history::DeferredDecoratorEditHistory,
    events: &country_core::resources::walls::decorator::DeferredDecoratorOpEvents,
    audio: &mut EventWriter<country_core::systems::audio::PlayDecoratorSoundCmd>,
) {
    use crate::apply_decorator_op::{apply_decorator_op, DecoratorChangePatch, DecoratorOp};
    let source = *storage.decorator_info(accepted.decorator_id).unwrap();
    let mut moved = source;
    super::move_ops::calculate_move_dst(&mut moved, accepted.destination, walls);
    archive_move_address(accepted.storage, storage, archivist, archival);
    archive_move_address(accepted.target_storage, storage, archivist, archival);
    let id = if let Some(merge) = accepted.merge {
        let op = DecoratorOp::remove_at(accepted.decorator_id, source, accepted.position, false);
        apply_decorator_op(
            &op,
            kind,
            accepted.storage,
            storage,
            Some(archivist),
            Some(history),
            events,
        );
        merge.target
    } else {
        accepted.decorator_id
    };
    let mut patch = DecoratorChangePatch::rotation(
        id,
        accepted.target_storage,
        moved.destination(),
        accepted.position,
    );
    patch.rank = Some(accepted.merge.map_or(accepted.rank, |merge| merge.rank));
    patch.extra_high = stair_height.map(f32::to_bits);
    let source_address = if accepted.merge.is_some() {
        accepted.target_storage
    } else {
        accepted.storage
    };
    let op = DecoratorOp::from_change(source_address, patch);
    apply_decorator_op(
        &op,
        kind,
        source_address,
        storage,
        Some(archivist),
        Some(history),
        events,
    );
    if phase == 3 {
        audio.write(country_core::systems::audio::PlayDecoratorSoundCmd::at(
            9,
            accepted.position,
        ));
    }
}

/// 原1407a4e04..5176及闭包1407a82f0：父装饰悬停时标注实际自动杂物。
fn highlight_child_autoclutter(
    child: &country_core::systems::ui_clutter::ui_autoclutter::DisplayChildAutoClutterInfo,
    stream: &mut country_core::systems::ui::signifiers::signifier_create::UiSignifierStream,
) {
    use country_core::systems::ui::signifiers::signifier_factory::{
        SignifierEntityBuilder, SignifierFactory, SignifierMesh,
    };
    use country_core::utils::debug_viz::DebugLine;
    let q = child.transform.rotation;
    let u = Vec3::new(q.x, q.y, q.z);
    let dot = (u.y * 0.0 + u.x) + u.z * 0.0;
    let length_squared = (u.y * u.y + u.x * u.x) + u.z * u.z;
    let base = Vec3::X * (q.w * q.w - length_squared);
    let cross = Vec3::new(u.y * 0.0 - u.z * 0.0, u.z - u.x * 0.0, u.x * 0.0 - u.y);
    let forward = cross * (q.w + q.w) + (u * (dot + dot) + base);
    let reciprocal = 1.0 / (forward.z * forward.z + forward.x * forward.x).sqrt();
    let mut bounds = child.mesh.bounds;
    bounds.set_orientation_xz(Vec2::new(forward.x * reciprocal, forward.z * reciprocal));
    bounds.translate(child.transform.translation);
    let ease = utils::easing_functions::EasingFunction::BackOut.apply(child.progress);
    let radius = (((bounds.footprint.size.x + bounds.footprint.size.y) * 0.5) * 0.5 + 0.1) * ease;
    let position = child.transform.translation;
    let circle = utils::geometry::Circle2d::new(Vec2::new(position.x, position.z), radius);
    DebugLine::new(circle.as_points3(0.1, position.y).collect())
        .line_width(1.0)
        .dashed()
        .show(stream);
    let small_radius = radius + -0.05;
    let small_radius = if small_radius < 0.5 {
        small_radius
    } else {
        0.5
    } * ease;
    let name = format!("child_auto_clutter_{position:?}_{small_radius:?}");
    let factory = SignifierFactory::from_closure(
        &name,
        std::sync::Arc::new(move |meshes, shaders, commands| {
            let entity = SignifierEntityBuilder::new(SignifierMesh::Empty)
                .animation_duration(0.0)
                .build(meshes, shaders, commands);
            commands.entity(entity).insert((
                country_core::components::Ui2dWsExtent {
                    extent: Vec2::splat(small_radius),
                    position,
                },
                country_core::components::Ui2dImage::new("link"),
            ));
            vec![entity]
        }),
    );
    stream.write_factory(&factory);
}

/// 原1407a7e80的有效输出：候选姿态与相对于目的表面的楼梯高度。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StairPlacementCandidate {
    pub candidate: super::decorator_interaction_intent::DecoratorIntentCandidate,
    pub height: f32,
}

/// 原MAXSS加无序掩码：第二参数为NaN时返回第一参数，相等时保留第二参数位型。
fn stair_max_number(first: f32, second: f32) -> f32 {
    if second.is_nan() || first > second { first } else { second }
}

/// 原MAXSS(0,h)→MINSS(100,h)：保留输入NaN及负零，不能用f32::max/min。
fn stair_clamp_height(height: f32) -> f32 {
    let nonnegative = if height < 0.0 { 0.0 } else { height };
    if nonnegative > 100.0 { 100.0 } else { nonnegative }
}

/// 原1407a7e80：目标平面截断地形射线，随后按原目的/禁用吸附分支恢复屋顶或地形。
/// 单独输出的世界点始终是地形夹取后的点；屋顶UV失败只改变目的与相对高度。
#[allow(clippy::too_many_arguments)]
pub fn find_stair_placement_candidate(
    resources: &RaycastResources<'_>,
    walls: &country_core::resources::walls::public_walls::PublicWalls,
    terrain: &country_core::resources::terrain_height::TerrainHeightsData<
        country_core::resources::terrain_height::TerrainHeightSsbo,
    >,
    height_offset: f32,
    previous: Option<&super::decorator_interaction_intent::DecoratorIntentCandidate>,
    disable_snapping: bool,
    ray: &utils::geometry::ray::Ray,
    desired_height: f32,
    position: &mut glam::Vec3,
) -> Option<StairPlacementCandidate> {
    use country_core::resources::walls::WallId;
    use super::calculate_decorator_dst::RaycastDecoratorDst;
    use super::decorator_interaction_intent::DecoratorIntentCandidate;
    use glam::{Quat, Vec2, Vec3};

    let mut plane_height = desired_height;
    if !disable_snapping {
        if let Some(DecoratorIntentCandidate {
            destination: RaycastDecoratorDst::Roof { payload, .. }, ..
        }) = previous {
            let floor = walls.get(WallId(*payload)).unwrap().flat_roof_y();
            plane_height = stair_max_number(floor + height_offset, plane_height);
        }
    }
    let (plane_t, mut target) = utils::geometry::plane::Plane {
        point: Vec3::new(0.0, plane_height, 0.0), normal: Vec3::Y,
    }.intersect_ray(*ray, 0.0)?;
    // 1407a7fc3..7fea传入的是平面交点TOI；不能以无限地形射线穿过目标平面。
    if let Some(hit) = resources.terrain.intersect_ray(*ray, plane_t) {
        target = hit.position;
    }
    let terrain_height = terrain.sample_at_world_xz(target.x, target.z);
    target.y = stair_max_number(terrain_height, target.y);
    *position = target;
    let mut relative_height = plane_height - height_offset;
    let mut destination = RaycastDecoratorDst::Terrain;
    let mut roof_coordinate = None;
    if disable_snapping {
        let lift = 100.0 - height_offset;
        // 原SIMD先执行0*lift；无穷/NaN输入不得被常量零替代。
        let origin = Vec3::new(target.x + 0.0 * lift, target.y + lift, target.z + 0.0 * lift);
        let hit = resources.raycast.raycast_w_terrain(
            &resources.terrain, origin, Vec3::new(-0.0, -1.0, -0.0), 0x22,
        );
        let source = hit.handle[0];
        let mut surface_height = terrain_height;
        // discriminant 1是有UV的真实命中；不能使用仅判0的旧is_hit helper。
        if !hit.is_miss() && source as u8 == 2 && source & 0x100 != 0 {
            destination = RaycastDecoratorDst::Roof { sub: 0, payload: hit.handle[1] };
            if hit.discriminant == 1 { roof_coordinate = Some(Vec2::from_array(hit.extra)); }
            surface_height = hit.position.y;
        }
        relative_height -= surface_height;
    } else if let Some(DecoratorIntentCandidate {
        destination: RaycastDecoratorDst::Roof { sub, payload }, ..
    }) = previous {
        let floor = walls.get(WallId(*payload)).unwrap().flat_roof_y();
        match country_core::resources::roofs::roof_shape::roof_uv_at_ws_pos(
            walls, &resources.raycast, WallId(*payload), Vec2::new(target.x, target.z),
        ) {
            Ok(uv) => {
                destination = RaycastDecoratorDst::Roof { sub: *sub, payload: *payload };
                roof_coordinate = Some(uv);
                relative_height -= floor;
            }
            Err(_) => relative_height -= terrain_height,
        }
    } else {
        relative_height -= terrain_height;
    }
    Some(StairPlacementCandidate {
        candidate: DecoratorIntentCandidate {
            rotation: previous.map_or(Quat::IDENTITY, |candidate| candidate.rotation),
            destination,
            position: target,
            roof_coordinate,
        },
        height: stair_clamp_height(relative_height),
    })
}

#[cfg(test)]
mod stair_placement_tests {
    use super::*;

    fn world() -> World {
        use country_core::{resources::{terrain_height::{TerrainHeightsData, TerrainHeightSsbo},
            window::WindowSize, read_mouse_depth_value::MouseDepthValue},
            systems::{camera_rig::MainCamera, collision::world::RaycastWorld}};
        let mut world = World::new();
        let mut terrain = TerrainHeightsData::<TerrainHeightSsbo>::new();
        terrain.set_height_range(0.0, 1.0);
        world.insert_resource(terrain);
        world.insert_resource(RaycastWorld::default());
        world.insert_resource(MainCamera::default());
        world.insert_resource(WindowSize::new(1280, 720));
        world.insert_resource(MouseDepthValue::default());
        world
    }

    #[test]
    fn actual_resources_keep_plane_bound_terrain_clamp_rotation_and_failed_output() {
        use country_core::resources::walls::public_walls::PublicWalls;
        use super::super::{calculate_decorator_dst::RaycastDecoratorDst,
            decorator_interaction_intent::DecoratorIntentCandidate};
        use glam::{Quat, Vec3};
        let mut world = world();
        let mut params = bevy_ecs::system::SystemState::<RaycastResources>::new(&mut world);
        let resources = params.get(&world);
        let walls = PublicWalls::default();
        let ray = utils::geometry::ray::Ray { origin: Vec3::new(1.0, 10.0, 2.0), direction: Vec3::NEG_Y };
        let previous = DecoratorIntentCandidate { rotation: Quat::from_rotation_y(0.7),
            destination: RaycastDecoratorDst::Terrain, position: Vec3::ZERO, roof_coordinate: None };
        let mut position = Vec3::splat(99.0);
        let result = find_stair_placement_candidate(&resources, &walls, &resources.terrain,
            0.5, Some(&previous), false, &ray, 3.0, &mut position).unwrap();
        // 地形在平面之后，不能把目标从y3拉到地形；只扣除原height_offset。
        assert_eq!(position, Vec3::new(1.0, 3.0, 2.0));
        assert_eq!(result.height, 2.5);
        assert_eq!(result.candidate.rotation, previous.rotation);
        assert_eq!(result.candidate.destination, RaycastDecoratorDst::Terrain);
        let below = find_stair_placement_candidate(&resources, &walls, &resources.terrain,
            0.0, None, false, &ray, -2.0, &mut position).unwrap();
        assert!(position.y >= 0.0 && position.y < 0.001);
        assert_eq!(below.height, 0.0);
        assert_eq!(below.candidate.rotation, Quat::IDENTITY);
        let fence = find_stair_placement_candidate(&resources, &walls, &resources.terrain,
            0.5, None, true, &ray, 3.0, &mut position).unwrap();
        assert_eq!(fence.candidate.destination, RaycastDecoratorDst::Terrain);
        assert_eq!(fence.height, 2.5);
        position = Vec3::splat(99.0);
        assert!(find_stair_placement_candidate(&resources, &walls, &resources.terrain,
            0.0, None, false, &ray, 11.0, &mut position).is_none());
        assert_eq!(position, Vec3::splat(99.0));
    }

    #[test]
    fn original_height_number_and_clamp_instructions_keep_raw_edges() {
        // 原二进制两个完整数值段直接执行；并未注入候选算法的期望结果。
        let fixture = include_str!(concat!(env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/1407a7e80-stair-height-native.txt"));
        for (index, row) in fixture.lines().enumerate() {
            let words: Vec<_> = row.split_whitespace()
                .map(|word| u32::from_str_radix(word, 16).unwrap()).collect();
            let first = f32::from_bits(words[1]);
            let actual = if words[0] == 0 {
                stair_max_number(first, f32::from_bits(words[2]))
            } else {
                stair_clamp_height(first)
            };
            assert_eq!(actual.to_bits(), words[3], "原指令case {index}");
        }
    }
}

#[cfg(test)]
mod original_move_math_tests {
    use super::*;
    #[allow(dead_code)]
    mod original {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m01-move-math-native-cases.rs"
        ));
    }
    fn same(actual: f32, expected: u32, context: &str) {
        if f32::from_bits(expected).is_nan() {
            assert!(actual.is_nan(), "{context}");
        } else {
            assert_eq!(actual.to_bits(), expected, "{context}");
        }
    }
    #[test]
    fn original_quaternion_product_128_complete_instruction_blocks() {
        for &(a, b, expected) in original::product_cases() {
            let actual = move_quaternion_product(
                Quat::from_array(a.map(f32::from_bits)),
                Quat::from_array(b.map(f32::from_bits)),
            );
            for (actual, expected) in actual.to_array().into_iter().zip(expected) {
                same(actual, expected, "完整四元数乘法");
            }
        }
    }
    #[test]
    fn original_projection_128_full_functions() {
        for &(matrices, point, size, expected) in original::projection_cases() {
            let a = matrices.map(f32::from_bits);
            let projection = glam::Mat4::from_cols_array(a[..16].try_into().unwrap());
            let view = glam::Mat4::from_cols_array(a[32..48].try_into().unwrap());
            let actual = move_project_with_matrices(
                Vec3::from_array(point.map(f32::from_bits)),
                Vec2::from_array(size.map(f32::from_bits)),
                view,
                projection,
            );
            for (actual, expected) in actual.to_array().into_iter().zip(expected) {
                same(actual, expected, "原两级屏幕投影");
            }
        }
    }
    #[test]
    fn original_grip_height_108_full_decision_blocks() {
        for &(ty, tag, height, base, cursor, expected) in original::grip_cases() {
            let mut info = DecoratorInfo {
                ty,
                ..Default::default()
            };
            info.u20 = (info.u20 & !0xff_0000) | (u32::from(tag) << 16);
            let mut extra = [0; 20];
            extra[..4].copy_from_slice(&height.to_le_bytes());
            info.set_extra(
                country_core::resources::walls::decorator::DecoratorExtraInfo::Stair(
                    country_core::resources::walls::decorator::StairExtra(extra),
                ),
            );
            same(
                decorator_grip_y(
                    DecoratorType(ty),
                    &info,
                    f32::from_bits(base),
                    f32::from_bits(cursor),
                ),
                expected,
                "原楼梯顶/底握点",
            );
        }
    }
}

#[cfg(test)]
mod original_move_world_tests {
    use super::*;
    use country_core::resources::{
        glade_border::{BorderShape, DisplayOutOfBorder, FeedbackInputOutOfBorder, GladeBorder},
        mouse_terrain_raycast::{CameraCursorRay, CursorPositionSS},
        onboarding::*,
        read_mouse_depth_value::MouseDepthValue,
        terrain_height::{TerrainHeightSsbo, TerrainHeightsData},
        walls::{
            decorator::{
                DecoratorDst, DecoratorIdGen, DeferredDecoratorOpEvents, TerrainDecoratorDst,
            },
            decorator_archivist::{LiveColliderPayload, LiveDecoratorArchivist},
            decorator_collision::DecoratorPhysicsColliders,
            decorator_subtype::{DecoratorMeshLibrary, DecoratorSubtype, DerivedDecoratorSubtypes},
            public_walls::WallEntities,
        },
        window::WindowSize,
    };
    use country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter;
    use country_core::systems::{
        audio::PlayDecoratorSoundCmd,
        backend::input::mod_key::Modifiers,
        camera_rig::MainCamera,
        collision::world::{RaycastWorld, RaycastWorldResult},
        mode_manager::{AppMode, RequestAppModeChange},
        ui::resources::ui_state::{UiInteraction, UiState, WidgetId},
        ui::signifiers::signifier_create::UiSignifierStream,
    };
    fn fixture() -> (World, Schedule) {
        let mut world = World::new();
        world.insert_resource(bevy_time::Time::<()>::default());
        world
            .resource_mut::<bevy_time::Time<()>>()
            .advance_by(std::time::Duration::from_secs_f32(1.0 / 60.0));
        world.init_resource::<Modifiers>();
        world.init_resource::<Events<super::super::ui_place_decorator::EvInitPlaceDecoratorInteractionState>>();
        world.init_resource::<Events<RequestAppModeChange>>();
        world.init_resource::<Events<NotifyHintSystemWindow>>();
        world.init_resource::<Events<NotifyHintSystemClutterWasMoved>>();
        world.init_resource::<Events<NotifyHintSystemStartedStairsContinue>>();
        world.init_resource::<Events<NotifyHintSystemStairsDetachedFromWall>>();
        world.init_resource::<Events<NotifyHintSystemStairsCanDetachFromWall>>();
        world.init_resource::<Events<DisplayOutOfBorder>>();
        world.init_resource::<Events<FeedbackInputOutOfBorder>>();
        world.init_resource::<Events<PlayDecoratorSoundCmd>>();
        world.insert_resource(AppMode::default());
        world.init_resource::<DecoratorAffordanceDispatcher>();
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<LiveDecoratorArchivist>();
        world.init_resource::<DeferredDecoratorOpEvents>();
        world.insert_resource(country_core::systems::backend::input::translate_input::DemoModeEnabled(false));
        world.init_resource::<DerivedDecoratorSubtypes>();
        world
            .resource_mut::<DerivedDecoratorSubtypes>()
            .insert(17, DecoratorSubtype(24));
        world.init_resource::<DecoratorMeshLibrary>();
        world.init_resource::<crate::blueprint::DecoratorBlueprints>();
        world.init_resource::<DecoratorPhysicsColliders>();
        world.insert_resource(country_core::resources::stairs::stairs_state::StairsState::new());
        world.init_resource::<PublicWalls>();
        world.init_resource::<WallEntities>();
        world.init_resource::<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>();
        world.init_resource::<crate::deferred_decorator_history::DeferredDecoratorEditHistory>();
        world.init_resource::<MoveDecoratorDeltaPositions>();
        world.init_resource::<DecoratorIdGen>();
        world.init_resource::<CachedDecoratorTransforms>();
        world.init_resource::<UiSignifierStream>();
        world.init_resource::<crate::backup::DecoratorBackup>();
        world.insert_resource(GladeBorder(BorderShape::Rectangle(
            utils::geometry::Rectangle2d::new_centered_axis_aligned(Vec2::splat(130.0)),
        )));
        world
            .init_resource::<country_core::resources::clutter_meshes::DecoratorAutoClutterChildren>(
            );
        world.insert_resource(RaycastWorld::new());
        let mut terrain = TerrainHeightsData::<TerrainHeightSsbo>::new();
        terrain.set_height_range(0.0, 0.0);
        world.insert_resource(terrain);
        world.insert_resource(WindowSize::new(1280, 720));
        let mut camera = MainCamera::new(1280.0 / 720.0);
        let inverse_view = glam::Mat4::from_rotation_translation(
            Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            Vec3::new(0.0, 5.0, 5.0),
        );
        camera.write_matrix(0xc0, &inverse_view);
        camera.write_matrix(0x80, &inverse_view.inverse());
        world.insert_resource(camera);
        world.insert_resource(MouseDepthValue {
            view_depth: 7.0,
            ..Default::default()
        });
        world.insert_resource(CursorPositionSS(Vec2::new(640.0, 360.0)));
        world.insert_resource(CameraCursorRay::new());
        world.insert_resource(UiState::at_startup());
        update_frame(&mut world, 2, Vec2::new(640.0, 360.0));
        let position = world
            .resource::<DecoratorAffordanceDispatcher>()
            .movement()
            .unwrap()
            .request
            .hit
            .position;
        let mut info = DecoratorInfo {
            ty: 0,
            rank: 1,
            state: 0,
            ..Default::default()
        };
        info.set_destination(DecoratorDst::Terrain(TerrainDecoratorDst {
            position_xz: Vec2::new(position.x, position.z),
            yaw: country_core::resources::walls::decorator::DecoratorRotationXZ(0.0),
        }));
        world
            .resource_mut::<DecoratorStorage>()
            .apply_decorator_place(DecoratorStorageAddr::Terrain, DecoratorId(17), &info);
        world.resource_mut::<CachedDecoratorTransforms>().insert(
            17,
            CachedDecoratorTransform {
                rotation: Quat::IDENTITY,
                translation: position,
                scale: Vec3::ONE,
            },
        );
        let mut schedule = Schedule::default();
        schedule.add_systems(
            (
                ui_move_decorator,
                crate::backup::create_or_restore_decorator_backup::<LiveColliderPayload>,
            )
                .chain(),
        );
        (world, schedule)
    }
    fn update_frame(world: &mut World, phase: u8, cursor: Vec2) {
        *world.resource_mut::<CursorPositionSS>() = CursorPositionSS(cursor);
        let camera = world.resource::<MainCamera>();
        let ray = utils::geometry::Ray::from_mouse_ss(
            cursor,
            Vec2::new(1280.0, 720.0),
            camera.read_matrix(0x40),
            camera.read_matrix(0xc0),
        );
        let position = world
            .resource::<TerrainHeightsData<TerrainHeightSsbo>>()
            .intersect_ray(ray, f32::MAX)
            .unwrap()
            .position;
        *world.resource_mut::<CameraCursorRay>() = CameraCursorRay {
            origin: ray.origin,
            dir: ray.direction,
        };
        let hit = RaycastWorldResult {
            handle: [0xb, 0, 0],
            position,
            normal: Vec3::Y,
            toi: 0.0,
            discriminant: 0,
            extra: [0.0; 2],
        };
        let ty = world
            .resource::<DecoratorStorage>()
            .decorator_info(DecoratorId(17))
            .map_or(0, |info| info.ty);
        world
            .resource_mut::<DecoratorAffordanceDispatcher>()
            .set_move(RequestMoveClutter {
                widget: WidgetId(17),
                hit,
                decorator_id: DecoratorId(17),
                ty: DecoratorType(ty),
            });
        let mut ui = UiState::new(UiInteraction {
            native_tag: 3,
            native_phase: phase,
            widget: WidgetId(17),
        });
        ui.set_intersection_tag(0);
        world.insert_resource(ui);
    }

    fn assert_real_diff_undo_redo(
        world: &mut World,
        id: DecoratorId,
        before: DecoratorInfo,
        after: DecoratorInfo,
        before_address: DecoratorStorageAddr,
        after_address: DecoratorStorageAddr,
    ) {
        use country_core::resources::history::decorator_edit::DecoratorHistoryState;
        use country_core::resources::walls::decorator_storage::DecoratorColliderMaps;
        let mut storage = world.remove_resource::<DecoratorStorage>().unwrap();
        let diff = world
            .resource_mut::<LiveDecoratorArchivist>()
            .extract_diff(&storage, false);
        assert!(!diff.is_empty());
        let mut physics = world.remove_resource::<RaycastWorld>().unwrap();
        let mut colliders = world
            .remove_resource::<DecoratorPhysicsColliders>()
            .unwrap();
        let mut ids = world.remove_resource::<DecoratorIdGen>().unwrap();
        let mut maps = DecoratorColliderMaps {
            world: &mut physics,
            collision_a: &mut colliders.first,
            collision_b: &mut colliders.second,
        };
        let events = world.resource::<DeferredDecoratorOpEvents>();
        storage.restore_decorators(&diff, true, &mut ids, &mut maps, events);
        assert_eq!(storage.lookup_storage(id), Some(before_address));
        assert_eq!(
            DecoratorHistoryState::from_info(storage.decorator_info(id).unwrap()),
            DecoratorHistoryState::from_info(&before)
        );
        storage.restore_decorators(&diff, false, &mut ids, &mut maps, events);
        assert_eq!(storage.lookup_storage(id), Some(after_address));
        assert_eq!(
            DecoratorHistoryState::from_info(storage.decorator_info(id).unwrap()),
            DecoratorHistoryState::from_info(&after)
        );
        drop(maps);
        world.insert_resource(storage);
        world.insert_resource(physics);
        world.insert_resource(colliders);
        world.insert_resource(ids);
    }
    #[test]
    fn real_main_hover_start_drag_commit_and_rejected_backup_use_authoritative_owners() {
        let (mut world, mut schedule) = fixture();
        let initial = *world
            .resource::<DecoratorStorage>()
            .decorator_info(DecoratorId(17))
            .unwrap();
        world
            .resource_mut::<MoveDecoratorDeltaPositions>()
            .move_previous_position = Some(Vec3::ONE);
        schedule.run(&mut world);
        assert!(world
            .resource::<MoveDecoratorDeltaPositions>()
            .move_previous_position
            .is_none());
        assert_eq!(
            *world
                .resource::<DecoratorStorage>()
                .decorator_info(DecoratorId(17))
                .unwrap(),
            initial
        );
        update_frame(&mut world, 3, Vec2::new(640.0, 360.0));
        schedule.run(&mut world);
        assert!(world.resource::<crate::backup::DecoratorBackup>().created);
        assert_eq!(
            world
                .resource::<Events<PlayDecoratorSoundCmd>>()
                .iter_current_update_events()
                .filter(|event| event.kind == 0x21)
                .count(),
            1
        );
        update_frame(&mut world, 4, Vec2::new(800.0, 360.0));
        schedule.run(&mut world);
        let moved = *world
            .resource::<DecoratorStorage>()
            .decorator_info(DecoratorId(17))
            .unwrap();
        assert_ne!(moved, initial);
        assert!(world
            .resource::<MoveDecoratorDeltaPositions>()
            .move_previous_position
            .is_some());
        assert!(
            !world
                .resource::<crate::deferred_decorator_history::DeferredDecoratorEditHistory>()
                .pending
        );
        update_frame(&mut world, 4, Vec2::new(100000.0, 360.0));
        let outside = world
            .resource::<DecoratorAffordanceDispatcher>()
            .movement()
            .unwrap()
            .request
            .hit
            .position;
        assert!(!world
            .resource::<GladeBorder>()
            .is_pos_inside(Vec2::new(outside.x, outside.z)));
        schedule.run(&mut world);
        assert_eq!(
            *world
                .resource::<DecoratorStorage>()
                .decorator_info(DecoratorId(17))
                .unwrap(),
            moved
        );
        assert_eq!(
            world
                .resource::<MoveDecoratorDeltaPositions>()
                .move_velocity,
            0.0
        );
        assert_eq!(world.resource::<Events<DisplayOutOfBorder>>().len(), 1);
        update_frame(&mut world, 1, Vec2::new(810.0, 360.0));
        schedule.run(&mut world);
        let history =
            world.resource::<crate::deferred_decorator_history::DeferredDecoratorEditHistory>();
        assert!(history.pending);
        assert!(!history.records.is_empty());
        assert!(!world.resource::<crate::backup::DecoratorBackup>().created);
        assert_eq!(world.resource::<DecoratorIdGen>().peek(), DecoratorId(0));
        assert_eq!(
            world
                .resource::<Events<NotifyHintSystemClutterWasMoved>>()
                .len(),
            1
        );
        assert_eq!(
            world
                .resource::<DecoratorStorage>()
                .lookup_storage(DecoratorId(17)),
            Some(DecoratorStorageAddr::Terrain)
        );
        let committed = *world
            .resource::<DecoratorStorage>()
            .decorator_info(DecoratorId(17))
            .unwrap();
        assert_real_diff_undo_redo(
            &mut world,
            DecoratorId(17),
            initial,
            committed,
            DecoratorStorageAddr::Terrain,
            DecoratorStorageAddr::Terrain,
        );
    }

    #[test]
    fn accepted_move_preserves_source_target_domains_and_merge_event_order_through_real_replay() {
        use super::super::decorator_interaction_intent::{
            AcceptedDecoratorIntent, DecoratorIntentMerge,
        };
        use bevy_ecs::system::SystemState;
        use country_core::resources::walls::decorator::{DecoratorRotationXZ, RoofDecoratorDst};
        use country_core::resources::walls::WallId;
        for merge in [false, true] {
            let (mut world, _) = fixture();
            let source_address = if merge {
                DecoratorStorageAddr::Wall(WallId(1))
            } else {
                DecoratorStorageAddr::Terrain
            };
            let target_address = DecoratorStorageAddr::Wall(WallId(2));
            let mut source = *world
                .resource::<DecoratorStorage>()
                .decorator_info(DecoratorId(17))
                .unwrap();
            if merge {
                world
                    .resource_mut::<DecoratorStorage>()
                    .apply_decorator_remove(DecoratorStorageAddr::Terrain, DecoratorId(17));
                source.ty = 0x38;
                source.set_extra(
                    country_core::resources::walls::decorator::DecoratorExtraInfo::Window(
                        country_core::resources::walls::decorator::WindowExtra(5),
                    ),
                );
                source.set_destination(DecoratorDst::Roof(RoofDecoratorDst {
                    coord: Vec2::new(0.3, 0.6),
                    yaw: DecoratorRotationXZ(0.5),
                }));
                world
                    .resource_mut::<DecoratorStorage>()
                    .apply_decorator_place(source_address, DecoratorId(17), &source);
                world
                    .resource_mut::<DecoratorStorage>()
                    .apply_decorator_place(target_address, DecoratorId(19), &source);
            }
            let accepted = AcceptedDecoratorIntent {
                storage: source_address,
                target_storage: target_address,
                destination: super::super::move_ops::MoveDstCandidate::Roof(RoofDecoratorDst {
                    coord: Vec2::new(0.6, 0.4),
                    yaw: DecoratorRotationXZ(0.3),
                }),
                merge: merge.then_some(DecoratorIntentMerge {
                    target: DecoratorId(19),
                    rank: 2,
                }),
                decorator_id: DecoratorId(17),
                position: Vec3::new(1.0, 2.0, 3.0),
                rank: 1,
            };
            let mut state: SystemState<(
                Res<PublicWalls>,
                ResMut<DecoratorStorage>,
                ResMut<LiveDecoratorArchivist>,
                country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,
                ResMut<crate::deferred_decorator_history::DeferredDecoratorEditHistory>,
                Res<DeferredDecoratorOpEvents>,
                EventWriter<PlayDecoratorSoundCmd>,
            )> = SystemState::new(&mut world);
            {
                let (walls, mut storage, mut archivist, archival, mut history, events, mut audio) =
                    state.get_mut(&mut world);
                apply_accepted_move(
                    &accepted,
                    None,
                    0,
                    1,
                    &walls,
                    &mut storage,
                    &mut archivist,
                    &archival,
                    &mut history,
                    &events,
                    &mut audio,
                );
            }
            state.apply(&mut world);
            let resulting_id = if merge {
                DecoratorId(19)
            } else {
                DecoratorId(17)
            };
            assert_eq!(
                world
                    .resource::<DecoratorStorage>()
                    .lookup_storage(resulting_id),
                Some(target_address)
            );
            let after = *world
                .resource::<DecoratorStorage>()
                .decorator_info(resulting_id)
                .unwrap();
            assert_eq!(after.rank, if merge { 2 } else { 1 });
            assert_eq!(after.dst_tag(), 2);
            let events = world.resource::<DeferredDecoratorOpEvents>();
            events.with_events(|events| {
                assert_eq!(events.len(), if merge { 2 } else { 1 });
                if merge {
                    assert_eq!(u64::from_le_bytes(events[0][..8].try_into().unwrap()), 4);
                }
                let event = events.last().unwrap();
                assert_eq!(
                    u64::from_le_bytes(event[..8].try_into().unwrap()),
                    match if merge {
                        target_address
                    } else {
                        source_address
                    } {
                        DecoratorStorageAddr::Wall(_) => 0,
                        DecoratorStorageAddr::Terrain => 1,
                        _ => 2,
                    }
                );
                assert_eq!(u64::from_le_bytes(event[0x10..0x18].try_into().unwrap()), 0);
                assert_eq!(u64::from_le_bytes(event[0x18..0x20].try_into().unwrap()), 2);
            });
            if merge {
                assert!(world
                    .resource::<DecoratorStorage>()
                    .decorator_info(DecoratorId(17))
                    .is_none());
                assert_real_diff_undo_redo(
                    &mut world,
                    DecoratorId(19),
                    source,
                    after,
                    target_address,
                    target_address,
                );
                assert!(world
                    .resource::<DecoratorStorage>()
                    .decorator_info(DecoratorId(17))
                    .is_none());
            } else {
                assert_real_diff_undo_redo(
                    &mut world,
                    DecoratorId(17),
                    source,
                    after,
                    source_address,
                    target_address,
                );
            }
        }
    }

    #[test]
    fn real_small_end_phases_change_only_seed_and_keep_independent_map_id() {
        // 原0/1都进入同一个结束分支；不能按旧注释把0静默当成无副作用取消。
        let mut original_union_events = std::collections::HashMap::new();
        for (phase, demo) in [(0, false), (1, false), (0, true), (1, true)] {
            let (mut world, mut schedule) = fixture();
            world.insert_resource(country_core::systems::backend::input::translate_input::DemoModeEnabled(demo));
            {
                let mut storage = world.resource_mut::<DecoratorStorage>();
                let info = storage.decorator_info_mut(DecoratorId(17)).unwrap();
                info.ty = 0x3d;
                // 真实Flag类型必须带style5；只换ty会制造历史回读主动规范化的无效夹具。
                info.set_extra(country_core::resources::walls::decorator::DecoratorExtraInfo::Flag(
                    country_core::resources::walls::decorator::FlagExtra(0)));
            }
            world
                .resource_mut::<DerivedDecoratorSubtypes>()
                .insert(17, DecoratorSubtype(15));
            update_frame(&mut world, 2, Vec2::new(640.0, 360.0));
            schedule.run(&mut world);
            let initial = *world
                .resource::<DecoratorStorage>()
                .decorator_info(DecoratorId(17))
                .unwrap();
            update_frame(&mut world, 3, Vec2::new(640.0, 360.0));
            schedule.run(&mut world);
            update_frame(&mut world, phase, Vec2::new(640.0, 360.0));
            schedule.run(&mut world);
            let changed = *world
                .resource::<DecoratorStorage>()
                .decorator_info(DecoratorId(17))
                .unwrap();
            assert_eq!(changed.decorator_id, initial.decorator_id.wrapping_add(1));
            assert_eq!(changed.destination(), initial.destination());
            assert_eq!(changed.u45, if demo { 0 } else { 5 });
            assert_eq!(
                world
                    .resource::<DecoratorStorage>()
                    .lookup_storage(DecoratorId(17)),
                Some(DecoratorStorageAddr::Terrain)
            );
            assert!(!world.resource::<crate::backup::DecoratorBackup>().created);
            assert!(
                world
                    .resource::<crate::deferred_decorator_history::DeferredDecoratorEditHistory>()
                    .pending
            );
            let native_events = world.resource::<DeferredDecoratorOpEvents>().with_events(|events| events.to_vec());
            if demo {
                // demo是原全局执行上下文；已应用事件中不补造此字段，原union逐字节相同。
                assert_eq!(&native_events, original_union_events.get(&phase).unwrap());
            } else {
                original_union_events.insert(phase, native_events);
            }
            assert_real_diff_undo_redo(&mut world, DecoratorId(17), initial, changed,
                DecoratorStorageAddr::Terrain, DecoratorStorageAddr::Terrain);
            assert_eq!(world.resource::<DecoratorStorage>().decorator_info(DecoratorId(17)).unwrap().u45,
                if demo { 0 } else { 5 });
        }
    }

    #[test]
    fn real_stair_bottom_click_preserves_height_offset_endpoint_and_place_initialization() {
        let (mut world, mut schedule) = fixture();
        let mut payload = [0; 20];
        payload[..4].copy_from_slice(&2f32.to_le_bytes());
        {
            let mut storage = world.resource_mut::<DecoratorStorage>();
            let info = storage.decorator_info_mut(DecoratorId(17)).unwrap();
            info.ty = 0x3f;
            info.set_extra(
                country_core::resources::walls::decorator::DecoratorExtraInfo::Stair(
                    country_core::resources::walls::decorator::StairExtra(payload),
                ),
            );
        }
        world
            .resource_mut::<DerivedDecoratorSubtypes>()
            .insert(17, DecoratorSubtype(25));
        let mut pose = world.resource::<CachedDecoratorTransforms>().transform(17);
        pose.translation.y = 2.0;
        world
            .resource_mut::<CachedDecoratorTransforms>()
            .insert(17, pose);
        world
            .resource_mut::<crate::blueprint::DecoratorBlueprints>()
            .insert_for(17, DecoratorStorageAddr::Terrain, 0)
            .set_cached_transform(pose);
        update_frame(&mut world, 2, Vec2::new(640.0, 360.0));
        schedule.run(&mut world);
        update_frame(&mut world, 3, Vec2::new(640.0, 360.0));
        schedule.run(&mut world);
        update_frame(&mut world, 1, Vec2::new(640.0, 360.0));
        schedule.run(&mut world);
        let state=world.resource::<Events<super::super::ui_place_decorator::EvInitPlaceDecoratorInteractionState>>()
            .iter_current_update_events().last().unwrap().0;
        assert_eq!(state.height_constraint, Some(2.0));
        assert_eq!(state.height_offset, -2.0);
        assert_eq!(state.last_height, 2.0);
        assert!(!state.native_flag_10);
        assert!(!state.fixed_height);
        assert_eq!(state.progress, 0);
        let endpoint = world
            .resource::<country_core::resources::stairs::stairs_state::StairsState>()
            .endpoint();
        assert_eq!(endpoint.tag(), 1);
        assert_eq!(endpoint.node(), 17);
        let requested = world
            .resource::<Events<RequestAppModeChange>>()
            .iter_current_update_events()
            .last()
            .unwrap()
            .0;
        assert_eq!(
            requested,
            AppMode::Decorator(country_core::systems::mode_manager::DecoratorType::Stairs)
        );
        assert_eq!(
            world
                .resource::<Events<NotifyHintSystemStartedStairsContinue>>()
                .len(),
            1
        );
        assert_eq!(world.resource::<DecoratorIdGen>().peek(), DecoratorId(0));
    }
}

#[cfg(test)]
mod original_move_signifier_tests {
    use super::*;
    use bevy_ecs::{system::Commands, world::CommandQueue};
    use country_core::{
        asset_libraries::{
            mesh_library::{AssetMeshLibrary, MeshAsset},
            shader_library::AssetShaderLibrary,
        },
        render::{mesh::Mesh, shader::RasterShaderProgram, test_device::TestDevice},
        systems::ui::signifiers::{
            animation::SignifierAnimation,
            signifier_create::{UiSignifierEntry, UiSignifierStream},
        },
        utils::components::transform::Transform,
    };
    fn libraries() -> (AssetMeshLibrary, AssetShaderLibrary) {
        let meshes = AssetMeshLibrary::new();
        meshes.add(MeshAsset::new(
            Mesh::new("decorator_delete_scribble"),
            Some("decorator_delete_scribble".to_owned()),
        ));
        let shaders = AssetShaderLibrary::new();
        shaders.add_raster(
            RasterShaderProgram::new_draw(
                "vertex_color_shader",
                Default::default(),
                &TestDevice(0),
            ),
            false,
        );
        (meshes, shaders)
    }
    #[test]
    fn remove_factory_owns_original_pose_until_wake_and_despawn() {
        let mut stream = UiSignifierStream::default();
        super::super::move_ops::show_decorator_remove_signifier(
            DecoratorId(17),
            &super::super::move_ops::RemoveSignifierSource {
                position: Vec3::new(1.0, 2.0, 3.0),
                direction: Vec3::Z,
            },
            &Vec3::new(2.0, 2.0, 3.0),
            &mut stream,
        );
        let UiSignifierEntry::Factory(factory) = stream.take_entries().pop().unwrap() else {
            panic!("删除图标必须是拥有工厂")
        };
        assert!(factory.name().unwrap().starts_with("despawn_decorator_"));
        let weak = std::sync::Arc::downgrade(factory.closure.as_ref().unwrap());
        let (meshes, shaders) = libraries();
        let mut world = World::new();
        let mut queue = CommandQueue::default();
        let entities = factory.wake(&meshes, &shaders, &mut Commands::new(&mut queue, &world));
        queue.apply(&mut world);
        assert_eq!(entities.len(), 1);
        let entity = entities[0];
        let pose = world.get::<Transform>(entity).unwrap();
        assert_eq!(pose.translation, Vec3::new(1.0, 2.0, 4.0));
        assert_eq!(pose.rotation, utils::xz_projective_rotation_bad(Vec3::Z));
        assert_eq!(
            world.get::<SignifierAnimation>(entity).unwrap().duration,
            0.0
        );
        let handle = *world
            .get::<country_core::asset_libraries::Handle<Mesh>>(entity)
            .unwrap();
        assert_eq!(
            handle,
            meshes.get_handle_by_name("decorator_delete_scribble")
        );
        drop(factory);
        assert!(weak.upgrade().is_none());
        assert!(world.despawn(entity));
    }
    #[test]
    fn child_autoclutter_factory_preserves_true_mesh_bounds_and_world_image_extent() {
        let mesh = country_core::resources::clutter_meshes::ClutterMesh {
            bounds: utils::geometry::Rectangle3d::new(
                Vec3::ZERO,
                Vec3::new(2.0, 3.0, 4.0),
                Vec2::X,
            ),
            mesh_name: "child",
            mesh: None,
            name_hash: utils::name_hash::NameHash::compute("child"),
            radius_xz: 3.0,
            native_group: 0,
            solid_texture: None,
            native_flag_46: false,
            native_flag_47: false,
        };
        let child =
            country_core::systems::ui_clutter::ui_autoclutter::DisplayChildAutoClutterInfo {
                mesh,
                nani_name: "nani_solid_vertex_color",
                solid_texture: None,
                transform: Transform {
                    rotation: Quat::from_rotation_y(0.5),
                    translation: Vec3::new(1.0, 2.0, 3.0),
                    scale: Vec3::ONE,
                },
                progress: 1.0,
            };
        let mut stream = UiSignifierStream::default();
        highlight_child_autoclutter(&child, &mut stream);
        let mut factories = stream.take_entries();
        assert_eq!(factories.len(), 2);
        let UiSignifierEntry::Factory(factory) = factories.pop().unwrap() else {
            panic!("link工厂")
        };
        assert_eq!(
            factory.name(),
            Some("child_auto_clutter_Vec3(1.0, 2.0, 3.0)_0.5")
        );
        let (meshes, shaders) = libraries();
        let mut world = World::new();
        let mut queue = CommandQueue::default();
        let entities = factory.wake(&meshes, &shaders, &mut Commands::new(&mut queue, &world));
        queue.apply(&mut world);
        let entity = entities[0];
        assert_eq!(
            world
                .get::<country_core::components::Ui2dImage>(entity)
                .unwrap()
                .name,
            "link"
        );
        let extent = world
            .get::<country_core::components::Ui2dWsExtent>(entity)
            .unwrap();
        assert_eq!(extent.extent, Vec2::splat(0.5));
        assert_eq!(extent.position, child.transform.translation);
        assert_eq!(
            world.get::<SignifierAnimation>(entity).unwrap().duration,
            0.0
        );
        assert!(world.despawn(entity));
    }
}
