//! wren 停靠点与鹪鹩更新（B 主基线，fresh 反编译级证据，2026-10-17 复核）。
//!
//! 证据（restorer-eco-critters/decomp/14231c7d0.c / 14231d6c0.c / 14231e390.c）：
//! - update_decorator_bird_spots 0x14231c7d0：HashMap<DecoratorId, Entity> 生命周期。
//! - update_roof_bird_spots 0x14231d6c0：逐屋顶随机分割布置停靠点 + recycle。
//! - update_wrens 0x14231e390：to_affine3a + DebugLine/DebugSphere + 音频 set_position +
//!   状态跳表 DAT_14304b638 分派（RE-ANALYSIS §1.1 解码 5 handler，本文件已实现）。
//!
//! update_wrens 状态机（跳表 0x14304b638，spot stride 0x50，state@+0x30）：
//! 0=计时等待、1=音频过渡、2/4=扫描续、3=smoothstep 移动；高度更新尾部
//! `clamp01(height + dt*(state==0 ? -50 : 5))`（[0x14304b3b0]=(-50,5)）。
//! 残余：0x140a12650 距目标位置查询资源身份、audio call 载荷、DEBUG debug_viz /
//! PlacementDataSsbo（core-type-gap）。

use bevy_ecs::prelude::*;
use country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use country_core::resources::roofs::commands::RecalculateRoofCmd;
use country_core::resources::roofs::roof_shape::Roof;
use country_core::resources::time::GameTime;
use country_core::resources::walls::public_walls::{OnWallChanged, PublicWalls};
use country_core::systems::mode_manager::{AppMode, WallShape};
use fastrand::Rng;
use glam::Vec3;
use std::collections::HashMap;

use crate::bird_spots::BirdSpotManager;
use crate::components::{BirdSpot, BirdSpots, Wren};
use crate::wren_boids::WrenAudio;

/// `SittableForBirds`（B component，0x10）：`DecoratorId u32@+0` + 局部偏移
/// `Vec3@+4`（world = 装饰物 Transform 变换该偏移）。country-core 仅注册未定义该类型，
/// 本 crate 以同名组件承载（宿主装饰物系统插入）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct SittableForBirds {
    pub decorator: u32,
    pub offset: Vec3,
}

/// `Local<DecoratorBirdSpots>`（B wren.rs 类型）：`HashMap<DecoratorId, Entity>`。
#[derive(Default)]
pub struct DecoratorBirdSpots {
    pub spots: HashMap<u32, Entity>,
}

/// 装饰物鸟点生成（0x14231c7d0，fresh 逐语句重建）。
///
/// PDB 符号签名（0x141d82940/0x141d835b0）：
/// `fn(Query<(&SittableForBirds, &Transform)>, ResMut<BirdSpotManager>,
///     Local<DecoratorBirdSpots>, Query<&BirdSpots>, Commands)`。
///
/// 逐语句：
/// - 逐 sittable：id = `SittableForBirds+0`；world = `Transform.translation +
///   Transform.rotation * offset`（decomp 展开的四元数矩阵）；
/// - 已存在且 `BirdSpots` 首点 pos 与 world **逐位有序相等**（含 NaN 不等）→ 保持；
/// - 否则 `HashMap::remove_entry` → `Entities::contains` 门控下 despawn 旧实体；
/// - 新建：`BirdSpotManager::gen_id()`（点 id）→ `create_spots()`（池分配）→
///   `spawn_empty + queue insert<BirdSpots>`（点 = {pos, id}）→ `HashMap::insert`；
/// - 收尾 `HashMap::retain(本次仍存在的 decorator id)`。
pub fn update_decorator_bird_spots(
    mut manager: ResMut<BirdSpotManager>,
    mut commands: Commands,
    sittables: Query<(&SittableForBirds, &crate::components::Transform)>,
    mut spots_q: Query<&BirdSpots>,
    mut local: Local<DecoratorBirdSpots>,
) {
    let mut seen: Vec<u32> = Vec::new();
    for (sittable, transform) in &sittables {
        let id = sittable.decorator;
        seen.push(id);
        // O：`t = translation + rotation * offset`（x/y/z 由四元数矩阵展开）。
        let world = transform.translation + transform.rotation * sittable.offset;

        if let Some(&entity) = local.spots.get(&id) {
            let same = spots_q
                .get(entity)
                .ok()
                .and_then(|s| s.spots.first())
                .map(|spot| {
                    // O：三轴有序相等（`==` 且非 NaN）。
                    spot.pos.x == world.x
                        && !spot.pos.x.is_nan()
                        && !world.x.is_nan()
                        && spot.pos.y == world.y
                        && !spot.pos.y.is_nan()
                        && !world.y.is_nan()
                        && spot.pos.z == world.z
                        && !spot.pos.z.is_nan()
                        && !world.z.is_nan()
                })
                .unwrap_or(false);
            if same {
                continue;
            }
            local.spots.remove(&id);
            // O：`Entities::contains` 后才 despawn。
            if spots_q.get_mut(entity).is_ok() {
                commands.entity(entity).despawn();
            }
        }

        // O：gen_id（点 id）→ create_spots（池分配）→ spawn + insert<BirdSpots>。
        let _spot_id = manager.gen_id();
        let allocated = manager.create_spots(1);
        let spots: Vec<BirdSpot> = allocated
            .into_iter()
            .map(|_| BirdSpot {
                pos: world,
                occupied: false,
            })
            .collect();
        let entity = commands.spawn(BirdSpots { spots }).id();
        local.spots.insert(id, entity);
    }
    // O：`HashMap::retain`（decorator 仍存在）。
    let keep: std::collections::HashSet<u32> = seen.into_iter().collect();
    local.spots.retain(|id, _| keep.contains(id));
}

