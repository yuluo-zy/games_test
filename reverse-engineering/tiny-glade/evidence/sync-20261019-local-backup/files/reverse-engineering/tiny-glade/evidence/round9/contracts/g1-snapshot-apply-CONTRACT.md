# G1 读档应用字节级契约（Round 9 ① 逆向取证）

> 取证日期：2026-10-19。只读取证，未修改任何源码。
> 证据基线：B patched `workspace/input/tiny-glade.exe`（Ghidra 只读会话 g1_r9..g1_r9h）、
> 36 个真实发行快照 `D:/game/ljxsj_92385/Tiny Glade/assets/starting-builds/{demo,full}/*/snapshots/*.snapshot`
> （JSON 头格式 `"Snapshot"`+u32 len+4B pad，JSON 起点 +0x10）。
> 新反编译落盘：`evidence/round9/tools/init-1417a9c50.c`、`dec-0x140a20d70.c`、
> `dec-0x14085ad80.c`、`dec-0x1404f4c30.c`、`dec-0x1417c0960.c`、`dec-0x1417c0cf0.c`。
> **并发状态**：取证期间另一 agent 正在改 `crates/country-core/src/systems/save/{snapshot.rs,load_game.rs}`
> （2026-10-19 11:39 mtime）。本契约按“当前工作树 + 原版机器码”双基线写；凡当前树已改动的
> 位置，同时给出原版证据，避免后续实现以旧行号为准。

## 0. 结论先行（三处需要纠偏）

1. **不存在 0x70B 曲线内联打包**。原版 `InnerWalls::load_state` 0x14085ad80 接收的是
   `Vec<(WallId, WallState)>`（记录 stride **0x78 = 8 + 0x70**），`WallState` 内 `Curve2`
   的点列是 Rust `Vec` 堆缓冲区；点数据在记录**之外**。真实样本单墙最多 **104 点**
   （`3445180338517407321.snapshot` wall 1，length 20.7149），无论按 2D 8B/点还是 3D 12B/点
   都远大于 0x70，故不可能是定长内联区。读档应走“语义 WallState 直接插入”，不是字节打包。
   **置信度 O（stride/调用面）+ I（Vec 头字段顺序）**。
2. **`_UNK_1429cd080/_UNK_1429cd090` 不是 ID XOR 掩码**。restore 0x140a20d70 在函数入口把
   32B 常量装入两个 XMM（`auVar6/auVar7`），随后与 `map+0x20` 的 16B key 异或并做 aHash 风格
   混合，最后交给 `FUN_140b776d0(map, hash, &id)`。**记录里的 DecoratorId 以 u32 明文读出并直接
   使用**（`uVar9 = *puVar27`），没有 rotate/XOR 解码步骤。r6 契约的“32 位旋转 XOR 16B 掩码”
   被本轮反编译证伪。置信度 O。
3. **不存在 0x16B 子记录**。restore 内层 stride 是 `0x16` 个 **u32** = **0x58B**；r6 把
   `0x16` 当成了字节。正确 inner 记录 = `{DecoratorId u32@+0, pad4, DecoratorHistoryState 0x50@+8}`。
   外层桶条目 stride 0x28。置信度 O。

---

## 1. 为什么曾 bail；原版 0x1417a9c50 的调用时序与接入点

### 1.1 旧 bail 的语义（已从当前工作树移除）

旧代码 `require_no_valid_snapshot`（`systems/save/load_game.rs`）在
`std::fs::File::open(path)` 成功时直接
`anyhow::bail!("B snapshot {} opened; original motomoto chunk/domain application @1417a9c50 is not restored")`。
理由（I，由原版语义反推）：原版把“可打开的最近快照”视为**权威数据源**，一旦打开成功就
不得回退到历史链 cursor-1 分支（`@14146dfa6` 是“无有效快照”分支）；当时 crate 既没有
motomoto chunk 装配，也没有域写回，只能显式失败，不能伪装成无快照继续加载。
当前工作树 11:39 版本已改为：打开成功 → `load_snapshot_file(path)` →
`LoadedSnapshotData`；解析失败 fatal（不静默回退）。见 `load_game.rs:246`
（`load_closest_valid_snapshot`）、`:1016`（选择快照）、`:1035`
（`init_game_state_from_snapshot(world, &resources)`）。置信度 O（当前源码）+ I（旧 bail 动机）。

