//! B `ui_systems/ui_decorator_affordance_dispatch.rs` — 装饰操作派发（O）。
//!
//! - `DecoratorAffordanceDispatcher::clear` 0x1407af330（O 机器码手解）：16B
//!   `{state u64@0, payload u64@8}` 写 `{6, 0}`。
//! - `ui_dispatch_decorator_affordance` 0x1407af340（O）：门控 AppMode∈{0,3,4,11,20}；
//!   `EscMenuState::is_open`（0x14093fbc0）→ 直接返回；构建 widget
//!   （WidgetParams::new 0x1409a8e10 + UiSounds::empty 0x1409a8fb0 + sounds
//!   0x1409a8e80 + occlude_by_world(0) 0x1409a8ee0 + `UiState::add`
//!   0x1409236d0，名字 "ui_decide_decorator_interaction" 0x142a835e6 31 字符）
//!   优先取StartPlace事件流最新96B载荷（EventInstance步长0x70）；随后维持拖动、
//!   处理Move/Rotate请求、克隆，或遍历真实射线命中发现放置/移动/旋转/高度/楼梯继续。
//!   失活AppMode或无有效候选时清为 {6, 0}。
//! - `create_decorator_widget<&str>` 0x1412823c0 / `<DecoratorHandleName>`
//!   0x141282300（O）：volume `{tag=5, layer}`、flags 字节 `bit1 | (name=='?'
//!   时 bit3)`、sounds 写 0xb0B 参数块 → `UiState::add`（&str 单态 0x1409236d0
//!   / DecoratorHandleName 单态 0x14125d180）。
use bevy_ecs::prelude::*;
use country_core::resources::gui::EscMenuState;
use country_core::systems::ui::resources::ui_state::{UiState, WidgetId};
use country_core::systems::ui::resources::widget_params::{UiSounds, WidgetParams, WidgetVolumeShape};
use country_core::systems::ui_clutter::ui_focus_decoclutter::{
    RequestMoveClutter, RequestRotateDecorator,
};
use country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction;
use country_core::{
    resources::{
        walls::{decorator::{DecoratorId, DecoratorType}, decorator_storage::DecoratorStorage,
            decorator_subtype::{DecoratorLibKey, DecoratorMeshLibrary},
            public_walls::PublicWalls, cached_decorator_transforms::CachedDecoratorTransforms},
        stairs::stairs_state::StairsState,
        terrain_height::{TerrainHeightsData, TerrainHeightSsbo},
        mouse_terrain_raycast::CameraCursorRay,
        cursor_terrain_raycast::CursorTerrainRaycast,
        onboarding::NotifyHintSystemCanContinueStairs,
    },
    systems::{mode_manager::AppMode, collision::world::{RaycastWorld, RaycastWorldResult},
        ui::signifiers::UiSignifierStream, backend::input::mod_key::Modifiers,
        ui_clutter::ui_focus_decoclutter::LastClonedClutter},
};

// 复用唯一身份；原名称为u64和u8组合，不是字符串。
pub use crate::decorator_resources::DecoratorHandleName;

/// 原交互enum安全投影：0普通放置、1杂物放置、2移动、3旋转、4改高度、5继续楼梯、6空。
/// AppMode的11/20不是此enum的状态。原各变体活跃载荷偏移不同，用owned类型保留。
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct DecoratorAffordanceDispatcher {
    /// @+0：待处理交互判别。
    pub state: u64,
    /// @+8：原128位enum判别的第二word，已恢复变体均为0；不是WidgetId或DecoratorId。
    pub payload: u64,
    /// 当前活动变体的装饰类型；这是安全 Rust 的语义投影，不是固定原生偏移。
    /// 各原变体writer必须保留真实类型，不能读取错误固定偏移。
    pub ty: u8,
    /// 活跃变体自有载荷；原enum判别第二word为0，不是widget或DecoratorId。
    pub interaction: Option<DecoratorAffordanceInteraction>,
}

impl Default for DecoratorAffordanceDispatcher {
    fn default() -> Self {
        // B 14034362c/3637 → insert_resource<DecoratorAffordanceDispatcher>@140139690。
        Self { state: 6, payload: 0, ty: 0, interaction: None }
    }
}

/// 保留真实请求、widget身份和完整命中字段；原载荷从128位判别后的+10开始。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DecoratorAffordanceInteraction {
    PlaceDecorator(PlaceDecoratorAffordance),
    PlaceClutter(PlaceClutterAffordance),
    ChangeHeight(DecoratorTargetAffordance),
    ContinueStair(DecoratorTargetAffordance),
    Move(MoveDecoratorAffordance),
    Rotate(RequestRotateDecorator),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MoveDecoratorAffordance {
    pub request: RequestMoveClutter,
    /// 原事件reader的MOVZX写0；命中发现时为info.ty!=63，不借用header第二word。
    pub native_flag_0x5d: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DecoratorTargetAffordance {
    pub widget: WidgetId,
    pub hit: RaycastWorldResult,
    pub decorator_id: DecoratorId,
    pub ty: DecoratorType,
}

/// B variant0 只有widget、完整命中与类型；没有活跃的相对四元数载荷。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlaceDecoratorAffordance {
    pub widget: WidgetId,
    pub hit: country_core::systems::collision::world::RaycastWorldResult,
    pub ty: country_core::resources::walls::decorator::DecoratorType,
}

/// B variant1：完整保留事件值与原 +68 标志；普通 variant0 的布局不同，不能伪造旋转。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlaceClutterAffordance {
    pub request: StartPlaceDecoClutterInteraction,
    /// 原事件 reader 写1，重新发现命中后写0；具体使用由真实放置消费者裁定。
    pub native_flag_0x68: bool,
}

impl DecoratorAffordanceDispatcher {
    /// 0x1407af330（O）：清空 → {6, 0}。
    pub fn clear(&mut self) {
        self.state = 6;
        self.payload = 0;
        self.interaction = None;
    }

    pub fn set_move(&mut self, request: RequestMoveClutter) {
        self.state = 2;
        self.payload = 0;
        self.ty = request.ty.0;
        self.interaction = Some(DecoratorAffordanceInteraction::Move(
            MoveDecoratorAffordance { request, native_flag_0x5d: false },
        ));
    }

    pub fn movement(&self) -> Option<&MoveDecoratorAffordance> {
        match &self.interaction {
            Some(DecoratorAffordanceInteraction::Move(movement)) if self.gated(2) => Some(movement),
            _ => None,
        }
    }

    pub fn set_place_clutter(&mut self, request: StartPlaceDecoClutterInteraction) {
        self.state = 1;
        self.payload = 0;
        self.ty = request.ty.native_tag();
        self.interaction = Some(DecoratorAffordanceInteraction::PlaceClutter(
            PlaceClutterAffordance { request, native_flag_0x68: true },
        ));
    }

    pub fn place_clutter(&self) -> Option<&PlaceClutterAffordance> {
        match &self.interaction {
            Some(DecoratorAffordanceInteraction::PlaceClutter(place)) if self.gated(1) => Some(place),
            _ => None,
        }
    }

    pub fn place_decorator(&self) -> Option<&PlaceDecoratorAffordance> {
        match &self.interaction {
            Some(DecoratorAffordanceInteraction::PlaceDecorator(place)) if self.gated(0) => Some(place),
            _ => None,
        }
    }

