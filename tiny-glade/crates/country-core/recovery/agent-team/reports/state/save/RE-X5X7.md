# RE-X5X7 — capture/init 快照链 + run_game_thread 3 命令

分析师: reanalyst-state
日期: 2026-10-16
范围: X5 capture_game_snapshot 0x1417a1d00 + init_game_state_from_snapshot 0x1417a9c50 的
Query/资源结构与逐节应用链；X7 run_game_thread 3 命令 rbx+0xb00 逐字段载荷。证据 = fresh
反编译（session-save/decomp 既有 0x1417a1d00/0x1417a9c50）+ skeleton PDB closure env + RNE 原字节。

---

## X5. capture/init 快照链

### 1. capture_game_snapshot 0x1417a1d00（系统体，184KB，decomp 4386 行）

**快照节结构（O，DIGEST §2.6 + 本批串证据）**：顶层 JSON 节序
`Version → Walls → Roofs → Decorators → WallColors → GateColors → Terrain → Trees → Raster`。
节键串（RNE 0x142cdc550 区逐字节）：`"Version"`(0x142cdc550) → `"Wall…"`(0x142cdc578)…；
每节先 `FUN_1428d8fe3`（alloc/grow 错误 panic，Location 0x142cdc320=slice.rs）→
`FUN_1428d9760`（debug log，模块串 0x142cdc1e0/1c0）→ 该节 Query 串化。

**域 Query/资源读取（O，skeleton + DIGEST）**：
| 快照节 | 数据源（Query/资源） | 证据 |
|---|---|---|
| Walls | Query<&Wall…> + PublicWalls(Res) + InnerWalls(ResMut) | DIGEST；PublicWalls 在 capture 参数面 |
| Roofs | Query<&Roof, Without<RoofMarkedForDeletion>> | skeleton Query 参数 |
| Decorators | DecoratorStorage(Res) + DecoratorArchivist(ResMut) | DIGEST；archivist 在参数面 |
| WallColors/GateColors | WallColorIds/GateColorIds/HalftimberColorIds(Res) | DIGEST；三色 Res 在参数面 |
| Terrain | TerrainEditingData(ResMut) + TerrainHeightsData(Res) + AllWorldRasterRead | DIGEST（flate2 deflate） |
| Trees | **Query<(&PrefabInstanceData,&PrefabInstance,&TreeComponent)>** → map 出 `(TreeStorageIndex, TreeData)` | skeleton closure_env（capture_game_snapshot_save_data::closure_env$0 逐字） |
| (Sheep) | Query<&mut SheepControl> | skeleton Query 参数 |
| (相机/时间/羊/glade) | MainCamera(ResMut)/TimeOfDay(ResMut)/GladeColorCustomization(ResMut) | capture_game_snapshot_save_data 组装 |

**capture_game_snapshot_save_data 0x1417a0430（O，711 行）**：~9 组世界数据
（FUN_141305170/141308b40/141309840 收集器）→ 构建 ~0x400B SnapshotSaveData 结构
（JSON 键串表 0x142a93bfe 起：walls/roofs/…）→ 返回 **Arc**（alloc 0x18 {strong=1,weak=1}
+ LOCK inc）。closure 用 tree Query 收集 Trees 节。

### 2. init_game_state_from_snapshot 0x1417a9c50（716 行，O 应用链）

`FUN_1417a9c50(parsed_data_ptr, snapshot_arc, …)`：把 `validate_and_load_closest_snapshot_data`
(0x1417a6e50) 解析出的各节数据**写回对应域资源**。调用链：
- `FUN_14004f570`（大 helper，逐节应用主函数）
- `FUN_14083ebe0` / `FUN_14085ad80` / `FUN_1408ff590` / `FUN_140a20d70` / `FUN_140b5f9f0`
  （各节应用 helper：分别对应 Raster/Terrain/Trees/Decorators 等写回）