### 1.2 原版调用时序（O）

`init_game_state_from_snapshot`（0x1417a9c50）**没有直接 call 调用者**：
`get_xrefs_to 0x1417a9c50` 仅返回 1 条 `DATA @0x14371898C`，该槽位是 4 字节
小端 RVA `50 9c 7a 01` = `0x017a9c50`。相邻静态表同型条目：
`0x143718374 → 0x1417a0430`（capture_game_snapshot_save_data）、
`0x143718464 → 0x1417a1d00`（capture_game_snapshot）、
`0x1437187DC → 0x1417a6e50`（validate_and_load_closest_snapshot_data）。
即这些函数都是按 Bevy system 描述符表间接调度的系统，而非普通函数调用。
置信度 O（xref + 表字节）。

原版签名（fresh 反编译 `init-1417a9c50.c`）：

```
void FUN_1417a9c50(longlong *snapshot_save_data, undefined8 inner_walls, longlong raw_chunk)
```

- `param_1` = 内存 `SnapshotSaveData`：Walls `Vec<(WallId,WallState)>` 在 `[5..8]`，
  Roofs 在 `[8..0xb]`，DecoratorStorage 在 `+0xb`（地址传入 restore），
  Wall/Gate/Halftimber 颜色 pair Vec 在 `[0xe..0x11]/[0x11..0x14]/[0x14..0x17]`，
  Trees/其它记录在后续槽位，Raster `[0x18],[0x19]`，Stairs `+0x29` 区。
- `param_2` = `InnerWalls` 目标（`FUN_14085ad80(param_2, &wall_vec)`）。
- `param_3` = 原始 chunk：先 `memcpy(header, param_3, 0xc0)`，随后读 14 个
  **0x20B descriptor**（首址 +0xc0，末址 +0x260）：`data ptr@+0`、`@+8`、`target@+0x10`、
  `count u32@+0x1c`。descriptor 内各域名字 **U**；crate 的 `SnapshotPrefix` 已用同一
  三偏移解析，但文件路径实际改用 `LoadedSnapshotData`，不再解引用这些指针。

原版 apply 顺序（按 `init-1417a9c50.c` 行号，O）：Walls(221) → Roofs(222..284) →
Decorators(288) → WallColors(289..315) → GateColors(317..343) → HalftimberColors(345..371) →
位图/记录循环(373..465；AutoClutter / TerrainHeightRange 归属 **U**) → Terrain 结构(466..517) →
Raster(518) → Terrain 高度(551..556) → Trees 0x40 源记录循环(560..640) → Stairs(678)。
（原版把部分内存写回交错在同一函数内；JSON 解析侧的节序仍是
Version→Walls→Roofs→Decorators→WallColors→GateColors→Halftimber→Terrain→
TerrainHeightRange→Trees→Raster→Stairs，见 `snapshot.rs:114 SNAPSHOT_SECTION_ORDER`。）

### 1.3 触发条件与“最近的接入点”

原版条件（O/I 合成）：
- `AppMode` tag == 14（LoadGameData）才运行 load_game 系统（当前 `try_load_game_system` 同判据）。
- `NextSessionKind` 解码后 sel==1（`parse_saved_game_data` 路径）或 `LoadGameSource::Parsed`
  才拥有快照根目录；sel==2 只回放 on_load 命令。
- 按 `nearest_snapshot_index(chain_index)` 从最近记录开始；文件打不开→warning 并删除该记录
  继续下一条；**文件打得开但解析失败 = fatal**。
- 完全没有可用记录 → `apply_history_without_snapshot`（原 `@14146dfa6`）。

