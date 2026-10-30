//! 原B `ui_systems/ui_place_decorator.rs`，1407a9c10完整放置主链。
//! 原九组参数、三个跳表、19B交互状态和独立source/target/ID均按真实读写面恢复。
//! 即时创建先于Widget交互，楼梯拆分使用前帧三维采样，约束/合并/颜色/支撑/历史
//! 与实际权威存储、碰撞归档和原事件消费方共用owner。
//! 原C曾丢弃三个有效跳表，当前证据为隔离修表并重启验证后的完整函数；
//! 原证、数值差分、共享接口纠正及尚未完成的整链验收见1407a9c10-place恢复.md。
use bevy_ecs::prelude::*;
use country_core::resources::glade_border::{DisplayOutOfBorder, FeedbackInputOutOfBorder};
use country_core::resources::stairs::stairs_state::StairsState;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator::DecoratorId;
use country_core::resources::walls::decorator::{DecoratorInfo, DecoratorDst, StairExtra};
use country_core::resources::walls::decorator_storage::DecoratorStorageAddr;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::systems::audio::PlayStairsAudioCmd;
use country_core::systems::mode_manager::RequestAppModeChange;
use country_core::systems::ui::resources::ui_state::UiState;
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;

/// 原Event携带完整19B活动状态，由Place只读消费；不能把它回写成自循环单位事件。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct EvInitPlaceDecoratorInteractionState(pub PlaceDecoratorInteractionState);

/// `Local<PlayedConnectSoundLastFrame>`（I：连接音节流）。
#[derive(Default)]
pub struct PlayedConnectSoundLastFrame(pub bool);

/// 原19B语义状态，不要求Rust编译器复制原私有ABI。
/// +8由Move原1407a7109传入竖直偏移，不能改成不存在的装饰ID。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct PlaceDecoratorInteractionState {
    pub height_constraint: Option<f32>,
    pub height_offset: f32,
    pub last_height: f32,
    pub native_flag_10: bool,
    pub fixed_height: bool,
    pub progress: u8,
}

/// 原1407abd63..dc3：relative * camera，保留四组乘法与两次加法的原约简树。
/// 直接改成另一代数公式会改变非单位四元数和抵消边界的位型。
fn placement_relative_rotation(relative: glam::Quat, camera: glam::Quat) -> glam::Quat {
    let a = relative.to_array();
    let b = camera.to_array();
    let reverse = [b[3], b[2], b[1], b[0]];
    let middle = [b[2], b[3], b[0], b[1]];
    let other = [b[1], b[0], b[3], b[2]];
    let x_sign = [1.0, -1.0, 1.0, -1.0];
    let y_sign = [1.0, 1.0, -1.0, -1.0];
    let z_sign = [-1.0, 1.0, 1.0, -1.0];
    glam::Quat::from_array(std::array::from_fn(|lane| {
        let x = (a[0] * reverse[lane]) * x_sign[lane];
        let y = (middle[lane] * a[1]) * y_sign[lane];
        let z = (other[lane] * a[2]) * z_sign[lane];
        let w = a[3] * b[lane];
        (z + y) + (x + w)
    }))
}

/// 原端点带自己的地址；不得用全局d2s查找替代原decorator_info(addr,id)。
fn stair_endpoint_info(
    stairs: &StairsState,
    storage: &DecoratorStorage,
) -> Option<(DecoratorStorageAddr, DecoratorId, DecoratorInfo)> {
    use country_core::resources::walls::WallId;
    let endpoint = stairs.endpoint();
    if endpoint.is_unset() { return None; }
    let raw = endpoint.observed();
    let address = match u32::from_le_bytes(raw[..4].try_into().unwrap()) {
        0 => DecoratorStorageAddr::Wall(WallId(u64::from_le_bytes(raw[8..16].try_into().unwrap()))),
        1 => DecoratorStorageAddr::Terrain,
        2 => DecoratorStorageAddr::Stair,
        tag => panic!("无效楼梯继续端点地址 {tag}"),
    };
    let id = DecoratorId(endpoint.node());
    storage.get(address)?.get(id).copied().map(|info| (address, id, info))
}

fn initial_placement_extra(
    ty: u8,
    stairs: &StairsState,
    storage: &DecoratorStorage,
    colors: &country_core::resources::glade_color_customization::GladeColorCustomization,
) -> Option<country_core::resources::walls::decorator::DecoratorExtraInfo> {
    use country_core::resources::walls::decorator::{DecoratorExtraInfo, WindowExtra, FlagExtra, LanternExtra, ChimneyExtra};
    match ty {
        0x38..=0x3a => Some(DecoratorExtraInfo::Window(WindowExtra(colors.window_light.0))),
        0x3b => Some(DecoratorExtraInfo::Chimney(ChimneyExtra { f: 0.0, smoke: 1 })),
        0x3c => Some(DecoratorExtraInfo::Lantern(LanternExtra(colors.lantern_brightness))),
        0x3d => Some(DecoratorExtraInfo::Flag(FlagExtra(1))),
        0x3f => {
            let previous = stair_endpoint_info(stairs, storage)
                .and_then(|(_, _, info)| (info.extra_disc() < 2).then(|| info.extra().as_stair()));
            let mut raw = [0; 20];
            if let Some(previous) = previous {
                raw[4..17].copy_from_slice(&previous.0[4..17]);
            } else {
                raw[4..8].copy_from_slice(&1.5_f32.to_le_bytes());
                raw[8..12].copy_from_slice(&colors.stairs.ladder_color.0.to_le_bytes());
                raw[12..16].copy_from_slice(&colors.stairs.magic_color.0.to_le_bytes());
                raw[16] = colors.stairs.railing_ty.native_tag();
            }
            Some(DecoratorExtraInfo::Stair(StairExtra(raw)))
        }
        _ => None,
    }
}

fn placement_raycast_destination(
    hit: &country_core::systems::collision::world::RaycastWorldResult,
    info: &DecoratorInfo,
) -> super::calculate_decorator_dst::RaycastDecoratorDst {
    use super::calculate_decorator_dst::{RaycastHitDisc, RaycastDecoratorDst, convert_raycast_hit_to_decorator_dst};
    use country_core::resources::walls::WallId;
    // miss的handle是非活动载荷；安全接口不能把规范化零字节误认成wall0。
    if hit.is_miss() { return RaycastDecoratorDst::Invalid; }
    let source = hit.handle[0];
    let source = match source as u8 {
        0 => RaycastHitDisc::Wall(WallId(hit.handle[1])),
        2 => RaycastHitDisc::Roof { flag: (source >> 8) as u8, sub: (source >> 16) as u8, payload: hit.handle[1] },
        0xb => RaycastHitDisc::Terrain,
        0xd => RaycastHitDisc::StairTop((source >> 32) | (hit.handle[1] << 32)),
        3 => RaycastHitDisc::Unsupported,
        5 | 9 | 0xc | 0xf | 0x10 => RaycastHitDisc::Collider,
        _ => RaycastHitDisc::Other,
    };
    convert_raycast_hit_to_decorator_dst(source, info)
}

fn placement_color(
    ty: u8,
    address: DecoratorStorageAddr,
    stairs: &StairsState,
    storage: &DecoratorStorage,
    colors: &country_core::resources::glade_color_customization::GladeColorCustomization,
    wall_colors: &country_core::resources::color_id::WallColorIds,
) -> u32 {
    let from_wall = |wall: country_core::resources::walls::WallId| {
        // 原142a83080表；原WallId身份即颜色表索引，缺行必须保留bounds失败。
        const TABLE: [u32; 11] = [100001, 100002, 100003, 100000, 100001, 100003,
            100002, 100003, 100003, 100000, 100011];
        let index = wall_colors.0[wall.0 as usize].wrapping_sub(1);
        if index < TABLE.len() as u32 { TABLE[index as usize] } else { 100000 }
    };
    match ty {
        0x38 => colors.windows.map_or(0, |value| value.0),
        0x39 | 0x3a => colors.windows.map_or(999, |value| value.0),
        0x3b => from_wall(address.as_wall()),
        0x3c => colors.lantern_light.0,
        0x3d => colors.flag.0,
        0x3f => {
            if !stairs.endpoint().is_unset() {
                stair_endpoint_info(stairs, storage).map_or(colors.stairs.stairs_color.0, |(_, _, info)| info.u28)
            } else if let DecoratorStorageAddr::Wall(wall) = address { from_wall(wall) }
            else { colors.stairs.stairs_color.0 }
        }
        _ => 0,
    }
}

/// 原closure14/15：只检查当前继续端点的实际邻居，缺缓存的邻居跳过。
fn stair_neighbor_exceeds_slope(
    stairs: &StairsState,
    transforms: &CachedDecoratorTransforms,
    anchor: glam::Vec3,
    downward: bool,
) -> bool {
    let endpoint = stairs.endpoint();
    assert!(!endpoint.is_unset(), "楼梯斜率探针要求实际继续端点");
    for neighbor in stairs.graph().neighbors(endpoint.node()) {
        if let Some(position) = transforms.try_position(neighbor) {
            let x = position.x - anchor.x;
            let z = position.z - anchor.z;
            let distance = (z * z + x * x).sqrt();
            // 原MAXSS以epsilon为第二操作数：NaN距离也选择epsilon。
            let distance = if distance > 0.00001 { distance } else { 0.00001 };
            let slope = (position.y - anchor.y) / distance;
            if if downward { slope < -0.25 } else { slope > 0.25 } { return true; }
        }
    }
    false
}

/// 原未进入固定高度候选的楼梯生成：完整保留低台阶、坡度邻居与陡降梯分支。
#[allow(clippy::too_many_arguments)]
fn adjust_new_stair_height(
    extra: &mut StairExtra,
    destination: DecoratorDst,
    hit: &country_core::systems::collision::world::RaycastWorldResult,
    disable_snapping: bool,
    fixed_height: bool,
    stairs: &StairsState,
    storage: &DecoratorStorage,
    transforms: &CachedDecoratorTransforms,
    walls: &country_core::resources::walls::public_walls::PublicWalls,
    terrain: &country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo>,
) {
    use glam::Vec3;
    let target = match destination {
        DecoratorDst::Terrain(value) => Vec3::new(value.position_xz.x,
            terrain.sample_at_world_xz(value.position_xz.x, value.position_xz.y), value.position_xz.y),
        _ => hit.position,
    };
    let Some((address, id, info)) = stair_endpoint_info(stairs, storage) else { return };
    let anchor = transforms.position(id.0);
    let floor = if info.dst_tag() == 2 {
        walls.get(address.as_wall()).unwrap().flat_roof_y()
    } else { terrain.sample_at_world_xz(anchor.x, anchor.z) };
    let anchor_above_floor = anchor.y - floor;
    // 原SUBSS后ADDSS：不能代数消成anchor.y，消去会改变取消误差。
    let recombined_y = floor + anchor_above_floor;
    let difference = recombined_y - target.y;
    let height = if disable_snapping {
        difference
    } else {
        let dx = target.x - anchor.x;
        let dz = target.z - anchor.z;
        let distance = (dz * dz + dx * dx).sqrt();
        if anchor_above_floor > 1.0 || stair_neighbor_exceeds_slope(stairs, transforms, anchor, false) {
            if difference > 0.0 && 0.4 * (difference + difference) < distance
                && distance < difference + difference
                && !stair_neighbor_exceeds_slope(stairs, transforms, anchor, true)
                && !fixed_height {
                0.0
            } else { difference }
        } else {
            let half_distance = distance * 0.5;
            if target.y + half_distance < floor { 0.0 }
            else { (recombined_y + half_distance) - target.y }
        }
    };
    let height = if height < 0.0 { 0.0 } else { height };
    let height = if height > 100000.0 { 100000.0 } else { height };
    extra.0[..4].copy_from_slice(&height.to_le_bytes());
}

