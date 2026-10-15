# systems/wall/merge_walls.rs

> 文件作用：墙合并系统：检测共线/相邻墙（地形采样），Curve::crashy_new_from_points 合并曲线，InnerWalls::new_wall；经 DecoratorArchivist::decorator_cull_allow_any 放行装饰归档重挂。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::merge_walls::merge_walls` | 0x140999920 | 0 | merge_walls（地形采样 + crashy_new_from_points + decorator_cull_allow_any） | O | wall\decomp\decomp-0x140999920.txt |

## 实现方式（算法级）（O，decomp-0x140999920）

**处理链**：12 参；事件 = param_7 EventReader，**行 40B** `{…, id_a u64@+8, id_b u64@+0x10, …, u32@+0x1c}`。
断言/panic：`assertion failed: ev_count < 2`（0x142abb086，行 0x1e=30，**每帧至多一个合并事件**）；
`id_a==id_b` → panic（0x142abb0f0，行 0x28）；缺墙 → panic_str（0x142abb130/190）。
ResMut/EventWriter（+0x1c，7 槽）：param_3/4/5/6/8/9/11。

1. **地形采样**：对 wall_a/b 各调 `FUN_14085aa60`（InnerWalls 方法）→ 各 0x28B 采样结构。
2. **曲线拼接**：分配 f32 缓冲，**反转一侧 + 追加另一侧**（连接点去重）；`FUN_141469430(&out, points, 0)`
   = **crashy_new_from_points**。
3. **新墙创建**：`new_id = *(InnerWalls+0xc0) + 1`（+0xc0 代计数 +1 即新 id）→ `FUN_141529bb0`
   （+0x30 工厂）→ `FUN_1402ca2f0(+0x30, new_id)`（wall_ids 插入）。
4. **装饰归档放行**：`FUN_140b2bc00`（decorator_cull_allow_any）×2（wall_a/b）。
5. **事件 ×3**：`FUN_1413e7230`（tag 串 `merge a`/`merge b`/`merge target` @0x142abb1c0/1c7/1ce）。
6. **装饰表合并**：`FUN_140a1ea40` = DecoratorStorage::on_walls_merged(a, b, new_id)。
7. **PendingMergeWalls 构建**：拼 **0x120B 结构**（两 0x28B 采样 + `wall_a@+0xf8, wall_b@+0x100,
   merged_id@+0x108, u32@+0x110/0x114/0x118`）；drop 失败 → panic
   `assertion failed: pending_wall_history.merge_walls.replace(PendingMergeWalls { adjust_height,
   wall_a: ev.wall_a, wall_b: ev.wall_b, wall_state_a, wall_state_b, merged_wall_id,
   merged_color_id: merged_wall_color, wall_color_a: ev.wall_a_original_color_id,
   wall_color_b: merged_wall_color, }).is_none()`（0x142abb1da，行 0x1a0）—— **揭示字段序与颜色字段**。
8. **地形高度**：`FUN_1409e9640`（TerrainHeightsData 采样，param_1 deref）于合并曲线**末点**。
9. **事件写**：param_8 EventWriter，**0x20B 行** `{u64@0, u8=4@8, {f32,f32}@0x10, f32@0x18}`（tag=4）。