当前 crate 接入点：
- 调用：`load_game.rs:1032-1035`，`init_game_state_from_snapshot(world, &SnapshotLoadResources{loaded})`。
- 实现：`snapshot.rs:1294`（World 级入口）→ `apply_loaded_snapshot_to_world`（`:1470`）。
- 旧的纯资源版 `apply_loaded_snapshot`（无 World）仍保留（`:1349` 附近），只覆盖
  Walls/Decorators/Terrain/Raster；生产路径应使用 World 版。

---

## 2. Roofs 节

### 2.1 真实 JSON schema（O，73/36 快照非空）

样本：`.../starting-builds/full/28-wizard-tower/snapshots/3327348694909461180.snapshot`（Roofs[0]）；
圆形样本：`.../full/06-blue-tower/snapshots/17798710246010479337.snapshot`（Roofs[0]）。

```json
{"id": 30, "color_id": 0, "gable_style": "Stepped",
 "flat_roof_style": "MatchingStoneOnly",
 "params": {"eave_length_01": 0.2, "flat_rim_style": "Rim", "height_01": 0.3189,
            "profile_01": 0.5, "ridge_dir": "Length", "ridge_length_01": 0.0,
            "tip_offset_01": [0.0, 0.0]},
 "pivot_y": 14.2517,
 "wall_shape": {"Rectangle": {"center": [0.9609,-1.3090], "length": 5.2380,
                              "orientation": [0.9565,0.2917], "width": 6.0007}}}
// Circle: {"Circle":{"center":[5.9210,-0.7797],"radius":2.46236}}   （rotation 可省略=0）
```

### 2.2 serde → 88B `Roof` 组件（O：`roof_shape.rs:341 Serialize` / `:447 Deserialize`）

| JSON | 目标偏移 | 类型/编码 | 置信度 |
|---|---|---|---|
| `id` | +0x00 | u64（WallId；样本与 Walls 节 id 同域） | O |
| `wall_shape` | +0x08..0x24 | 28B `ShapeParameters`：tag u32@0（0=Circle/1=Rectangle）；Circle center@4/8, radius@0xc, rotation@0x10；Rectangle axis@4/8, center@0xc/0x10, size(w,l)@0x14/0x18 | O |
| `params` | +0x24..0x40 | tip_offset_01[0]@0x24、[1]@0x28、profile_01@0x2c、height_01@0x30、ridge_length_01@0x34、eave_length_01@0x38、ridge_dir u8@0x3c（0=Width/1=Length）、flat_rim_style u8@0x3d（0=Rim/1=NoRim） | O |
| `flat_roof_style` | +0x40..0x4c | 12B：tag@0（0=Matching/1=MatchingStoneOnly/2=Override）、material@1（0=Stone/1=Wood，仅 Override）、color_id u32@4（仅 Override）、pattern_id u32@8 | O |
| `pivot_y` | +0x4c | f32（`Roof::floor_y`） | O |
| `color_id` | +0x50 | u32 | O |
| `gable_style` | +0x54 | u8（0=Stepped/1=Overhang；缺省 Stepped） | O |
| — | +0x55..0x57 | padding U | U |

### 2.3 apply 目标

原版：遍历 `Vec<Roof>`（stride 0x58），`if *(int*)(record+8)==2 break;` 否则把
wall_id/color/flat_style/params 投影成 **0x40B** 条目 push 到目标 Vec
（`FUN_1405627b0` 扩容；`init-1417a9c50.c:222-284`）。0x40 条目逐槽语义 **U**
（不含完整 shape/pivot，可能是 spawn/association 中间命令）。
crate 现状：`world.spawn(roof.clone())`（`snapshot.rs:1524`），仅产生带 `Roof` 组件的实体，
**未重建 `RoofAssociations`**。建议 apply = spawn + 用 `Roof::wall_id()` 更新关联表；
完成实体身份后再按 `RoofAssociations::rebuild` 契约接。置信度 I（spawn 等价）+ U（0x40 条目逐槽）。

