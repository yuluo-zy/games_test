# REBUILD 报告 — resources/critters/sheep（rebuild-gardensheep，2026-10-17）

> 依据：`2026-10-16-logic-verify-protocol.md`。只以 fresh 反编译为准（本会话重新
> get_functions 0x1412f03f0 / 0x1416f24f0 / 0x1413ff520 / 0x140080440 / 0x1416f9eb0 /
> 0x1405c9c50）。重建对象 = LOGIC.md 判出的 capture/apply 组件绑定分歧 + Sheep 布局矛盾。

## 一、函数分类（宏胶水 vs 真业务）

| VA | 函数 | verdict | 类别 | 本批处理 |
|---|---|---|---|---|
| 0x14089d4c0 | `Sheep::being_petted` | **判等** | 真业务 | 不动（红线） |
| 0x1413ff520 / 0x140080440 / 0x1413ff8a0 | `SheepMoveState` serde | 胶水-非业务（wire 判等） | 宏胶水 | 不动 |
| 0x1416f9eb0 / 0x1405c9c50 / 0x1416f9fd0 | `SheepSnapshot`/`SheepControlSnapshot` serde | 胶水-非业务（wire 判等） | 宏胶水 | 不动 |
| 0x1412f03f0（内联） | `capture_sheep_snapshot` | 判等（逻辑）/ **组件绑定分歧** | 真业务 | **重建**：修正三元组绑定 |
| 0x1416f24f0 | `apply_sheep_snapshot` | 判等（逻辑）/ **组件绑定分歧** | 真业务 | **重建**：修正三元组绑定 |
| — | `Sheep` 结构（sheep.rs:61） | **布局矛盾** | 真业务（部分字段） | **重建**：+0x30=goal（非 move_state） |

## 二、capture（0x1412f03f0）/ apply（0x1416f24f0）— 组件绑定重建

**fresh 反编译（2026-10-17）逐字数据流**：

capture 尾部每实体读三组件数组：
- `snapshot+0x24 = *(Transform@+0x18)`、`snapshot+0x1c = *(Transform@+0x10)` → **move_state**
- `*snapshot = *CritterGoal@+0x00`、`snapshot+8 = *(CritterGoal@+0x08)` → **position + goal-tag**
  （CritterGoal 16B = `position: Vec3@0x00` + `goal_tag: u32@0x0c`）
- `snapshot+0x10 = *(Sheep@+0x30)`、`snapshot+0x18 = *(Sheep@+0x38)` → **goal payload**

apply 每实体写三组件：
- `Transform@+0x10/+0x18 ← snapshot+0x1c/+0x24`（move_state → **utils Transform.translation 槽**）
- `CritterGoal@+0x00…0x0f ← snapshot+0x00…0x0f`（position + goal tag）
- `Sheep@+0x30/+0x38 ← snapshot+0x10/+0x18`（goal payload）

即原生绑定为（LOGIC §2 结论，本会话逐字确认）：
- **position ← CritterGoal@0x00**（非 Transform.translation）
- **goal ← Sheep@+0x30 payload，Option tag 来自 CritterGoal@0x0c**
- **move_state ← utils Transform.translation@+0x10**

旧 Rust 三元组 `(&Transform,&CritterGoal,&SheepMoveState)`
（position←transform.translation、goal←CritterGoal.0、move_state←SheepMoveState 组件）
绑定目标错位：position 源在原生是 move_state 槽，goal 源在原生是 position 槽。

**重建**：
- `capture_sheep_snapshot` / `apply_sheep_snapshot` 三元组改为 `(&Transform, &Sheep, &CritterGoal)`：
  - capture：`position ← critter_goal.position`；`goal ← (goal_tag!=0).then_some(sheep.goal)`；
    `move_state ← SheepMoveState::from_translation(transform.translation)`。
  - apply：`transform.translation ← to_translation(move_state)`；`sheep.goal ← goal payload`；
    `critter_goal ← {position, goal_tag = is_some as u32}`。
- `resources/critters/mod.rs`：`CritterGoal(Option<Vec3>)` → 显式
  `{ position: Vec3 @0x00, goal_tag: u32 @0x0c }`（原生 16B 布局；Option 语义无法表示
  "position 恒存在 + tag 分离"，故改显式字段）。
