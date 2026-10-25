# Round5 C CONTRACT — apply 节→记录解析层（Walls / Decorators / Terrain）

> 逆向 agent ① 交付。证据：capstone InnerWalls::load_state（0x14085ad80）、DecoratorStorage::
> restore_snapshot（0x140a20d70）反编译、procs.tsv（FUN_1402ca2f0=HashMap::insert<WallId,()>）、
> crate inner_walls.rs/decorator_storage.rs 现状。
> 状态：**三节载荷形状/解析序定案可直接落地；逐字段解码待取证（中优先）；X4 待取证**。

---

## 1. Walls 0x78B 节 → InnerWallStateRecord（O 形状 + 待证字段）

**节字节解析**：
- 节载荷 = `{ count: u64@0, ptr: *const u8@8, … }`（load_state `*param_2`=count、`param_2[1]`=ptr）。
- 记录元素 **0x78B = 120B = 15 qword**；count 条连续。
- 逐记录：`FUN_1402ca2f0` = **`HashMap::insert<WallId, ()>`**（procs 权威）——**记录首 qword = WallId**，
  每记录插入"已见墙"集（uStack_78）。
- 聚合写 InnerWalls 目标 +0x34..+0x5c（WallChanges/状态字段区）；0x78B 全 15 qword 的
  **WallState 字段解码**（curve 点列/height/style/…）**待取证**（中优先，需拆 load_state 逐槽）。

**② 待办**：crate `InnerWallStateRecord(pub [u8; 0x78])` + `load_state` 骨架已就位（inner_walls.rs:31/97）——
补 0x78B 解码：record[0]=WallId → `HashMap::insert`；其余 qword → WallState 构造（待①字段定后）。

## 2. Decorators 0x28B 节 → DecoratorStorage 记录（O 形状 + 解析序）

**节字节解析**（restore_snapshot 0x140a20d70）：
- 节载荷 = `{ptr@+8, len@+0x10}`，元素 **0x28B = 40B**。
- 每 0x28B 记录 = `{状态数据 16B（ptr qword@0 + 值 qword@8）, 子计数 qword@0x10}`。
- 每记录 → `FUN_140a1cf30(storage, state_ptr, value)` 恢复一个装饰存储状态；
- 内层子记录 stride **0x16 = 22B**（`puVar27 + count*0x16`，含 +0x52 状态字节、`uVar9+1` id 计数）——
  为每记录携带的装饰条目（编码 id + 状态）。
- 前置：`decorator_to_storage`/`removed`/地形/楼梯桶空、+0x1b0 标志清。
- **解析序**：逐 0x28B 记录 → 恢复状态 → 逐 0x16B 子记录写入。

**crate 对照**：`DecoratorAddrSnapshot{encoded_id:u64, addr, state:u8}`（16B）是近似——
原生 0x28B/0x16B 节**结构不同**。② 把 JSON 节填充为 0x28B 记录（或保留近似+显式标注）；
原生记录逐字段（16B 内 ptr/值拆分、0x16B 子记录编码）**待取证**（中优先）。

## 3. Terrain strokes 字节布局 — **待取证**

- 节字节 = flate2 deflate 压缩（"decompressing terrain data" @0x142cdc800）→ 解压 → tg_terrain
  `TerrainEditingData{strokes, next_stroke_id, next_stroke_index}`。
- 解压后 strokes 的**序列化字节格式**（tg_terrain 外部依赖 serde/bincode 形态）**待取证**（中优先）；
  crate `TerrainEditingData(pub tg_terrain::stroke::TerrainEditingData)` 包装存在。
- ② 待办：flate2 解压 → tg_terrain 反序列化 → 写 TerrainEditingData；格式定后补。

## 4. X4 活羊 Query — **待取证**

- sync save 0x140a3f470（81KB，decomp-r5-0x140a3f470.txt）sheep 捕获段未拆——
  X4 Query 面（活羊 → SheepControlSnapshot）中优先待证；② 现 `sheep_section_serialized`
  接调用方传入的 `Option<&SheepControlSnapshot>`，X4 即其真实调用方。

## 5. 证据索引
- capstone 0x14085ad80（load_state 逐字段 + FUN_1402ca2f0 调用点）；procs.tsv 0x1402ca2f0
  = HashMap::insert<WallId,()>。
- wall/decomp/decomp-0x140a20d70.txt（restore_snapshot 543 行：0x28B/0x16B 循环）。
- session-save/ALGORITHMS.md §2.5（flate2/TerrainEditingData）。
- crate：inner_walls.rs:25-31/91-120（InnerWallStateRecord+load_state 骨架）、
  decorator_storage.rs:299-317（DecoratorAddrSnapshot 近似）、snapshot.rs:236-270（dispatch）。