    pub fn height_adjustment(&self) -> Option<&DecoratorTargetAffordance> {
        match &self.interaction {
            Some(DecoratorAffordanceInteraction::ChangeHeight(target)) if self.gated(4) => Some(target),
            _ => None,
        }
    }

    pub fn continued_stair(&self) -> Option<&DecoratorTargetAffordance> {
        match &self.interaction {
            Some(DecoratorAffordanceInteraction::ContinueStair(target)) if self.gated(5) => Some(target),
            _ => None,
        }
    }

    fn set_target(&mut self, target: DecoratorTargetAffordance, continue_stair: bool) {
        self.state = if continue_stair { 5 } else { 4 };
        self.payload = 0;
        self.ty = target.ty.0;
        self.interaction = Some(if continue_stair {
            DecoratorAffordanceInteraction::ContinueStair(target)
        } else { DecoratorAffordanceInteraction::ChangeHeight(target) });
    }

    fn set_place_decorator(&mut self, place: PlaceDecoratorAffordance) {
        self.state = 0;
        self.payload = 0;
        self.ty = place.ty.0;
        self.interaction = Some(DecoratorAffordanceInteraction::PlaceDecorator(place));
    }

    pub fn set_rotate(&mut self, request: RequestRotateDecorator) {
        self.state = 3;
        self.payload = 0;
        self.ty = request.ty.0;
        self.interaction = Some(DecoratorAffordanceInteraction::Rotate(request));
    }

    /// 原 validate_blueprints@140827294..2c9 的 decorator_ty 内联：
    /// clear(6,0) 无图标；其它活动变体取得其真实类型，不读取重构结构的裸偏移。
    pub fn decorator_ty(&self) -> Option<country_core::resources::walls::decorator::DecoratorType> {
        use country_core::resources::walls::decorator::DecoratorType;
        if self.state == 6 && self.payload == 0 {
            return None;
        }
        let ty = match (&self.interaction, self.state, self.payload) {
            (Some(DecoratorAffordanceInteraction::PlaceDecorator(place)), 0, 0) => place.ty,
            (Some(DecoratorAffordanceInteraction::ChangeHeight(target)), 4, 0)
            | (Some(DecoratorAffordanceInteraction::ContinueStair(target)), 5, 0) => target.ty,
            (Some(DecoratorAffordanceInteraction::PlaceClutter(place)), 1, 0) => DecoratorType(place.request.ty.native_tag()),
            (Some(DecoratorAffordanceInteraction::Move(movement)), 2, 0) => movement.request.ty,
            (Some(DecoratorAffordanceInteraction::Rotate(request)), 3, 0) => request.ty,
            _ => DecoratorType(self.ty),
        };
        Some(ty)
    }

    pub fn rotation(&self) -> Option<&RequestRotateDecorator> {
        match &self.interaction {
            Some(DecoratorAffordanceInteraction::Rotate(request)) if self.gated(3) => Some(request),
            _ => None,
        }
    }

    /// 原128位enum判别检查。
    pub fn gated(&self, state: u64) -> bool {
        self.state == state && self.payload == 0
    }

}

/// 装饰交互 widget 名（O：0x142a835e6，31 字符；"ui_decide_decorator_interaction"）。
pub const DECIDE_DECORATOR_INTERACTION_WIDGET: &str = "ui_decide_decorator_interaction";

/// B1407af340原十四组参数；Raycasting借现有同World的world/terrain，不另存碰撞世界。
#[allow(clippy::too_many_arguments)]
pub fn ui_dispatch_decorator_affordance(
    mode: Res<AppMode>,
    raycasting: Res<RaycastWorld>,
    ray: Res<CameraCursorRay>,
    mut dispatcher: ResMut<DecoratorAffordanceDispatcher>,
    ui: Res<UiState>,
    cursor: Res<CursorTerrainRaycast>,
    _signifiers: Res<UiSignifierStream>,
    sources: (Res<DecoratorStorage>, Res<DecoratorMeshLibrary>,
        Res<TerrainHeightsData<TerrainHeightSsbo>>, Res<PublicWalls>,
        Res<StairsState>, Res<CachedDecoratorTransforms>),
    esc: Res<EscMenuState>,
    previous: Res<crate::blueprint::PrevDecoratorBlueprints>,
    modifiers: Res<Modifiers>,
    mut requests: (EventReader<StartPlaceDecoClutterInteraction>,
        EventReader<RequestMoveClutter>, EventReader<RequestRotateDecorator>,
        EventWriter<NotifyHintSystemCanContinueStairs>),
    cloned: Res<LastClonedClutter>,
    stairs: Res<StairsState>,
) {
    if esc.is_open() { return; }
    let native_mode = mode.native_tag();
    if !matches!(native_mode, 0 | 3 | 4 | 11 | 20) {
        dispatcher.clear();
        return;
    }
    ui.add(DECIDE_DECORATOR_INTERACTION_WIDGET,
        WidgetParams::new(WidgetVolumeShape { tag: 5,
            size: glam::Vec2::new(f32::MAX, 0.0), ..Default::default() }, [0; 2])
            .sounds(UiSounds::empty()).occlude_by_world(false));
    // 原先消费完整StartPlace流，取跨两buffer的最后一个；优先于旧拖动和其它请求。
    if let Some(request) = requests.0.read().last() {
        dispatcher.set_place_clutter(*request);
        return;
    }
    if refresh_dispatch_interaction(&mut dispatcher, &ui, &raycasting,
        &sources.2, &ray, &cursor, native_mode == 20) { return; }
    if native_mode == 3 {
        if let Some(request) = requests.1.read().last() { dispatcher.set_move(*request); }
        else if let Some(request) = requests.2.read().last() { dispatcher.set_rotate(*request); }
        else { dispatcher.clear(); }
        return;
    }
    if native_mode == 4 {
        if let Some(request) = requests.2.read().last() {
            dispatcher.set_rotate(*request);
            return;
        }
    }
    let mask = if dispatcher.decorator_ty().is_some_and(|ty| ty.0 == 63) { 0x23 } else { 0x30ab };
    if native_mode == 20 {
        let hit = raycasting.raycast_w_terrain(&sources.2, ray.origin, ray.dir, mask | 0x100);
        let hit = if hit.is_miss() { cursor_fallback_hit(&cursor) } else { hit };
        let ty = country_core::systems::mode_manager::ClutterMeshTy::from_native_tag(cloned.tag)
            .expect("original LastClonedClutter type");
        let widget = create_decorator_widget(&ui, "place_decorator",
            dispatch_distance(ray.origin, hit.position) - 0.01, false, UiSounds::empty());
        dispatcher.set_place_clutter(StartPlaceDecoClutterInteraction {
            relative_rotation: cloned.rotation, widget, hit, ty,
        });
        return;
    }
    let mask = discovery_mask(mask, *mode);
    discover_dispatch_interaction(&mut dispatcher, *mode, &ui, &raycasting, &ray,
        &cursor, &sources.0, &sources.1, &sources.2, &sources.3, &stairs,
        &sources.5, &previous, *modifiers, &mut requests.3, mask);
}

