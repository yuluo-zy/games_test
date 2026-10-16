//! B `ui_systems/ui_move_decorator.rs` — 装饰移动交互（0x1407a4850，O 结构；
//! 81KB fresh 反编译）。
//!
//! 主要观察（O）：`Local<CacheDecoclutterRotationOnGrab>`；`MoveDecoratorDeltaPositions`
//! （0x10B 记录 `{u64 pos, byte@8=1}` 队列，Vec grow 0x140562630 族）；
//! 相位机（`bVar10 = (dragged-2)<3 ? dragged-2 : 3`，case 0 悬停/1 拖中/2 释放/3 空闲）；
//! 拖中：`MainCamera::xz_projective_rotation`（0x140911e10）矩阵（_UNK_1429cd1xx）
//! → 拖动差量 + 0x10B 记录推入 + 类型字节检查（`(t-0x38)<8` 窗族 seed 变更
//! 0x140833150）+ 0x28B 移动 op（kind 0x21）+ `backup.created==0 → 复位 + 置
//! CREATE（+0x282=1）+ return`；`convert_raycast_hit_to_decorator_dst`
//! （0x14127c990）→ 目的 tag 跳表 UNK_142a829e8（4 路：wall/roof/terrain/stair
//! 处理器含 handle_moving_out_wall_decorator_part 0x141279410 /
//! handle_moving_out_roof_decorator_part 0x14127bab0 / show_decorator_icon
//! 0x1407adb20 / try_get_grab_offset_ss_if_valid 0x141277bb0 / calculate_move_dst
//! 0x1412801f0 / show_decorator_remove_signifier 0x14127fe10 / apply_decorator_op
//! 0x141276e30）；事件面：DeleteDecoratorCmd/ChangeDecoratorTypeCmd/
//! HandleStairAttachedDecoratorsOnStairPointDeletion（读）、RequestAppModeChange/
//! PlayStairsAudioCmd/EvInitPlaceDecoratorInteractionState/RequestRotateDecorator/
//! InstaCreateStairPoint/NotifyHint*（写，core 未落者登记）。
use bevy_ecs::prelude::*;

use crate::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher;

/// `Local<CacheDecoclutterRotationOnGrab>`（lib.rs 符号；抓取时缓存旋转，I）。
#[derive(Default)]
pub struct CacheDecoclutterRotationOnGrab {
    /// 抓取时缓存的 xz 投影旋转（I）。
    pub cached_yaw: f32,
}

/// `MoveDecoratorDeltaPositions`（lib.rs 符号；0x10B 记录 `{u64 pos, byte=1}`；
/// O 布局）。
#[derive(Resource, Clone, Debug, Default)]
pub struct MoveDecoratorDeltaPositions {
    pub entries: Vec<(u64, u8)>,
}

impl MoveDecoratorDeltaPositions {
    /// 0x10B 记录推入（O：grow 0x140562630 + `[+8]=1`）。
    pub fn push(&mut self, pos: u64) {
        self.entries.push((pos, 1));
    }
}

/// 射线资源（lib.rs 符号；SystemParam 胶水——init_state 0x1407a84d0 5 字段
/// 面：Raycasting（0x1408d99b0）+ 4 资源；derive 胶水不计业务）。
/// 射线资源（lib.rs 符号；SystemParam 胶水——init_state 0x1407a84d0 5 字段
/// 面：Raycasting（0x1408d99b0）+ 4 资源；derive 胶水不计业务。本实现以
/// 普通结构承载字段面；SystemParam derive 由宿主接线时启用）。
#[allow(dead_code)]
pub struct RaycastResources<'w> {
    /// 射线世界（core collision world；O：0x1408d99b0 SystemParam init）。
    pub raycast: &'w country_core::systems::collision::world::RaycastWorld,
    /// 地形高度（O：0x1412773f0 intersect_ray 消费）。
    pub terrain: &'w country_core::resources::terrain_height::TerrainHeightsData<
        country_core::resources::terrain_height::TerrainHeightSsbo,
    >,
}

/// B 0x1407a4850（O 结构）：移动主系统（结构面；相位/跳表细节登记）。
#[allow(clippy::too_many_arguments)]
pub fn ui_move_decorator(
    _ui: Res<country_core::systems::ui::resources::ui_state::UiState>,
    _stream: ResMut<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,
    dispatcher: Res<DecoratorAffordanceDispatcher>,
    _storage: ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,
    _interaction: ResMut<super::MoveDecorationInteractionState>,
    _backup: ResMut<crate::backup::DecoratorBackup>,
    mut _deltas: ResMut<MoveDecoratorDeltaPositions>,
    mut _local: Local<CacheDecoclutterRotationOnGrab>,
) {
    if !dispatcher.gated(3) {
        return;
    }
    // I：相位机 + 目的跳表 + 处理器（handle_moving_out_*/show_*/apply）细节 U，
    // 见模块头证据登记。
    let _ = &mut _deltas;
}

/// B 0x1407a7e80（O 结构）：楼梯放置候选计算（0x50B 记录输出；遮罩 0x22/0x23
/// + 地形/屋顶射线；'?' 类型分支）。
pub fn find_stair_placement_candidate(
    _dispatcher: &DecoratorAffordanceDispatcher,
    _ray: &utils::geometry::ray::Ray,
    _terrain: &country_core::resources::terrain_height::TerrainHeightsData<
        country_core::resources::terrain_height::TerrainHeightSsbo,
    >,
) -> Option<glam::Vec3> {
    None
}
