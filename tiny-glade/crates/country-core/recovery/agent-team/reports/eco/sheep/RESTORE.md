# restorer-eco — sheep 模块 RESTORE 报告

日期：2026-10-15
状态：fix_pending（reviewer 边界裁决=中央；本批验收=SheepMoveState serde + capture/apply 闭合 + 存档 Sheep 节接入）
涉及文件：`resources/critters/sheep.rs`、`resources/critters/sheep/control.rs`

## 一、原函数 → Rust 位置 → 证据

| 原函数 | 地址 | Rust 位置 | 证据 |
|---|---|---|---|
| SheepMoveState serde（externally-tagged） | 0x1413ff520 / 0x1413ff8a0 / 0x140080440 | sheep.rs:20 | decomp-0x1413ff520（判别@+0；Idle/Walking 字段偏移+1/+4/+8 逐字段）；0x1416f2800 Debug |
| SheepSnapshot/SheepControlSnapshot serde | 0x1416f9eb0 / 0x1416f9d30 / 0x1405c9c50 | control.rs:14-21 | decomp-0x1416f9eb0（3-entry map：Vec3→Option\<Vec3\>→SheepMoveState）+ **rne-read 字段名 sheep@0x142c91fa9、position/goal/move_state@0x142c91f49/51/55** |
| capture_sheep_snapshot | 0x1412f03f0（内联） | control.rs:35 | 章节 control.rs.md；re-analyst：Query=(Entity,&mut Transform,&mut Sheep,&mut CritterGoal)→Vec\<SheepSnapshot\>，内联于 save_game 0x140a3f470 |
| apply_sheep_snapshot | 0x1416f24f0 | control.rs:51 | decomp-0x1416f24f0（40B 条目→组件 {+0x00,+0x08,+0x10,+0x18,+0x30,+0x38}） |
| Sheep::being_petted | 0x14089d4c0 | ——（边界外） | decomp-0x14089d4c0（f32@+0x58 > 0.01） |

## 二、实现说明

- **SheepMoveState**：判别字节@+0；0=Idle{timeout_s}，非 0=Walking{goal_change_timeout, <f32>, force_reach_goal}。serde 字段序（wire）= timeout_s, goal_change_timeout, force_reach_goal，与 0x1413ff520 逐字段对上；当前枚举声明序即该 serde 序。Walking 中间 f32 字段名待取证（内存序 goal_change_timeout, <f32>, force_reach_goal）。
- **SheepSnapshot/SheepControlSnapshot**：字段名 rne-read 定案（position/goal/move_state/sheep），serde 3-entry map 序与 0x1416f9eb0 一致；存档 Sheep 节 = Option\<SheepControlSnapshot\>（None→"null"），save_game to_json_string 消费。
- **capture_sheep_snapshot**：按 Query 序收集 (position, goal, move_state)。原生 Query 含完整 `&mut Sheep` 组件（跨域 systems/critters 属主）；本 crate 以 `&SheepMoveState` 借用行代替（Sheep owner 边界外，登记映射偏差）。
- **apply_sheep_snapshot**：zip 恢复 position/goal/move_state，保留 rotation/scale 与其他羊状态；与原生"zips query order with saved order, stops at shorter, overwrites exactly three fields"一致。
- 两个测试：wire 字段与变体、捕获/恢复保留其他字段，均通过。

## 三、依赖复用边界
- serde derive 复用依赖（项目约束：serde visitor 胶水不计业务）。
- `CritterGoal`（resources/critters/mod.rs）、`Transform`（utils 共享外壳）复用。

## 四、仍缺失/待取证（边界外，不伪造）
1. **完整 Sheep 组件 owner**：being_petted（+0x58>0.01）与 register_required_components ×5 属 systems/critters 跨域；中央已裁决本批边界外。
2. **SheepMoveState::Walking 第 2 f32 字段名**：待取证。
3. **Sheep 组件完整布局**：跨域 U。
4. **startup_sheep**（0x140a093e0）：羊动画网格 + FMOD event:/SFX/Sheep/Baah 载入，依赖动画注册链 + 音频管线，登记待接线。
5. **存档 Sheep 节接入**（save_game to_json_string::<Option<SheepControlSnapshot>>）：save 组模块，消费端由 save_game 接入。

## 五、inventory 建议
- `resources/critters/sheep.rs`、`resources/critters/sheep/control.rs` 提升为 `present_requires_semantic_review`。
- 建议 reviewer 复核：①serde 字段名 rne-read 依据；②capture Query 的 Sheep 组件替代映射；③apply 40B 布局与 zip 恢复的一致性边界。