fn dispatch_distance(origin: glam::Vec3, position: glam::Vec3) -> f32 {
    let delta = origin - position;
    (delta.z * delta.z + (delta.y * delta.y + delta.x * delta.x)).sqrt()
}

fn discovery_mask(mut mask: u32, mode: AppMode) -> u32 {
    let stair = matches!(mode, AppMode::Decorator(country_core::systems::mode_manager::DecoratorType::Stairs));
    if mode.native_tag() == 0 || stair { mask |= 0x4000; }
    if mode.native_tag() != 4 && !stair { mask |= 0x100; }
    if mode.native_tag() == 4 { 0x5000 } else { mask }
}

fn stair_height_area(hit_y: f32, base: f32, extra: f32, terrain: bool) -> bool {
    let height = base + extra + 0.8;
    // 原UCOMISS/JBE遇NaN走false，不能改成 !(hit_y <= height)。
    hit_y > if terrain { height + 0.1 } else { height }
}

fn chimney_height_bounds(words: &[u32]) -> utils::geometry::rectangle::Rectangle3d {
    let value = |i| f32::from_bits(words[i]);
    utils::geometry::rectangle::Rectangle3d::new(
        glam::Vec3::new(value(2) + 0.1, (value(5) - 0.6) + 0.1, value(3) + 0.1),
        glam::Vec3::new(1.35, 1.2, 1.35), glam::Vec2::new(value(6), value(7)))
}

fn cursor_fallback_hit(cursor: &CursorTerrainRaycast) -> RaycastWorldResult {
    RaycastWorldResult { handle: [11, 0, 0], position: cursor.position(),
        normal: glam::Vec3::Y, toi: 0.0, discriminant: 0, extra: [0.0; 2] }
}

fn refresh_dispatch_interaction(
    dispatcher: &mut DecoratorAffordanceDispatcher, ui: &UiState,
    world: &RaycastWorld, terrain: &TerrainHeightsData<TerrainHeightSsbo>,
    ray: &CameraCursorRay, cursor: &CursorTerrainRaycast, cloning: bool,
) -> bool {
    if dispatcher.payload != 0 { return false; }
    let Some(active) = &mut dispatcher.interaction else { return false; };
    let (widget, hit, ty) = match active {
        DecoratorAffordanceInteraction::PlaceDecorator(place) => (place.widget, &mut place.hit, place.ty),
        DecoratorAffordanceInteraction::PlaceClutter(place) => (place.request.widget, &mut place.request.hit, DecoratorType(place.request.ty.native_tag())),
        DecoratorAffordanceInteraction::Move(movement) => (movement.request.widget, &mut movement.request.hit, movement.request.ty),
        DecoratorAffordanceInteraction::Rotate(request) => (request.widget, &mut request.hit, request.ty),
        DecoratorAffordanceInteraction::ChangeHeight(target)
        | DecoratorAffordanceInteraction::ContinueStair(target) => (target.widget, &mut target.hit, target.ty),
    };
    if ui.widget_context(widget).dragged() == 2 && !cloning { return false; }
    let found = world.raycast_w_terrain(terrain, ray.origin, ray.dir, if ty.0 == 63 { 0x23 } else { 0x30ab });
    *hit = if found.is_miss() { cursor_fallback_hit(cursor) } else { found };
    if ty.0 == 63 && hit.handle[0] as u8 == 2 && hit.handle[0] >> 8 & 1 == 0 {
        hit.handle[0] = (hit.handle[0] & !0xff) | 11;
    }
    ui.add_with_id(widget, decorator_widget_params(dispatch_distance(ray.origin, hit.position) - 0.05,
        false, UiSounds::empty()));
    true
}

fn dispatch_hover_sound() -> UiSounds {
    let mut sounds = UiSounds::empty();
    sounds.slots[0][..8].copy_from_slice(&7u64.to_le_bytes());
    sounds
}