/// 原楼梯固定高度前段。返回是否实际采用了约束候选；失败保留此前真实命中候选。
#[allow(clippy::too_many_arguments)]
fn constrain_stair_placement(
    candidate: &mut Option<super::decorator_interaction_intent::DecoratorIntentCandidate>,
    extra: &mut StairExtra,
    state: &mut PlaceDecoratorInteractionState,
    disable_snapping: bool,
    hit: &country_core::systems::collision::world::RaycastWorldResult,
    stairs: &StairsState,
    storage: &DecoratorStorage,
    transforms: &CachedDecoratorTransforms,
    resources: &super::ui_move_decorator::RaycastResources,
    walls: &country_core::resources::walls::public_walls::PublicWalls,
    terrain: &country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo>,
    cursor: glam::Vec2,
) -> bool {
    use super::calculate_decorator_dst::RaycastDecoratorDst;
    if let Some((_, id, info)) = stair_endpoint_info(stairs, storage) {
        if info.dst_tag() < 2 {
            state.fixed_height = true;
            state.last_height = transforms.position(id.0).y;
        } else {
            let height = f32::from_le_bytes(info.extra().as_stair().0[..4].try_into().unwrap());
            let all_flat = height == 0.0 && stairs.graph().neighbors(stairs.endpoint().node()).all(|node| {
                let Some(neighbor) = storage.lookup_decorator_info(DecoratorId(node)) else { return false };
                if neighbor.dst_tag() < 2 { return false; }
                f32::from_le_bytes(neighbor.extra().as_stair().0[..4].try_into().unwrap()) == 0.0
            });
            if all_flat {
                state.fixed_height = false;
                state.last_height = 0.0;
                state.height_constraint = None;
            } else {
                state.last_height = transforms.position(id.0).y;
                if state.height_constraint.is_some() && disable_snapping {
                    state.height_constraint = Some(state.last_height);
                }
                if height == 0.0 { state.fixed_height = false; }
            }
        }
    }
    if state.fixed_height && !disable_snapping
        && candidate.is_some_and(|candidate| matches!(candidate.destination, RaycastDecoratorDst::Wall(_))) {
        state.height_constraint = Some(hit.position.y);
    }
    let constraint = if disable_snapping && state.fixed_height { Some(state.last_height) }
        else { state.height_constraint };
    if !(state.fixed_height || disable_snapping) { return false; }
    if !disable_snapping && !candidate.is_some_and(|candidate| matches!(candidate.destination,
        RaycastDecoratorDst::Roof { .. } | RaycastDecoratorDst::Terrain)) { return false; }
    // 原在约束None的情况下仍先生成鼠标射线，再检查Option；保留其required资源调用。
    let matrices = resources.camera.camera_matrices();
    let ray = utils::geometry::ray::Ray::from_mouse_ss(cursor, resources.window.as_vec2(),
        matrices.projection.inverse(), matrices.inverse_view);
    let Some(constraint) = constraint else { return false };
    let mut position = glam::Vec3::ZERO;
    let Some(mut replacement) = super::ui_move_decorator::find_stair_placement_candidate(
        resources, walls, terrain, state.height_offset, candidate.as_ref(), disable_snapping,
        &ray, constraint + state.height_offset, &mut position,
    ) else { return false };
    if replacement.height < 0.3 && !disable_snapping {
        replacement.candidate.position.y -= replacement.height;
        replacement.height = 0.0;
    }
    *candidate = Some(replacement.candidate);
    extra.0[..4].copy_from_slice(&replacement.height.to_le_bytes());
    true
}

/// 原最终写入前的相同目的短路和支撑继承。true表示本帧原本应直接返回。
fn prepare_stair_support_inheritance(
    ty: u8,
    destination: DecoratorDst,
    address: DecoratorStorageAddr,
    id: DecoratorId,
    stairs: &StairsState,
    storage: &DecoratorStorage,
    removed: &mut country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports,
) -> bool {
    use country_core::resources::stairs::stairs_state::StairGraphEdge;
    let Some((source, previous, info)) = stair_endpoint_info(stairs, storage) else { return false };
    // 原1407a9ab0按活动enum载荷比较，而非72B Info或无效padding。
    if destination == info.destination() && source == address { return true; }
    if ty == 0x3f {
        let inherited = stairs.graph().edges(previous.0)
            .any(|(first, second, _)| removed.contains(&StairGraphEdge::new(first, second)));
        let edge = StairGraphEdge::new(previous.0, id.0);
        if inherited { removed.record(edge); } else { removed.erase(&edge); }
    }
    false
}

/// 原InstaCreateStairPoint的值载荷。仅解既有事件，不复制原指针/容器布局。
#[derive(Clone, Copy, Debug, PartialEq)]
struct InstantStairPoint {
    address: DecoratorStorageAddr,
    destination: DecoratorDst,
    split: Option<(DecoratorId, DecoratorId)>,
    extra: StairExtra,
    position: glam::Vec3,
    color: u32,
}

/// 原即时杂物创建：先解析原目的来源，再分配正式ID、切换编辑模式并应用真实Place。
#[allow(clippy::too_many_arguments)]
fn apply_instant_decorator(
    event: &country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco,
    demo: bool,
    storage: &mut DecoratorStorage,
    archivist: &mut country_core::resources::walls::decorator_archivist::LiveDecoratorArchivist,
    archival: &country_core::resources::walls::decorator_archivist::DecoratorArchivalResources<'_>,
    history: &mut crate::deferred_decorator_history::DeferredDecoratorEditHistory,
    events: &country_core::resources::walls::decorator::DeferredDecoratorOpEvents,
    ids: &mut country_core::resources::walls::decorator::DecoratorIdGen,
    world: &country_core::systems::collision::world::RaycastWorld,
    terrain: &country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo>,
    app_mode: &mut EventWriter<RequestAppModeChange>,
) {
    use country_core::resources::walls::{WallId, decorator::{DecoratorRotationXZ, RoofDecoratorDst, TerrainDecoratorDst}};
    use country_core::systems::mode_manager::{AppMode, EditSubject};
    use glam::{Vec2, Vec3};
    let (address, destination) = if event.destination[0] as u32 == 0 {
        let mut wall = event.destination[2];
        let uv = if event.destination[0] & 0x1_0000_0000 != 0 {
            Vec2::new(f32::from_bits(event.destination[1] as u32), f32::from_bits((event.destination[1] >> 32) as u32))
        } else {
            let origin = Vec3::new(event.position.x + 0.0, event.position.y + 0.05, event.position.z + 0.0);
            let hit = world.raycast_w_terrain(terrain, origin, Vec3::new(-0.0, -1.0, -0.0), 0x22);
            if !hit.is_miss() && hit.discriminant & 1 != 0 {
                if hit.handle[0] as u8 == 2 { wall = hit.handle[1]; }
                Vec2::from_array(hit.extra)
            } else { Vec2::splat(0.5) }
        };
        (DecoratorStorageAddr::Wall(WallId(wall)), DecoratorDst::Roof(RoofDecoratorDst {
            coord: uv, yaw: DecoratorRotationXZ(event.rotation) }))
    } else {
        (DecoratorStorageAddr::Terrain, DecoratorDst::Terrain(TerrainDecoratorDst {
            position_xz: Vec2::new(event.position.x, event.position.z), yaw: DecoratorRotationXZ(event.rotation) }))
    };
    history.pending = true;
    let id = ids.generate();
    let ty = event.ty.native_tag();
    let slot = ty.wrapping_sub(0x38);
    assert!(slot >= 8 || slot == 6, "即时杂物创建不能接收窗口、烟囱、灯、旗或楼梯类型");
    app_mode.write(RequestAppModeChange(AppMode::Editing(EditSubject::DecoClutter { decorator_id: id.0 })));
    let mut info = DecoratorInfo { rank: 1, ty, u40: 3 << 8, ..Default::default() };
    info.set_destination(destination);
    if let Some(extra) = event.extra { info.set_extra(extra); }
    let op = crate::apply_decorator_op::DecoratorOp::place_at(id, info, event.position, 0, true, demo);
    crate::apply_decorator_op::apply_place_with_resources(&op, 0, address, storage,
        archivist, Some(history), archival, events);
}

impl InstantStairPoint {
    fn from_event(event: &country_core::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint) -> Self {
        use country_core::resources::walls::WallId;
        let raw = event.observed();
        assert_eq!(raw[0x68], 0x3f, "瞬时楼梯点事件必须携带原楼梯类型");
        let u32_at = |offset| u32::from_le_bytes(raw[offset..offset + 4].try_into().unwrap());
        let u64_at = |offset| u64::from_le_bytes(raw[offset..offset + 8].try_into().unwrap());
        let address = match u64_at(0) {
            0 => DecoratorStorageAddr::Wall(WallId(u64_at(8))),
            1 => DecoratorStorageAddr::Terrain,
            2 => DecoratorStorageAddr::Stair,
            value => panic!("无效楼梯点存储地址 {value}"),
        };
        let mut info_bytes = [0; 72];
        info_bytes[..40].copy_from_slice(&raw[0x10..0x38]);
        let destination = crate::apply_decorator_op::bytes_to_info(&info_bytes).destination();
        let extra = StairExtra(raw[0x44..0x58].try_into().unwrap());
        Self {
            address,
            destination,
            split: (raw[0x38] & 1 != 0).then(|| (DecoratorId(u32_at(0x3c)), DecoratorId(u32_at(0x40)))),
            extra,
            position: glam::Vec3::from_array(std::array::from_fn(|lane| f32::from_bits(u32_at(0x58 + lane * 4)))),
            color: u32_at(0x64),
        }
    }

    /// 原1407aa463..54b与140a20c70：正式点rank=1，ID独立生成，state由apply参数写0。
    fn operation(self, id: DecoratorId, demo: bool) -> crate::apply_decorator_op::DecoratorOp {
        let mut info = DecoratorInfo { rank: 1, ty: 0x3f, u28: self.color, ..Default::default() };
        info.set_destination(self.destination);
        info.set_extra(country_core::resources::walls::decorator::DecoratorExtraInfo::Stair(self.extra));
        crate::apply_decorator_op::DecoratorOp::place_at(id, info, self.position, 0, true, demo)
    }
}

/// 原0x28B拆分待重锚记录；值拥有输入点，不借用跨EventReader帧的指针。
#[derive(Clone, Copy)]
struct StairSplitReanchor {
    position: glam::Vec3,
    first: DecoratorId,
    second: DecoratorId,
    coordinate: glam::Vec2,
    yaw: country_core::resources::walls::decorator::DecoratorRotationXZ,
    inserted: DecoratorId,
    attached: DecoratorId,
    ty: u8,
}

