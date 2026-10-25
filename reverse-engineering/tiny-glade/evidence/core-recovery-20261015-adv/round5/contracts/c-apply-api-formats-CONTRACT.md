# Round5 C CONTRACT — apply API 数据格式（InnerWalls / DecoratorStorage / Terrain）

> 逆向 agent ① 交付。证据：wall/decomp（InnerWalls::load_state 245 行、DecoratorStorage::restore_snapshot
> 543 行）+ session-save ALGORITHMS（flate2/TerrainEditingData）+ crate 现状。
> 状态：**三 API 签名+载荷形状定案可直接落地；逐字段布局部分待取证；剩余 5 域待取证**。

---

## 1. InnerWalls::load_state（0x14085ad80）

- **签名**：`(param_1 = &mut InnerWalls, param_2 = &[InnerWallState; stride 0x78])`——
  `*param_2` = 记录数、`param_2[1]` = 记录数组 ptr、元素 **0x78B（120B）**（`param_2[1]` 迭代 `lVar16*0xf`）。
- **载荷形状**：墙内状态记录 Vec，每记录 0x78B；逐记录经 `FUN_1402ca2f0(rec)` 转换写入 InnerWalls 内部
  （+0x34..+0x5c 字段区，含 +0x48/+0x4c 计数、+0x50/+0x54 双 qword 状态）。
- 前置：`param_1` 各内层 Vec 空（param_1[3]/[0xf]/[0x15] 空判）。
- **② 待办**：crate `InnerWalls` 缺 load_state——按 0x78B 记录实现
  `InnerWalls::load_state(&mut self, records: &[…])`；逐字段名（+0x34..+0x5c 各槽语义）**待取证**（中优先）。

## 2. DecoratorStorage::restore_snapshot（0x140a20d70）

- **签名**：`(param_1 = &mut DecoratorStorage, param_2 = snapshot: &[记录 stride 0x28], param_3 = &mut u32 count, param_4)`——
  `param_2` = {ptr@+8, len@+0x10}，元素 **0x28B（40B）**。
- **载荷形状**：每 0x28B 记录 = `{状态数据 16B（ptr + 值）, 子计数 8B}`；内层子记录 stride **0x16（22B）**
  （`puVar27 + count*0x16` 迭代，`uVar9+1` 计 +0x52 状态字节）。逐装饰 `FUN_140a1cf30` 恢复。
- 前置：`decorator_to_storage`/`removed`/地形/楼梯桶空（`param_1` +0x48/+0x78/+0x90/+0x120 空判、
  +0x1b0 不可变标志清）。
- **crate 对照**：`DecoratorStorageSnapshot{states: Vec<DecoratorAddrSnapshot{encoded_id,addr,state}>}`
  （decorator_storage.rs:312）是**近似模型**（16B 条目），与原生 0x28B/0x16B 记录**格式不符**——
  snapshot.rs 已登记"非 serde 接线"。
- **② 待办**：把 crate restore_snapshot 对齐原生 0x28B 节格式（或保留近似 + 显式标注）；
  原生记录各字段（0x28B 内 ptr/值/子计数、0x16B 子记录）**待取证**（中优先）。
- **可直接落地**：签名 + 空桶前置 + 逐装饰恢复序（crate take/restore 已存在）。

## 3. Terrain 数据（flate2 @0x142cdc800）

- 快照 Terrain 节 = **flate2 deflate 压缩字节**（"decompressing terrain data" @0x142cdc800）→ 解压 →
  `TerrainEditingData{strokes, next_stroke_id, next_stroke_index}`（capture 侧 0x1417a1d00 载荷 O）。
- crate `TerrainEditingData(pub tg_terrain::stroke::TerrainEditingData)`（terrain_editing.rs:8，包装外部
  tg_terrain 依赖，不重实现）。
- **② 待办**：flate2 解压 → 按 tg_terrain stroke 反序列化写入 TerrainEditingData 资源。
- **待取证**：解压后 strokes 的序列化字节布局（tg_terrain 的 serde/二进制格式，中优先）。

## 4. 剩余 5 域（Roofs/WallColors/GateColors/Trees/Raster）— **待取证**

- 无 apply 函数解码（init_game_state_from_snapshot 内联循环对应域未逐指令映射）；
  snapshot.rs apply_loaded_snapshot 对 `_` 域仅 log。② 保持登记不伪造写入。
- 目标资源 I：Roofs→Roof 存储、WallColors/GateColors→颜色资源、Trees→TreeStorage、
  Raster→WorldRaster（白名单）。**需再取证**（中优先）。

## 5. 证据索引
- wall/decomp/decomp-0x14085ad80.txt（load_state 245 行）、decomp-0x140a20d70.txt
  （restore_snapshot 543 行）。
- session-save/ALGORITHMS.md §2.5（capture_game_snapshot 载荷、flate2、节序）。
- crate：resources/walls/decorator_storage.rs:299-380（DecoratorAddrSnapshot/DecoratorStorageSnapshot
  近似 + take/restore）、resources/terrain_editing.rs（TerrainEditingData 包装）、
  systems/save/snapshot.rs:236-270（apply dispatch 现状）。