/// 屋顶鸟点生成（0x14231d6c0）。
///
/// PDB 符号签名（0x141d81f90）：
/// `fn(Query<(Entity, &mut Transform, &mut Wren), Without<SittableForBirds>>,
///     EventReader<RecalculateRoofCmd>, EventReader<OnWallChanged>,
///     Query<(&Roof, Option<&BirdSpots>)>, ResMut<BirdSpotManager>, Commands,
///     Res<PublicWalls>, Res<UiSignifierStream>, Res<GameTime>)`。
///
/// **状态：部分实现（未实现-需重做）**。已证并落库：
/// - 事件驱动：`RecalculateRoofCmd` / `OnWallChanged` 标记需重算的屋顶；
/// - `PublicWalls::roof_entity` 取屋顶实体；实体不存在 → `BirdSpotManager::recycle`
///   （屋顶移除路径）；
/// - 形状门：`Roof` 截面 tag==1（矩形）且 `!is_flat` 才产生脊线停靠点；
/// - 矩形门：`RoofRidgeDims::from` + `calc_roof_tip_center_ls` → `Rectangle2d`
///   （`orientation_mat2`/`new_axis_aligned`）→ `get_edge_pts`，两端距 > 0.5 才采样；
///   否则空点集。
///
/// **未实现（分歧点/解锁条件）**：
/// - `utils::rng::init_rng` 的种子来源（寄存器实参 U）；
/// - `utils::random_splits` 的 count/amplitude 实参（U）与逐点 `fastrand::Rng::f32`
///   偏置用途；
/// - 逐点 `Roof::height(t)` + `swizzlers::xvy` + `apply_roof_noise` 的采样平面
///   （脊线 u 值 → 世界点）未闭合；
/// - `ClutterVersion::is_initial` 分支（0x54 style byte）选边差异；
/// - `UiSignifierStream` 调试写入、wren 实体对旧点的重定位路径。
/// 解锁条件：上述实参需 fresh 反汇编逐寄存器闭合（当前 ghidra 变量合并无法区分）。
pub fn update_roof_bird_spots(
    time: Res<GameTime>,
    walls: Res<PublicWalls>,
    mut manager: ResMut<BirdSpotManager>,
    mut commands: Commands,
    mut roof_changed: EventReader<RecalculateRoofCmd>,
    mut wall_changed: EventReader<OnWallChanged>,
    mut roofs: Query<(&Roof, Option<&BirdSpots>)>,
    wrens: Query<(Entity, &mut crate::components::Transform, &mut Wren), Without<SittableForBirds>>,
) {
    let _ = (&time, &wrens);
    // 收集需重算的墙（事件载荷 = WallId）。
    let mut dirty: Vec<country_core::resources::walls::WallId> = Vec::new();
    for event in roof_changed.read() {
        dirty.push(event.0);
    }
    for event in wall_changed.read() {
        dirty.push(event.0);
    }
    for wall in dirty {
        let Some(roof_entity) = walls.roof_entity(wall) else {
            // 屋顶已移除：回收（O：`recycle::<closure_env$3>`）。
            manager.recycle_all();
            continue;
        };
        let Ok((roof, _existing)) = roofs.get_mut(roof_entity) else {
            manager.recycle_all();
            continue;
        };
        // 形状门：tag==1（矩形）&& !is_flat（O 14231daf6/14231db0x）。
        let rect_roof = roof.observed()[8..12]
            .try_into()
            .map(u32::from_le_bytes)
            .unwrap_or(0)
            == 1;
        if !rect_roof || roof.is_flat() {
            let spots = manager.create_spots(0);
            let _ = spots;
            commands.spawn(BirdSpots { spots: Vec::new() });
            continue;
        }
        // 脊线矩形门（几何部分已证；采样段未实现 → 空点集并登记）。
        let dims = country_core::resources::roofs::roof_ridge::RoofRidgeDims::from(roof);
        let tip = country_core::resources::roofs::roof_ridge::calc_roof_tip_center_ls(
            roof,
            dims.get(),
        );
        let rect = utils::geometry::rectangle::Rectangle2d::new_axis_aligned(
            glam::Vec2::new(tip[0], tip[1]),
            glam::Vec2::new(dims.get()[0], dims.get()[1]),
        );
        let _ = roof.is_gable();
        let (a, b) = rect.get_edge_pts(0);
        let edge_len = (b - a).length();
        let spots = if 0.5 < edge_len {
            // 未实现：init_rng/random_splits/height/xvy/apply_roof_noise 采样（见函数注）。
            Vec::new()
        } else {
            Vec::new()
        };
        let _ = manager.create_spots(spots.len());
        commands.spawn(BirdSpots { spots });
    }
}