#[allow(clippy::too_many_arguments)]
fn discover_dispatch_interaction(
    dispatcher: &mut DecoratorAffordanceDispatcher, mode: AppMode, ui: &UiState,
    world: &RaycastWorld, ray: &CameraCursorRay, cursor: &CursorTerrainRaycast,
    storage: &DecoratorStorage, meshes: &DecoratorMeshLibrary,
    terrain: &TerrainHeightsData<TerrainHeightSsbo>, walls: &PublicWalls,
    stairs: &StairsState, transforms: &CachedDecoratorTransforms,
    previous: &crate::blueprint::PrevDecoratorBlueprints, modifiers: Modifiers,
    notify: &mut EventWriter<NotifyHintSystemCanContinueStairs>, mask: u32,
) {
    use country_core::{resources::walls::decorator::DecoratorDst,
        systems::mode_manager::{DecoratorType as ToolDecoratorType, EditSubject}};
    let selected = if let AppMode::Decorator(ty) = mode { Some(DecoratorType(ty.native_tag())) } else { None };
    let editing_stair = if let AppMode::Editing(EditSubject::StairPoint { decorator_id }) = mode { Some(decorator_id) } else { None };
    let connected = editing_stair.map(|id| stairs.get_all_connected_nodes(id));
    let make_place = |hit: RaycastWorldResult, ty: DecoratorType| {
        let widget = create_decorator_widget(ui, "place_decorator", dispatch_distance(ray.origin, hit.position), ty.0 == 63, UiSounds::empty());
        PlaceDecoratorAffordance { widget, hit, ty }
    };
    for mut hit in world.raycast_intersections_with_ray(terrain, ray.origin, ray.dir, mask) {
        if hit.is_miss() { break; }
        match hit.handle[0] as u8 {
            0 | 2 | 11 | 13 => {
                if hit.handle[0] as u8 == 13 {
                    if let Some(id) = editing_stair {
                        if !petgraph::algo::has_path_connecting(stairs.graph(), id, (hit.handle[0] >> 32) as u32, None) { continue; }
                    }
                }
                if let Some(ty) = selected {
                    if ty.0 == 63 && hit.handle[0] as u8 == 2 && hit.handle[0] >> 8 & 1 == 0 {
                        hit.handle[0] = (hit.handle[0] & !0xff) | 11;
                    }
                    dispatcher.set_place_decorator(make_place(hit, ty));
                } else { dispatcher.clear(); }
                return;
            }
            6 | 7 | 8 | 10 | 14 => {
                if let Some(ty) = selected {
                    hit.position = cursor.position();
                    hit.normal = glam::Vec3::Y;
                    hit.discriminant = 0;
                    dispatcher.set_place_decorator(make_place(hit, ty));
                } else { dispatcher.clear(); }
                return;
            }
            4 | 5 => {}
            _ => panic!("original decorator affordance hit variant"),
        }
        let id = DecoratorId((hit.handle[0] >> 32) as u32);
        if let Some(connected) = &connected {
            if !connected.contains(&id.0) { continue; }
            if stairs.graph().neighbors(id.0).count() <= 1 { notify.write(NotifyHintSystemCanContinueStairs); }
        }
        let placing_stair = matches!(mode, AppMode::Decorator(ToolDecoratorType::Stairs));
        if placing_stair {
            let info = storage.decorator_info(id).expect("original stair placement hit has no decorator");
            if info.ty != 63 {
                hit.position = cursor.position(); hit.normal = glam::Vec3::Y; hit.discriminant = 0;
                dispatcher.set_place_decorator(make_place(hit, DecoratorType(63)));
                return;
            }
        }
        let Some(info) = storage.decorator_info(id) else { dispatcher.clear(); return; };
        let ty = DecoratorType(info.ty);
        let is_clutter = info.ty.wrapping_sub(56) == 6 || info.ty.wrapping_sub(56) > 7;
        let terrain_lantern_or_flag = info.ty & 0x3e == 0x3c && hit.handle[1] as u32 == 1;
        let mut rotation_area = false;
        let mut height_area = false;
        if terrain_lantern_or_flag {
            let DecoratorDst::Terrain(dst) = info.destination() else { panic!("original lantern/flag terrain destination"); };
            let mut bounds = meshes.collision_record(DecoratorLibKey::LANTERN_TERRAIN_ROTATION).bounds;
            bounds.rotate_projective_xz(dst.yaw.to_projective_xz_mat2());
            bounds.translate(glam::Vec3::new(dst.position_xz.x,
                terrain.sample_at_world_xz(dst.position_xz.x, dst.position_xz.y), dst.position_xz.y));
            rotation_area = bounds.contains_point(hit.position);
        } else if info.ty == 63 {
            let endpoint = stairs.endpoint();
            if !endpoint.is_unset() && endpoint.node() != id.0
                && stairs.graph().neighbors(id.0).count() <= 1 && !modifiers.disable_snapping()
                && transforms.try_transform(id.0).is_some_and(|t| hit.position.y + 1.0 > t.translation.y)
            {
                let widget = create_decorator_widget_handle(ui, DecoratorHandleName(u64::from(id.0), 255),
                    dispatch_distance(ray.origin, hit.position), true, dispatch_hover_sound());
                dispatcher.set_target(DecoratorTargetAffordance { widget, hit, decorator_id: id, ty }, true);
                return;
            }
            if placing_stair { continue; }
            let height = || {
                let extra = info.extra().as_stair();
                f32::from_le_bytes(extra.0[..4].try_into().unwrap())
            };
            height_area = match info.destination() {
                DecoratorDst::Roof(_) => stair_height_area(hit.position.y,
                    walls.get(storage.lookup_storage(id).expect("original stair address").as_wall())
                        .expect("original stair roof wall").flat_roof_y(), height(), false),
                DecoratorDst::Terrain(dst) => stair_height_area(hit.position.y,
                    terrain.sample_at_world_xz(dst.position_xz.x, dst.position_xz.y), height(), true),
                DecoratorDst::Stair(_) => panic!("original nested stair destination"),
                _ => false,
            };
        } else if info.ty == 59 {
            if let Some(params) = previous.map.get(&id.0).and_then(|bp| bp.chimney_assembly.as_deref()) {
                height_area = chimney_height_bounds(&params.words).contains_point(hit.position);
            }
        }
        let widget = create_decorator_widget_handle(ui, DecoratorHandleName(u64::from(id.0), 255),
            dispatch_distance(ray.origin, hit.position), ty.0 == 63, dispatch_hover_sound());
        if rotation_area || ((is_clutter || terrain_lantern_or_flag) && modifiers.disable_snapping()) {
            dispatcher.set_rotate(RequestRotateDecorator { widget, hit, decorator_id: id,
                amount: if is_clutter { 0.0 } else { 3.0 }, ty });
        } else if height_area {
            dispatcher.set_target(DecoratorTargetAffordance { widget, hit, decorator_id: id, ty }, false);
        } else {
            dispatcher.set_move(RequestMoveClutter { widget, hit, decorator_id: id, ty });
            if let Some(DecoratorAffordanceInteraction::Move(movement)) = &mut dispatcher.interaction {
                movement.native_flag_0x5d = ty.0 != 63;
            }
        }
        return;
    }
    dispatcher.clear();
}

/// 兼容已有独立旋转差分测试的窄入口。生产仅注册上面的完整十四组派发系统，
/// 不能同时注册本入口，否则两个独立reader会重复消费同一个交互请求。
#[allow(clippy::too_many_arguments)]
pub fn ui_dispatch_focused_decorator_requests(
    mode: Res<country_core::systems::mode_manager::AppMode>, ui: Res<UiState>,
    esc: Res<EscMenuState>,
    ray: Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,
    cursor: Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,
    raycasting: Res<country_core::systems::collision::world::RaycastWorld>,
    terrain: Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo>>,
    mut moves: EventReader<RequestMoveClutter>, mut rotations: EventReader<RequestRotateDecorator>,
    mut dispatcher: ResMut<DecoratorAffordanceDispatcher>,
) {
    use country_core::systems::collision::world::RaycastWorldResult;
    if esc.is_open() || mode.native_tag()!=3 {return;}
    let global=WidgetParams::new(WidgetVolumeShape {tag:5,size:glam::Vec2::new(f32::MAX,0.0),..Default::default()},[0;2])
        .sounds(UiSounds::empty()).occlude_by_world(false);
    ui.add(DECIDE_DECORATOR_INTERACTION_WIDGET,global);
    if let Some(active)=&mut dispatcher.interaction {
        let (widget,hit,ty)=match active {
            DecoratorAffordanceInteraction::PlaceDecorator(place)=>(place.widget,&mut place.hit,place.ty),
            DecoratorAffordanceInteraction::ChangeHeight(target)
            | DecoratorAffordanceInteraction::ContinueStair(target)=>(target.widget,&mut target.hit,target.ty),
            DecoratorAffordanceInteraction::PlaceClutter(place)=>(place.request.widget,&mut place.request.hit,country_core::resources::walls::decorator::DecoratorType(place.request.ty.native_tag())),
            DecoratorAffordanceInteraction::Move(movement)=>(movement.request.widget,&mut movement.request.hit,movement.request.ty),
            DecoratorAffordanceInteraction::Rotate(request)=>(request.widget,&mut request.hit,request.ty),
        };
        if ui.widget_context(widget).dragged()!=2 {
            let mask=if ty.0==b'?' {0x23} else {0x30ab};
            let found=raycasting.raycast_w_terrain(&terrain,ray.origin,ray.dir,mask);
            *hit=if found.is_miss() {
                RaycastWorldResult{handle:[11,0,0],position:cursor.position(),normal:glam::Vec3::Y,toi:0.0,discriminant:0,extra:[0.0;2]}
            } else {found};
            if ty.0==b'?' && hit.handle[0] as u8==2 && (hit.handle[0]>>8)&1==0 {
                hit.handle[0]=(hit.handle[0]&!0xff)|11;
            }
            let distance=dispatch_distance(ray.origin,hit.position)-0.05;
            ui.add_with_id(widget,decorator_widget_params(distance,false,UiSounds::empty()));
            return;
        }
    }
    if let Some(request)=moves.read().last() {dispatcher.set_move(*request);}
    else if let Some(request)=rotations.read().last() {dispatcher.set_rotate(*request);}
    else {dispatcher.clear();}
}