样本限制：36 个样本共 73 个 Roof，shape 仅见 Rectangle/Circle（tag 0/1），未观察 tag 2。

---

## 3. WallColors / GateColors / Trees

### 3.1 三张颜色表（Wall/Gate/Halftimber）

真实 JSON：`[[index, color_id], …]`，只含非默认项（capture 侧 `iter_nondefault`）。
样本：`.../full/06-blue-tower/.../17798710246010479337.snapshot` → `WallColors=[[1,8],[2,8]]`；
`.../full/.../5476761123159047511.snapshot` → `HalftimberColors=[[6,2],[8,2]]`。
统计：WallColors 431 行 / 33 个快照；HalftimberColors 20 行 / 10 个快照；
**GateColors 在 36 个样本中全部为空数组**（0 行）。

原版（O）：`init-1417a9c50.c:289-315 / 317-343 / 345-371` 是**三段逐字相同的循环**，
源 pair 16B（u64 index + u32 color + pad），目标条目 0x18B `{+0 u64 世代, +8 u64 index,
+0x10 u32 color, +0x14 u8 0}` push 到三张表各自的 Vec。三表处理完全同构，故
GateColors 形状 = WallColors 形状（O，代码同构；取值面样本缺失，标 I）。

crate 目标：`resources/color_id.rs` 的 `WallColorIds`(:34)/`GateColorIds`(:60)/
`HalftimberColorIds`(:86)，均为 `Vec<u32>`（len 20480，缺省 0）。写回：
`write_color_table`/`apply_color_rows`（`snapshot.rs:1815`）把 `rows[index]=color`。
这与原版“稀疏 set”语义等价（I）；越界当前报错（O 代码行为，是否等同原版断言 U）。

### 3.2 Trees

真实 JSON：`[[index, {id, position:[x,y,z], rotation_degrees, scale,
prefab_name?, is_preplaced?}], …]`；`index` 是 **u64 TreeStorage 键**（样本可见 100001），
`id` 是 **i32 放置变体**（与 index 不同）。样本：
`.../full/15-lake-house/.../16597920645917222749.snapshot` →
`[100001, {"id":52,"is_preplaced":true,"position":[-13.7418,0.3045,27.4706],
"prefab_name":"clearing/tree/mid1","rotation_degrees":110.333,"scale":1.0673}]`。
统计：590 条 / 11 个快照；其中 **50 条（8 个文件）完全缺 `prefab_name` 与 `is_preplaced`**
（例如 `13765819955734695850.snapshot` 全部 3 条），所以原版 Deserialize 必须对这两个字段
取缺省（I，样本驱动）：`prefab_name=""`、`is_preplaced=false`。
`TreeEditData`（`resources/history/tree_edit.rs:26`）当前无 `#[serde(default)]`，
会导致这些快照整节解析失败——**必须补 default**。

目标映射（crate 现有运行时 owner，I）：
| JSON | 目标 |
|---|---|
| `index` | `TreeStorage` 键 / `TreeComponent.id: u64` |
| `id` | `SpawnPrefabCmd.variant: u32` |
| `position` | 实体位置（若有）与 `TreeMoveBend{position.xz}`、`SpawnPrefabCmd.position` |
| `rotation_degrees` | `SpawnPrefabCmd.rotation` |
| `scale` | `SpawnPrefabCmd.scale` |
| `prefab_name` | `PrefabName` → `SpawnPrefabCmd.prefab_name` |
| `is_preplaced` | `TreeComponent.is_preplaced` |
| （快照无生长进度） | `TreeAnimation::preplaced()`（0.75/1/1/false） |

crate 现状：`snapshot.rs:1663` 已按上表 spawn 并写 `TreeStorage`；缺 default 是唯一硬缺口。
原版 init 的树循环（`init-1417a9c50.c:560-640`）源 stride 0x40、目标条目 0x48B，逐槽 **U**；
`prefab_name` 为空时的缺失 prefab 回退策略 **U**（不得编造 prefab）。

