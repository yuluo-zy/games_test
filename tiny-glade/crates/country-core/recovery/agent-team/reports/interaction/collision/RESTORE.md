# RESTORE — collision/world.rs（卡A5）

实现者: restorer-interaction
日期: 2026-10-16

## 核对结论

`systems/collision/world.rs`（170 函数，rapier3d 碰撞世界胶水）确无 src——库内仅
`systems/collision.rs`（已锚定常量：`SubworldIndex` 17 槽、Terrain=5/掩码 0x20、
`SubworldColliderHandle` 空判别 0x11）。intersect_meshes.rs / visualize_colliders.rs 亦缺。

## 现状核对

- `systems/collision.rs`（既有）：SubworldIndex 槽/位标志/Terrain 锚定。
- inventory 显示 `systems/collision/world.rs` 170 函数（业务 75 / 内联 36 / 胶水 54）：
  `RaycastMeshData::new_convex`（0x1408cec10）、`Raycasting::{raycast,spherecast,
  intersects_any}`（0x1408cc2e0/390/320）、`SubworldColliderHandle::new`（0x1408cc620）、
  `RaycastWorld::{raycast, legacy_raycast, raycast_intersections_with_ray,
  mesh_intersect_collider_diff_publisher}`（0x1408cc870/680/8a0/670）等。
- 依赖 rapier3d（optional feature native-physics 已入 Cargo.toml）——库内接线可在
  `native-physics` 特征下实现。

## 登记（待后续卡闭合）

170 函数 rapier 胶水（17 子世界槽 ray/sphere/shape 查询、dirty 瓦片生命周期
flush/publish/QueryPipeline::update、collider upsert/remove 泛型族、parry 网格构造）
超出本卡诚实预算，登记为 GAP。与 walls/collision/decorator_collision 的衔接待碰撞组
闭合后补。编译基线 0 error（本卡未改 src）。

## 二. R1 核心查询族实现（coordinator 续派，2026-10-16）

`systems/collision/world.rs` 新建（`#[cfg(feature="native-physics")]` rapier 查询 + 默认壳）：
- `SubworldColliderHandle::new`（0x1408cc620，{index u32, gen u32, slot u8}，24 B，空判别 0x11）。
- `RaycastMeshData::new_convex`（0x1408cec10：parry convex_hull，tag=4 + Box/vtable/hash）。
- `RaycastWorld`（17 槽 ColliderSet/QueryPipeline/RigidBodySet）+ `raycast`
  （0x1408cc870，mask 遍历 + cast_ray_and_get_normal 最近命中）+ `legacy_raycast` +
  `mesh_intersect_collider_diff_publisher`。
- `Raycasting::{raycast, spherecast, intersects_any}`（raycast 已落地；spherecast 桩待
  PounceLight rapier fork 的 QueryPipeline 球体 API 取证）。

## 四. R1 终验 FIX（reviewer REVIEW2，2026-10-16）

`intersects_any`（0x1408cc320，旧包装，掩码=0/语义待核）已补实现：native-physics 下
`raycast(...).is_some()`，非 native 壳返回 false。双构建 exit 0 复验。其余全过。

- **登记待续**：dirty 瓦片生命周期（flush/publish/QueryPipeline::update）、collider
  upsert/remove 泛型族、parry 网格构造、intersect_meshes(21)/visualize_colliders(6)。

## 三. E7 EditCommand ABI 对齐（re-analyst RE-EDITCMD，登记待做）

原 `EditTerrainStrokeCmd` = 0x28 B（40 B），字段序 **{falloff, height, position, id}**，
id@+0x1c 硬锚（生产者 0x140aef1f0 + 消费者 0x14094e000 fresh 双侧一致）；`Adjustment`
紧凑打包 {disc u32, [value f32 仅非 Keep]}（仅 falloff→position@0xc、仅 height→@0x10，
非 tg-terrain 8B×2 固定）；falloff.value 按 0.002 帧灵敏度；+0x20 u64 = 事件 id/计数。
**对齐建议**：按原字节序调整 tg_terrain 共享 crate 声明 + 加往返测试。未决：position y/z
语义名/+0x20 与 EventInstance id 关系/SetTerrainStrokePtsCmd 完整布局（U）。**登记待做**：
涉及共享 crate 改动，需先核现有消费者（edit_terrain_stroke）再改。

## 留给 reviewer 的核对点

1. collision.rs 既有常量锚定（SubworldIndex 17 槽/Terrain=5）是否正确。
2. world.rs GAP 登记与碰撞组台账一致。