/// B1412823c0/187B：第四参数XMM3 f32写volume+4距离；保留构造器遮挡字节
/// 和flag bit0，添加bit1，类型'?'时再添加bit3。
pub fn create_decorator_widget(
    ui: &UiState,
    name: &str,
    distance: f32,
    is_question_mark: bool,
    sounds: UiSounds,
) -> WidgetId {
    // 原 141282463 调 UiWidgetContext::id@1406868b0，返回真实注册键。
    ui.add(name, decorator_widget_params(distance, is_question_mark, sounds)).id
}

/// B141282300/187B：相同参数构造，复用UiState::add<DecoratorHandleName>真实哈希。
pub fn create_decorator_widget_handle(
    ui: &UiState,
    name: DecoratorHandleName,
    distance: f32,
    is_question_mark: bool,
    sounds: UiSounds,
) -> WidgetId {
    ui.add_with_id(name.to_widget_id(), decorator_widget_params(distance, is_question_mark, sounds)).id
}

fn decorator_widget_params(distance: f32, is_question_mark: bool, sounds: UiSounds) -> WidgetParams {
    let mut params = WidgetParams::new(WidgetVolumeShape {
        tag: 5,
        size: glam::Vec2::new(distance, 0.0),
        ..Default::default()
    }, [0; 2]).sounds(sounds);
    params.flags.0 |= 2 | (u8::from(is_question_mark) << 3);
    params
}

#[cfg(test)]
mod widget_tests {
    use super::*;

    #[test]
    fn both_real_registry_entries_keep_native_distance_flags_and_sounds() {
        for distance in [0.0f32, -0.0, 1.0, -4.5, 125.0, f32::INFINITY] {
            for question in [false, true] {
                let ui = UiState::at_startup();
                let mut sounds = UiSounds::empty();
                for (i, byte) in sounds.slots.iter_mut().flatten().enumerate() {
                    *byte = (i as u8).wrapping_mul(7).wrapping_add(3);
                }
                let name = DecoratorHandleName(0x123456789abcdef0, 7);
                create_decorator_widget(&ui, "decorator", distance, question, sounds);
                create_decorator_widget_handle(&ui, name, distance, question, sounds);
                for id in [WidgetId::from_name("decorator"), name.to_widget_id()] {
                    let params = ui.widget(id).unwrap();
                    assert_eq!(params.volume.tag, 5);
                    assert_eq!(params.volume.size.x.to_bits(), distance.to_bits());
                    assert_eq!(params.flags.0, if question { 11 } else { 3 });
                    assert!(params.occlude_by_world);
                    assert_eq!(params.sounds, sounds);
                }
            }
        }
    }
}

#[cfg(test)]
mod native_validation_type_projection_tests {
    use super::*;
    use country_core::{
        resources::walls::decorator::{DecoratorId, DecoratorType},
        systems::collision::world::RaycastWorldResult,
    };

    #[test]
    fn only_exact_clear_variant_omits_type_and_does_not_reuse_stale_request() {
        let mut dispatcher = DecoratorAffordanceDispatcher {
            state: 6,
            payload: 0,
            ty: 56,
            interaction: None,
        };
        assert_eq!(dispatcher.decorator_ty(), None);
        for (state, payload) in [
            (0, 0),
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (5, 0),
            (6, 1),
            (11, 63),
            (20, 0),
        ] {
            dispatcher.state = state;
            dispatcher.payload = payload;
            assert_eq!(
                dispatcher.decorator_ty(),
                Some(DecoratorType(56)),
                "state={state},payload={payload}"
            );
        }
        dispatcher.clear();
        assert_eq!(dispatcher.decorator_ty(), None);
    }

    #[test]
    fn active_owned_move_and_rotate_use_their_actual_typed_payload() {
        let mut dispatcher = DecoratorAffordanceDispatcher::default();
        dispatcher.set_move(RequestMoveClutter {
            widget: WidgetId::from_name("真实Move请求"),
            hit: RaycastWorldResult::miss(),
            decorator_id: DecoratorId(9),
            ty: DecoratorType(63),
        });
        dispatcher.ty = 56;
        assert_eq!(dispatcher.decorator_ty(), Some(DecoratorType(63)));
        dispatcher.set_rotate(RequestRotateDecorator {
            widget: WidgetId::from_name("真实Rotate请求"),
            hit: RaycastWorldResult::miss(),
            decorator_id: DecoratorId(10),
            amount: 0.25,
            ty: DecoratorType(58),
        });
        dispatcher.ty = 56;
        assert_eq!(dispatcher.decorator_ty(), Some(DecoratorType(58)));
        dispatcher.clear();
        assert_eq!(dispatcher.decorator_ty(), None);
    }
}

#[cfg(test)]
mod dispatcher_chain_tests {
    use super::*;
    use country_core::systems::{mode_manager::{EditSubject, DecoratorType as ToolDecoratorType},
        ui::resources::ui_state::UiInteraction};
    use glam::{Quat, Vec3};

