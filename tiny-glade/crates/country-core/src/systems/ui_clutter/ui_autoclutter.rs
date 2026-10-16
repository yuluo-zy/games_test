//! `ui_autoclutter.rs` — 右键把 auto-clutter 转 deco-clutter（B 0x14099ca60，O）。
//!
//! 证据：`chapters/systems/ui_clutter/ui_autoclutter.rs.md`、`ui-input/DIGEST.md`、
//! `ui-input/decomp/decomp-0x14099ca60.txt`（1404 行）、re-analyst-interaction：
//! 0x48 B spawn 事件统一布局（tag=2@0、subtype=3@+0x29、type@+0x3c、entity@+0x40）；
//! 旋转 = `FUN_1408b83d0`（`ClutterMeshTy::from`）+ 常量区 UNK_1429cd1/1429258。
//!
//! 转换流程（decomp）：消费点击事件 → 解析点击 clutter 记录（0xa0B，type @+0x94）→
//! `maintain_autoclutter_parent_hashmap`（行 410）维护 clutter→parent 映射（0x60B
//! 分配；hashbrown insert @code_r0x00014099d329）→ 按记录类型生成 InstaCreateDeco
//! （decorator 放置）或 InstaCreateStairPoint（台阶点）→ 经 `FUN_1408b8e90`
//! （clutter_mesh_ty_data）+ 变换计算推 0x48B 事件。
//!
//! 卡点：`maintain_autoclutter_parent_hashmap` 映射表槽位布局 U（0x60B 载荷字段名
//! 待取证）；`InstaCreateDeco/InstaCreateStairPoint/StartPlaceDecoClutterInteraction`
//! 事件类型为全局事件 registry 合同（wp20），本域先定义最小 struct。
use bevy_ecs::prelude::*;

use super::{
    AutoClutterEntityIndex,
    rmb_autoclutter::{EvRightClickedAutoClutter, get_clutter_rot_in_roof_space},
};

/// 已解析的 auto-clutter parent（0x60B 槽位 O 观测字段 + crate 转换输入 I 模型）。
///
/// ① T3 t3-ui_clutter-parent-map-0x60B：native parent 记录按 `slot * 0x60` 索引
/// （decomp-0x14099ca60 行 480-520），已观测字段：
/// - `+0x00` u32 tag/判别（`< 3` 门才进入转换）
/// - `+0x10` u32 计数/类别候选
/// - `+0x30` u64 `ClutterMeshTy::from` 第一参（FUN_1408b83d0）
/// - `+0x38` u64 `ClutterMeshTy::from` 第二参
/// 其余区间（+0x04..0x0f、+0x14..0x2f、+0x3c..0x5f）**U**，不填默认值。
/// 下侧字段（parent_entity/roof_space/w/x/y/entry_angle）为 crate 转换数学的输入
/// （ui_clutter DELIVERY 的 I 模型，非原生槽位偏移声明）。
#[derive(Clone, Copy, Debug)]
pub struct AutoClutterParent {
    /// O `+0x00`：tag/判别（`< 3` 门进入转换）。
    pub tag: u32,
    /// O `+0x10`：计数/类别候选。
    pub slot_count: u32,
    /// O `+0x30`：`ClutterMeshTy::from` 第一参。
    pub mesh_ty_arg: u64,
    /// O `+0x38`：`ClutterMeshTy::from` 第二参。
    pub mesh_ty_arg2: u64,
    /// parent 实体 id（roof/ground）。
    pub parent_entity: u64,
    /// roof-space 判别（roof 上 clutter 的变换基准）。
    pub roof_space: bool,
    /// parent 四元数分量 w/x/y（roof-space 旋转输入）。
    pub w: f32,
    pub x: f32,
    pub y: f32,
    /// parent 入口角（`CircularF32` 输入）。
    pub entry_angle: f32,
}