fn apply_instant_stair_point(
    event: InstantStairPoint,
    demo: bool,
    storage: &mut DecoratorStorage,
    archivist: &mut country_core::resources::walls::decorator_archivist::LiveDecoratorArchivist,
    archival: &country_core::resources::walls::decorator_archivist::DecoratorArchivalResources<'_>,
    history: &mut crate::deferred_decorator_history::DeferredDecoratorEditHistory,
    events: &country_core::resources::walls::decorator::DeferredDecoratorOpEvents,
    ids: &mut country_core::resources::walls::decorator::DecoratorIdGen,
    stairs: &mut StairsState,
    reanchors: &mut Vec<StairSplitReanchor>,
) {
    history.pending = true;
    let id = ids.generate();
    let op = event.operation(id, demo);
    crate::apply_decorator_op::apply_place_with_resources(&op, 0, event.address,
        storage, archivist, Some(history), archival, events);
    if let Some((first, second)) = event.split {
        // 原在实际place之后变更图，并且先去掉继续端点。
        stairs.unset_endpoint();
        stairs.add_edge(first.0, id.0);
        stairs.add_edge(id.0, second.0);
        stairs.disconnect_edge(first.0, second.0);
        if let Some(bucket) = storage.get(DecoratorStorageAddr::Stair) {
            for (&attached, info) in &bucket.decorators {
                if info.dst_tag() != 4 { continue; }
                let DecoratorDst::Stair(destination) = info.destination() else { unreachable!() };
                if destination.first_id == first.0 && destination.second_id == second.0 {
                    reanchors.push(StairSplitReanchor { position: event.position, first, second,
                        coordinate: destination.coordinate, yaw: destination.yaw, inserted: id,
                        attached: DecoratorId(attached), ty: info.ty });
                }
            }
        }
    }
}

/// 原1407aabe0遍历拆分记录；严格使用previous.sampled_points的三维长度缓存，
/// 近点查询投影XZ。不可改用本帧requests或重算成二维长度。
#[allow(clippy::too_many_arguments)]
fn reanchor_split_stair_decorators(
    reanchors: &[StairSplitReanchor],
    requests: &country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests,
    storage: &mut DecoratorStorage,
    archivist: &mut country_core::resources::walls::decorator_archivist::LiveDecoratorArchivist,
    archival: &country_core::resources::walls::decorator_archivist::DecoratorArchivalResources<'_>,
    history: &mut crate::deferred_decorator_history::DeferredDecoratorEditHistory,
    events: &country_core::resources::walls::decorator::DeferredDecoratorOpEvents,
) {
    use country_core::resources::{stairs::{stairs_state::StairSegment, stairs_assembly_params::StairGraphEdgeIdentity},
        walls::decorator::{DecoratorRotationXZ, StairDecoratorDst}};
    use crate::apply_decorator_op::{DecoratorChangePatch, DecoratorOp, apply_decorator_op};
    for record in reanchors {
        if record.ty == 0x40 { break; }
        let key = StairGraphEdgeIdentity::from(StairSegment::new(record.first, record.second));
        let Some(previous) = requests.prev_requests().sampled_points.get(&key) else { continue };
        let Ok(curve) = utils::curve::Curve3::try_new_from_points(previous.points.clone(), true) else { continue };
        let split_u = curve.get_approx_u_from_pos(glam::Vec2::new(record.position.x, record.position.z));
        let mut x = record.coordinate.x;
        let mut yaw = record.yaw.0;
        // 原UCOMISS/JBE：无序与右半段一起走第二分支；不能把NaN变为左半段。
        let (segment, u) = if split_u > record.coordinate.y {
            (StairSegment::new(record.first, record.inserted),
                utils::scalar::inverse_lerp(0.0, split_u, record.coordinate.y))
        } else {
            x = f32::from_bits(x.to_bits() ^ 0x8000_0000);
            yaw += std::f32::consts::PI;
            (StairSegment::new(record.inserted, record.second),
                utils::scalar::inverse_lerp(split_u, 1.0, record.coordinate.y))
        };
        let (first, second) = segment.nodes();
        let address = DecoratorStorageAddr::Stair;
        let patch = DecoratorChangePatch::rotation(record.attached, address,
            DecoratorDst::Stair(StairDecoratorDst { first_id: first.0, second_id: second.0,
                coordinate: glam::Vec2::new(x, u), yaw: DecoratorRotationXZ(yaw) }), record.position);
        // 同一权威归档先保存完整碰撞状态，typed change仍消费原同一archivist。
        archivist.archive_current_decorators_at_with_resources(address, storage, archival, "move preview src");
        apply_decorator_op(&DecoratorOp::from_change(address, patch), 0, address, storage,
            Some(archivist), Some(history), events);
    }
}

/// 原ContinueStair分支：立即联通端点，按目标同步整个连通分量的颜色/样式，
/// 原Local只抑制连续预览连接音，释放帧仍播放确认音并退出编辑。
#[allow(clippy::too_many_arguments)]
fn continue_stair_connection(
    target: &super::ui_decorator_affordance_dispatch::DecoratorTargetAffordance,
    ui: &UiState,
    storage: &mut DecoratorStorage,
    stairs: &mut StairsState,
    transforms: &CachedDecoratorTransforms,
    previous_sound: bool,
    audio: &mut EventWriter<PlayStairsAudioCmd>,
    app_mode: &mut EventWriter<RequestAppModeChange>,
) -> bool {
    use country_core::systems::mode_manager::{AppMode, EditSubject};
    use country_core::resources::walls::decorator::DecoratorExtraInfo;
    let phase = ui.widget_context(target.widget).dragged();
    let endpoint = stairs.endpoint();
    let mut played = false;
    if !endpoint.is_unset() {
        stairs.add_edge(endpoint.node(), target.decorator_id.0);
        if let Some(source) = storage.lookup_decorator_info(target.decorator_id).copied() {
            for node in stairs.get_all_connected_nodes(endpoint.node()) {
                if let Some(info) = storage.lookup_decorator_info_mut(DecoratorId(node)) {
                    // 原先颜色写入，再逐项取Stair extra，保留required失败与写入顺序。
                    info.u28 = source.u28;
                    let source_extra = source.extra().as_stair();
                    let mut extra = info.extra().as_stair();
                    extra.0[8..12].copy_from_slice(&source_extra.0[8..12]);
                    extra.0[16] = source_extra.0[16];
                    info.set_extra(DecoratorExtraInfo::Stair(extra));
                }
            }
        }
        played = true;
        if !(phase >= 2 && previous_sound) {
            let extra = storage.lookup_decorator_info(target.decorator_id)
                .and_then(|info| (info.extra_disc() < 2).then(|| info.extra().as_stair()));
            if let (Some(extra), Some(position)) = (extra, transforms.try_position(target.decorator_id.0)) {
                let height = f32::from_le_bytes(extra.0[..4].try_into().unwrap());
                let width = f32::from_le_bytes(extra.0[4..8].try_into().unwrap());
                audio.write(PlayStairsAudioCmd { kind: if phase < 2 { 6 } else { 5 },
                    position, values: [width, height, 0.0, 0.0], tail: [0.0; 2], bytes: [0; 3] });
            }
        }
    }
    if phase < 2 {
        stairs.unset_endpoint();
        app_mode.write(RequestAppModeChange(AppMode::Editing(EditSubject::None)));
    }
    played
}

#[cfg(test)]
mod instant_stair_tests {
    use super::*;
    use country_core::{resources::{stairs::{stairs_state::StairSegment,
        stairs_assembly_params::{PlayermadeStairsAssemblyRequests, StairGraphEdgeIdentity,
            SampledStairsPoints, StairSamplesMetadata}}, walls::{decorator::{DecoratorIdGen,
        DecoratorExtraInfo, DecoratorRotationXZ, TerrainDecoratorDst, StairDecoratorDst, DeferredDecoratorOpEvents},
        decorator_archivist::{LiveDecoratorArchivist, DecoratorArchivalResources},
        decorator_collision::DecoratorPhysicsColliders}}, systems::{collision::world::RaycastWorld,
        ui_clutter::ui_autoclutter::InstaCreateStairPoint}};
    use crate::deferred_decorator_history::DeferredDecoratorEditHistory;
    use glam::{Vec2, Vec3};

    fn public_world() -> World {
        use country_core::{resources::{walls::{public_walls::PublicWalls,
            decorator_subtype::{DecoratorMeshLibrary, DerivedDecoratorSubtypes}},
            stairs::stairs_removed_supports::StairsRemovedSupports,
            color_id::WallColorIds, glade_border::{GladeBorder, BorderShape},
            glade_color_customization::GladeColorCustomization,
            terrain_height::{TerrainHeightsData, TerrainHeightSsbo},
            window::WindowSize, read_mouse_depth_value::MouseDepthValue,
            mouse_terrain_raycast::CursorPositionSS, onboarding::NotifyHintSystemStairsPlaced},
            systems::{camera_rig::MainCamera, backend::input::{mod_key::Modifiers, translate_input::DemoModeEnabled},
                mode_manager::{AppMode, DecoratorType, ClutterMeshTy},
                ui_clutter::ui_autoclutter::InstaCreateDeco}};
        let mut world = World::new();
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<LiveDecoratorArchivist>();
        world.init_resource::<DeferredDecoratorOpEvents>();
        world.init_resource::<CachedDecoratorTransforms>();
        world.init_resource::<DerivedDecoratorSubtypes>();
        world.init_resource::<DecoratorMeshLibrary>();
        world.init_resource::<DecoratorPhysicsColliders>();
        world.init_resource::<RaycastWorld>();
        world.init_resource::<super::super::ui_move_decorator::MoveDecoratorDeltaPositions>();
        world.init_resource::<crate::backup::DecoratorBackup>();
        world.init_resource::<crate::blueprint::DecoratorBlueprints>();
        world.init_resource::<super::super::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>();
        world.init_resource::<DecoratorIdGen>();
        world.init_resource::<GladeColorCustomization>();
        world.init_resource::<StairsState>();
        world.init_resource::<StairsRemovedSupports>();
        world.init_resource::<PublicWalls>();
        world.init_resource::<WallColorIds>();
        world.insert_resource(GladeBorder(BorderShape::Circle(utils::geometry::Circle2d::new(Vec2::ZERO, 65.0))));
        let mut terrain = TerrainHeightsData::<TerrainHeightSsbo>::new();
        terrain.set_height_range(0.0, 1.0);
        world.insert_resource(terrain);
        world.init_resource::<PlayermadeStairsAssemblyRequests>();
        world.insert_resource(AppMode::Decorator(DecoratorType::Clutter { ty: ClutterMeshTy::PlantPotV1 }));
        world.insert_resource(UiState::at_startup());
        world.init_resource::<Modifiers>();
        let mut time = bevy_time::Time::<()>::default();
        time.advance_by(std::time::Duration::from_millis(16));
        world.insert_resource(time);
        world.insert_resource(MainCamera::new(16.0 / 9.0));
        world.insert_resource(WindowSize::new(1280, 720));
        world.init_resource::<MouseDepthValue>();
        world.insert_resource(CursorPositionSS(Vec2::new(100.0, 200.0)));
        world.init_resource::<UiSignifierStream>();
        world.init_resource::<PlaceDecoratorInteractionState>();
        world.init_resource::<DeferredDecoratorEditHistory>();
        world.init_resource::<DemoModeEnabled>();
        world.init_resource::<Events<EvInitPlaceDecoratorInteractionState>>();
        world.init_resource::<Events<DisplayOutOfBorder>>();
        world.init_resource::<Events<FeedbackInputOutOfBorder>>();
        world.init_resource::<Events<RequestAppModeChange>>();
        world.init_resource::<Events<InstaCreateDeco>>();
        world.init_resource::<Events<InstaCreateStairPoint>>();
        world.init_resource::<Events<NotifyHintSystemStairsPlaced>>();
        world.init_resource::<Events<PlayStairsAudioCmd>>();
        world
    }

