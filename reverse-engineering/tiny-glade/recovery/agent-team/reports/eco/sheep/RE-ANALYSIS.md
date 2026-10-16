# RE-ANALYSIS: resources/critters/sheep（生态组 · sheep）

> 会话：eco re-analyst（2026-10-15）。范围：`resources/critters/sheep.rs`、`resources/critters/sheep/control.rs`、`systems/debug/sheep_controls.rs`、`systems/audio/sheep_audio.rs`、`startup/startup_sheep.rs`。
> 依据：B patched EXE 机器码（RNE oracle）+ 本轮新反编译（eco-reanalyst/decomp/）+ PDB 泛型实例记录。
> 记号：O=机器码证实 / I=有界推断 / U=未恢复；「旧结论 X → 新证据 Y」=本章修正。
> 不改 reconstruction 源码。

## 1. 函数清单表（本轮定点深挖项）

| 函数 | 地址 | 状态 | 行为一句话 |
|---|---|---|---|
| SheepSnapshot serialize（compact/pretty/value） | 0x1416f9eb0 / 0x1416f9d30 / 0x1405c9c50 | O | **SheepSnapshot = {position: Vec3, goal: Option<Vec3>, move_state: SheepMoveState}** |
| SheepControlSnapshot serialize | 0x1416f9fd0 / 0x1405c9dd0 | O | **SheepControlSnapshot = {sheep: Vec<SheepSnapshot>}**（存档 Sheep 节） |
| SheepMoveState serialize | 0x140080440 | O | enum：0=Idle{timeout_s} / 非0=Walking{goal_change_timeout,<f32>,force_reach_goal} |
| capture_sheep_snapshot | 内联（行 29） | O（宿主内联+PDB 泛型） | Query `(Entity,&mut Transform,&mut Sheep,&mut CritterGoal)` → Vec<SheepSnapshot>；内联于 save_game 0x140a3f470 / load_game |
| to_json_string\<Option\<SheepControlSnapshot\>\> | 0x140a51d40 | O | None→"null"，Some→FUN_1416f9fd0 序列化（Sheep 节载荷） |
| apply_sheep_snapshot | 0x1416f24f0 | O（逐字段拷贝映射） | 40B SheepSnapshot 元素 → sheep 组件多字段（见 §3） |
| sheep_controls | 0x1416f11a0 | O（主系统） | 查询 + 拾取 + 快照应用 + Transform 写回 + Events 计数 |
| SheepControl::new | 0x1416f0ea0 | O | SheepControl 组件构造（692B，PickingMode + 快照 Option 字段） |

## 2. 快照字段（serde 名，O rne-read 串池）—— 存档协议核心

- **SheepControlSnapshot = `{ "sheep": Vec<SheepSnapshot> }`**（字段名 `sheep`@0x142c91fa9；serialize 0x1416f9fd0/0x1405c9dd0 单 serialize_entry）。
- **SheepSnapshot = `{ "position": Vec3, "goal": Option<Vec3>, "move_state": SheepMoveState }`**（字段名 `position`@0x142c91f49、`goal`@0x142c91f51、`move_state`@0x142c91f55）。
- SheepSnapshot 内存元素 = **40B**（apply_sheep_snapshot Vec 步长 5×qword=40B）：`{+0x00 u64, +0x08 u64, +0x10 u64, +0x18 u32, +0x1c u64, +0x24 u32}`（= Vec3 12B + pad4 + Option<Vec3> 16B + SheepMoveState 12B）。
- **SheepMoveState enum（O）**：判别字节@+0，0=`Idle` / 非0=`Walking`（串 `Idle`@0x14293079c、`Walking`@0x1429307a9、`timeout_s`@0x1429307a0、`goal_change_timeout`@0x1429307b0、`force_reach_goal`@0x1429307c3）。
  - `Idle { timeout_s: f32 }`
  - `Walking { goal_change_timeout: f32, <f32 第2字段>, force_reach_goal: bool }`（serialize 3 字段 f32/f32/bool；第 2 f32 字段名 U）。
- **存档 Sheep 节 = `Option<SheepControlSnapshot>`**：None → `"null"`；Some → `{"sheep":[{position,goal,move_state},...]}`。读档经 `parse_saved_game_data::closure_env$0` `serde_json::from_str<SheepControlSnapshot>`。

## 3. apply_sheep_snapshot 字段映射（O，0x1416f24f0）

```
签名 (param_1=查询面, param_2=Vec<SheepSnapshot> 40B 元素)
遍历 Query 命中羊实体，逐元素:
  data+0x00 ← snapshot+0x00 (u64)
  data+0x08 ← snapshot+0x08 (u64)
  data+0x10 ← snapshot+0x1c (u64)
  data+0x18 ← snapshot+0x24 (u32)
  data+0x30 ← snapshot+0x10 (u64)
  data+0x38 ← snapshot+0x18 (u32)
  附加 uVar4(param_1+0x14 f32) 写 3 个 f32 数组（步长 4）
（并行写 0x30B/0x68B/0x10B 步长组件数组 = Transform/Sheep/CritterGoal 多组件）
```

## 4. 修正记录（旧结论 → 新证据）

- sheep/control.rs 章「capture_sheep_snapshot：I（推断）」→ **O**：PDB 泛型实例证实 Query 签名 + 40B 元素布局 + save_game 宿主内联。
- debug/sheep_controls.rs 章「apply_sheep_snapshot：O（名），逐字段拷贝 U」→ **O（逐字段映射）**；「sheep_controls：O（名）」→ **O（主系统算法）**。
- sheep.rs 章补 SheepMoveState 枚举（Idle/Walking + 字段）与 serde 名串池。

## 5. 未决（U，不臆造）

1. SheepMoveState::Walking 第 2 f32 字段名（串池余 `sync_before_*` 族属他型，需回放/控制消费端交叉）。
2. Sheep 组件完整 0x?B 布局（being_petted f32@+0x58 已 O；apply 写面 +0x00..0x38 已 O；+0x40..0x57 与其余字段 U——业务体在 systems/critters 移动状态机，跨域）。
3. capture_sheep_snapshot 的 40B 元素字段 ↔ SheepSnapshot serde 字段（position/goal/move_state）精确内存映射（apply 写面已给 6 字段，完整对应待 save_game 巨型函数内联细节）。

## 6. 证据落盘（本轮新增）

- `evidence/country-core-deep/eco-reanalyst/decomp/decomp-0x1416f9eb0.txt`（SheepSnapshot compact）
- `evidence/country-core-deep/eco-reanalyst/decomp/decomp-0x1416f9d30.txt`（SheepSnapshot pretty）
- `evidence/country-core-deep/eco-reanalyst/decomp/decomp-0x1416f9fd0.txt`（SheepControlSnapshot compact）
- `evidence/country-core-deep/eco-reanalyst/decomp/decomp-0x1405c9c50.txt`（SheepSnapshot value）
- `evidence/country-core-deep/eco-reanalyst/decomp/decomp-0x140080440.txt`（SheepMoveState value serialize）
- `evidence/country-core-deep/eco-reanalyst/decomp/decomp-0x1403e2d60.txt`（serialize_field<SheepMoveState>）
- `evidence/country-core-deep/eco-reanalyst/decomp/decomp-0x1416f0ea0.txt`（SheepControl::new）
- 章节已升级：chapters/resources/critters/sheep/{sheep.rs,control.rs}、chapters/systems/debug/sheep_controls.rs.md。