/// `maintain_autoclutter_parent_hashmap`（行 410）：clutter→parent 真实映射表。
/// 原版维护哈希映射（0x60B 分配入表）；当前以 hashbrown `HashMap<entity, parent>`
/// 承载，槽位布局 U 待取证。**非计数器**（修复"计数器冒充 parent map"水分）。
#[derive(Resource, Clone, Debug, Default)]
pub struct AutoClutterParentMap(pub hashbrown::HashMap<u64, AutoClutterParent>);

impl AutoClutterParentMap {
    /// 维护一条 clutter→parent 映射（插入/更新）。
    pub fn maintain(&mut self, clutter: u64, parent: AutoClutterParent) {
        self.0.insert(clutter, parent);
    }
    /// 移除（clutter 删除时清理）。
    pub fn remove(&mut self, clutter: u64) -> Option<AutoClutterParent> {
        self.0.remove(&clutter)
    }
    /// 查询 parent（ui_convert 消费）。
    pub fn parent_of(&self, clutter: u64) -> Option<AutoClutterParent> {
        self.0.get(&clutter).copied()
    }
}

/// 转换产生的装饰放置事件（I：wp20 合同 `InstaCreateDeco`；字段布局 U）。
#[derive(Event, Clone, Copy, Debug)]
pub struct InstaCreateDeco {
    pub entity: u64,
    pub ty: crate::systems::mode_manager::ClutterMeshTy,
    pub position: glam::Vec3,
    pub rotation: f32,
    pub parent: u64,
}

/// 转换产生的台阶点放置事件（I：wp20 合同 `InstaCreateStairPoint`；字段布局 U）。
#[derive(Event, Clone, Copy, Debug)]
pub struct InstaCreateStairPoint {
    pub entity: u64,
    pub ty: crate::systems::mode_manager::ClutterMeshTy,
    pub position: glam::Vec3,
    pub rotation: f32,
    pub parent: Option<u64>,
}

/// `ui_convert_autoclutter_to_decoclutter` @0x14099ca60（O 结构）。
///
/// 消费 `EvRightClickedAutoClutter` → parent 解析（真实映射表）→ roof-space 旋转
/// （`get_clutter_rot_in_roof_space`，**rotation=0 桩已删除**）→ InstaCreateDeco /
/// InstaCreateStairPoint 生产。decomp 的 0xa0B 记录遍历 + 0x48B 事件推流（type < 3
/// 走台阶点分支）为宿主实体机制依赖面，此处以实体索引记录的类型判别登记。
pub fn ui_convert_autoclutter_to_decoclutter(
    mut right_clicks: EventReader<EvRightClickedAutoClutter>,
    index: Res<AutoClutterEntityIndex>,
    parents: Res<AutoClutterParentMap>,
    mut insta_deco: EventWriter<InstaCreateDeco>,
    mut insta_stair: EventWriter<InstaCreateStairPoint>,
) {
    for click in right_clicks.read() {
        let Some(record) = index.0.get(&click.entity).copied() else {
            continue;
        };
        let parent = parents.parent_of(click.entity);
        // 真实 roof-space 旋转（O：get_clutter_rot_in_roof_space 接入转换路径）。
        let rotation =
            get_clutter_rot_in_roof_space(record.w, record.x, record.y, record.entry_angle);
        // 记录类型判别（O：decomp `if (*puVar22 < 3)` 走台阶点分支；记录 type 字段
        // 语义 I）。roof-space parent → 装饰放置，否则台阶点。
        if (record.ty as u8) < 3 {
            insta_stair.write(InstaCreateStairPoint {
                entity: click.entity,
                ty: click.ty,
                position: click.position,
                rotation,
                parent: parent.map(|p| p.parent_entity),
            });
        } else {
            insta_deco.write(InstaCreateDeco {
                entity: click.entity,
                ty: click.ty,
                position: click.position,
                rotation,
                parent: parent
                    .map(|p| p.parent_entity)
                    .unwrap_or(click.entity),
            });
        }
    }
}