/// 鹪鹩 per-spot 状态机记录（二进制 spot 0x50 记录的语义投影；字段偏移见各注释）。
/// 状态跳表 DAT_14304b638（RE-ANALYSIS §1.1 解码，index=`*(uint*)(spot+0x30)`）：
/// 0=计时/等待（timer 倒计时；到点或距目标<1.0 → state1）、1=音频过渡（→2）、
/// 2/4=扫描续（无 handler）、3=smoothstep 移动（→重置后高度更新）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct WrenSpotState {
    /// +0x30：状态字节（0..4）。
    pub state: u32,
    /// +0x34：计时（state0 倒计时；state3 复用作 dir.x 方向分量）。
    pub timer: f32,
    /// +0x38：state3 方向分量 y。
    pub dir_y: f32,
    /// +0x3c：state3 方向分量 z。
    pub dir_z: f32,
    /// +0x40：smoothstep 进度计时（state3 `+= dt`，>6.0 → 重置）。
    pub progress: f32,
    /// +0x44：动画流经值（state0/1 经 XMM15 原样回写；state3 = 新 progress）。
    pub anim_val: f32,
    /// +0x48：高度（`clamp01(height + dt*(state==0 ? -50 : 5))`，表 [0x14304b3b0]=(-50,5)）。
    pub height: f32,
}

/// update_wrens（0x14231e390）。
/// ① 逐鹪鹩 PlacementDataSsbo::to_affine3a（放置矩阵；crate 以 Wren.pos 承载）→
///    audio set_position（0x14231ebde call 0x140a57da0，跳表分派前）。
/// ② 首只活跃鹪鹩 → 状态跳表 DAT_14304b638 + (state@+0x30)*4 分派（见 wren_spot_step）。
/// ③ DEBUG_SHOW_HAVEN_BIRD_POPULATION DebugLine / DEBUG_SHOW_BIRD_SPOTS DebugSphere /
///    _DEBUG_AIRBORNE_BIRD_COUNT / to_affine3a / PlacementDataSsbo 资源 → 未接线
///    （core-type-gap：debug_viz / placement SSBO 缓冲，crate 无）。
pub fn update_wrens(
    time: Res<GameTime>,
    mode: Res<AppMode>,
    cursor: Res<CursorTerrainRaycast>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Wren, &mut WrenSpotState, &mut WrenAudio)>,
) {
    let dt = time.delta;
    let cursor_pos = cursor.position();
    // O 0x14231e7d9-0x14231f5fb：模式 → h0 距离阈值（`xmm7`）。
    let proximity = wren_proximity_threshold(*mode);
    for (entity, mut wren, mut spot, mut audio) in &mut query {
        // ① 音频 set_position（跳表分派前，0x14231ebde）。
        audio.handle.set_position(wren.pos);
        // ② 状态机 + 跳表分派。
        wren_spot_step(
            &mut spot,
            &mut wren.pos,
            dt,
            cursor_pos,
            proximity,
            entity,
            &mut commands,
        );
    }
}

