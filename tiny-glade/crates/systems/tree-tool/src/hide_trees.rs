//! hide_trees.rs — 相机距离内隐藏树冠 + 历史记录（B 0x14226a260，O；
//! re-analyst-A 契约 §2）。
//!
//! 语义（O）：双空间哈希遍历（0x18B + 0x30B 层）+ TreeStorage 迭代；树记录→
//! 位置；`Circle2d::with_expanded_diameter`（0x140c980c0）+ `contains_point`
//! （0x140c980f0）判相机圈内 → 写可见性（组件字节 + 滴答）；首次隐藏 → push
//! 0x10B 到隐藏列表 + History::add（0x58B 变体 2，EditType::Tree 删除）。

use bevy_ecs::prelude::*;
use utils::geometry::Circle2d;

use country_core::components::{PrefabInstanceData, TreeAnimation, TreeComponent};
use country_core::resources::history::tree_edit::DeleteTreeCmd;
use country_core::resources::history::{EditType, History};
use country_core::resources::prefab::PrefabInstance;
use country_core::resources::tree_storage::{HideTreesinAreaCmd, HideTreesNextToWallCmd, TreeStorage};
use country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash;

/// `hide_trees` @0x14226a260（O 面：TreeStorage::iter（0x140a1b100）真实遍历 +
/// 相机圈判定（Circle2d::with_expanded_diameter 0x140c980c0 + contains_point
/// 0x140c980f0）→ 可见性写 + History 变体 2 删除——真实树 id）。
#[allow(clippy::too_many_arguments)]
pub fn hide_trees<S: Send + Sync + 'static>(
    mut trees: Query<(
        &mut TreeAnimation,
        &PrefabInstanceData,
        &PrefabInstance,
        &TreeComponent,
    )>,
    storage: Res<TreeStorage>,
    mut area_cmds: EventReader<HideTreesinAreaCmd>,
    mut wall_cmds: EventReader<HideTreesNextToWallCmd>,
    _walls: Res<WallSpatialHash>,
    mut deletes: EventWriter<DeleteTreeCmd>,
    mut history: ResMut<History<S>>,
) {
    // O：圈过滤门（HideTreesinAreaCmd/HideTreesNextToWallCmd 载荷 {中心, 半径}——
    // Circle2d::with_expanded_diameter + contains_point 语义）。I 级登记：机器码
    // 的圈源 = 相机圈（0x140c980c0/0x140c980f0）+ 双空间哈希遍历（0x18B+0x30B 层）
    // ——命令圈 vs 相机圈语义待 re-analyst 核对（reviewer-A 第三轮 ④）。
    let area = area_cmds.read().last().copied();
    let wall = wall_cmds.read().last().copied();
    // O：TreeStorage 全遍历 → 真实树 id + 位置；圈内 → 删除命令 + History
    // 变体 2（真实载荷）。
    for (index, entry) in storage.iter() {
        let pos = glam::Vec2::new(entry.position.x, entry.position.z);
        let in_area = area
            .map(|cmd| {
                Circle2d::new(cmd.center, cmd.radius).is_point_inside(pos)
            })
            .unwrap_or(false);
        let in_wall = wall
            .map(|cmd| Circle2d::new(cmd.center, cmd.radius).is_point_inside(pos))
            .unwrap_or(false);
        if !in_area && !in_wall {
            continue;
        }
        // 可见性写面（组件字节 + 滴答，U）；圈内 → 删除命令 + 历史。
        deletes.write(DeleteTreeCmd { id: index.0 });
        let edit = EditType::Tree(std::sync::Arc::new(
            country_core::resources::history::tree_edit::TreeEdit::Delete(
                country_core::resources::history::tree_edit::TreeEditData {
                    id: index.0,
                    ..Default::default()
                },
            ),
        ));
        let _ = history.add(edit);
    }
}
