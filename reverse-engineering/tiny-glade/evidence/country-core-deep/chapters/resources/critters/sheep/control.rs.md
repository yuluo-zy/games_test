# resources/critters/sheep/control.rs

> 文件作用：**SheepControlSnapshot/SheepSnapshot（serde 快照）——会话存档 "Sheep" 节载荷。**

函数总计 15：业务 3 / 内联无独立符号 1 / 派生胶水 11

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::critters::sheep::control::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x1416f9eb0 | 11 | 派生胶水 | O（2026-10-15 深挖）：**SheepSnapshot serialize** —— JSON 对象 3 字段：`position` (Vec3)、`goal` (Option<Vec3>)、`move_state` (SheepMoveState)。**字段名串（O rne-read）**：`position`@0x142c91f49、`goal`@0x142c91f51、`move_state`@0x142c91f55 | eco-reanalyst\decomp\decomp-0x1416f9eb0.txt |
| `country_core::resources::critters::sheep::control::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> > >` | 0x1416f9d30 | 11 | 派生胶水 | O：SheepSnapshot serialize（pretty 同构，字段名串同上） | eco-reanalyst\decomp\decomp-0x1416f9d30.txt |
| `country_core::resources::critters::sheep::control::_::impl$0::serialize<serde_json::value::ser::Serializer>` | 0x1405c9c50 | 11 | 派生胶水 | O：SheepSnapshot serialize → serde_json value（3 字段 Vec3/Option<Vec3>/SheepMoveState，callee 0x1403e2410/0x1403e2670/0x1403e2d60） | eco-reanalyst\decomp\decomp-0x1405c9c50.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::critters::sheep::control::capture_sheep_snapshot`（行 29）— O（2026-10-15）：绵羊快照捕获。**Query 签名（PDB 泛型实例证实）**：`QueryIter<(Entity, &mut Transform, &mut Sheep, &mut CritterGoal)>` → `.map(closure)` → `Vec<SheepSnapshot>`（from_iter collect）。内联于 `systems/save/save_game.rs`（0x140a3f470，pdb_inline_direct）+ `systems/save/load_game.rs`。closure_env$0 捕获 (Entity, Transform, Sheep, CritterGoal) 组装 40B 元素。

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`closure`、`deserialize`、`eq`、`expecting`、`visit_map`、`visit_seq`、`visit_str` 等 11 项

## 实现方式（算法级，2026-10-15 O）

### SheepControlSnapshot / SheepSnapshot 结构（serde 字段名 O，rne-read 串池）

- **`SheepControlSnapshot = { "sheep": Vec<SheepSnapshot> }`**（1 字段；serialize 0x1416f9fd0/0x1405c9dd0 单 serialize_entry；字段名 `sheep`@0x142c91fa9）。
- **`SheepSnapshot = { "position": Vec3, "goal": Option<Vec3>, "move_state": SheepMoveState }`**（3 字段；字段名 `position`@0x142c91f49、`goal`@0x142c91f51、`move_state`@0x142c91f55）。
- SheepSnapshot 内存元素 = **40B**（apply_sheep_snapshot 0x1416f24f0 的 Vec 步长 5×qword=40B 证实）：`{+0x00 u64, +0x08 u64, +0x10 u64, +0x18 u32, +0x1c u64, +0x24 u32}`（= Vec3 position 12B + pad + Option<Vec3> 16B + SheepMoveState 判别/载荷 12B）。

### SheepMoveState 枚举（O，serialize 0x140080440）

判别字节 @+0：**0 = `Idle`**、**非 0 = `Walking`**（serde 串 `Idle`@0x14293079c、`Walking`@0x1429307a9、`timeout_s`@0x1429307a0、`goal_change_timeout`@0x1429307b0、`force_reach_goal`@0x1429307c3）：
```
Idle    { timeout_s: f32 }
Walking { goal_change_timeout: f32, <f32 第2字段>, force_reach_goal: bool }   # serialize 3 字段（f32,f32,bool）
```
（Walking 第 2 f32 字段名待 shader/回放消费端交叉（串池余 sync_before_* 族属他型，U）。

### 存档协议

"session" 存档 **Sheep 节** = `Option<SheepControlSnapshot>`（`to_json_string<Option<SheepControlSnapshot>>` 0x140a51d40）：None → 字面量 `"null"`；Some → `{"sheep":[{position,goal,move_state},...]}`。读档经 `parse_saved_game_data::closure_env$0` 调 `serde_json::from_str<SheepControlSnapshot>`。
