# sheep fresh 复核报告（restorer-eco-fresh，2026-10-17）

> 只以 fresh 反编译为准（Ghidra `--worker shared` 2026-10-17 19:32）：
> capture 迭代器 0x1412f03f0、apply 0x1416f24f0、serde 族 0x1416f9eb0 等。
> 范围：Sheep 结构布局 + capture/apply 绑定（本轮 11 项中的 sheep 项）。

## 函数/类型表

| VA | 项 | verdict | 说明 |
|---|---|---|---|
| 0x1412f03f0 | capture_sheep_snapshot | **判等** | Query = `(Entity, &mut Transform, &mut Sheep, &mut CritterGoal)`；快照 40B：`position ← CritterGoal@+0x00`（含 tag@+0x0c）、`goal payload ← Sheep@+0x30/+0x38`、`move_state ← Transform@+0x10/+0x18`（12B translation 槽）。当前实现三元组 `(&Transform,&Sheep,&CritterGoal)` 逐绑定一致。 |
| 0x1416f24f0 | apply_sheep_snapshot | **判等** | 每实体写：`Transform@+0x10/+0x18 ← snapshot+0x1c/+0x24`、`CritterGoal 16B ← snapshot+0x00..0x0f`、`Sheep@+0x30/+0x38 ← snapshot+0x10/+0x18`；zip 最短、query 序、不增删羊。当前实现一致。 |
| 0x14089d4c0 | Sheep::being_petted | 判等（不动） | `[+0x58] > 0.01`（0x142960674）。 |
| — | Sheep 布局 | **已修正** | `+0x30 = goal Vec3`（Option tag 在 `CritterGoal@+0x0c`）；`move_state` 在 utils `Transform.translation`；`position` 在 `CritterGoal@+0x00`。旧“+0x30 内嵌 SheepMoveState”注释已被 fresh 否定。 |
| 0x1413ff520 等 | SheepMoveState / Snapshot serde | 胶水（wire 判等） | 不动。 |

## 本轮变更

1. `src/resources/critters/sheep.rs`：新增原生事件
   `CmdCheckForSheepInsideBuilding`（16B；ivy pruner 的脏墙事件之一，
   事件 +0x08 = `WallId`）。这是 sheep 域类型，供 ivy_pruner 7 参原生签名
   使用；+0x00 字未被 ivy 消费，未造字段。
2. 其余 sheep 代码仅复核，无逻辑改动。

## 测试

- `cargo check -p country-core --lib` 通过。
- `cargo test -p country-core --lib critters::sheep`：**6 passed**
  （being_petted 阈值/NaN、serde wire、capture/apply 绑定与 zip 短路、
  move_state↔translation 三变体双射）。
- 全库 `cargo test -p country-core --lib`：1974 passed / 0 failed。

## 登记

- 完整 Sheep owner（其余 ~20 字段）/ 4 个宿主组件仍属 systems/critters
  跨域，DISPATCH 边界外，未造。
- `resources::events::OnCircleRotated` 跨域未声明；ivy_pruner 暂用
  `OnWallChanged` 等价脏墙源，待该事件落地后恢复 7 参原生签名。