    fn ordinary_place(world: &mut World, phase: u8, position: Vec3) {
        use super::super::ui_decorator_affordance_dispatch::{DecoratorAffordanceDispatcher,
            DecoratorAffordanceInteraction, PlaceDecoratorAffordance};
        use country_core::{resources::walls::decorator::DecoratorType,
            systems::{collision::world::RaycastWorldResult, ui::resources::ui_state::{WidgetId, UiInteraction}}};
        let widget = WidgetId(4242);
        let mut ui = UiState::new(UiInteraction { native_tag: 2, native_phase: phase, widget });
        ui.set_intersection_tag(0);
        world.insert_resource(ui);
        world.insert_resource(DecoratorAffordanceDispatcher { state: 0, payload: 0, ty: 0,
            interaction: Some(DecoratorAffordanceInteraction::PlaceDecorator(PlaceDecoratorAffordance {
                widget, hit: RaycastWorldResult { handle: [0xb, 0, 0], position,
                    normal: Vec3::Y, toi: 1.0, discriminant: 0, extra: [0.0; 2] }, ty: DecoratorType(0) })) });
    }

    #[test]
    fn public_system_multiframe_widget_backup_preview_release_and_border_feedback() {
        use bevy_ecs::schedule::IntoScheduleConfigs;
        use super::super::ui_move_decorator::MoveDecoratorDeltaPositions;
        let mut world = public_world();
        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.add_systems((ui_place_decorator,
            crate::backup::create_or_restore_decorator_backup::<country_core::resources::walls::decorator_archivist::LiveColliderPayload>).chain());
        ordinary_place(&mut world, 3, Vec3::new(1.0, 0.0, 2.0));
        schedule.run(&mut world);
        assert!(world.resource::<DecoratorStorage>().get(DecoratorStorageAddr::Terrain).is_none());
        assert!(world.resource::<crate::backup::DecoratorBackup>().created);
        assert_eq!(world.resource::<DecoratorIdGen>().peek(), DecoratorId(0));
        ordinary_place(&mut world, 4, Vec3::new(1.0, 0.0, 2.0));
        schedule.run(&mut world);
        let storage = world.resource::<DecoratorStorage>();
        assert_eq!(storage.get(DecoratorStorageAddr::Terrain).unwrap().decorators[&0].state, 1);
        assert!(!world.resource::<DeferredDecoratorEditHistory>().pending);
        ordinary_place(&mut world, 4, Vec3::new(2.0, 0.0, 2.0));
        schedule.run(&mut world);
        assert_eq!(world.resource::<DecoratorStorage>().get(DecoratorStorageAddr::Terrain).unwrap().decorators.len(), 1);
        assert_eq!(world.resource::<DecoratorIdGen>().peek(), DecoratorId(0));
        assert_eq!(world.resource::<MoveDecoratorDeltaPositions>().velocity, 1.0 / 0.016);
        ordinary_place(&mut world, 1, Vec3::new(2.0, 0.0, 2.0));
        schedule.run(&mut world);
        let info = world.resource::<DecoratorStorage>().get(DecoratorStorageAddr::Terrain).unwrap().decorators[&0];
        assert_eq!((info.state, info.rank, info.ty, info.u45), (0, 1, 0, 5));
        let DecoratorDst::Terrain(destination) = info.destination() else { panic!() };
        assert_eq!(destination.position_xz, Vec2::new(2.0, 2.0));
        assert_eq!(world.resource::<DecoratorIdGen>().peek(), DecoratorId(1));
        assert!(world.resource::<DeferredDecoratorEditHistory>().pending);
        assert!(!world.resource::<crate::backup::DecoratorBackup>().created);
        ordinary_place(&mut world, 3, Vec3::new(70.0, 0.0, 2.0));
        schedule.run(&mut world);
        ordinary_place(&mut world, 1, Vec3::new(70.0, 0.0, 2.0));
        schedule.run(&mut world);
        assert_eq!(world.resource::<Events<DisplayOutOfBorder>>().len(), 1);
        assert_eq!(world.resource::<Events<FeedbackInputOutOfBorder>>().len(), 1);
        assert_eq!(world.resource::<DecoratorStorage>().get(DecoratorStorageAddr::Terrain).unwrap().decorators.len(), 1);
        assert_eq!(world.resource::<DecoratorIdGen>().peek(), DecoratorId(2));
    }

    #[test]
    fn public_system_real_instant_streams_last_initialization_and_dispatcher_preemption() {
        use country_core::{resources::stairs::stairs_state::ObservedStairEndpoint,
            systems::{mode_manager::{AppMode, EditSubject, ClutterMeshTy},
                collision::world::RaycastWorldResult, ui_clutter::ui_autoclutter::InstaCreateDeco,
                ui::resources::ui_state::WidgetId}};
        use super::super::ui_decorator_affordance_dispatch::{DecoratorAffordanceDispatcher,
            DecoratorAffordanceInteraction, DecoratorTargetAffordance};
        let mut world = public_world();
        world.insert_resource(AppMode::Editing(EditSubject::None));
        let initial = PlaceDecoratorInteractionState { height_constraint: Some(7.0), height_offset: 0.5,
            last_height: 3.0, native_flag_10: true, fixed_height: true, progress: 2 };
        world.resource_mut::<Events<EvInitPlaceDecoratorInteractionState>>()
            .send(EvInitPlaceDecoratorInteractionState(PlaceDecoratorInteractionState { progress: 1, ..initial }));
        world.resource_mut::<Events<EvInitPlaceDecoratorInteractionState>>()
            .send(EvInitPlaceDecoratorInteractionState(initial));
        let mut endpoint = [0; 24];
        endpoint[..8].copy_from_slice(&1_u64.to_le_bytes());
        endpoint[16..20].copy_from_slice(&55_u32.to_le_bytes());
        world.resource_mut::<StairsState>().try_set_endpoint(ObservedStairEndpoint::from_observed(endpoint));
        world.insert_resource(DecoratorAffordanceDispatcher { state: 5, payload: 0, ty: 63,
            interaction: Some(DecoratorAffordanceInteraction::ContinueStair(DecoratorTargetAffordance {
                widget: WidgetId(444), hit: RaycastWorldResult::miss(), decorator_id: DecoratorId(66),
                ty: country_core::resources::walls::decorator::DecoratorType(63) })) });
        world.resource_mut::<Events<InstaCreateDeco>>().send(InstaCreateDeco {
            destination: [2, 0, 0], extra: None, rotation: -0.25,
            position: Vec3::new(1.0, 0.0, 2.0), ty: ClutterMeshTy::PlantPotV1 });
        world.resource_mut::<Events<InstaCreateStairPoint>>().send(event(false));
        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.add_systems(ui_place_decorator);
        schedule.run(&mut world);
        assert_eq!(*world.resource::<PlaceDecoratorInteractionState>(), initial);
        assert_eq!(world.resource::<DecoratorIdGen>().peek(), DecoratorId(2));
        let bucket = world.resource::<DecoratorStorage>().get(DecoratorStorageAddr::Terrain).unwrap();
        assert_eq!(bucket.decorators[&0].ty, 0);
        assert_eq!(bucket.decorators[&1].ty, 63);
        assert_eq!(bucket.decorators[&0].extra_disc(), 3);
        assert_eq!(bucket.decorators[&1].rank, 1);
        assert!(!world.resource::<StairsState>().graph().contains_edge(55, 66));
        assert_eq!(world.resource::<Events<RequestAppModeChange>>().len(), 1);
        // 同一个Schedule保存EventReader游标；下一帧不得再次分配这两个ID。
        schedule.run(&mut world);
        assert_eq!(world.resource::<DecoratorIdGen>().peek(), DecoratorId(2));
        assert_eq!(*world.resource::<PlaceDecoratorInteractionState>(), PlaceDecoratorInteractionState::default());
        assert!(world.resource::<StairsState>().graph().contains_edge(55, 66));
    }

    #[test]
    fn actual_constraint_owners_keep_wall_lock_flat_component_and_missing_neighbor_distinct() {
        use country_core::{resources::{stairs::stairs_state::ObservedStairEndpoint,
            walls::{WallId, cached_decorator_transforms::CachedDecoratorTransform}},
            systems::collision::world::RaycastWorldResult};
        use super::super::{calculate_decorator_dst::RaycastDecoratorDst,
            decorator_interaction_intent::DecoratorIntentCandidate, ui_move_decorator::RaycastResources};
        let mut world = public_world();
        let mut params = bevy_ecs::system::SystemState::<RaycastResources>::new(&mut world);
        let resources = params.get(&world);
        let mut storage = DecoratorStorage::default();
        let mut transforms = CachedDecoratorTransforms::default();
        transforms.insert(10, CachedDecoratorTransform { rotation: glam::Quat::IDENTITY,
            translation: Vec3::new(1.0, 4.0, 2.0), scale: Vec3::ONE });
        let mut stairs = StairsState::default();
        let mut endpoint = [0; 24];
        endpoint[8..16].copy_from_slice(&5_u64.to_le_bytes());
        endpoint[16..20].copy_from_slice(&10_u32.to_le_bytes());
        stairs.try_set_endpoint(ObservedStairEndpoint::from_observed(endpoint));
        let mut source = DecoratorInfo { u20: 1 << 16, ty: 63, ..Default::default() };
        source.set_extra(DecoratorExtraInfo::Stair(StairExtra([0; 20])));
        storage.apply_decorator_place(DecoratorStorageAddr::Wall(WallId(5)), DecoratorId(10), &source);
        let initial = DecoratorIntentCandidate { rotation: glam::Quat::from_rotation_y(0.7),
            destination: RaycastDecoratorDst::Wall(WallId(7)), position: Vec3::new(2.0, 8.0, 3.0), roof_coordinate: None };
        let hit = RaycastWorldResult { handle: [1, 7, 0], position: initial.position,
            normal: Vec3::X, toi: 1.0, discriminant: 0, extra: [0.0; 2] };
        let walls = country_core::resources::walls::public_walls::PublicWalls::default();
        let mut extra = StairExtra([0x5a; 20]);
        let mut candidate = Some(initial);
        let mut state = PlaceDecoratorInteractionState::default();
        assert!(!constrain_stair_placement(&mut candidate, &mut extra, &mut state, false,
            &hit, &stairs, &storage, &transforms, &resources, &walls, &resources.terrain, Vec2::ZERO));
        assert!(state.fixed_height);
        assert_eq!(state.last_height, 4.0);
        assert_eq!(state.height_constraint, Some(8.0));
        assert_eq!(candidate.unwrap().position, initial.position);
        assert_eq!(extra.0, [0x5a; 20]);
        // 地形零高度连通分量清除锁；缺失邻居记录会阻止这个清除，不能当平坦成功。
        endpoint[..8].copy_from_slice(&1_u64.to_le_bytes());
        stairs.try_set_endpoint(ObservedStairEndpoint::from_observed(endpoint));
        stairs.add_edge(10, 20);
        source.set_destination(DecoratorDst::Terrain(TerrainDecoratorDst {
            position_xz: Vec2::ZERO, yaw: DecoratorRotationXZ(0.0) }));
        storage.apply_decorator_place(DecoratorStorageAddr::Terrain, DecoratorId(20), &source);
        storage.apply_decorator_remove(DecoratorStorageAddr::Wall(WallId(5)), DecoratorId(10));
        storage.apply_decorator_place(DecoratorStorageAddr::Terrain, DecoratorId(10), &source);
        assert!(!constrain_stair_placement(&mut candidate, &mut extra, &mut state, false,
            &hit, &stairs, &storage, &transforms, &resources, &walls, &resources.terrain, Vec2::ZERO));
        assert_eq!((state.fixed_height, state.last_height, state.height_constraint), (false, 0.0, None));
        storage.apply_decorator_remove(DecoratorStorageAddr::Terrain, DecoratorId(20));
        state.fixed_height = true;
        state.height_constraint = Some(9.0);
        assert!(!constrain_stair_placement(&mut candidate, &mut extra, &mut state, false,
            &hit, &stairs, &storage, &transforms, &resources, &walls, &resources.terrain, Vec2::ZERO));
        assert_eq!((state.fixed_height, state.last_height, state.height_constraint), (false, 4.0, Some(9.0)));
        assert_eq!(candidate.unwrap().rotation, initial.rotation);
        assert_eq!(extra.0, [0x5a; 20]);
    }