- grow 族 FUN_140562db0/870/7b0/630（Vec 重建）
- puffin trace（verbosity>3 时，串 0x142cdc6c0/888/898/8b0）——每节应用有 trace scope。
- 收尾 `func_0x000140d4a330`（日志/事件）。

**逐节应用链（O 级结论，供 restorer）**：
```
parsed SnapshotSaveData（0x1417a6e50 产出 0x1b0B）
  ├─ Walls → PublicWalls/InnerWalls 重建（FUN_140b5f9f0 族）
  ├─ Roofs → Query<&mut Roof> 逐实体应用
  ├─ Decorators → DecoratorStorage 状态写回
  ├─ WallColors/GateColors → 三色 ID 表写回
  ├─ Terrain → TerrainEditingData/TerrainHeightsData 重建（含 deflate 解压数据）
  ├─ Trees → TreeStorageIndex/TreeData 写回树存储
  ├─ Sheep → Query<&mut SheepControl> 快照恢复
  └─ 相机/时间/glade → MainCamera/TimeOfDay/GladeColorCustomization
```
（各节 helper 地址已列；逐 helper 内部字段映射可对 0x1417a9c50 分段比对——残留 §3。）

---

## X7. run_game_thread 3 命令（rbx+0xb00 数组，O 部分）

**数组布局**：3 × 0x38B MetaCommand，`cmd0@+0xb00 / cmd1@+0xb38 / cmd2@+0xb70`；
append_on_load<[MetaCommand;3]> 调用点 0x1405a3440（rcx=[rbx+0x2590] NextSession 域 /
rdx=[rbx+0x440] on_load Vec / r8=[rbx+0xb00] 数组）。

**已确认（RNE 原字节 0x1405a3330..0x1405a3460）**：
| 命令 | tag | 载荷字段 |
|---|---|---|
| **cmd2**（+0xb70） | **变体 8**（`0x8000000000000008`，0x1405a340c 直写） | +0x08 = [rbx+0x2230]（16B movups）、+0x18 = [rbx+0x2240]（16B）、+0x28 = rdi |
| **cmd1**（+0xb38） | **未直写**（疑从源字段复制） | +0x10 = [rbx+0x2c00]（movups）、+0x20 = [rbx+0x2c10]（movups）、+0x30 = [rbx+0x25c0] |
| **cmd0**（+0xb00） | **未直写** | 构造区在 0x1405a3330 之前（更早字节） |

**源字段线索**：`[rbx+0x2bf0]` 存 **变体 5**（`0x8000000000000005`，0x1405a3334 直写）——
疑似 cmd0 或一个 MetaCommand 模板的 tag；`[rbx+0x2590]` 存变体 1（0x1405a3345）——
疑 NextSession kind（load 语义 kind=0x8000000000000002 为常规，此处 1=解析存档路径）。
字节标志区 [rbx+0xb4..0xbb]（0x1405a31e0 区 c6 83 b4..bb）为 NextSession ignore_snapshot 等。

**结论（供 restorer）**：cmd2 = 变体 8 命令（载荷来自 [rbx+0x2230]/[rbx+0x2240]/rdi 三源）；
cmd0/cmd1 的 tag 与完整载荷需对 0x1405a3330 前段做 **ReVa High P-code 定点**（39KB 函数
非 Ghidra 可切——残留 §3）。但**结构结论已可落**：run_game_thread 构建 3 条 MetaCommand
（含变体 8），压入 on_load 作为新会话初始命令集；命令内容与 commander_update case 0/8 分派
（RE-SESSION-SWITCH §1）对应。

---

## 3. 未决疑问

1. init_game_state_from_snapshot 各节 helper（FUN_14004f570 等）内部逐字段映射——需
   0x1417a9c50 分段比对。
2. run_game_thread cmd0/cmd1 的 tag 与逐字段载荷——需 ReVa p-code（39KB 函数）。
3. capture_game_snapshot_save_data 的 ~0x400B SnapshotSaveData 逐字段 layout。
