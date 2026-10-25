# WP10 契约 1/3 — `resources::tree_storage`（树/植被实例存储）

版本 **B**（EXE `cf048e27…`，PDB `2d9686b8…`）。骨架证据：`skeleton/B/crates/country-core/src/resources/tree_storage.rs.{pdb,enriched,structures}.json.gz`；
反编译：Ghidra `0x140a38560`(insert)/`0x140a38460`(get)/`0x140a385a0`(remove)/`0x140a38600`(next_index)/`0x140a3d320`+`0x140a3da10`(register_required_components)。
标注：**O**=反编译/PDB 直接支持；**I**=调用链/名称组合；**U**=未恢复。所有 `resources::` 引用为 typed gap，不臆造字段。

## 1. 类型清单（O，structures `named_symbol_entities`，全部 `name_and_relations_only`）

| 类型 | 角色 | 消费者（enriched inline host / caller） |
|---|---|---|
| `TreeStorage` | ECS 资源（树索引 → 记录） | `system_tree::impl$0::build`（tree-tool）；`forest::impl$0::build`；`country_core::startup::spawn_preplaced_trees`；`wall::eraser::ui_bulldozer::wall_eraser_bulldoze`（iter/get）；`systems/prefab.rs::create_prefab_rt`（BirdHaven::new 内联于 :147） |
| `TreeStorageEntry` | 单树记录（map 值） | insert/get 返回值（U 布局，见 §3 冲突） |
| `ForestTree` | 组件（森林树标记） | `impl$10::default` 内联于 `forest::impl$0::build`（行 100） |
| `BirdHaven` | 组件（鸟巢/树栖点） | `BirdHaven::new` 内联于 `create_prefab_rt`（行 147）；`system_critters::wren` 的 `Query<(ref BirdHaven, ref PrefabInstanceData)>`（structures 交叉引用 O） |
| `InteractableForestPlacements` | ECS 资源（可交互森林放置） | `clear` 内联于 `startup/clearing.rs::{init_clearing,uninit_clearing}`（行 110） |
| `InteractableForestTree` | 记录/组件 | 名 O；字段 U |
| `HideTreesNextToWallCmd` | 命令事件 | `system_tree::hide_trees::hide_trees`（hide_trees.rs，行 O） |
| `HideTreesinAreaCmd` | 命令事件 | 同上 |
| `MoveTreeCmd` | 命令事件 | `system_tree::process_tree_cmds::process_tree_cmds`（行 O，见 tree-tool 契约） |

函数（enriched functions，行号 = pdb.jsonl line 表）：

| 函数 | 行 | 反编译地址 | 语义（O） |
|---|---|---|---|
| `TreeStorage::insert` | 66 | 0x140a38560 | hashbrown `insert<u64,…>`；`next_index = max(next_index, index+1)`（`+0x30`） |
| `TreeStorage::iter` | 54 | （内联于 wall_eraser_bulldoze） | 迭代树记录（供墙擦除遍历） |
| `TreeStorage::get` | 58 | 0x140a38460 | 先判 `+0x18`（非空 u64）==0 → None；否则 hashbrown get |
| `TreeStorage::remove` | 72 | 0x140a385a0 | hashbrown `remove_entry` |
| `TreeStorage::next_index` | 77 | 0x140a38600 | `+0x30 += 1`（分配下一个索引；返回旧值/新值 U） |
| `impl$6::default` | 33 | — | `TreeStorage` 的 Default（空 map + next_index=0） |
| `impl$11::register_required_components` | 116 | 0x140a3d320 | bevy 组件注册（`ForestTree` 族） |
| `impl$12::register_required_components` | 131 | 0x140a3da10 | bevy 组件注册（`BirdHaven` 族） |
| `InteractableForestPlacements::clear` | 110 | — | 清空可交互森林放置 |

## 2. `TreeStorage` 布局（O 反编译 + I 语义）

- `+0x18`：u64 非空/数量标记（`get` 先查 ==0 → None；PDB 反编译 get `in_RCX[3] != 0`）。
- `+0x20`：hashbrown `HashMap`（键 `u64`；`get` 经 `hash_one<RandomState, ref GlobalId>` 说明键实际为 `GlobalId` = u64 新类型，O）。
- `+0x30`：`next_index: u64`（insert/next_index 读写，O）。

## 3. map 值类型冲突（关键裁决，typed gap）

- tree-tool 早前恢复（reva 反汇编 0x140a38460/0x140a38560/0x140a385a0）：`TreeEntryValue = {prefab_index u32, entity u32}`（8B，Move 分支低/高 32 位读取）。
- enriched `insert` callee 签名：`insert<u64, ref_<system_clutter::autoclutter::AutoClutterObject>,…>` → 值类型为 `&AutoClutterObject`。
- 裁决：两证据冲突，无法单边定论；登记为 **U**（typed gap）。实现用 `TreeStorageEntry { prefab_index: u32, entity: u32 }` 占位并注明冲突，不声称是作者原字段。

## 4. 命令/组件（O 名，U 字段）

- `MoveTreeCmd { index: u64, position: Vec3 }`（I：tree-tool 反汇编 0x142267430 事件槽 0x20，payload 20B）。
- `HideTreesNextToWallCmd` / `HideTreesinAreaCmd`：hide_trees 消费者（U 布局）。
- `ForestTree`、`BirdHaven`：组件标记/数据（`BirdHaven::new` 在 prefab 生成时创建，U 字段）。

## 5. 生产/消费关系

- **生产者**：`startup::spawn_preplaced_trees`（行 66 内联 insert）、`process_tree_cmds`（Insert 分支 insert）、`forest::manage_tree_prefabs`、`startup::clearing`（InteractableForestPlacements）。
- **消费者**：`system_tree::process_tree_cmds`（get/remove）、`ui_bulldozer::wall_eraser_bulldoze`（iter/get）、`system_critters::wren`（BirdHaven → roof/decorator bird spots，见 critters 契约）、grass/garden pass 间接（树木 canopy 位置来源，I）。
- **清除**：`clearing::{init_clearing, uninit_clearing}` 调用 `InteractableForestPlacements::clear`（O）。

## 6. 失败分支 / 边界（O/I）

- `get`：非空标记 ==0 → `None`（不 panic）。
- `next_index`：`+0x30` 自增；insert 同时做 `max(next_index, index+1)` → 索引单调不回退（O）。
- `process_tree_cmds` Move 分支：`TreeStorage::get` 未命中 → `log::error!("MoveTreeCmd references a non-existing tree")`（O 字符串，I 行为）。

## 7. typed gap 汇总

- `TreeStorageEntry` 精确布局与 map 值类型（§3 冲突）。
- `ForestTree`/`BirdHaven`/`InteractableForestTree` 字段与尺寸。
- `HideTreesNextToWallCmd`/`HideTreesinAreaCmd` payload。
- `next_index()` 返回旧值或新值。
- `TreeStorage` 是否含 `+0x10` 等其它字段（反编译仅观察到 +0x18/+0x20/+0x30）。
