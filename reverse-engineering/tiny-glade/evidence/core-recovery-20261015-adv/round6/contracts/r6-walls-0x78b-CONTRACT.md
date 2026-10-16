# Round6 CONTRACT — Walls 0x78B 记录字段解码 + WallState 布局

> 逆向 agent ① 交付。证据：wall 章节（InnerWalls map 条目 stride 0x78 + _try_update_state 写偏移）、
> inner_wall_state.rs 现状、decomp-0x14085ad80（load_state）。状态：**记录骨架定案；WallState
> 内部曲线区待取证**。

---

## 1. 0x78B 记录 = {WallId u64@0, WallState 0x70B@8}（O）

- InnerWalls HashMap 条目 = `{WallId u64 key, WallState 0x70B value}`，步长 **0x78**（章节 §条目）。
- `InnerWallStateRecord([u8;0x78])` 即该条目：**qword[0] = WallId**（O：`FUN_1402ca2f0`
  = `HashMap::insert<WallId,()>` 键）、**qword[1..15] = WallState（0x70B）**。
- load_state（0x14085ad80）：逐 0x78B 记录 → WallId 插入已见集 → WallState 写回 InnerWalls map
  （+0x34..+0x5c 聚合字段区 = InnerWalls 头字段）。

## 2. WallState 字段（O，_try_update_state 闭包族写偏移）

| 偏移 | 宽 | 语义（O） |
|---|---|---|
| +0x54 | f32 | **height**（update_wall_state_height 0x14085ad60 → 0x140aaaca0 写 +0x54） |
| +0x60 | u8 | **freehand_type**（update_freehand_ty 0x140aa99e0 写 +0x60） |
| +0x64 | u8 | **arch_type**（update_arch_type 0x14085ad40 → 0x140aa9eb0 写 +0x64） |
| +0x67 | u8 | **removed_supports**（update_supports 0x140aa9800 写 +0x67） |
| 其余 | — | 曲线点列/其他 U（update_arch_height 写 f32 偏移待解） |

## 3. Walls 节 JSON 形状 — **待取证（中优先）**

- 快照 Walls 节 = 采集器 0x141308b40 `Vec<(WallId, WallState)>` 的 serde。
- JSON 值形态（结构化对象 vs 扁平 15 qword 数组）需拆 capture/validate 的 serde——**待取证**；
  ② 可先按"0x78B 记录 = {WallId, WallState 0x70B}"落地 load_state 的 parse（record[0] 解码 +
  WallState 0x70B 拷贝），JSON→0x78B 反序列化待证后补。

## 4. 可实现性
- **可直接落地**：record[0]=WallId 解码、WallState 0x70B 整体拷贝、+0x54/+0x60/+0x64/+0x67
  字段（若 ② 需逐字段写）。
- **待取证**：WallState 曲线区（+0x00..0x53 等）、JSON 形状。

## 5. 证据索引
- chapters/resources/walls/inner_walls.rs.md（map 条目 stride 0x78、_try_update_state 写偏移）。
- wall/decomp/decomp-0x14085ad80.txt（load_state 245 行）、0x140aa9eb0/99e0/9800/aca0（写闭包）。
- crate resources/walls/inner_wall_state.rs（WallState 现状）、inner_walls.rs:25-35/91-120。