    #[test]
    fn public_continue_stair_syncs_connected_style_and_throttles_only_preview_audio() {
        use country_core::{resources::{stairs::stairs_state::ObservedStairEndpoint,
            walls::cached_decorator_transforms::CachedDecoratorTransform},
            systems::{collision::world::RaycastWorldResult, mode_manager::{AppMode, EditSubject},
                ui::resources::ui_state::{WidgetId, UiInteraction}}};
        use super::super::ui_decorator_affordance_dispatch::{DecoratorAffordanceDispatcher,
            DecoratorAffordanceInteraction, DecoratorTargetAffordance};
        let mut world = public_world();
        let widget = WidgetId(444);
        for id in [10, 20, 30] {
            let mut raw = [0; 20];
            raw[..4].copy_from_slice(&(id as f32 * 0.3).to_le_bytes());
            raw[4..8].copy_from_slice(&(id as f32 * 0.1).to_le_bytes());
            raw[8..12].copy_from_slice(&(id + 100_u32).to_le_bytes());
            raw[12..16].copy_from_slice(&(id + 200_u32).to_le_bytes());
            raw[16] = u8::from(id == 20);
            let mut info = DecoratorInfo { rank: 1, ty: 63, decorator_id: id, u28: id + 100000, ..Default::default() };
            info.set_destination(DecoratorDst::Terrain(TerrainDecoratorDst {
                position_xz: Vec2::new(id as f32, 0.0), yaw: DecoratorRotationXZ(0.0) }));
            info.set_extra(DecoratorExtraInfo::Stair(StairExtra(raw)));
            world.resource_mut::<DecoratorStorage>().apply_decorator_place(DecoratorStorageAddr::Terrain, DecoratorId(id), &info);
        }
        let target_position = Vec3::new(20.0, 6.0, 0.0);
        world.resource_mut::<CachedDecoratorTransforms>().insert(20, CachedDecoratorTransform {
            rotation: glam::Quat::IDENTITY, translation: target_position, scale: Vec3::ONE });
        let mut endpoint = [0; 24];
        endpoint[..8].copy_from_slice(&1_u64.to_le_bytes());
        endpoint[16..20].copy_from_slice(&10_u32.to_le_bytes());
        world.resource_mut::<StairsState>().try_set_endpoint(ObservedStairEndpoint::from_observed(endpoint));
        world.resource_mut::<StairsState>().add_edge(10, 30);
        world.insert_resource(DecoratorAffordanceDispatcher { state: 5, payload: 0, ty: 63,
            interaction: Some(DecoratorAffordanceInteraction::ContinueStair(DecoratorTargetAffordance {
                widget, hit: RaycastWorldResult::miss(), decorator_id: DecoratorId(20),
                ty: country_core::resources::walls::decorator::DecoratorType(63) })) });
        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.add_systems(ui_place_decorator);
        for phase in [3, 4, 1] {
            world.insert_resource(UiState::new(UiInteraction { native_tag: 2, native_phase: phase, widget }));
            schedule.run(&mut world);
            let audio = world.resource_mut::<Events<PlayStairsAudioCmd>>().drain().collect::<Vec<_>>();
            if phase == 4 { assert!(audio.is_empty()); }
            else {
                assert_eq!(audio.len(), 1);
                assert_eq!(audio[0].kind, if phase < 2 { 6 } else { 5 });
                assert_eq!(audio[0].position, target_position);
                assert_eq!(&audio[0].values[..2], &[2.0, 6.0]);
            }
            assert!(world.resource::<StairsState>().graph().contains_edge(10, 20));
            for id in [10, 20, 30] {
                let storage = world.resource::<DecoratorStorage>();
                let info = storage.lookup_decorator_info(DecoratorId(id)).unwrap();
                let extra = info.extra().as_stair();
                assert_eq!(info.u28, 100020);
                assert_eq!(u32::from_le_bytes(extra.0[8..12].try_into().unwrap()), 120);
                assert_eq!(extra.0[16], 1);
                // 原只传播样式+8与栏杆+16，独立高度、宽度、魔法颜色不能一并覆盖。
                assert_eq!(f32::from_le_bytes(extra.0[..4].try_into().unwrap()), id as f32 * 0.3);
                assert_eq!(f32::from_le_bytes(extra.0[4..8].try_into().unwrap()), id as f32 * 0.1);
                assert_eq!(u32::from_le_bytes(extra.0[12..16].try_into().unwrap()), id + 200);
            }
        }
        assert!(world.resource::<StairsState>().endpoint().is_unset());
        let changes = world.resource_mut::<Events<RequestAppModeChange>>().drain().collect::<Vec<_>>();
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].0, AppMode::Editing(EditSubject::None));
        assert_eq!(world.resource::<DecoratorIdGen>().peek(), DecoratorId(0));
    }

    #[test]
    fn public_start_place_owned_rotation_keeps_native_flag_state_and_confirmation_separate() {
        use bevy_ecs::schedule::IntoScheduleConfigs;
        use country_core::systems::{mode_manager::{AppMode, ClutterMeshTy},
            collision::world::RaycastWorldResult,
            ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction,
            ui::resources::ui_state::WidgetId};
        use super::super::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher;
        let mut world = public_world();
        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.add_systems((ui_place_decorator,
            crate::backup::create_or_restore_decorator_backup::<country_core::resources::walls::decorator_archivist::LiveColliderPayload>).chain());
        let request = StartPlaceDecoClutterInteraction { relative_rotation: glam::Quat::from_rotation_y(0.4),
            widget: WidgetId(4242), hit: RaycastWorldResult { handle: [0xb, 0, 0], position: Vec3::new(4.0, 0.0, 5.0),
                normal: Vec3::Y, toi: 1.0, discriminant: 0, extra: [0.0; 2] }, ty: ClutterMeshTy::PlantPotV1 };
        ordinary_place(&mut world, 3, request.hit.position);
        world.resource_mut::<DecoratorAffordanceDispatcher>().set_place_clutter(request);
        world.insert_resource(AppMode::CloneClutter);
        schedule.run(&mut world);
        let info = world.resource::<DecoratorStorage>().get(DecoratorStorageAddr::Terrain).unwrap().decorators[&0];
        // 原flag1让apply state为0，但尚未确认且不消耗正式ID；不能把这三件事合成is_preview。
        assert_eq!(info.state, 0);
        assert_eq!(world.resource::<DecoratorIdGen>().peek(), DecoratorId(0));
        assert!(!world.resource::<DeferredDecoratorEditHistory>().pending);
        world.resource::<DeferredDecoratorOpEvents>().with_events(|events| {
            assert_eq!((events[0][0x44], events[0][0x80]), (0, 0));
        });
        let expected = country_core::resources::walls::decorator::DecoratorRotationXZ::from_quat(
            placement_relative_rotation(request.relative_rotation,
                world.resource::<country_core::systems::camera_rig::MainCamera>().xz_projective_rotation()));
        let DecoratorDst::Terrain(destination) = info.destination() else { panic!() };
        assert_eq!(destination.yaw, expected);
        ordinary_place(&mut world, 1, request.hit.position);
        world.resource_mut::<DecoratorAffordanceDispatcher>().set_place_clutter(request);
        schedule.run(&mut world);
        assert_eq!(world.resource::<DecoratorIdGen>().peek(), DecoratorId(1));
        assert!(world.resource::<DeferredDecoratorEditHistory>().pending);
        assert_eq!(world.resource::<Events<RequestAppModeChange>>().len(), 1);
        world.resource::<DeferredDecoratorOpEvents>().with_events(|events| {
            assert_eq!((events.last().unwrap()[0x44], events.last().unwrap()[0x80]), (1, 0));
        });
    }

    #[test]
    fn public_stair_widget_inherits_height_style_removed_support_and_advances_real_endpoint() {
        use bevy_ecs::schedule::IntoScheduleConfigs;
        use country_core::{resources::{stairs::{stairs_state::{ObservedStairEndpoint, StairGraphEdge},
            stairs_removed_supports::StairsRemovedSupports}, walls::cached_decorator_transforms::CachedDecoratorTransform},
            systems::mode_manager::{AppMode, DecoratorType}};
        use super::super::ui_decorator_affordance_dispatch::{DecoratorAffordanceDispatcher, DecoratorAffordanceInteraction};
        let mut world = public_world();
        world.insert_resource(AppMode::Decorator(DecoratorType::Stairs));
        let mut extra = [0; 20];
        extra[..4].copy_from_slice(&4.0_f32.to_le_bytes());
        extra[4..8].copy_from_slice(&1.5_f32.to_le_bytes());
        extra[8..12].copy_from_slice(&77_u32.to_le_bytes());
        extra[16] = 1;
        for (id, x) in [(10, 0.0), (20, 1.0)] {
            let mut info = DecoratorInfo { rank: 1, ty: 63, u28: 100777, decorator_id: id, u45: 5, ..Default::default() };
            info.set_destination(DecoratorDst::Terrain(TerrainDecoratorDst { position_xz: Vec2::new(x, 0.0), yaw: DecoratorRotationXZ(0.0) }));
            info.set_extra(DecoratorExtraInfo::Stair(StairExtra(extra)));
            world.resource_mut::<DecoratorStorage>().apply_decorator_place(DecoratorStorageAddr::Terrain, DecoratorId(id), &info);
            world.resource_mut::<CachedDecoratorTransforms>().insert(id, CachedDecoratorTransform {
                rotation: glam::Quat::IDENTITY, translation: Vec3::new(x, 4.0, 0.0), scale: Vec3::ONE });
        }
        world.resource_mut::<DecoratorIdGen>().reserve(DecoratorId(20));
        world.resource_mut::<StairsState>().add_edge(10, 20);
        let mut endpoint = [0; 24];
        endpoint[..8].copy_from_slice(&1_u64.to_le_bytes());
        endpoint[16..20].copy_from_slice(&10_u32.to_le_bytes());
        world.resource_mut::<StairsState>().try_set_endpoint(ObservedStairEndpoint::from_observed(endpoint));
        world.resource_mut::<StairsRemovedSupports>().record(StairGraphEdge::new(10, 20));
        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.add_systems((ui_place_decorator, super::super::ui_stairs_misc::continue_stairs,
            crate::backup::create_or_restore_decorator_backup::<country_core::resources::walls::decorator_archivist::LiveColliderPayload>).chain());
        for phase in [3, 4, 1] {
            ordinary_place(&mut world, phase, Vec3::new(2.0, 0.0, 0.0));
            let mut dispatcher = world.resource_mut::<DecoratorAffordanceDispatcher>();
            dispatcher.ty = 63;
            let Some(DecoratorAffordanceInteraction::PlaceDecorator(place)) = dispatcher.interaction.as_mut() else { panic!() };
            place.ty = country_core::resources::walls::decorator::DecoratorType(63);
            drop(dispatcher);
            schedule.run(&mut world);
            // 原unleash边界排空当前帧队列，下一帧不能重复消费旧preview事件。
            world.resource::<DeferredDecoratorOpEvents>().unleash(&mut Vec::new());
        }
        let info = world.resource::<DecoratorStorage>().get(DecoratorStorageAddr::Terrain).unwrap().decorators[&21];
        let actual = info.extra().as_stair();
        assert_eq!(f32::from_le_bytes(actual.0[..4].try_into().unwrap()), 4.0);
        assert_eq!(&actual.0[4..17], &extra[4..17]);
        assert_eq!((info.state, info.rank, info.ty, info.u28), (0, 1, 63, 100777));
        assert_eq!(world.resource::<DecoratorIdGen>().peek(), DecoratorId(22));
        assert_eq!(world.resource::<StairsState>().endpoint().node(), 21);
        assert!(world.resource::<StairsState>().graph().contains_edge(10, 21));
        assert!(world.resource::<StairsRemovedSupports>().contains(&StairGraphEdge::new(10, 21)));
        assert_eq!(world.resource::<Events<country_core::resources::onboarding::NotifyHintSystemStairsPlaced>>().len(), 1);
        assert_eq!(world.resource::<LiveDecoratorArchivist>().special1.as_ref().unwrap().items.len(), 2);
    }

    fn event(split: bool) -> InstaCreateStairPoint {
        let mut raw = [0; 0x70];
        raw[..8].copy_from_slice(&1_u64.to_le_bytes());
        let mut destination = DecoratorInfo::default();
        destination.set_destination(DecoratorDst::Terrain(TerrainDecoratorDst {
            position_xz: Vec2::new(3.0, 0.0), yaw: DecoratorRotationXZ(0.0) }));
        raw[0x10..0x38].copy_from_slice(&crate::apply_decorator_op::info_to_bytes(destination)[..40]);
        raw[0x38] = u8::from(split);
        raw[0x3c..0x40].copy_from_slice(&1_u32.to_le_bytes());
        raw[0x40..0x44].copy_from_slice(&2_u32.to_le_bytes());
        raw[0x44..0x48].copy_from_slice(&3.0_f32.to_le_bytes());
        raw[0x48..0x4c].copy_from_slice(&1.5_f32.to_le_bytes());
        raw[0x58..0x5c].copy_from_slice(&3.0_f32.to_le_bytes());
        raw[0x5c..0x60].copy_from_slice(&2.0_f32.to_le_bytes());
        raw[0x64..0x68].copy_from_slice(&100003_u32.to_le_bytes());
        raw[0x68] = 0x3f;
        InstaCreateStairPoint::from_observed(raw)
    }

    #[test]
    fn relative_rotation_complete_native_kernel_matches_finite_and_signed_zero_inputs() {
        let fixture = include_str!(concat!(env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/1407abd63-place-quat-native.txt"));
        for (index, row) in fixture.lines().enumerate() {
            let words: Vec<_> = row.split_whitespace().map(|word| u32::from_str_radix(word, 16).unwrap()).collect();
            let q = |start| glam::Quat::from_array(std::array::from_fn(|lane| f32::from_bits(words[start + lane])));
            let result = placement_relative_rotation(q(0), q(4)).to_array().map(f32::to_bits);
            assert_eq!(result.as_slice(), &words[8..12], "原旋转case {index}");
        }
    }

    #[test]
    fn slope_complete_original_closures_match_real_graph_and_optional_cache() {
        use country_core::resources::{stairs::stairs_state::ObservedStairEndpoint,
            walls::cached_decorator_transforms::CachedDecoratorTransform};
        let fixture = include_str!(concat!(env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/1407ad8c0-stair-slope-native.txt"));
        for (index, row) in fixture.lines().enumerate() {
            let columns: Vec<_> = row.split_whitespace().collect();
            let count = columns[0].parse::<usize>().unwrap();
            let bytes: Vec<_> = (0..columns[3].len()).step_by(2)
                .map(|offset| u8::from_str_radix(&columns[3][offset..offset + 2], 16).unwrap()).collect();
            let f32_at = |offset| f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
            let anchor = Vec3::new(f32_at(0), f32_at(4), f32_at(8));
            let mut stairs = StairsState::default();
            let mut endpoint = [0; 24];
            endpoint[..8].copy_from_slice(&1_u64.to_le_bytes());
            endpoint[16..20].copy_from_slice(&55_u32.to_le_bytes());
            stairs.try_set_endpoint(ObservedStairEndpoint::from_observed(endpoint));
            stairs.add_node(55);
            let mut transforms = CachedDecoratorTransforms::default();
            for neighbor in 0..count {
                let node = neighbor as u32 + 1;
                stairs.add_edge(55, node);
                let offset = 12 + neighbor * 16;
                if bytes[offset] != 0 {
                    transforms.insert(node, CachedDecoratorTransform { rotation: glam::Quat::IDENTITY,
                        translation: Vec3::new(f32_at(offset + 4), f32_at(offset + 8), f32_at(offset + 12)), scale: Vec3::ONE });
                }
            }
            assert_eq!(stair_neighbor_exceeds_slope(&stairs, &transforms, anchor, true), columns[1] == "1", "原下降坡度case {index}");
            assert_eq!(stair_neighbor_exceeds_slope(&stairs, &transforms, anchor, false), columns[2] == "1", "原上升坡度case {index}");
        }
    }

    #[test]
    fn complete_native_stair_height_selection_matches_actual_floor_graph_and_cache() {
        use country_core::{resources::{stairs::stairs_state::ObservedStairEndpoint,
            terrain_height::{TerrainHeightsData, TerrainHeightSsbo},
            walls::{cached_decorator_transforms::CachedDecoratorTransform, public_walls::PublicWalls,
                decorator::RoofDecoratorDst}}, systems::collision::world::RaycastWorldResult};
        let fixture = include_str!(concat!(env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/1407acc79-stair-height-native.txt"));
        let decode = |text: &str| -> Vec<u8> { (0..text.len()).step_by(2)
            .map(|offset| u8::from_str_radix(&text[offset..offset + 2], 16).unwrap()).collect() };
        for (index, row) in fixture.lines().enumerate() {
            let columns: Vec<_> = row.split_whitespace().collect();
            let count = columns[2].parse::<usize>().unwrap();
            let bytes = decode(columns[3]);
            let f32_at = |offset| f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
            let anchor = Vec3::new(f32_at(0), f32_at(4), f32_at(8));
            let target = Vec3::new(f32_at(12), f32_at(16), f32_at(20));
            let floor = f32_at(24);
            let mut terrain = TerrainHeightsData::<TerrainHeightSsbo>::new();
            terrain.set_height_range(floor, floor + 1.0);
            let mut stairs = StairsState::default();
            let mut endpoint = [0; 24];
            endpoint[..8].copy_from_slice(&1_u64.to_le_bytes());
            endpoint[16..20].copy_from_slice(&55_u32.to_le_bytes());
            stairs.try_set_endpoint(ObservedStairEndpoint::from_observed(endpoint));
            stairs.add_node(55);
            let mut storage = DecoratorStorage::default();
            let mut source = DecoratorInfo { ty: 63, rank: 1, ..Default::default() };
            source.set_destination(DecoratorDst::Terrain(TerrainDecoratorDst {
                position_xz: Vec2::new(anchor.x, anchor.z), yaw: DecoratorRotationXZ(0.0) }));
            storage.apply_decorator_place(DecoratorStorageAddr::Terrain, DecoratorId(55), &source);
            let mut transforms = CachedDecoratorTransforms::default();
            transforms.insert(55, CachedDecoratorTransform { rotation: glam::Quat::IDENTITY,
                translation: anchor, scale: Vec3::ONE });
            for neighbor in 0..count {
                let node = neighbor as u32 + 1;
                stairs.add_edge(55, node);
                let offset = 28 + neighbor * 16;
                if bytes[offset] != 0 {
                    transforms.insert(node, CachedDecoratorTransform { rotation: glam::Quat::IDENTITY,
                        translation: Vec3::new(f32_at(offset + 4), f32_at(offset + 8), f32_at(offset + 12)), scale: Vec3::ONE });
                }
            }
            // 这里验证原已完成目标解析之后的高度块，目标Roof查询/接受门另由Intent验收。
            let destination = DecoratorDst::Roof(RoofDecoratorDst { coord: Vec2::ZERO, yaw: DecoratorRotationXZ(0.0) });
            let hit = RaycastWorldResult { handle: [2, 1, 0], position: target, normal: Vec3::Y,
                toi: 1.0, discriminant: 1, extra: [0.0; 2] };
            let mut extra = StairExtra([0; 20]);
            adjust_new_stair_height(&mut extra, destination, &hit, columns[0] == "1", columns[1] == "1",
                &stairs, &storage, &transforms, &PublicWalls::default(), &terrain);
            assert_eq!(&extra.0[..4], decode(columns[4]), "原完整高度块case {index}");
        }
    }

    #[test]
    fn actual_event_storage_graph_previous_curve_and_owned_archive_reanchor_both_sides() {
        let mut world = World::new();
        let mut storage = DecoratorStorage::default();
        let mut stairs = StairsState::default();
        stairs.add_edge(1, 2);
        for (id, u) in [(10, 0.2), (11, 0.8)] {
            let mut info = DecoratorInfo { decorator_id: id + 100, ty: 0, rank: 2, ..Default::default() };
            info.set_destination(DecoratorDst::Stair(StairDecoratorDst { first_id: 1, second_id: 2,
                coordinate: Vec2::new(0.4, u), yaw: DecoratorRotationXZ(-0.7) }));
            storage.get_or_insert_default(DecoratorStorageAddr::Stair).insert(DecoratorId(id), info);
            storage.decorator_to_storage.update(DecoratorId(id), DecoratorStorageAddr::Stair);
        }
        let mut requests = PlayermadeStairsAssemblyRequests::default();
        requests.previous.sampled_points.insert(StairGraphEdgeIdentity::from(StairSegment::new(DecoratorId(1), DecoratorId(2))),
            SampledStairsPoints { points: vec![Vec3::ZERO, Vec3::new(10.0, 10.0, 0.0)],
                tangents: vec![], widths: vec![], auxiliary_words: vec![], metadata: StairSamplesMetadata {
                    info_28: 0, extra_08: 0, extra_0c: 0, removed_support: false, extra_10: 0 } });
        let mut ids = DecoratorIdGen::default();
        ids.reserve(DecoratorId(8));
        world.insert_resource(storage);
        world.insert_resource(LiveDecoratorArchivist::default());
        world.insert_resource(RaycastWorld::default());
        world.insert_resource(DecoratorPhysicsColliders::default());
        world.insert_resource(DeferredDecoratorEditHistory::default());
        let events = DeferredDecoratorOpEvents::default();
        let mut params = bevy_ecs::system::SystemState::<(ResMut<DecoratorStorage>,
            ResMut<LiveDecoratorArchivist>, DecoratorArchivalResources,
            ResMut<DeferredDecoratorEditHistory>)>::new(&mut world);
        let (mut storage, mut archivist, archival, mut history) = params.get_mut(&mut world);
        let decoded = InstantStairPoint::from_event(&event(true));
        let mut reanchors = Vec::new();
        apply_instant_stair_point(decoded, false, &mut storage, &mut archivist, &archival,
            &mut history, &events, &mut ids, &mut stairs, &mut reanchors);
        assert_eq!(ids.peek(), DecoratorId(10));
        assert_eq!(reanchors.len(), 2);
        assert!(stairs.graph().contains_edge(1, 9));
        assert!(stairs.graph().contains_edge(9, 2));
        assert!(!stairs.graph().contains_edge(1, 2));
        reanchor_split_stair_decorators(&reanchors, &requests, &mut storage, &mut archivist,
            &archival, &mut history, &events);
        let placed = storage.get(DecoratorStorageAddr::Terrain).unwrap().decorators[&9];
        assert_eq!((placed.rank, placed.ty, placed.state, placed.u45, placed.u28), (1, 0x3f, 0, 5, 100003));
        assert_eq!(placed.extra(), DecoratorExtraInfo::Stair(decoded.extra));
        let bucket = storage.get(DecoratorStorageAddr::Stair).unwrap();
        let DecoratorDst::Stair(left) = bucket.decorators[&10].destination() else { panic!() };
        let DecoratorDst::Stair(right) = bucket.decorators[&11].destination() else { panic!() };
        assert_eq!((left.first_id, left.second_id), (1, 9));
        assert_eq!((right.first_id, right.second_id), (2, 9));
        assert_eq!(left.coordinate, Vec2::new(0.4, 0.2 / 0.3));
        assert_eq!(right.coordinate, Vec2::new(-0.4, (0.8 - 0.3) / (1.0 - 0.3)));
        assert_eq!(right.yaw.0, -0.7 + std::f32::consts::PI);
        assert_eq!(bucket.decorators[&10].decorator_id, 110);
        assert_eq!(bucket.decorators[&11].decorator_id, 111);
        assert_eq!(history.records.len(), 3);
        assert!(history.pending);
        assert!(archivist.special1.is_some() && archivist.special2.is_some());
        assert_eq!(archivist.special2.as_ref().unwrap().items.len(), 2);
        assert_eq!(events.with_events(|events| events.len()), 3);
    }
}

/// B1407a9c10原九组资源：即时创建、继续楼梯、真实Widget放置及其历史/回滚副作用。
#[allow(clippy::too_many_arguments)]
pub fn ui_place_decorator(
    owners: (
        ResMut<DecoratorStorage>,
        ResMut<country_core::resources::walls::decorator_archivist::LiveDecoratorArchivist>,
        Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,
        Res<CachedDecoratorTransforms>,
        country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,
        country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,
        ResMut<super::ui_move_decorator::MoveDecoratorDeltaPositions>,
        ResMut<crate::backup::DecoratorBackup>,
        Res<super::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,
        ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,
        Res<country_core::resources::glade_color_customization::GladeColorCustomization>,
        ResMut<StairsState>,
        EventReader<EvInitPlaceDecoratorInteractionState>,
    ),
    mut removed_supports: ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,
    environment: (
        Res<country_core::resources::walls::public_walls::PublicWalls>,
        Query<&country_core::resources::roofs::Roof>,
        Res<country_core::resources::color_id::WallColorIds>,
        Res<country_core::resources::glade_border::GladeBorder>,
        Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo>>,
        Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,
    ),
    input: (
        Res<country_core::systems::mode_manager::AppMode>,
        Res<UiState>,
        Res<country_core::systems::backend::input::mod_key::Modifiers>,
        Res<bevy_time::Time<()>>,
        super::ui_move_decorator::RaycastResources,
        Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,
        ResMut<UiSignifierStream>,
    ),
    mut place_state: ResMut<PlaceDecoratorInteractionState>,
    mut history: ResMut<crate::deferred_decorator_history::DeferredDecoratorEditHistory>,
    notifications: (
        EventWriter<DisplayOutOfBorder>,
        EventWriter<FeedbackInputOutOfBorder>,
        EventWriter<RequestAppModeChange>,
        EventReader<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>,
        EventReader<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>,
        EventWriter<country_core::resources::onboarding::NotifyHintSystemStairsPlaced>,
    ),
    world: (
        Res<country_core::systems::collision::world::RaycastWorld>,
        // 原进程全局d90由config.dev.demo写入；现宿主已保存在这个同一owner。
        Res<country_core::systems::backend::input::translate_input::DemoModeEnabled>,
    ),
    audio: (Local<PlayedConnectSoundLastFrame>, EventWriter<PlayStairsAudioCmd>),
) {
    let (mut storage, mut archivist, events, transforms, derived, archival, mut deltas,
        mut backup, dispatcher, mut ids, colors, mut stairs, mut inits) = owners;
    let (walls, roofs, wall_colors, border, terrain, requests) = environment;
    let (mode, ui, modifiers, time, raycast, cursor, mut stream) = input;
    let (mut display, mut feedback, mut app_mode, mut instant_decorators, mut instant_stairs, mut hints) = notifications;
    let (world, demo) = world;
    let (mut played_connect_sound, mut stairs_audio) = audio;
    let previous_position = deltas.previous_position.take();
    if mode.native_tag() != 11 { *place_state = PlaceDecoratorInteractionState::default(); }
    if let Some(event) = inits.read().last() { *place_state = event.0; }
    let mut had_instant = false;
    for event in instant_decorators.read() {
        apply_instant_decorator(event, demo.0, &mut storage, &mut archivist, &archival,
            &mut history, &events, &mut ids, &world, &terrain, &mut app_mode);
        had_instant = true;
    }
    let mut reanchors = Vec::new();
    for event in instant_stairs.read() {
        let event = InstantStairPoint::from_event(event);
        apply_instant_stair_point(event, demo.0, &mut storage, &mut archivist, &archival,
            &mut history, &events, &mut ids, &mut stairs, &mut reanchors);
        had_instant = true;
    }
    reanchor_split_stair_decorators(&reanchors, &requests, &mut storage, &mut archivist,
        &archival, &mut history, &events);
    if had_instant { return; }
    if let Some(target) = dispatcher.continued_stair() {
        played_connect_sound.0 = continue_stair_connection(target, &ui, &mut storage,
            &mut stairs, &transforms, played_connect_sound.0, &mut stairs_audio, &mut app_mode);
        return;
    }
    if dispatcher.place_decorator().is_none() && dispatcher.place_clutter().is_none() {
        if modifiers.is_disable_snapping_just_pressed() && !stairs.endpoint().is_unset() {
            if let Some(position) = transforms.try_position(stairs.endpoint().node()) {
                place_state.height_constraint = Some(position.y);
            }
        }
        played_connect_sound.0 = false;
        return;
    }
    use super::{calculate_decorator_dst::RaycastDecoratorDst,
        decorator_interaction_intent::{DecoratorIntentCandidate, DecoratorInteractionIntent, calculate_decorator_interaction_intent},
        move_ops::calculate_move_dst};
    use country_core::resources::walls::{WallId, decorator::DecoratorExtraInfo};
    use country_core::systems::mode_manager::{AppMode, EditSubject};
    let (widget, hit, ty, native_flag, relative) = if let Some(place) = dispatcher.place_clutter() {
        (place.request.widget, place.request.hit, place.request.ty.native_tag(),
            place.native_flag_0x68, Some(place.request.relative_rotation))
    } else {
        let place = dispatcher.place_decorator().unwrap();
        // 原普通writer最终发布flag0；Quat区不活动，不能从旧union字节读取旋转。
        (place.widget, place.hit, place.ty.0, false, None)
    };
    if !backup.created {
        backup.state = 1;
        place_state.native_flag_10 = native_flag;
        if !native_flag { return; }
    }
    let widget = ui.widget_context(widget);
    if !widget.is_active() { return; }
    let phase = widget.dragged();
    let finished = phase < 2;
    let completed_or_native = finished || native_flag;
    if finished {
        history.pending = true;
        backup.consume_after_processing();
    }
    if phase == 2 && mode.native_tag() != 20 {
        place_state.progress = 0;
    } else {
        if place_state.progress == 1 { return; }
        if place_state.native_flag_10 { backup.state = 1; }
    }
    let id = if finished { ids.generate() } else { ids.peek() };
    // 原baseline始终WallCompound、空extra niche2、rank1；候选目的另传给intent。
    let baseline = DecoratorInfo { u20: 1 << 16, u40: 2 << 8, rank: 1,
        u45: if demo.0 { 0 } else { 5 }, ty, ..Default::default() };
    let mut extra = initial_placement_extra(ty, &stairs, &storage, &colors);
    let destination = placement_raycast_destination(&hit, &baseline);
    let eye = raycast.camera.eye_position();
    let dx = eye.x - hit.position.x;
    let dz = eye.z - hit.position.z;
    let reciprocal = 1.0 / (dz * dz + dx * dx).sqrt();
    let mut flat_only = (dz * reciprocal) * hit.normal.z + (dx * reciprocal) * hit.normal.x <= 0.0;
    let slot = ty.wrapping_sub(0x38);
    let mut candidate = if destination == RaycastDecoratorDst::Invalid { None } else {
        let rotation = if slot == 6 || slot >= 8 {
            placement_relative_rotation(relative.unwrap_or(glam::Quat::IDENTITY), raycast.camera.xz_projective_rotation())
        } else if matches!(destination, RaycastDecoratorDst::Terrain)
            || (matches!(destination, RaycastDecoratorDst::Roof { .. }) && slot == 7) {
            raycast.camera.xz_projective_rotation()
        } else {
            utils::xz_projective_rotation_from_forward(hit.normal)
        };
        Some(DecoratorIntentCandidate { rotation, destination, position: hit.position,
            roof_coordinate: (hit.discriminant == 1).then(|| glam::Vec2::from_array(hit.extra)) })
    };
    let height_adjusted = if ty == 0x3f {
        let DecoratorExtraInfo::Stair(ref mut extra) = *extra.as_mut().unwrap() else { unreachable!() };
        let adjusted = constrain_stair_placement(&mut candidate, extra, &mut place_state,
            modifiers.disable_snapping(), &hit, &stairs, &storage, &transforms, &raycast,
            &walls, &terrain, cursor.0);
        if adjusted { flat_only = false; }
        adjusted
    } else { false };
    let intent = if let Some(candidate) = candidate {
        let source = match candidate.destination {
            RaycastDecoratorDst::Wall(wall) => DecoratorStorageAddr::Wall(wall),
            RaycastDecoratorDst::Roof { payload, .. } => DecoratorStorageAddr::Wall(WallId(payload)),
            RaycastDecoratorDst::Terrain => DecoratorStorageAddr::Terrain,
            RaycastDecoratorDst::StairTop(_) => DecoratorStorageAddr::Stair,
            RaycastDecoratorDst::Invalid => unreachable!(),
        };
        calculate_decorator_interaction_intent(&walls, &roofs, &requests, candidate, id,
            &baseline, &[], &storage, derived.as_view(), &transforms, source,
            !modifiers.disable_snapping(), flat_only, &mut stream, &world)
    } else { DecoratorInteractionIntent::UnsupportedTerrain { position: hit.position } };
    if std::env::var_os("TG_M6_DECORATOR_TRACE").is_some() {
        eprintln!("M6_PLACE id={id:?} phase={phase} finished={finished} intent={intent:?} backup_state={} progress={}", backup.state, place_state.progress);
    }
    let accepted = match intent {
        DecoratorInteractionIntent::Accepted(accepted) => accepted,
        DecoratorInteractionIntent::UnsupportedTerrain { .. } => {
            show_decorator_icon(ty, cursor.0, &mut stream);
            played_connect_sound.0 = false;
            return;
        }
        DecoratorInteractionIntent::Rejected { .. } => {
            backup.state = 2;
            place_state.progress = if place_state.progress < 2 { 1 } else { 2 };
            show_decorator_icon(ty, cursor.0, &mut stream);
            played_connect_sound.0 = false;
            return;
        }
    };
    if matches!(accepted.destination, super::move_ops::MoveDstCandidate::Terrain(_))
        && !border.is_pos_inside(glam::Vec2::new(accepted.position.x, accepted.position.z)) {
        display.write(DisplayOutOfBorder);
        if finished { feedback.write(FeedbackInputOutOfBorder); }
        show_decorator_icon(ty, cursor.0, &mut stream);
        played_connect_sound.0 = false;
        return;
    }
    let position = accepted.position;
    deltas.velocity = previous_position.map_or(0.0, |previous| {
        let dx = position.x - previous.x;
        let dy = position.y - previous.y;
        let dz = position.z - previous.z;
        (dz * dz + (dy * dy + dx * dx)).sqrt() / time.delta_secs()
    });
    deltas.previous_position = Some(position);
    let mut placed = DecoratorInfo::default();
    calculate_move_dst(&mut placed, accepted.destination, &walls);
    place_state.progress = 2;
    let state = u8::from(!completed_or_native);
    if let Some(merge) = accepted.merge {
        use crate::apply_decorator_op::{DecoratorChangePatch, DecoratorOp, apply_decorator_op};
        let mut patch = DecoratorChangePatch::rotation(merge.target,
            accepted.target_storage, placed.destination(), position);
        patch.rank = Some(merge.rank);
        archivist.archive_current_decorators_at_with_resources(accepted.storage, &storage, &archival, "move preview src");
        archivist.archive_current_decorators_at_with_resources(accepted.target_storage, &storage, &archival, "move preview dst");
        apply_decorator_op(&DecoratorOp::from_change(accepted.storage, patch), state, accepted.storage,
            &mut storage, Some(&mut *archivist), Some(&mut *history), &events);
        played_connect_sound.0 = false;
        return;
    }
    placed.u28 = placement_color(ty, accepted.target_storage, &stairs, &storage, &colors, &wall_colors);
    if finished && mode.native_tag() == 20 {
        app_mode.write(RequestAppModeChange(AppMode::Editing(EditSubject::None)));
    }
    if ty == 0x3f && !height_adjusted {
        let DecoratorExtraInfo::Stair(ref mut extra) = *extra.as_mut().unwrap() else { unreachable!() };
        adjust_new_stair_height(extra, placed.destination(), &hit, modifiers.disable_snapping(),
            place_state.fixed_height, &stairs, &storage, &transforms, &walls, &terrain);
    }
    if prepare_stair_support_inheritance(ty, placed.destination(), accepted.target_storage, id,
        &stairs, &storage, &mut removed_supports) { return; }
    if finished && ty == 0x3f {
        hints.write(country_core::resources::onboarding::NotifyHintSystemStairsPlaced);
    }
    placed.rank = 1;
    placed.ty = ty;
    placed.u40 = 3 << 8;
    if let Some(extra) = extra { placed.set_extra(extra); }
    // 原最终Place使用原命中位置；merge事件与速度使用accepted.position，二者不能混同。
    let op = crate::apply_decorator_op::DecoratorOp::place_at(id, placed, hit.position, 0,
        !native_flag || finished, demo.0);
    crate::apply_decorator_op::apply_place_with_resources(&op, state, accepted.target_storage,
        &mut storage, &mut archivist, Some(&mut history), &archival, &events);
    played_connect_sound.0 = false;
}

/// 原 1407adb20：以屏幕位置命名的真实图标工厂，不携带装饰 ID。
pub fn show_decorator_icon(icon: u8, pos: glam::Vec2, stream: &mut UiSignifierStream) {
    use country_core::systems::ui::signifiers::signifier_factory::SignifierFactory;
    let name = format!("decorator_{pos:?}");
    let factory = SignifierFactory::from_closure(
        &name,
        std::sync::Arc::new(move |meshes, shaders, commands| {
            vec![decorator_cursor_icon(pos, icon, meshes, shaders, commands)]
        }),
    );
    stream.write_factory(&factory);
}

/// 原 1407adcb0/492B：解析原类型图标表，交给现有二维 UI 组件与绘制消费方。
pub fn decorator_cursor_icon(
    pos: glam::Vec2,
    icon: u8,
    meshes: &country_core::asset_libraries::mesh_library::AssetMeshLibrary,
    shaders: &country_core::asset_libraries::shader_library::AssetShaderLibrary,
    commands: &mut Commands,
) -> Entity {
    use country_core::components::{Ui2DFollowMouse, Ui2dExtent, Ui2dImage, UiInactive};
    use country_core::systems::ui::signifiers::signifier_factory::{
        SignifierEntityBuilder, SignifierMesh,
    };
    let entity = SignifierEntityBuilder::new(SignifierMesh::Empty)
        .animation_duration(0.0)
        .build(meshes, shaders, commands);
    commands.entity(entity).insert((
        Ui2dExtent {
            pos,
            size: glam::Vec2::new(34.5, 58.0),
        },
        Ui2dImage::new(decorator_icon_name(icon)),
        UiInactive,
    ));
    commands.entity(entity).insert(Ui2DFollowMouse);
    entity
}

fn decorator_icon_name(icon: u8) -> &'static str {
    match icon.wrapping_sub(0x38) {
        0..=2 => "decorator_cursor_icon/window",
        3 => "decorator_cursor_icon/chimney",
        4 => "decorator_cursor_icon/lantern",
        5 => "decorator_cursor_icon/flag",
        // 原范围外类型也落 slot6；原 slot6/7 的两名字均为 clutter。
        _ => "decorator_cursor_icon/clutter",
    }
}

#[cfg(test)]
mod icon_factory_tests {
    use super::*;
    use bevy_ecs::world::{CommandQueue, World};
    use country_core::{
        asset_libraries::{
            mesh_library::AssetMeshLibrary, shader_library::AssetShaderLibrary,
            transient_mesh::TransientMesh,
        },
        components::{Ui2DFollowMouse, Ui2dExtent, Ui2dImage, UiInactive},
        systems::ui::signifiers::{
            animation::SignifierAnimation, signifier_create::UiSignifierEntry,
        },
    };

    #[test]
    fn original_type_table_keeps_every_byte_tag_and_duplicate_clutter_names() {
        for ty in 0..=u8::MAX {
            let expected = match ty {
                0x38..=0x3a => "decorator_cursor_icon/window",
                0x3b => "decorator_cursor_icon/chimney",
                0x3c => "decorator_cursor_icon/lantern",
                0x3d => "decorator_cursor_icon/flag",
                _ => "decorator_cursor_icon/clutter",
            };
            assert_eq!(decorator_icon_name(ty), expected);
        }
    }

    #[test]
    fn real_factory_has_position_identity_and_wakes_one_actual_ui_entity() {
        let mut stream = UiSignifierStream::default();
        let pos = glam::Vec2::new(-4.5, 12.0);
        show_decorator_icon(0x3c, pos, &mut stream);
        let UiSignifierEntry::Factory(factory) = stream.take_entries().pop().unwrap() else {
            panic!("factory expected")
        };
        assert_eq!(factory.name(), Some("decorator_Vec2(-4.5, 12.0)"));
        let meshes = AssetMeshLibrary::new();
        let shaders = AssetShaderLibrary::new();
        shaders.add_raster(
            country_core::render::shader::RasterShaderProgram::new_draw(
                "vertex_color_shader",
                Default::default(),
                &country_core::render::test_device::TestDevice(1),
            ),
            false,
        );
        let mut world = World::new();
        let mut queue = CommandQueue::default();
        let entities = factory.wake(&meshes, &shaders, &mut Commands::new(&mut queue, &world));
        queue.apply(&mut world);
        assert_eq!(entities.len(), 1);
        let id = entities[0];
        assert_eq!(world.get::<Ui2dExtent>(id).unwrap().pos, pos);
        assert_eq!(
            world.get::<Ui2dExtent>(id).unwrap().size,
            glam::Vec2::new(34.5, 58.0)
        );
        assert_eq!(
            world.get::<Ui2dImage>(id).unwrap().name,
            "decorator_cursor_icon/lantern"
        );
        assert_eq!(world.get::<Ui2dImage>(id).unwrap().layer, 0);
        assert!(world.get::<UiInactive>(id).is_some());
        assert!(world.get::<Ui2DFollowMouse>(id).is_some());
        assert!(world.get::<TransientMesh>(id).is_some());
        assert_eq!(world.get::<SignifierAnimation>(id).unwrap().duration, 0.0);
        assert_eq!(world.get::<SignifierAnimation>(id).unwrap().flag_8, 0);
        let handle = *world
            .get::<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>>(id)
            .unwrap();
        let mesh = meshes.get(handle).unwrap();
        // 原Empty分支没有任何attribute，required POSITION accessor本来就应panic。
        assert!(mesh.attributes.is_empty());
        assert!(mesh.indices.is_empty());
        let registry = std::sync::Arc::clone(meshes.transient_meshes());
        world.despawn(id);
        assert_eq!(*registry.lock().unwrap(), vec![handle]);
    }

    #[test]
    fn original_identity_omits_icon_type_and_retains_nonfinite_position() {
        fn name(icon: u8, pos: glam::Vec2) -> String {
            let mut stream = UiSignifierStream::default();
            show_decorator_icon(icon, pos, &mut stream);
            let UiSignifierEntry::Factory(factory) = stream.take_entries().pop().unwrap() else {
                panic!("factory expected")
            };
            factory.name().unwrap().to_owned()
        }
        assert_eq!(name(0x38, glam::Vec2::ZERO), name(0x3d, glam::Vec2::ZERO));
        assert!(name(0x38, glam::Vec2::new(f32::NAN, f32::INFINITY)).contains("NaN"));
    }
}