    fn world(mode: AppMode, esc: EscMenuState) -> (World, Schedule) {
        let mut world = World::new();
        world.insert_resource(mode);
        world.insert_resource(esc);
        world.insert_resource(RaycastWorld::new());
        world.insert_resource(CameraCursorRay { origin: Vec3::new(3.0, 9.0, -4.0), dir: -Vec3::Y });
        world.init_resource::<DecoratorAffordanceDispatcher>();
        world.insert_resource(UiState::at_startup());
        world.init_resource::<CursorTerrainRaycast>();
        world.init_resource::<UiSignifierStream>();
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<DecoratorMeshLibrary>();
        world.insert_resource(TerrainHeightsData::<TerrainHeightSsbo>::new());
        world.init_resource::<PublicWalls>();
        world.init_resource::<StairsState>();
        world.init_resource::<CachedDecoratorTransforms>();
        world.init_resource::<crate::blueprint::PrevDecoratorBlueprints>();
        world.init_resource::<Modifiers>();
        world.init_resource::<LastClonedClutter>();
        world.init_resource::<crate::deferred_decorator_history::DeferredDecoratorEditHistory>();
        world.init_resource::<Events<StartPlaceDecoClutterInteraction>>();
        world.init_resource::<Events<RequestMoveClutter>>();
        world.init_resource::<Events<RequestRotateDecorator>>();
        world.init_resource::<Events<NotifyHintSystemCanContinueStairs>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(ui_dispatch_decorator_affordance);
        (world, schedule)
    }
    fn hex(s: &str) -> Vec<u8> {
        s.as_bytes().chunks_exact(2).map(|v| u8::from_str_radix(std::str::from_utf8(v).unwrap(), 16).unwrap()).collect()
    }
    fn u32_at(raw: &[u8], at: usize) -> u32 { u32::from_le_bytes(raw[at..at+4].try_into().unwrap()) }
    fn u64_at(raw: &[u8], at: usize) -> u64 { u64::from_le_bytes(raw[at..at+8].try_into().unwrap()) }
    fn native_request(raw: &[u8]) -> StartPlaceDecoClutterInteraction {
        let f = |at| f32::from_bits(u32_at(raw, at));
        StartPlaceDecoClutterInteraction {
            relative_rotation: Quat::from_xyzw(f(0), f(4), f(8), f(12)),
            widget: WidgetId(u64_at(raw, 16)),
            hit: RaycastWorldResult { handle: [u64_at(raw,24),u64_at(raw,32),u64_at(raw,40)],
                position: Vec3::new(f(48),f(52),f(56)), normal: Vec3::new(f(60),f(64),f(68)),
                // 原语义命中的+30为判别，+34/+38为UV；宿主toi不是这个ABI中的活跃字段。
                toi: 0.0, discriminant: u32_at(raw,72), extra: [f(76),f(80)] },
            ty: country_core::systems::mode_manager::ClutterMeshTy::from_native_tag(raw[88]).unwrap(),
        }
    }
    fn assert_request_bits(place: &PlaceClutterAffordance, native: &[u8]) {
        let raw = &native[..16];
        assert_eq!(place.request.relative_rotation.to_array().map(f32::to_bits),
            std::array::from_fn(|i| u32_at(raw,i*4)));
        let hit = &native[16..76];
        assert_eq!(place.request.hit.handle, [u64_at(hit,0),u64_at(hit,8),u64_at(hit,16)]);
        assert_eq!(place.request.hit.position.to_array().map(f32::to_bits), std::array::from_fn(|i|u32_at(hit,24+i*4)));
        assert_eq!(place.request.hit.normal.to_array().map(f32::to_bits), std::array::from_fn(|i|u32_at(hit,36+i*4)));
        assert_eq!(place.request.hit.discriminant,u32_at(hit,48));
        assert_eq!(place.request.hit.extra.map(f32::to_bits),[u32_at(hit,52),u32_at(hit,56)]);
        assert_eq!(place.request.widget,WidgetId(u64_at(native,80)));
        assert_eq!(place.native_flag_0x68,u32::from(native[88])!=0);
        assert_eq!(place.request.ty.native_tag(),native[89]);
    }
    fn mode(tag: u32) -> Option<AppMode> {
        Some(match tag {
            0 => AppMode::Editing(EditSubject::None),
            1 => AppMode::Editing(EditSubject::Wall(1)),
            2 => AppMode::Editing(EditSubject::Roof(1)),
            3 => AppMode::Editing(EditSubject::DecoClutter { decorator_id: 1 }),
            4 => AppMode::Editing(EditSubject::StairPoint { decorator_id: 1 }),
            5 => AppMode::Editing(EditSubject::Terrain(1)),
            6 => AppMode::Wall(country_core::systems::mode_manager::WallShape::Circle),
            10 => AppMode::Terrain,
            11 => AppMode::Decorator(ToolDecoratorType::Stairs),
            12 => AppMode::Color,
            19 => AppMode::Photomode,
            20 => AppMode::CloneClutter,
            // 原非法raw tag不可能由安全AppMode构造；原向量保留，不能假造Rust变体。
            _ => return None,
        })
    }
    #[test]
    fn actual_fourteen_parameter_world_matches_original_start_place_dispatch_vectors() {
        let rows = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m01-dispatch-start-place-native.tsv"));
        let mut checked = 0;
        for line in rows.lines().filter(|line| !line.starts_with('#') && !line.is_empty()) {
            let fields: Vec<_> = line.split('\t').collect();
            let Some(mode) = mode(fields[0].parse().unwrap()) else { continue; };
            let esc = fields[1] == "1";
            let a: usize = fields[2].parse().unwrap();
            let before: usize = fields[4].parse().unwrap();
            let payloads: Vec<_> = fields[6].split(';').map(hex).collect();
            let (mut world, mut schedule) = world(mode, if esc { EscMenuState::Main } else { EscMenuState::Closed });
            for bytes in &payloads[..before] { world.send_event(native_request(bytes)); }
            if before != 0 { schedule.run(&mut world); }
            for bytes in &payloads[before..a.max(before)] { world.send_event(native_request(bytes)); }
            world.resource_mut::<Events<StartPlaceDecoClutterInteraction>>().update();
            for bytes in &payloads[a.max(before)..] { world.send_event(native_request(bytes)); }
            schedule.run(&mut world);
            let dispatcher = world.resource::<DecoratorAffordanceDispatcher>();
            assert_eq!(dispatcher.state, fields[7].parse::<u64>().unwrap(), "{line}");
            assert_eq!(dispatcher.payload, fields[8].parse::<u64>().unwrap(), "{line}");
            if fields[9] != "-" {
                assert_request_bits(dispatcher.place_clutter().unwrap(), &hex(fields[9]));
                let global = world.resource::<UiState>().widget(WidgetId::from_name(DECIDE_DECORATOR_INTERACTION_WIDGET)).unwrap();
                assert_eq!(global.volume.tag, 5);
                assert_eq!(global.volume.size.x.to_bits(), f32::MAX.to_bits());
                assert!(!global.occlude_by_world);
                assert_eq!(global.sounds,UiSounds::empty());
            } else {
                assert!(dispatcher.interaction.is_none());
                assert!(world.resource::<UiState>().widget(WidgetId::from_name(DECIDE_DECORATOR_INTERACTION_WIDGET)).is_none());
            }
            assert!(world.resource::<crate::deferred_decorator_history::DeferredDecoratorEditHistory>().records.is_empty(), "派发本身不能提交世界历史");
            checked += 1;
        }
        assert_eq!(checked,291);
    }

    #[test]
    fn pending_events_survive_menu_and_mode_gates_and_are_not_replayed() {
        let request = native_request(&hex("0000803e0000003f000040bf0000603f78563412000000000b00000000000000000000000000000000000000000000000000803f0000004000004040000000000000803f00000000000000000000000000000000000000001b00000000000000"));
        let (mut world, mut schedule) = world(AppMode::Terrain,EscMenuState::Main);
        world.send_event(request);
        schedule.run(&mut world);
        assert!(world.resource::<DecoratorAffordanceDispatcher>().place_clutter().is_none());
        *world.resource_mut::<EscMenuState>() = EscMenuState::Closed;
        schedule.run(&mut world);
        assert!(world.resource::<DecoratorAffordanceDispatcher>().place_clutter().is_none());
        *world.resource_mut::<AppMode>() = AppMode::Editing(EditSubject::None);
        schedule.run(&mut world);
        let placed = world.resource::<DecoratorAffordanceDispatcher>().place_clutter().unwrap();
        assert_eq!(placed.request.widget,request.widget);
        schedule.run(&mut world);
        assert_eq!(world.resource::<DecoratorAffordanceDispatcher>().state,6);
        assert!(world.resource::<DecoratorAffordanceDispatcher>().interaction.is_none());
    }