/// h0 距离阈值（fresh 0x14231e7d9-0x14231f5fb）：
/// - `AppMode::Wall(Circle|Rectangle)`（tag 6 + `byte[+4]∈{1,2}`）→ **3.0**；
/// - tag ∈ {7 Path, 8 Cobblestone, 9 Eraser, 10 Terrain, 15 Water} → **1.0**；
/// - 其余 → **0.0**（关闭距离查询）。
/// 0.0/1.0/3.0 常量实读：0x142925904=1.0、0x1429cd078=3.0；`edx-6<15` 归一 +
/// `rcx-2<4`（tag 7..10）/`rcx==0xa`（tag 15）/`rcx==1`（tag 6）分支逐指令对应。
/// 资源身份按 tag 集合与 WallShape 子字节取 AppMode（U→强候选；原 RESTORE 登记 U）。
pub fn wren_proximity_threshold(mode: AppMode) -> f32 {
    match mode {
        AppMode::Wall(WallShape::Circle | WallShape::Rectangle) => 3.0,
        AppMode::Wall(WallShape::Freeform) => 0.0,
        AppMode::Path | AppMode::Cobblestone(_) | AppMode::Eraser(_) | AppMode::Terrain
        | AppMode::Water => 1.0,
        _ => 0.0,
    }
}

/// per-spot 状态机（跳表 DAT_14304b638 5 handler：0x14231ebf8/0x14231e930/0x14231e9a9/
/// 0x14231ec82/0x14231edf4）。状态 2/4 = 实体扫描续（bevy 查询天然承担，无 handler）。
#[allow(clippy::too_many_arguments)]
fn wren_spot_step(
    spot: &mut WrenSpotState,
    pos: &mut Vec3,
    dt: f32,
    cursor_pos: Vec3,
    proximity: f32,
    entity: Entity,
    commands: &mut Commands,
) {
    match spot.state {
        0 => {
            // h0（0x14231ebf8）：`[+0x34] -= dt`；`0 >= timer → H1 中途`；
            // 且 `threshold > 0`（门）且光标平面距离 `< threshold` → H1 中途；
            // 否则高度更新。
            spot.timer -= dt;
            if spot.timer <= 0.0 {
                wren_h1_mid(spot, dt);
                return;
            }
            if proximity > 0.0 {
                let dx = cursor_pos.x - pos.x;
                let dz = cursor_pos.z - pos.z;
                let dist = (dx * dx + dz * dz).sqrt();
                if proximity > dist {
                    wren_h1_mid(spot, dt);
                    return;
                }
            }
            wren_height_update(spot, dt);
        }
        1 => {
            // h1（0x14231e930）：保存 [+0x44] → H1 中途（音频槽写 U + pos-record
            // +0x1c/+0x20/+0x24={1,1,1} U + state→2 + audio call 0x140a57300 U 载荷）
            // → 高度更新尾部 + [+0x44] 回写。
            wren_h1_mid(spot, dt);
        }
        2 | 4 => {
            // h2（0x14231e9a9）/h4（0x14231edf4）：实体/嵌套实体扫描续（无状态写入）。
        }
        3 => {
            // h3（0x14231ec82）：smoothstep 位置移动 → 高度更新尾部。
            wren_h3_move(spot, pos, dt, entity, commands);
            wren_height_update(spot, dt);
        }
        _ => {}
    }
}

/// H1 中途（0x14231e937）：写音频槽（`mov dword [rcx], eax`，查询值 U）+
/// 位置记录 +0x1c/+0x20/+0x24 = 1.0（r13 记录，crate 未建模，U）+ `qword [+0x30]=2`
/// （state=2 **且 +0x34 timer 清零**，B-ecstate FIX）+ audio call 0x140a57300（U 载荷）。
/// 随后高度更新尾部（`[+0x44]` 原样回写）。
fn wren_h1_mid(spot: &mut WrenSpotState, dt: f32) {
    // B-ecstate FIX：`qword [r12+0x30] = 2` 一次写 8B —— state=2 且 timer(+0x34)=0。
    spot.state = 2;
    spot.timer = 0.0;
    // [+0x1c]/[+0x20]/[+0x24]=1.0 属 r13 位置记录（crate 未建模，登记 U）；
    // 音频槽 u32 写（查询值）+ call 0x140a57300（载荷）同样登记 U。
    wren_height_update(spot, dt);
}

