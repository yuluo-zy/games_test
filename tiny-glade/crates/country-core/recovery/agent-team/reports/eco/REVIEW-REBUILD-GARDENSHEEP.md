# REVIEW-REBUILD-GARDENSHEEP — garden + sheep 重建复验（reviewer-eco，2026-10-17）

复核对象：rebuild-gardensheep（garden/REBUILD.md + sheep/REBUILD.md）。方法：与 fresh 反编译/
反汇编逐函数判等。实测：cargo check 过（10 既有 warning，0 error）；lib test 目标被既有
render/rhapsody GraphicsDevice 重构错误阻塞（非本批文件）。

## 逐项核对（协调者 6 点）

### 1. 分类 — ✅ 确认

4 真业务重建（record_garden_history / startup_garden_brush / capture / apply）；draw_garden /
being_petted **判等不动**（红线）；garden build 插件 + draw_garden 3 闭包 + SheepMoveState/
SheepSnapshot serde = 胶水不重建。✓

### 2. record_garden_history — ✅ 判等

fresh epilogue 0x14169601a→01f 无 RAX 测试、无失败事件写；`History::add` 失败走 panic_fmt
（无 Err 返回）。现实现**删除 `failures.write(HistoryRecordingFailed)` 参数与分支**，
`let _ = record_raster_history(...)` 丢弃 Result（匹配原生"调用即返回"）✓；garden/tests.rs 仍断言
该事件为空（胶水注册保留）。

### 3. startup_garden_brush — ✅ 判等

2 次迭代（load_json_as_mesh→load_mesh_into_library→load_shader→spawn_empty+insert），第 2 次
`insert<(DrawableMeshBundle, DontCastShadow)>`；**shader 路径纠偏**：
`/solid_vertex_color_garden_stone.raster.hlsl`（len44→程序名 solid_vertex_color_garden_stone）+
`/garden_lowpoly_flower.raster.hlsl`（len34→garden_lowpoly_flower）——无 garden_stone/ 前缀；
8 串 read_memory 实证（0x142c7d468…531）。经 GardenBrushHost 边界（同 startup_quad）✓。

### 4. capture/apply 组件绑定 — ✅ 判等

三元组改 `(&Transform, &Sheep, &CritterGoal)`：position←CritterGoal.position@+0x00、
goal←Sheep@+0x30 payload（Option tag 来自 CritterGoal.goal_tag@+0x0c）、
move_state←utils Transform.translation@+0x10（from_translation/to_translation）；
`CritterGoal { position: Vec3, goal_tag: u32 }` 显式 16B 布局 ✓。旧绑定（position←transform/
goal←CritterGoal/move_state←SheepMoveState 组件）错位已修 ✓。

### 5. Sheep 布局 + SheepMoveState↔Vec3 编码 — ✅ 判等

Sheep `{ goal: Vec3 @+0x30, being_petted_timer: f32 @+0x58 }`（+0x30 否定旧"内嵌
move_state"声明）；being_petted 逻辑逐位保留；SheepMoveState↔translation 12B 编码
（tag 0=Idle/1=Walking/0x101=Walking+force_reach_goal，a=timeout_s|goal_change_timeout、
b=0|timeout_s）bijection 测试过 ✓。+0x3c..0x57 待取证不造；完整 Sheep owner 仍 systems/critters
跨域边界外 ✓。

### 6. 2 处既有编译阻塞 — ✅ 修复确认

`building_history_transactions.rs` 补 `HistoryRecordingValidation` import（E0405 遮蔽）；
`world_raster.rs::record_raster_history` 补 `M: HistoryRecordingValidation` 泛型界
（HistoryTransactions::add 共享界）——均为既有重构阻塞的最小修复，非 garden/sheep 业务改动 ✓。

## 残留（诚实登记）

Sheep +0x3c..0x57 字段、完整 Sheep owner（systems/critters 跨域）、garden pin 渲染/scatter/
全局调度（render/外部）——均边界外。

## 结论与 DISPATCH 建议

- **garden+sheep 重建项 = 闭合**：4 真业务重建与 fresh 反编译判等；draw_garden/being_petted
  判等不动；2 处既有编译阻塞修复确认。
- 建议：REBUILD-GARDEN=review_passed（限定）；REBUILD-SHEEP=review_passed（限定）；残留
  （Sheep +0x3c..0x57 / 完整 owner / garden pin 渲染）登记不阻塞。
