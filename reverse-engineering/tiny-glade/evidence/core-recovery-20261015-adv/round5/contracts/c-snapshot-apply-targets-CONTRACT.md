# Round5 C CONTRACT — init_game_state_from_snapshot 逐域应用目标资源映射

> 逆向 agent ① 交付。证据：session-save/ALGORITHMS.md（快照节序 + 解析器）+ 反编译
> decomp-r5-0x1417a9c50（26KB）+ procs.tsv（apply 函数权威名）+ wall 域 decomp。
> 状态：**节序 + 2 域 apply 函数定案；其余域目标资源 I/待取证**。

---

## 1. 快照节序（O，validate_and_load_closest_snapshot_data 0x1417a6e50）

JSON 解析按节名分派（跳表 @0x142cdca80/98/b0），节序：
**Version → Walls → Roofs → Decorators → WallColors → GateColors → Terrain → Trees → Raster**
（另有 save 节 Camera(3)/TimeOfDay(4)/Sheep(5)/Colors(6)，见 B 契约节序）。
- Version 不符 → panic "Version mismatch; expected , got "（@0x142cdc840）。
- **Terrain 数据 flate2 deflate 解压**（"decompressing terrain data" @0x142cdc800）。
- Raster 键白名单外 → "Ignoring unknown raster data in snapshot: "（@0x142cdc7c0）。
- `init_game_state_from_snapshot`（0x1417a9c50）：load_game 拷贝 **0x1b0B 解析数据** → 逐域应用。

## 2. 逐域 apply（O 结构 + 2 函数定案）

| 域 | apply | 载荷形状（O） | 判定 |
|---|---|---|---|
| **Walls** | **`InnerWalls::load_state` 0x14085ad80**（procs 权威） | 逐墙记录 → InnerWalls 内部 | **可直接落地** |
| **Decorators** | **`DecoratorStorage::restore_snapshot` 0x140a20d70**（procs 权威；decomp 头 PDB 名 take_snapshot 错位） | DecoratorStorageSnapshot{states} | **可直接落地** |
| 0x58B 记录域 | 内联循环（kind==2 门、count@+0x50、+0x24/+0x30/+0x40/+0x48 字段）→ 0x40B 条目 Vec | Roofs/装饰记录候选 | 待取证 |
| 3× 0x18B 条目域 | 内联循环（{u64, u32, u8@0x14}，`FUN_140562db0` grow）→ Vec@+0x58/+0x70/+0x88 | WallColors/GateColors/楼梯候选 | 待取证 |
| 位图→0x30B 记录域 | 内联位图迭代（0x30B 步长）→ lStack_78 Vec | Raster/地形格候选 | 待取证 |
| Terrain | capture 侧 `TerrainEditingData{strokes, next_stroke_id, next_stroke_index}`（O） | 解压后写入 TerrainEditingData | **目标资源定案；写入序待证** |
| TimeOfDay/Sheep/Camera/Colors | save 节 4/5/3/6（B 契约） | TimeOfDayState/SheepControl/CameraRigParametersCache/GladeColorCustomization | **目标资源 I（协调人假设合理）；apply 序待证** |

## 3. 快照结构（O，decomp-r5-0x1417a9c50 读面）

- 头 0xc0B 整体拷贝 + **11 个域描述符** {ptr@X, len@X+0x10, count@X+0x1c}，域基偏移：
  0xc0 / 0xe0 / 0x100 / 0x120 / 0x140 / 0x160 / 0x180 / 0x1a0 / 0x1c0 / 0x1e0 / 0x240。
- 每域应用 = 读描述符 → 目标资源 Vec（param_1 内 +0x40..）`FUN_1405627b0/db0` grow →
  写条目 → count+1。
- 目标 Vec 布局（cap@+0x20 / ptr@+0x28 / len@+0x30 / count@+0x40）。

## 4. ② 待办（apply_loaded_snapshot 从"登记"变"真实写回"）
1. **可直接落地**：Walls → InnerWalls::load_state 形状；Decorators → restore_snapshot 形状；
   Terrain → TerrainEditingData（解压后写回）。
2. **待取证**：Roofs/WallColors/GateColors/Trees/Raster 域的目标资源与写入序（需拆
   init_game_state_from_snapshot 0x58B/0x18B/位图三循环的 param_1 偏移↔资源映射，中优先）。

## 5. 证据索引
- session-save/ALGORITHMS.md §2.4-2.6（capture_game_snapshot 0x1417a1d00 载荷、
  validate_and_load_closest_snapshot_data 0x1417a6e50 节序、init_game_state_from_snapshot 0x1b0B）。
- round5/decomp-r5-0x1417a9c50.txt（26KB 逐域循环）。
- procs.tsv：0x14085ad80 = InnerWalls::load_state、0x140a20d70 = DecoratorStorage::restore_snapshot。
- wall/decomp/decomp-0x14085ad80.txt、decomp-0x140a20d70.txt。