    #[test]
    fn start_place_preempts_active_drag_and_retains_lower_priority_request_cursors() {
        let mut raw = [0;96]; raw[88]=27; raw[16..24].copy_from_slice(&77u64.to_le_bytes());
        raw[24]=11; raw[12..16].copy_from_slice(&1f32.to_le_bytes());
        let request=native_request(&raw);
        let (mut world, mut schedule)=world(AppMode::Editing(EditSubject::DecoClutter{decorator_id:9}),EscMenuState::Closed);
        world.resource_mut::<DecoratorAffordanceDispatcher>().set_rotate(RequestRotateDecorator{widget:WidgetId(8),hit:request.hit,decorator_id:DecoratorId(8),amount:1.0,ty:DecoratorType(27)});
        world.insert_resource(UiState::new(UiInteraction{native_tag:3,native_phase:3,widget:WidgetId(8)}));
        world.send_event(RequestMoveClutter{widget:WidgetId(9),hit:request.hit,decorator_id:DecoratorId(9),ty:DecoratorType(27)});
        world.send_event(RequestRotateDecorator{widget:WidgetId(10),hit:request.hit,decorator_id:DecoratorId(10),amount:0.25,ty:DecoratorType(27)});
        world.send_event(request);
        schedule.run(&mut world);
        assert_eq!(world.resource::<DecoratorAffordanceDispatcher>().place_clutter().unwrap().request.widget,WidgetId(77));
        world.insert_resource(UiState::at_startup());
        schedule.run(&mut world);
        assert_eq!(world.resource::<DecoratorAffordanceDispatcher>().movement().unwrap().request.decorator_id,DecoratorId(9));
        assert!(!world.resource::<DecoratorAffordanceDispatcher>().movement().unwrap().native_flag_0x5d);
        schedule.run(&mut world);
        assert_eq!(world.resource::<DecoratorAffordanceDispatcher>().rotation().unwrap().decorator_id,DecoratorId(10));
        schedule.run(&mut world);
        assert!(world.resource::<DecoratorAffordanceDispatcher>().interaction.is_none());
    }

    #[test]
    fn original_native_discovery_masks_match_all_eighty_four_inputs() {
        let rows=include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m01-dispatch-mask-native.tsv"));
        let mut count=0;
        for line in rows.lines().filter(|s| !s.starts_with('#')) {
            let f:Vec<u32>=line.split('\t').map(|s|s.parse().unwrap()).collect();
            let mode=if f[0]==11 { AppMode::Decorator(ToolDecoratorType::from_native_tag(f[1]as u8).unwrap()) }
                else { mode(f[0]).unwrap() };
            let dispatcher=DecoratorAffordanceDispatcher{state:u64::from(f[2]),ty:f[3]as u8,..Default::default()};
            let base=if dispatcher.decorator_ty()==Some(DecoratorType(63)){0x23}else{0x30ab};
            assert_eq!(discovery_mask(base,mode),f[4],"{line}");
            assert_eq!(f[5],f32::MAX.to_bits());count+=1;
        }
        assert_eq!(count,84);
    }

    #[test]
    fn original_instruction_geometry_distance_and_stair_nan_boundaries_match() {
        let rows=include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m01-dispatch-geometry-native.tsv"));
        let mut count=[0;3];
        for line in rows.lines().filter(|s|!s.starts_with('#')) {
            let f:Vec<_>=line.split('\t').collect();
            let scalar=|s|f32::from_bits(u32_at(&hex(s),0));
            match f[0] {
                "C"=>{
                    let raw=hex(f[1]);let words:Vec<_>=raw.chunks_exact(4).map(|v|u32::from_le_bytes(v.try_into().unwrap())).collect();
                    let point=hex(f[2]);let point=Vec3::from_array(std::array::from_fn(|i|f32::from_bits(u32_at(&point,i*4))));
                    let bounds=chimney_height_bounds(&words);let expected=hex(f[3]);
                    let actual=[bounds.footprint.axis.x,bounds.footprint.axis.y,bounds.footprint.center.x,bounds.footprint.center.y,
                        bounds.footprint.size.x,bounds.footprint.size.y,bounds.bottom,bounds.top].map(f32::to_bits);
                    assert_eq!(actual,std::array::from_fn(|i|u32_at(&expected,i*4)),"{line}");
                    assert_eq!(bounds.contains_point(point),f[4]=="1","{line}");count[0]+=1;
                }
                "D"=>{
                    let raw=hex(f[1]);let delta=Vec3::from_array(std::array::from_fn(|i|f32::from_bits(u32_at(&raw,i*4))));
                    assert_eq!(dispatch_distance(delta,Vec3::ZERO).to_bits(),scalar(f[2]).to_bits(),"{line}");count[1]+=1;
                }
                "S"=>{
                    assert_eq!(stair_height_area(scalar(f[4]),scalar(f[2]),scalar(f[3]),f[1]=="1"),f[5]=="1","{line}");count[2]+=1;
                }
                _=>panic!("unknown native row"),
            }
        }
        assert_eq!(count,[90,512,162]);
    }

    #[test]
    fn real_rapier_decorator_hover_moves_rotates_and_unloads_without_stale_request() {
        use country_core::{resources::walls::{decorator::DecoratorInfo,decorator_storage::DecoratorStorageAddr},
            systems::collision::world::{upsert_collider,remove_collider,RaycastGeoSource,RaycastMeshData}};
        let (mut world,mut schedule)=world(AppMode::Editing(EditSubject::None),EscMenuState::Closed);
        let id=DecoratorId(42);let addr=DecoratorStorageAddr::Terrain;
        {let mut storage=world.resource_mut::<DecoratorStorage>();
            storage.get_or_insert_default(addr).insert(id,DecoratorInfo{ty:27,..Default::default()});
            storage.decorator_to_storage.insert(id,addr);}
        let mesh=std::sync::Arc::new(RaycastMeshData::new_cuboid_at(Vec3::splat(2.0),Vec3::ZERO));
        let mut colliders=std::collections::HashMap::new();
        upsert_collider(&mut world.resource_mut::<RaycastWorld>(),&mut colliders,id,
            RaycastGeoSource::decorator_interaction(id,addr,0),mesh.clone(),Vec3::new(3.,2.,-4.),Quat::IDENTITY);
        world.resource_mut::<RaycastWorld>().flush_colliders();
        schedule.run(&mut world);
        let movement=*world.resource::<DecoratorAffordanceDispatcher>().movement().expect("真实slot8命中进入移动");
        assert_eq!(movement.request.decorator_id,id);
        assert_eq!(movement.request.hit.handle[0]as u8,4);
        assert_eq!(movement.request.hit.position,Vec3::new(3.,3.,-4.));
        assert!(movement.native_flag_0x5d);
        assert_eq!(movement.request.widget,DecoratorHandleName(u64::from(id.0),255).to_widget_id());
        world.resource_mut::<Modifiers>().flags=1;
        schedule.run(&mut world);
        let rotation=*world.resource::<DecoratorAffordanceDispatcher>().rotation().expect("modifier切换旋转");
        assert_eq!(rotation.decorator_id,id);assert_eq!(rotation.amount.to_bits(),0f32.to_bits());
        let widget=world.resource::<UiState>().widget(rotation.widget).unwrap();
        assert_eq!(widget.sounds,dispatch_hover_sound());
        assert_eq!(widget.volume.size.x.to_bits(),6f32.to_bits());
        remove_collider(&mut world.resource_mut::<RaycastWorld>(),&mut colliders,&id).unwrap();
        world.resource_mut::<RaycastWorld>().flush_colliders();
        world.resource_mut::<DecoratorStorage>().decorator_to_storage.remove(id);
        world.resource_mut::<DecoratorStorage>().get_mut(addr).unwrap().decorators.remove(&id.0);
        schedule.run(&mut world);
        assert_eq!(world.resource::<DecoratorAffordanceDispatcher>().state,6);
        assert!(world.resource::<DecoratorAffordanceDispatcher>().interaction.is_none());
        assert!(colliders.is_empty());
        assert_eq!(std::sync::Arc::strong_count(&mesh),1,"卸载释放真实registry网格所有权");
        assert!(world.resource::<crate::deferred_decorator_history::DeferredDecoratorEditHistory>().records.is_empty());
    }