---

## 4. Walls 记录 0x70B “曲线区”：结论与偏移表

### 4.1 原版真实的记录形态（O）

- 记录 stride：`FUN_14085ad80` 迭代 `0x78` 字节；`param_1[5..8]` 是
  `Vec<(WallId, WallState)>` 的 ptr/len/cap，按值移入 load_state
  （`dec-0x14085ad80.c:61-100,221`）。因此 `0x78 = sizeof((WallId u64, WallState 0x70))`。
- `load_state` 只做：3 个 empty 断言 → 空记录直接返回 → 逐记录把 WallId 插
  `changes.created` → HashMap 重建 → `max_id = max(WallId)`。它**从不读取曲线点字节**。
- 结论：`WallState` 的 Curve2 是堆对象（`Vec` 头 + 外部缓冲）。文件 JSON 中的
  `curve.points` 反序列化成同一 `Vec`，**不需要任何 0x70 内联打包**。
  置信度：stride/调用面 O；Vec 头字段顺序 I（依据 crate 同构 `Curve2` 布局与
  `from_record_bytes` 的 O 偏移）。

### 4.2 WallState 0x70 偏移表

| 偏移 | 字段 | 类型 | 置信度 |
|---|---|---|---|
| +0x00 | Curve2.points Vec（ptr,len,cap） | 3×u64，堆外 `[f32;2]` | I（头）/ O（堆外） |
| +0x18 | Curve2.points_u Vec（ptr,len,cap） | 3×u64，堆外 f32 | I（头）/ O（堆外） |
| +0x30 | 未解释 4B（padding 或 curve 辅助标量） | U | U |
| +0x34 | Curve2.length | f32（缓存值，不重算） | O |
| +0x38 | shape | 0x1cB（tag u32@0） | O |
| +0x54 | height | f32 | O |
| +0x58 | terrain_floor | f32 | O |
| +0x5c | arch/observed scalar | f32 | O |
| +0x60 | plaster bits | u32 | O |
| +0x64 | arch_ty | u8 | O |
| +0x65 | structure_ty / clutter_version | u8 | O |
| +0x66 | freehand_ty | u8 | O |
| +0x67 | removed_supports | u8 | O |
| +0x68 | style | u8 | O |
| +0x69 | metadata byte | u8 | O |
| +0x6a..0x6f | padding | U | U |

### 4.3 样本证据与“样本受限”说明

- 36 快照共 162 条 Wall 记录、1870 个点；点数分布 2..104，`points.len()==points_u.len()`
  恒成立；极值 `3445180338517407321.snapshot` wall 1 = 104 点 / length 20.7149。
- 所有 1870 个 `points[i][1] == 0.0`（曲线以 [x,0,z] 写出，见 `SavedCurve3`：
  `curve2_serialized_as_curve3`，`saved_wall_state.rs`）；`points_u` 首 0/末 1（162/162）。
  置信度 O。
- **没有缺失的“打包算法”**；当前 crate 的 `encode_wall_record`/`from_record_bytes`
  只保留 length、丢点列，是本轮唯一需要替换的 Walls 实现点。
  若仍要走字节记录路径，则“样本受限”的不是点列格式，而是 0x70 内 +0x00..0x33 的
  Vec 头字段逐槽命名（U）——但读档不需要它。

---

## 5. Decorators：0x28B 外层 / 0x58B 内层 / ID 常量

### 5.1 原版记录形状（O，纠正 r6）

`restore_snapshot` 0x140a20d70：
- 外层：`pauVar19 + i*0x28`，即 **0x28B** = `{storage_disc u64@0, storage_payload u64@8,
  桶 Vec 头 @0x10/0x18/0x20}`。`plVar17=*(u64*)record`、payload=`*(u64*)(record+8)`；
  桶长度读 `pauVar19[2]`（+0x20），条目指针经 `pauVar19[1]+8`。
