# B 组 sheep DELIVERY（2026-10-16 第三轮实现）

> 实现对象：sheep-DELTA.md 的真实剩余缺口。组：B。文件：`resources/critters/*`、
> `systems/debug/sheep_controls.rs`（中央 agent 批准落地）、`src/lib.rs`（`pub mod debug;` 接线）。

## 真实代码接线

1. **`Sheep` ECS component**（`resources/critters/sheep.rs`）：stride 0x68 布局注释；
   已证字段 `move_state: SheepMoveState`（native +0x30）与 `being_petted_timer: f32`
   （+0x58）。`+0x00..0x2f / +0x3c..0x57 / +0x5c..0x67` 无 assembly 证据 → 结构体不填占位
   （SPEC §2.1 约束）。
2. **`Sheep::being_petted` 谓词**（0x14089d4c0）：`being_petted_timer > f32::from_bits(0x292f)`。
   常量 0x142930f74 已 rne-read 复核 raw `2f 29 00 00`；`ucomiss+seta` 语义 = 严格大于，
   NaN 为假（与 `>` 一致）。测试锁定。
3. **save "Sheep" 节接口**：`resources/critters/sheep/control.rs` 新增
   `sheep_section_json(Option<&SheepControlSnapshot>) -> String`（O `to_json_string
   <Option<SheepControlSnapshot>>` @0x140a51d40；None→"null"）。中央 agent 已批准接口路径，
   **接线由 C 组 save agent（a9c6f52b7cc06d63f）在 save_game.rs 完成**（定序 B→C）。
4. **真实 query owner**：`systems/debug/sheep_controls.rs`（新建，中央 agent 批准）：
   `SheepControl` 资源（@0x1416f0ea0）+ `sheep_controls` 系统（@0x1416f11a0，query
   `(Entity, &mut Transform, &mut Sheep, &mut CritterGoal)`，按 query 序 capture）+
   `apply_sheep_snapshot` 系统（@0x1416f24f0，zip 短路只写 translation/goal/move_state）。
   `src/lib.rs` 加 `pub mod debug;` 接线。

## candidate 链测试

- `resources/critters/sheep.rs`：being_petted 阈值字节/NaN 语义、Sheep serde round-trip。
- `resources/critters/sheep/control.rs`：`sheep_section_json(None) == "null"`、
  `Some` 的 wire 形状与 capture 结果一致。
- `systems/debug/sheep_controls.rs`：capture 系统按 query 序构建 `Vec<SheepSnapshot>`
  （复制 translation/goal/move_state）；apply 系统只写三字段（同 0x1416f24f0 语义）。

## 原侧验证

- being_petted 常量：`rne-read --hex 0x142930f74` = `2f29000001000000`（f32 u32=0x292f）✓。
- 布局偏移：capture iterator 0x1412f03f0 步进 `uVar*0x68`、move_state@+0x30 判别读、
  +0x58 f32 谓词读（SPEC §2.1 O 表）✓。
- save 节槽位 `SaveFileSection::Sheep = 5` + 名称 "Sheep" + `Option<SheepControlSnapshot>`
  载荷（DELTA 已核对）✓。

## 仍缺项（待取证 / 组外登记）

- Sheep 完整字段布局：`+0x00..0x2f / +0x3c..0x57 / +0x5c..0x67`（SP T1，不凭名猜字段）。
- `register_required_components ×5`（SP T5）：`#[require]` 目标组件需 PDB/消费核对，未落地。
- `sheep_controls` 原生系统内联的 picking/UI 交互分支（拾取后写 move_state=0x101 +
  timeout_s@+0x34、`PickingMode::is_some`）待取证，未伪造。
- spawn（startup_sheep 是资产载入非实体 spawn）与 movement 状态机（旧草稿谓词作废）保持待取证。
- save_game.rs "Sheep" 节实际写盘：**C 组接线中**（仲裁：B 提供接口、C 写盘）。
