//! B `ui_systems/ui_stairs_viz.rs` — 楼梯可视化（O 结构）。
//!
//! - `visualize_ui_stairs` 0x140791510（O 结构）：读取两个游标状态（各
//!   {ptr@0, base@8}；@0x18/@0x38 界限、@0x40 计数）计算差值计数；构建
//!   5 节点指示符记录（0x18B {flag, u64, u64} 族，FUN_140d4d740/0x140d64e00
//!   分配器——每节点 `[8]+=1`）；写入 0x130B+ 楼梯可视化记录（5 位置对 ×
//!   位置 + 链接态）；复位拷贝 + FUN_140604c00（帧重置）；按计数跳表
//!   UNK_142a81b90 派发（楼梯态机）。
//! - `insert_point_ui` 0x14079bc20（O 结构）：插入点 widget 构建 +
//!   射线（0x140c904d0/0x140959e70 数学）+ signifier（0x140b2cf90）+ 名称
//!   哈希（0x1405b0d70）；网格/端插入判定 I。
use bevy_ecs::prelude::*;
use country_core::resources::stairs::stairs_state::StairsState;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use country_core::systems::ui::signifiers::UiAppendSignifier;
use glam::Vec3;

use crate::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher;
use crate::ui_systems::MoveDecorationInteractionState;

/// 楼梯可视化输出（I：0x130B+ 记录；字段语义 U——5 位置对 + 链接态）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct StairsVisualization {
    /// 5 个可视化点（O：5 节点分配）。
    pub points: [Vec3; 5],
    /// 有效点数。
    pub count: u8,
}

/// B 0x140791510（O 结构）：楼梯可视化主系统。
#[allow(clippy::too_many_arguments)]
pub fn visualize_ui_stairs(
    _ui: Res<country_core::systems::ui::resources::ui_state::UiState>,
    mut stream: ResMut<UiSignifierStream>,
    mut interaction: ResMut<MoveDecorationInteractionState>,
    stairs: Res<StairsState>,
    mut viz: ResMut<StairsVisualization>,
) {
    interaction.reset_interaction();
    let endpoint = stairs.endpoint();
    if endpoint.is_unset() {
        viz.count = 0;
        return;
    }
    let node = endpoint.node();
    let connected = stairs.get_all_connected_nodes(node);
    let mut count = 0u8;
    for (i, n) in connected.iter().enumerate().take(5) {
        let _ = n;
        let pos = Vec3::new(i as f32, 0.0, 0.0); // I：位置来自游标状态（U）。
        viz.points[i] = pos;
        stream.write(UiAppendSignifier {
            tag: 1,
            kind: 1,
            position: pos,
            scale: 1.0,
            name: u64::from(*n),
        });
        count += 1;
    }
    viz.count = count;
    let _ = &mut stream;
}

/// 插入点状态（I：插入点位置/判定）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct InsertPointState {
    pub pos: Vec3,
    pub valid: bool,
}

/// B 0x14079bc20（O 结构）：楼梯插入点 UI。
#[allow(clippy::too_many_arguments)]
pub fn insert_point_ui(
    _ui: Res<country_core::systems::ui::resources::ui_state::UiState>,
    _dispatcher: Res<DecoratorAffordanceDispatcher>,
    mut stream: ResMut<UiSignifierStream>,
    _storage: Res<DecoratorStorage>,
    _stairs: Res<StairsState>,
    state: ResMut<InsertPointState>,
) {
    if !state.valid {
        return;
    }
    stream.write(UiAppendSignifier {
        tag: 1,
        kind: 1,
        position: state.pos,
        scale: 1.0,
        name: 0,
    });
}