- 内层：`puVar27 + n*0x16`（`uint*`），即 **0x58B** = `{DecoratorId u32@0, pad4,
  DecoratorHistoryState 0x50@+8}`。**不是 0x16B**。
- 对齐证明：`ty` 读 `*(byte*)(record+0x52)` → state+0x4a；`style` 读 +0x56 → state+0x4e；
  `rank` = `(byte)record[0x50]` → state+0x48；`u4c`=record[0x54]；`u4d`=record[0x55]；
  `u30`=record[0x38]；`u34`=record[0x3c]；`u38`=record[0x40]；`u3c`=record[0x44]；
  `u28`=record[0x30]；`dst_wall/secondary/payload/u18/u20`=record+0x08/0x10/0x18/0x20/0x28。
  与 `decorator_edit.rs:53 DecoratorHistoryState` + `:309 to_bytes` 完全一致。置信度 O。

### 5.2 JSON ↔ state 偏移（O，serializer 0x1404f4c30 / 0x1417c0960）

`DecoratorHistoryState::serialize` 原版逐字段（键串常量见
`0x142ce172d..0x142ce17d5`，含 `"DecoratorHistoryState"` 前缀）：

| JSON 字段 | state 偏移 | 类型 | 写出条件 | 置信度 |
|---|---|---|---|---|
| `dst`（Wall/Roof/Terrain/Stair） | 0x00..0x24 | 见 §5.3 | 始终 | O |
| `ty` | +0x4a | u8（0x38..0x3f） | 始终 | O |
| `rank` | +0x48 | u8（`{"x":…}`） | 始终 | O |
| `color_id` | +0x28 | u32 | ≠0 | O |
| `seed` | **+0x2c** | u32 | **始终** | O |
| `clv` | +0x49 | u8 | ≠0 | O |
| `h_offset` | +0x30 | f32 | ≠0 或 NaN | O |
| `allow_connection` | +0x4c | bool | 仅 false | O |
| `window_light_color` | +0x34 | u32 | ≠0 | O |
| `lantern_brightness` | +0x38 | f32 | ≠1.0 或 NaN | O |
| `chimney_smoke` | +0x4d | bool | 仅 false | O |
| `stair_width` | +0x3c | f32 | ≠1.0 或 NaN | O |
| `ladder_color` | +0x40 | u32 | ≠0 | O |
| `railing` | +0x4b | u8：0 无/1 Low/2 High/3 Wooden | ≠0 | O |
| `stairs_supports` | +0x4e | u8：0/1（1→"None"） | ≠0 | O |
| `magic_color` | +0x44 | u32 | ≠0 | O |
| `chimney_h_offset` | **U** | 36 样本恒 0.0 | 始终 | U（见 §5.5） |
| +0x24 | 未写出 | u32 | — | U |
| +0x4f | padding | — | — | U |

**关键纠偏（O）**：`seed` 写的是 state+0x2c，**不是桶键**。真实样本有 5 条
`seed != pair-id`（如 `4045235744475252327.snapshot` wall 23：key 9 / seed 18；
`16386978011644581742.snapshot` wall 16：key 2/4/6，seed 2/5/8）。因此当前
`DecoratorInfoJson::to_history_state` 用 `id << 32` 写 +0x2c 是错的，必须改为
`u64::from(color_id) | (u64::from(self.seed) << 32)`；桶键（pair 第一元素）只用于
restore 的 map key / `DecoratorIdGen.reserve`。
`DecoratorInfo` 文档把 +0x2c 命名为 `decorator_id` 属于旧命名；语义上是 **seed**（I，命名层面）。

### 5.3 dst 编码（O，`decorator_history_state_json` + `to_info` 交叉）