/// 高度更新尾部（0x14231e969）：`height = clamp01(height + dt * (state==0 ? -50 : 5))`
/// （表 [0x14304b3b0]=(-50.0,5.0)；`sete al` 按当前 state==0 取因子）。
/// `[+0x44] = xmm15`：state0/1 为经保存原样回写（本重建 anim_val 未被中间改动，
/// 保持即可）；state3 由 h3_move 已把新 progress 写入 anim_val。
fn wren_height_update(spot: &mut WrenSpotState, dt: f32) {
    let factor = if spot.state == 0 {
        f32::from_bits(0xc2480000) // -50.0
    } else {
        f32::from_bits(0x40a00000) // 5.0
    };
    spot.height = (spot.height + dt * factor).clamp(0.0, 1.0);
}

/// h3 smoothstep 移动（0x14231ec82，fresh 反汇编 2026-10-17 逐指令复核）：
/// - `t = clamp01(progress / 0.3)`（0x1429258d8=0.3）
/// - `smooth = t*t*(3-2t)`（0x1429cd078=3.0）
/// - `phase = 0.5 + 0.5*sin(max(0, progress-0.7) * 2π)`（0x142f9b080=-0.7、0x142934674=2π、
///   0x142925870=0.5；call 0x142923fd0=sinf）——原式 `progress + (-0.7)`。
/// - `vx = 4*(1-smooth) + 10*smooth`；`dvy = 2*(1-phase) - 0.6*phase`
///   （[0x14292e6b0]=(1,1,..)、[0x142b2e410]=(4,2)、[0x14304b3a0]=(10,0.6)）。
/// - **移动精确式**（fresh 0x14231ed04-0x14231ed62 复核，纠正 REVIEW 的合并式）：
///   `pos.x += vx*dir_x*dt`；`pos.y += vx*dvy*dir_y*dt`；`pos.z += vx*dir_z*dt`
///   （xmm2=dir_y*vx、xmm3=dir_z*vx，shufps 只把 (vx,dvy) 与 (dir_x,dir_y) 配对）。
/// - `progress += dt`；`progress > 6.0`（0x142934690）→ `Commands::entity(..).despawn()`
///   （0x140d14770 实为 `bevy_ecs::EntityCommands::despawn`，非 panic；原登记已纠正）。
/// - 尾部 `[+0x44] = progress × 7.0`（0x142a7cd48=7.0）。
fn wren_h3_move(
    spot: &mut WrenSpotState,
    pos: &mut Vec3,
    dt: f32,
    entity: Entity,
    commands: &mut Commands,
) {
    let t = (spot.progress / f32::from_bits(0x3e99999a)).clamp(0.0, 1.0);
    let smooth = t * t * (3.0 - 2.0 * t);
    let phase = 0.5
        + 0.5
            * (0.0_f32.max(spot.progress - f32::from_bits(0xbf333333))
                * f32::from_bits(0x40c90fdb))
                .sin();
    let vx = 4.0 * (1.0 - smooth) + 10.0 * smooth;
    let dvy = 2.0 * (1.0 - phase) - 0.6 * phase;
    let dir_x = spot.timer; // +0x34 复用作 dir.x
    pos.x += vx * dir_x * dt;
    pos.y += vx * dvy * spot.dir_y * dt;
    pos.z += vx * spot.dir_z * dt;
    spot.progress += dt;
    let anim = spot.progress * f32::from_bits(0x40e00000);
    if spot.progress > f32::from_bits(0x40c00000) {
        // >6.0 → `EntityCommands::despawn()`（0x140d14770；实体映射 = 本 crate 的
        // wren/spot 宿主实体）。
        commands.entity(entity).despawn();
    }
    // 尾部 `[+0x44] = xmm15`（新 progress × 7.0，0x142a7cd48=7.0）。
    spot.anim_val = anim;
}

/// 鹪鹩 RNG（fastrand；出生与移动抖动，O）。
#[derive(Resource, Debug)]
pub struct WrenRng(pub std::sync::Mutex<Rng>);

impl Default for WrenRng {
    fn default() -> Self {
        Self(std::sync::Mutex::new(Rng::with_seed(0x606f)))
    }
}

/// 轨迹标记（plan_duck 出生路径的鹪鹩侧不用；装饰物鸟点查询的占位类型）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct WrenTrajectoryMarker;
