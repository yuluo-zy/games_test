# REVIEW — resources/critters/sheep（reviewer-eco，2026-10-15）

独立核对现有实现（sheep.rs / sheep/control.rs）。证据：collision-eco/decomp/decomp-0x1413ff520
（SheepMoveState serialize 已逐条比对）、章节 resources/critters/sheep*.md、inventory.json。
restorer 尚未提交 RESTORE.md，本报告为对当前 src 的独立审查基线。

## 逐函数三栏对照

### resources/critters/sheep.rs（原 28 项）

| 函数 | 原行为 | Rust 现状 | 判定 |
|---|---|---|---|
| SheepMoveState serde（serialize 0x1413ff520/0x1413ff400、deserialize 0x1413ff8a0 族） | Idle={"Idle":{"timeout_s":f}}；Walking={"Walking":{"timeout_s":f,"goal_change_timeout":f,"force_reach_goal":b}}，字段名/序经 0x142c5b4cf/0x142c5b4d3/0x142c5b4dc/0x142c5b4e3/0x142c5b4f6 长度核对（4/9/7/19/16） | derive 外部标签，字段名/序一致；测试断言 JSON 文本一致 | **OK** |
| impl$1/7/8/9/10 register_required_components | Bevy 组件注册（Bevy 胶水，不计业务） | Sheep 组件族未建（无组件类型） | **GAP 注记**（见 §结论） |
| Sheep::being_petted 0x14089d4c0 | =f32@+0x58>0.01（ucomiss 常量 0x142960674） | **未实现**（无 Sheep 类型） | **GAP**（受 DISPATCH"不复制空壳 Sheep"边界约束） |

### resources/critters/sheep/control.rs（原 15 项）

| 函数 | 原行为 | Rust 现状 | 判定 |
|---|---|---|---|
| SheepControlSnapshot/SheepSnapshot serde（serialize 0x1416f9eb0/0x1416f9d30/0x1405c9c50） | 快照 wire schema | 实现 {sheep:[{position,goal,move_state}]} | **OK（实现）/ 待取证（wire）**——本机缺 0x1416f9eb0 decomp（证据指向 evidence/core-recovery-20261015/critters/REPORT.md，本机不存在）；字段序未与原生逐条核对 |
| capture_sheep_snapshot | 捕获迭代器（inline，+0x1c pos/+0 goal/+0x10 move_state） | 按传入查询序收集 position/goal/move_state，不存实体 id、不排序 | **OK** |
| apply_sheep_snapshot | 恢复消费（0x1416f24f0，在 systems/debug/sheep_controls.rs 域） | zip 最短序覆盖三字段，不改 rotation/scale、不增删羊 | **OK（实现）/ 待取证（wire）**——本机缺 decomp |
| clone/eq/deserialize glue | 派生胶水 | derive | **OK** |

### 相关类型

- CritterGoal（resources/critters/mod.rs）：Option<Vec3> 16B，serde transparent——快照序列化证其为
  Option<Vec3>（null 无位置载荷）。合理。**OK 注记**。
- 真实 Sheep 所有者（systems/debug/sheep_controls.rs：SheepControl::new 0x1416f0ea0、
  sheep_controls 0x1416f11a0、handle_mouse_picking）不在本组 DISPATCH 范围，但 inventory 显示
  apply_sheep_snapshot 原属该文件；现置于 control.rs 属位置调整，不影响语义。

## 结论

- **已实现并核验**：SheepMoveState wire 变体与字段（对 0x1413ff520 逐条对上）；capture/apply
  语义（查询序、最短 zip、保留无关字段）。
- **证据缺口**：control.rs 快照/恢复的 decomp（0x1416f9eb0/0x1416f24f0）本机不存在，wire 字段序
  未逐条核对——标"待取证"。
- **GAP**：Sheep 组件族（being_petted、register_required_components×5）未建；受 DISPATCH
  "不复制空壳 Sheep"边界约束，但验收门槛"sheep.rs 28 函数 + control.rs 15 函数快照/状态闭合"
  未完全达成（28 项中业务项 only being_petted + 注册胶水）。
- 状态建议：**fix_pending**（或由 coordinator 将 Sheep 组件族显式降为本批边界外并注明）——
  至少补 control.rs 快照 wire 的原生证据后再言闭合。

## 遗留

- systems/debug/sheep_controls.rs（Sheep 所有者/生命周期/鼠标拾取）、save_game 快照往返的
  真实接线：属本模块 remaining。