| dst | tag | JSON | state 字节 |
|---|---|---|---|
| Wall | 0（side="Inner"）/1（其它） | `{"Wall":{"anchor":{anchor_ws,corner,side?,platform?,is_bottom_door?,wall_height_relative_y?},"coord":{x,y}}}` | coord x@0x00/y@0x04；platform u64@0x08；wall_height_relative_y f32@0x10；anchor_ws[0..2]@0x14/0x18/0x1c；corner@0x20；is_bottom_door@0x21；tag@0x22 |
| Roof | 2 | `{"Roof":{"mesh_uv":[u,v],"relative_yaw_radians"}}` | uv@0x00/0x04；yaw@0x08 |
| Terrain | 3 | `{"Terrain":{"pos_xz":[x,z],"yaw_radians"}}` | pos@0x00/0x04；yaw@0x08 |
| Stair | 4 | `{"Stair":{"graph_edge":[a,b],"uv":[u,v],"yaw_radians"}}` | edge@0x00/0x04；uv@0x08/0x0c；yaw@0x10 |

### 5.4 `_UNK_1429cd080` / `_UNK_1429cd090` 实值（O）

rne-read（`python D:/game/reverse-engineering/tiny-glade/scripts/rne-read.py --hex 0x1429cd080 32`）：

```
0x1429cd080: 75 65 73 70 65 6d 6f 73 61 72 65 6e 65 67 79 6c
0x1429cd090: 6d 6f 64 6e 61 72 6f 64 73 65 74 79 62 64 65 74
```

按 u64 LE：

| 地址 | u64 | 按字节读（大端显示 ASCII） |
|---|---|---|
| 0x1429cd080 | 0x736f6d6570736575 | `somepseu` |
| 0x1429cd088 | 0x6c7967656e657261 | `lygenera` |
| 0x1429cd090 | 0x646f72616e646f6d | `dorandom` |
| 0x1429cd098 | 0x7465646279746573 | `tedbytes` |

即 4 个 8B 词按字节反转后拼出 `somepseu|lygenera|dorandom|tedbytes`，
等价于 `"somepseudorandomlygeneratedbytes"` 的中两词交换/词内字节反转。
用途（O，`dec-0x140a20d70.c:113-114, 280-313`）：装入 `auVar6/auVar7` →
与 `map+0x20` 的 16B key 异或 → aHash 风格 mix → `FUN_140b776d0(map, hash, id)`。
故它是**固定 hasher seeds**，不是 ID 掩码；crate 的 `SNAPSHOT_ID_MASK=0` 与
`encode/decode_snapshot_id` 的 rotate_right(32) 应视为**未证实实现**，读档路径不要使用
（直接明文 id）。置信度：字节值 O；hasher 用途 O；常量命名 I。

### 5.5 `chimney_h_offset`（样本受限）

真实样本中 47 条 wall/roof 装饰带 `"chimney_h_offset": 0.0`（Cottage/Gothic/ArrowSlit/
Chimney/Lantern/Flag 都有），Stairs 用 `"h_offset"`（非 0 实测）。B 中存在键串
`chimney_h_offset` @0x142ce17c6（字符串与 `DecoratorHistoryState` 键表相邻），但
Ghidra 无 xref、.text 中亦无 RIP-relative LEA 命中，尚未定位其写出函数/原生偏移。
**样本受限**：36 个样本该值恒为 0.0，无法区分它是 +0x30 的别名还是独立字段。
建议：反序列化加 `#[serde(alias = "chimney_h_offset")]` 到 `h_offset`（默认 0.0），
先保证不丢字段；若将来出现非零样本再定独立槽位。置信度 U（映射）/ O（字段存在与样本值）。

---

## 6. 置信度与未知项汇总

- O：§2 serde 映射、§3 颜色 pair 形状与三表同构、§3 Trees 字段与 50/590 缺省、§4 stride 0x78/
  调用面/点数极值、§5.1 0x28/0x58 形状、§5.2 字段偏移与 seed、§5.4 常量字节与 hash 用途、
  §1 xref/静态表/调用面。
- I：WallState Vec 头字段顺序、颜色表稀疏写入等价、Trees→crate 组件映射、
  `chimney_h_offset`→h_offset 别名、Roof spawn 等价。
