# REVIEW — systems/collision(卡A5 终验 → 闭合·核对+登记)

审查员: reviewer-interaction | 日期: 2026-10-16
基线: restorer RESTORE.md(reports/interaction/collision/,本卡未改 src)。

## 核对

### ① systems/collision.rs 常量锚定 — PASS
`SUBWORLD_SLOT_COUNT=17`、`SubworldIndex::TERRAIN` slot=5、`to_bitflag=1<<槽号`(Terrain
=0x20)、`EMPTY_COLLIDER_HANDLE_TAG=0x11`(= 槽数,SubworldColliderHandle 空判别)✓,与
collision-eco/ALGORITHMS 三方锚定一致;测试覆盖。

### ② GAP 确认 — PASS
- `systems/collision/world.rs`(170 函数:业务 75/内联 36/胶水 54):src 确无 ✓。
- `systems/collision/intersect_meshes.rs`(21)、`visualize_colliders.rs`(6):src 亦缺 ✓。
- 仅 `systems/collision.rs`(8 函数)在库内 ✓。

### ③ inventory 分组 + 依赖 — PASS
world.rs 业务函数(RaycastMeshData::new_convex 0x1408cec10/Raycasting::{raycast 0x1408cc2e0,
spherecast 0x1408cc390,intersects_any 0x1408cc320}/SubworldColliderHandle::new 0x1408cc620/
RaycastWorld::{raycast 0x1408cc870,legacy_raycast 0x1408cc680,raycast_intersections_with_ray
0x1408cc8a0,mesh_intersect_collider_diff_publisher 0x1408cc670})与 inventory 一致 ✓。
`rapier3d` optional(`native-physics = ["dep:rapier3d"]`、`rapier3d = { workspace = true,
optional = true }`)已入 Cargo.toml,库内接线可在 native-physics 特征下实现 ✓。

### ④ 诚实预算判定 — PASS
170 函数 rapier 胶水(17 子世界槽 ray/sphere/shape 查询、dirty 瓦片生命周期
flush/publish/QueryPipeline::update、collider upsert/remove 泛型族、parry 网格构造)超本卡
诚实预算,登记 GAP 待碰撞组后续卡,未低质填充 ✓。与 walls/collision/decorator_collision
衔接待碰撞组闭合后补。

## 结论与 DISPATCH 建议

- 卡A5 复验通过:**核对+登记闭合**;world.rs 170/intersect_meshes 21/visualize_colliders 6
  GAP 归碰撞组后续卡。
- 建议:卡A5 → 闭合。本卡未改 src,编译状态不受影响(现全 lib 唯一 error 为
  audio_resources.rs,audio 组在途,不属本卡)。