- `systems/debug/sheep_controls.rs`（apply 原生属主）：同步修正同一绑定。
- Rust 位置：`src/resources/critters/sheep/control.rs`、`src/resources/critters/mod.rs`、
  `src/systems/debug/sheep_controls.rs`。

## 三、SheepMoveState ↔ Transform.translation 槽编码（O 一致性）

原生把 SheepMoveState 存进 utils Transform 的 translation（12B @+0x10/+0x18），capture/apply
仅对称拷贝 12B（函数内无任何 move_state 内容分支）。据此定义可逆编码（sheep.rs 新增
`SheepMoveState::to_translation/from_translation`）：

```text
+0x00 u32 tag       0 = Idle, 1 = Walking, 0x101 = Walking+force_reach_goal
+0x04 f32 a         Idle: timeout_s;         Walking: goal_change_timeout
+0x08 f32 b         Idle: 0.0;               Walking: timeout_s
translation = (f32::from_bits(tag), a, b)
```

证据：serde 判别字节 @+0（0x1413ff520 `*in_RCX == 0` → Idle）；picking 写 tag 0x101
= Walking+force_reach_goal（sheep_controls 0x1416f11a0 注记）；Walking 内存序
goal_change_timeout, timeout_s（sheep.rs 既有 O 注记）。tag 用 `f32::from_bits` 存
位型，保证判别字节（translation.x 低字节）与 serde 读取一致。

## 四、Sheep 结构布局（sheep.rs:61）— 重建

**fresh 反编译否定旧注释**：capture/apply 中 stride-0x68（=Sheep）@+0x30 读/写的是
**goal（Vec3 payload）**，move_state 读/写自 stride-0x30（utils Transform）@+0x10。
`+0x58`（being_petted，0x14089d4c0）正确不动。

重建：`Sheep { move_state: SheepMoveState }` → `Sheep { goal: Vec3 /* +0x30 */, being_petted_timer: f32 /* +0x58 */ }`；
Option tag 不在 Sheep（在 CritterGoal@+0x0c）。模块注释同步修正（"内嵌 SheepMoveState @+0x30" 错误声明
删除；`+0x3c..0x57` 仍待取证不造）。

> 完整 Sheep owner（其余 ~20 字段 / 5 组件 register_required_components）仍属
> systems/critters 跨域，DISPATCH 边界外，未造。

## 五、编译检查

- `cargo check -p country-core`：**通过**（本模块 0 警告）。
- `cargo test --lib`：**测试二进制因既有重构损坏无法构建**（55 个既有错误全来自
  render/rhapsody 域 `Device`/`GraphicsDevice` 元组构造重构）。本批新文件
  （sheep.rs / control.rs / mod.rs / sheep_controls.rs）不在错误清单。
- **行为验证**：临时 integration test（已跑后删除）编译 country-core lib 并对
  `capture_sheep_snapshot`/`apply_sheep_snapshot` 断言 position←CritterGoal、
  goal←Sheep、move_state←translation 绑定与 zip 短路，对 `to_translation/from_translation`
  断言 3 变体 bijection + tag 位（0/0x1/0x101）—— **3 项全过**。
- 前置修复（登记）：`recovered_building/building_history_transactions.rs` 补 trait import、
  `systems/world_raster.rs::record_raster_history` 补泛型界（garden 调用链依赖，见 garden REBUILD §四）。

## 六、自检

- capture/apply 逻辑（query 序、zip 最短、3 字段覆盖）维持判等；仅绑定目标修正。✓
- 测试更新：`capture_and_apply_use_query_order_and_preserve_other_fields` 断言
  position←CritterGoal、goal←Sheep、move_state←translation、rotation/scale 与其它羊状态保留；
  `move_state_roundtrips_through_the_transform_translation_slot` 断言 3 变体 bijection +
  tag 位（0/0x1/0x101）。✓
- 未删除判等函数（being_petted 逻辑/测试逐位保留）。✓
- 未改他模块业务逻辑（CritterGoal 为 critters 共享组件，本次按原生布局改显式字段，
  属 sheep 绑定修正的必要项；仅 2 处既有编译阻塞的最小修复）。✓