- U：descriptor 14 域逐名、WallState +0x30、+0x6a..0x6f、Decorator state +0x24/+0x4f、
  Roofs 0x40 中间条目逐槽、Trees 0x48 目标记录逐槽、`chimney_h_offset` 原生槽位、
  prefab_name 为空时的缺失 prefab 回退、GateColors 非空取值面。

---

## 7. ② 实现 agent 可直接照做的步骤（≤12）

1. **Walls 改走语义路径**：给 `InnerWalls` 增加
   `load_snapshot_walls(Vec<(WallId, WallState)>)`，逐字复刻 0x14085ad80 的三条 empty
   断言、created 插入、HashMap 重建、`max_id=记录最大 WallId`；`apply_loaded_snapshot_to_world`
   的 Walls 分支直接 `serde_json::from_value::<Vec<(u64, WallState)>>` 后调用它。
   停止使用 `encode_wall_record`（它零化点列）。保留字节记录 API 供其它调用者。
2. 复核 `saved_wall_state.rs`：`curve.points` 中间通道恒 0、`points_u` 不重算、`length`
   直接用缓存值；`has_skirts`→style 的 alias/默认保持。
3. **Roofs**：保持 `Vec<Roof>` 反序列化与 `world.spawn(roof)`；spawn 后用
   `roof.wall_id()` 更新 `RoofAssociations`（补一个 insert/rebuild 入口）。遇到 shape tag≥2
   按原版 `break` 语义停下并 log，不要伪造。
4. **三色表**：保持 `Vec<(u64,u32)>` 行解析；`write_color_table` 分派
   Wall/Gate/Halftimber；`GateColors` 复用同一分支（原版三段循环逐字相同）。
5. **Trees serde**：给 `TreeEditData.prefab_name` 加 `#[serde(default)]`、
   `is_preplaced` 加 `#[serde(default)]`，否则 50/590 条目使整节失败。
6. **Trees apply**：保持 `index→TreeComponent.id/TreeStorage`、`tree.id→SpawnPrefabCmd.variant`、
   position/scale/rotation_degrees/prefab_name/is_preplaced 映射；写
   `TreeAnimation::preplaced()`；`prefab_name==""` 时走既有缺失 prefab 策略，不编造名字。
7. **Decorators 纠错**：`DecoratorInfoJson::to_history_state` 的
   `u28 = color_id | (seed << 32)`（用 JSON `seed`，不是桶键 id）；桶键只给
   `DecoratorIdGen`/map key。为 5 条 seed≠key 样本加回归测试。
8. **Decorators 记录模型**：删除/弃用 `DecoratorSectionRecord` 0x28 作为“节记录”的用法；
   快照状态按 `(DecoratorId u32, pad, DecoratorHistoryState 0x50)` 处理；不要 rotate/XOR id。
9. **掩码常量**：把 `SNAPSHOT_ID_MASK` 标注为“固定 hasher seed 误称”，
   `encode/decode_snapshot_id` 标注未证实且读档不用；在文档记录
   `0x1429cd080/090` 的 32B 实值与用途。
10. **chimney_h_offset**：给 `h_offset` 加 `#[serde(alias = "chimney_h_offset")]`（默认 0.0）；
    在类型注释登记“原生槽位 U，36 样本恒 0”。
11. **读档时序**：确保 `try_load_game_system` 只在 snapshot `Some` 时调用
    `init_game_state_from_snapshot`，且发生在 fresh world（InnerWalls/DecoratorStorage 为空）
    之后；`load_snapshot_file` 失败必须 fatal，禁止回退历史链。
12. **样本回归**：写测试扫描 36 个 `.snapshot`，断言解析出 162 walls（全部 curve.points 非空）、
    73 roofs、134 decorators、431 wall colors、20 halftimber、0 gate、590 trees；
    覆盖 104 点墙、5 条 seed≠key、50 条缺 prefab 字段、Circle/Rectangle 两种 roof shape。
