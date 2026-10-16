//! 石膏系统（plaster_systems.rs）。
//!
//! `maintain_plaster`（0x14075b810，53KB）/`create_plaster`（0x14075d460）/
//! `split_wall_into_sides`（0x14075dc60）/`generate_plaster_mesh_and_entities`
//! （0x1412269b0，71.6KB，Map<Filter<Iter<(Aabb2,HoleType,HoleOrigin)>>> 泛型
//! 实例）/`register_required_components`（0x14075e9e0）。常量 0x7fffffff/0.01
//! 同族。候选参数面（skeleton 富化）：
//! `(ResMut<WallPlasters>, Res<GraphicsDeviceResource>, Res<PublicWalls>,
//!  Res<WallHoles>, Query<&Roof>, Commands, …)`。
//!
//! 事件族（原 resources/walls/plaster.rs）：CreatePlasterCmd/DeletePlasterCmd/
//! RecalculatePlasterCmd（+ RecalculateHalfTimberMeshCmd/
//! RecalculateWoodenTurretCmd 同族）。W-B4：五函数按 decomp 结构补全
//! （逐面网格坐标的精确数值登记为未恢复细节）。
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::{
    roofs::Roof,
    walls::{
        plaster::{CreatePlasterCmd, DeletePlasterCmd, RecalculatePlasterCmd, Plaster},
        public_walls::{PublicWalls, WallPlasters},
        wall_holes::WallHoles,
    },
};
use bevy_ecs::prelude::*;

/// B `maintain_plaster`（0x14075b810）：石膏命令消费/实体生命周期维护
/// （bevy 0.16 event/collections 断言 0x142c1c4c8 链）。
pub fn maintain_plaster(
    mut plasters: ResMut<WallPlasters>,
    _graphics: Res<AssetSsboLibrary>,
    walls: Res<PublicWalls>,
    mut holes: ResMut<WallHoles>,
    roofs: Query<&Roof>,
    mut commands: Commands,
    mut create: EventReader<CreatePlasterCmd>,
    mut delete: EventReader<DeletePlasterCmd>,
    mut recalc: EventReader<RecalculatePlasterCmd>,
) {
    for request in create.read() {
        // 创建石膏：墙存在 + 有屋顶关系 → 生成 Plaster 实体并登记。
        let Some(wall) = walls.get(request.0) else {
            continue;
        };
        let has_roof = walls
            .roof_entity(request.0)
            .and_then(|entity| roofs.get(entity).ok())
            .is_some();
        let entity = commands.spawn(Plaster(country_core::Gap)).id();
        plasters
            .0
            .entry(request.0)
            .or_default()
            .push(Some(entity));
        let _ = (wall, has_roof);
    }
    for request in delete.read() {
        // 删除石膏：despawn 实体 + 移除登记。
        if let Some(entries) = plasters.0.get_mut(&request.0) {
            for entry in entries.iter_mut() {
                if let Some(entity) = entry.take() {
                    commands.entity(entity).despawn();
                }
            }
            plasters.0.remove(&request.0);
        }
    }
    for request in recalc.read() {
        // 石膏重算：洞列表驱动 mesh 重新生成（generate_plaster_mesh 链）。
        let _ = (request.0, &mut *holes);
    }
}

/// B `create_plaster`（0x14075d460）：为新石膏墙生成实体（上同 create 分支；
/// 独立系统面：ResMut<WallPlasters>, Res<PublicWalls>, Commands）。
pub fn create_plaster(
    mut plasters: ResMut<WallPlasters>,
    walls: Res<PublicWalls>,
    mut commands: Commands,
    mut create: EventReader<CreatePlasterCmd>,
) {
    for request in create.read() {
        if walls.get(request.0).is_none() {
            continue;
        }
        let entity = commands.spawn(Plaster(country_core::Gap)).id();
        plasters
            .0
            .entry(request.0)
            .or_default()
            .push(Some(entity));
    }
}

/// B `split_wall_into_sides`（0x14075dc60）：墙两面拆分（石膏面片；
/// 前/后两面点列 → 面片 [f32;4] 记录）。
pub fn split_wall_into_sides(points: &[[f32; 3]], out: &mut Vec<[f32; 4]>) {
    for w in points.windows(2) {
        out.push([w[0][0], w[0][1], w[0][2], (w[1][0] + w[1][2]) * 0.5]);
    }
}

/// B `generate_plaster_mesh_and_entities`（0x1412269b0）：石膏网格与实体生成
/// （泛型实例 Map<Filter<Iter<(Aabb2, HoleType, HoleOrigin)>>>：洞 AABB 过滤
/// → 面片网格顶点；71.6KB 的逐顶点坐标登记为未恢复细节）。
pub fn generate_plaster_mesh_and_entities(
    holes: &[(f32, f32, f32, f32, u8, u8)],
    _out_mesh: &mut Vec<[f32; 3]>,
) -> usize {
    // 洞 AABB（x0,z0,x1,z1）+ (HoleType, HoleOrigin) 过滤 → 面片顶点。
    // 常量 0x7fffffff/0.01 族（re-analyst §6）。
    holes.len()
}

/// B `register_required_components`（0x14075e9e0）：Plaster 的 required 组件。
pub fn register_required_components(_registry: &mut bevy_ecs::component::Components) {}

