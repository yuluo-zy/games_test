# REVIEW2 — resources/critters/sheep 二轮复核（reviewer-eco，2026-10-15）

复核对象：restorer 二轮（RESTORE.md，serde 字段名 rne-read 定案 + capture/apply 闭合测试）。
首轮 待取证项（control.rs 快照 wire）现已有 re-analyst decomp 证据，逐条核毕。

## 逐项复核

### SheepMoveState serde — ✅ 全验证（首轮待取证闭合）

- decomp 0x1413ff520（serialize）：Idle=1 f32 字段；Walking=3 字段（f32,f32,bool），
  字段名依次 9/19/16 字符。
- RNE 直读字段名（rne-read 0x142c5b4d3/e3/f6）：**"timeout_s"(9)、"goal_change_timeout"(19)、
  "force_reach_goal"(16)**——逐一命中。
- 现实现 `Idle{timeout_s}` / `Walking{timeout_s,goal_change_timeout,force_reach_goal}`（serde 声明序
  = wire 序），与原生一致；测试断言 JSON 文本一致。OK。
- **"Walking 中间 f32 字段名待取证"已闭合**：= goal_change_timeout（rne-read 定案）。
  注：sheep.rs 注释所写"内存序 goal_change_timeout 在前、中间字段名待取证"是 re-analyst 的
  内存序描述，与 serde/wire 序无关；wire 序以 0x1413ff520+rne-read 为准（timeout_s 在前）。

### SheepSnapshot / SheepControlSnapshot serde — ✅ 全验证（首轮待取证闭合）

- decomp 0x1416f9eb0（SheepSnapshot::serialize）：`serialize_entry` 依次写
  **Vec3 → Option<Vec3> → SheepMoveState**（position→goal→move_state）。
- RNE 直读字段名（rne-read 0x142c91f49/51/55、0x142c91fa9）：**"position"/"goal"/"move_state"、
  "sheep"**。
- 现实现 `SheepControlSnapshot{sheep:Vec<SheepSnapshot>}`、
  `SheepSnapshot{position,goal,move_state}` 与原生一致。OK。

### capture/apply — ✅ 闭合（测试过）

- capture：按查询序（+0x1c pos/+0 goal/+0x10 move_state）收集，不存实体 id、不排序。OK。
- apply：zip 最短序覆盖三字段，保留 rotation/scale、不增删羊。OK。
- 实测 `cargo test --lib`：sheep 2 测试过（全库 1584 过/3 败，3 败在他组，与 eco 无关）。

### 接线状态 — 边界内已闭，边界外登记

- 现 crate 内**无消费方调用** capture/apply（save_game / sheep_controls 均不在 core 当前树）。
- 存档 Sheep 节接入 = save 组 save_game 消费端（state 组 in_progress）；Sheep 组件 owner /
  being_petted、startup_sheep FMOD 链 = 中央裁决边界外。登记为 remaining，不判本批缺失。

## 结论与 DISPATCH 建议

- **sheep = review_passed（限定：移动状态 + 快照 schema + 捕获/恢复闭合）**：
  首轮 待取证项（control.rs wire、中间字段名）已由 re-analyst decomp + RNE 直读全数闭合。
  remaining（Sheep owner/being_petted、startup_sheep、存档节接线）按中央裁决边界外，
  属后续/他组任务。
