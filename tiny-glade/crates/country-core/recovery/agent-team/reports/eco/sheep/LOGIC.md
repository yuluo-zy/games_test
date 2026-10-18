# LOGIC 判等报告 — resources/critters/sheep（logic-verifier-eco-b，2026-10-16 fresh 反编译）

> 依据：`2026-10-16-logic-verify-protocol.md`。只以 fresh 反编译为准（本会话重新
> get_functions 0x14089d4c0/0x140080440/0x1413ff520/0x1413ff8a0/0x1405c9c50/0x1416f9eb0/
> 0x1416f9fd0/0x1416f24f0/0x1412f03f0 + disassemble_function 0x14089d4c0/0x1416f24f0；
> 另解 0x140a3f470 save_game 验证 capture 内联 Query）。
> 审计范围：`src/resources/critters/sheep.rs`、`sheep/control.rs`。

## 函数表

| VA | 函数 | Rust 位置 | verdict | 分歧点 |
|---|---|---|---|---|
| 0x14089d4c0 | `Sheep::being_petted` | sheep.rs:79 | **判等** | 无 |
| 0x1413ff520 / 0x140080440 / 0x1413ff8a0 | `SheepMoveState` serde | sheep.rs:17 | 胶水-非业务（wire 形状验证） | derive；字段名 rne-read 定案 |
| 0x1405c9c50 / 0x1416f9eb0 / 0x1416f9fd0 | `SheepSnapshot`/`SheepControlSnapshot` serde | control.rs:17-27 | 胶水-非业务（wire 形状验证） | derive；字段名 rne-read 定案 |
| 0x1412f03f0（内联） | `capture_sheep_snapshot` | control.rs:31 | **判等（逻辑）/ 组件绑定分歧** | 见 §2 |
| 0x1416f24f0 | `apply_sheep_snapshot` | control.rs:49 | **判等（逻辑）/ 组件绑定分歧** | 见 §2 |
| — | `Sheep` 结构（sheep.rs:61） | sheep.rs:61 | **未实现-需重做（布局矛盾）** | 见 §3 |

## §1 判等函数

- **being_petted**（0x14089d4c0，机器码 4 条）：
  `MOVSS XMM0,[RCX+0x58]; UCOMISS XMM0,[0x142960674]; SETA AL; RET`
  0x142960674 = `0a d7 23 3c` = f32 0.01（read_memory 实证）。`ucomiss+seta` = 严格大于，
  无序(NaN)取 false。Rust `self.being_petted_timer > f32::from_bits(0x3c23d70a)` 逐位一致。**判等**。
- **SheepMoveState serde**（0x1413ff520 Compact / 0x140080440 Value）：判别字节@+0，0=Idle；
  Idle 写 `{"Idle":{...}}` 1×f32；Walking 写 `{"Walking":{...}}` 序 f32,f32,bool；外标签。
  字段名/序（timeout_s, goal_change_timeout, force_reach_goal）rne-read 定案。Rust 声明序一致。胶水。
- **SheepSnapshot/SheepControlSnapshot serde**（0x1416f9eb0：`{`+serialize_entry(Vec3)+
  (Option<Vec3>)+(SheepMoveState)+`}`；0x1416f9fd0：`{`+serialize_entry(Vec<SheepSnapshot>)+`}`）。
  = {position,goal,move_state}、{sheep:Vec}。Rust 一致。胶水。

## §2 capture（0x1412f03f0）/ apply（0x1416f24f0）— 判等（逻辑）+ 组件绑定分歧

**逻辑判等**（save_game 0x140a3f470 内联 capture Query 同型，`init<QueryIter<(Entity,&mut
Transform,&mut Sheep,&mut CritterGoal),()>>` + `from_iter<SheepSnapshot, Map<.., closure_env_0>>`）：
- 查询序迭代，每实体收集 40B（0x28）快照 {position, goal, move_state}；不存实体 id、不排序。
- apply zip 查询序与存档序，`RSI += 0x28` 步进，`RSI == end` 即停（停止于较短者），
  恰好覆盖 3 字段，不增删羊。与 Rust capture/apply（query 序、zip 最短、3 字段覆盖）一致。

**组件绑定分歧（fresh 反编译，关键）**：
apply 每实体写三个组件数组（stride 0x30 / 0x68 / 0x10）+ 3 个 stride-4 change-tick 写：
- **move_state**（快照 +0x1c/+0x24）→ stride-0x30 组件 @+0x10/+0x18
- **goal**（快照 +0x10/+0x18）→ stride-0x68 组件（=Sheep，104B）@+0x30/+0x38
- **position + goal-tag**（快照 +0x00..0x0f 16B）→ stride-0x10 组件 @+0x00
- stride-4 change 写：ECX=[bundle+0x14] → 3 meta 指针（Bevy change detection 胶水）

按 Query (Transform, Sheep, CritterGoal) 与组件大小（utils Transform=0x30/t@0x10、
Sheep=0x68、CritterGoal=0x10）推断原生绑定为：
- **position ← CritterGoal@0x00**（非 Transform.translation）
- **goal ← Sheep@+0x30**（Option<Vec3>；tag 来自 CritterGoal@0x0c）
- **move_state ← utils Transform.t（translation 字段）@+0x10**

即原生把 SheepMoveState 存放在 **Transform 的 translation 槽**、把 position 存在
**CritterGoal**、把 goal 存在 **Sheep@+0x30**。Rust control.rs 用 `(&Transform,&CritterGoal,
&SheepMoveState)` 三元组（position←transform.translation、goal←goal.0、move_state←*move_state）
承载相同 3 个字段值，但绑定目标与原生长相不合：Rust 的 position 源（Transform.translation）
在原生是 move_state 槽，Rust 的 goal 源（CritterGoal）在原生是 position 槽。

## §3 Sheep 结构布局矛盾（sheep.rs:61 注释）

sheep.rs 注释称 `Sheep +0x30 = 12B 内嵌 SheepMoveState`（判别@+0x30、+0x31 force_reach_goal、
+0x38 timeout_s），并引 0x1412f03f0（snapshot iterator）为证。fresh 反编译否定该证：
0x1412f03f0/0x1416f24f0 中 stride-0x68（=Sheep）@+0x30 读/写的是 **goal（Option<Vec3>）**，
move_state 读/写自 stride-0x30（Transform）@+0x10。`being_petted @+0x58`（0x14089d4c0）正确，
但 `+0x30` 字段应为 goal 而非 move_state。Rust `Sheep { move_state: SheepMoveState }` 的
字段放置与原生不符，需重做（完整 Sheep owner 属 systems/critters 跨域，DISPATCH 边界外）。

## 遗留
- apply 原生属 systems/debug/sheep_controls.rs 域（inventory），现置 control.rs 属位置调整，
  不影响本批语义。
- 存档 Sheep 节接线（save_game to_json_string<Option<SheepControlSnapshot>>）为 save 组消费端。