    #[test]
    fn actual_terrain_clone_and_held_drag_keep_rotation_type_and_widget() {
        use country_core::systems::mode_manager::ClutterMeshTy;
        let (mut world,mut schedule)=world(AppMode::CloneClutter,EscMenuState::Closed);
        let rotation=Quat::from_xyzw(0.2,-0.4,0.8,0.7);
        *world.resource_mut::<LastClonedClutter>()=LastClonedClutter{rotation,tag:27};
        world.resource_mut::<CursorTerrainRaycast>().update(Some(Vec3::new(8.,0.,6.)));
        schedule.run(&mut world);
        let place=*world.resource::<DecoratorAffordanceDispatcher>().place_clutter().unwrap();
        assert_eq!(place.request.relative_rotation.to_array().map(f32::to_bits),rotation.to_array().map(f32::to_bits));
        assert_eq!(place.request.ty,ClutterMeshTy::from_native_tag(27).unwrap());
        assert_eq!(place.request.widget,WidgetId::from_name("place_decorator"));
        assert_eq!(place.request.hit.handle[0]as u8,11);
        assert!(place.native_flag_0x68);
        let params=world.resource::<UiState>().widget(place.request.widget).unwrap();
        assert_eq!(params.volume.size.x.to_bits(),(dispatch_distance(Vec3::new(3.,9.,-4.),place.request.hit.position)-0.01).to_bits());
        *world.resource_mut::<AppMode>()=AppMode::Editing(EditSubject::DecoClutter{decorator_id:42});
        world.insert_resource(UiState::new(UiInteraction{native_tag:3,native_phase:3,widget:place.request.widget}));
        world.resource_mut::<CameraCursorRay>().origin=Vec3::new(-1.,8.,3.);
        world.send_event(RequestMoveClutter{widget:WidgetId(123),hit:place.request.hit,decorator_id:DecoratorId(42),ty:DecoratorType(27)});
        schedule.run(&mut world);
        let held=*world.resource::<DecoratorAffordanceDispatcher>().place_clutter().unwrap();
        assert_eq!(held.request.widget,place.request.widget);
        assert_eq!(held.request.relative_rotation.to_array().map(f32::to_bits),rotation.to_array().map(f32::to_bits));
        assert_eq!(held.request.ty,place.request.ty);assert!(held.native_flag_0x68);
        let params=world.resource::<UiState>().widget(held.request.widget).unwrap();
        assert_eq!(params.volume.size.x.to_bits(),(dispatch_distance(Vec3::new(-1.,8.,3.),held.request.hit.position)-0.05).to_bits());
        world.insert_resource(UiState::at_startup());schedule.run(&mut world);
        assert_eq!(world.resource::<DecoratorAffordanceDispatcher>().movement().unwrap().request.widget,WidgetId(123));
    }

    #[test]
    fn real_stair_interaction_routes_height_continue_and_degree_rejection() {
        use country_core::{resources::{stairs::stairs_state::ObservedStairEndpoint,
            walls::{decorator::{DecoratorInfo,DecoratorDst,TerrainDecoratorDst,DecoratorRotationXZ,DecoratorExtraInfo,StairExtra},
                decorator_storage::DecoratorStorageAddr,cached_decorator_transforms::CachedDecoratorTransform}},
            systems::collision::world::{upsert_collider,RaycastGeoSource,RaycastMeshData}};
        let id=DecoratorId(42);let addr=DecoratorStorageAddr::Terrain;
        let (mut world,mut schedule)=world(AppMode::Editing(EditSubject::StairPoint{decorator_id:id.0}),EscMenuState::Closed);
        let mut info=DecoratorInfo{ty:63,..Default::default()};
        info.set_destination(DecoratorDst::Terrain(TerrainDecoratorDst{position_xz:glam::Vec2::new(3.,-4.),yaw:DecoratorRotationXZ(0.)}));
        info.set_extra(DecoratorExtraInfo::Stair(StairExtra::default()));
        {let mut storage=world.resource_mut::<DecoratorStorage>();storage.get_or_insert_default(addr).insert(id,info);storage.decorator_to_storage.insert(id,addr);}
        world.resource_mut::<StairsState>().add_node(id.0);
        let mut colliders=std::collections::HashMap::new();
        upsert_collider(&mut world.resource_mut::<RaycastWorld>(),&mut colliders,id,
            RaycastGeoSource::decorator_interaction(id,addr,1),
            std::sync::Arc::new(RaycastMeshData::new_cuboid_at(Vec3::splat(2.),Vec3::ZERO)),
            Vec3::new(3.,2.,-4.),Quat::IDENTITY);
        world.resource_mut::<RaycastWorld>().flush_colliders();
        schedule.run(&mut world);
        assert_eq!(world.resource::<DecoratorAffordanceDispatcher>().height_adjustment().unwrap().decorator_id,id);
        assert_eq!(world.resource::<Events<NotifyHintSystemCanContinueStairs>>().len(),1);
        let mut endpoint=[0;24];endpoint[16..20].copy_from_slice(&7u32.to_le_bytes());
        world.resource_mut::<StairsState>().add_node(7);
        world.resource_mut::<StairsState>().try_set_endpoint(ObservedStairEndpoint::from_observed(endpoint));
        let cached=|y|CachedDecoratorTransform{rotation:Quat::IDENTITY,translation:Vec3::new(3.,y,-4.),scale:Vec3::ONE};
        world.resource_mut::<CachedDecoratorTransforms>().insert(id.0,cached(4.));
        schedule.run(&mut world);
        assert!(world.resource::<DecoratorAffordanceDispatcher>().height_adjustment().is_some(),"严格hitY+1>cacheY，不把边界算可继续");
        world.resource_mut::<CachedDecoratorTransforms>().insert(id.0,cached(3.99));
        schedule.run(&mut world);
        assert_eq!(world.resource::<DecoratorAffordanceDispatcher>().continued_stair().unwrap().decorator_id,id);
        world.resource_mut::<Modifiers>().flags=1;schedule.run(&mut world);
        assert!(world.resource::<DecoratorAffordanceDispatcher>().height_adjustment().is_some());
        world.resource_mut::<Modifiers>().flags=0;
        world.resource_mut::<StairsState>().add_edge(id.0,8);
        world.resource_mut::<StairsState>().add_edge(id.0,9);
        schedule.run(&mut world);
        assert!(world.resource::<DecoratorAffordanceDispatcher>().height_adjustment().is_some(),"度数2拒绝继续但保留真实高度交互");
        world.resource_mut::<StairsState>().unset_endpoint();
        *world.resource_mut::<AppMode>()=AppMode::Decorator(ToolDecoratorType::Stairs);
        schedule.run(&mut world);
        assert!(world.resource::<DecoratorAffordanceDispatcher>().continued_stair().is_none());
        assert!(world.resource::<DecoratorAffordanceDispatcher>().height_adjustment().is_none(),"楼梯放置模式跳过不可继续的节点而非改高度");
    }
}
