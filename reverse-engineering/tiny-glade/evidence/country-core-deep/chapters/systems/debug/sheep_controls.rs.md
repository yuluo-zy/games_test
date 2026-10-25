# systems/debug/sheep_controls.rs

> 文件作用：**绵羊调试控制（拾取/快照捕获/应用）。**

函数总计 11：业务 5 / 内联无独立符号 2 / 派生胶水 4

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::debug::sheep_controls::impl$2::clone_behavior | 0x140002370 | 25 | bevy Component clone_behavior 派生空实现（全局共享地址，非业务） | O（派生胶水） | audio-misc\decomp\decomp-0x140002370.txt, camera-time\decomp\decomp-0x140002370.txt, collision-eco\decomp\decomp-0x140002370.txt |
| country_core::systems::debug::sheep_controls::impl$2::register_required_components | 0x1416f2910 | 25 | SheepControl 组件注册（bevy 胶水） | O（胶水） | audio-misc\decomp\decomp-0x1416f2910.txt |
| country_core::systems::debug::sheep_controls::SheepControl::new | 0x1416f0ea0 | 57 | 调试控制组件构造 | O（2026-10-15）：SheepControl 组件构造（含 PickingMode/快照字段初始化，692B） | eco-reanalyst\decomp\decomp-0x1416f0ea0.txt |
| country_core::systems::debug::sheep_controls::sheep_controls | 0x1416f11a0 | 97 | 调试绵羊控制（PickingMode 拾取） | O（2026-10-15 深挖）：查询 + 拾取 + apply_sheep_snapshot 应用 + 位置/朝向写回 + 事件计数 | audio-misc\decomp\decomp-0x1416f11a0.txt |
| country_core::systems::debug::sheep_controls::apply_sheep_snapshot | 0x1416f24f0 | 358 | 应用绵羊快照 | O（2026-10-15 深挖）：40B SheepSnapshot 元素逐字段拷到 sheep 组件 | audio-misc\decomp\decomp-0x1416f24f0.txt |

## 实现方式（算法级）（2026-10-15 定点深挖）

### apply_sheep_snapshot @0x1416f24f0（O，逐字段拷贝映射）

签名 `(param_1=SystemParam 查询面, param_2=Vec<SheepSnapshot> 40B 元素)`。遍历 Query 命中羊实体，逐元素读 SheepSnapshot 40B、写 sheep 组件多字段：
- **SheepSnapshot 元素 40B** = `{+0x00 u64, +0x08 u64, +0x10 u64, +0x18 u32, +0x1c u64, +0x24 u32}`（步长 5×qword）。
- **写入面（每羊）**：`data+0x00 ← snapshot+0x00`、`data+0x08 ← snapshot+0x08`、`data+0x10 ← snapshot+0x1c`、`data+0x18 ← snapshot+0x24`、`data+0x30 ← snapshot+0x10`、`data+0x38 ← snapshot+0x18`（两个 0x68B 步长组件数组 + 一个 0x30B + 一个 0x10B 组件数组并行写）。
- **附加**：`uVar4 = param_1+0x14` 的 f32（帧时间/公共值）写 3 个 f32 数组（`lStack_60/68/70 + idx*4`）。

### sheep_controls @0x1416f11a0（O，主系统）

- 查询羊实体（`(Entity, ...)` 元组，bevy archetype 位图迭代）；puffin/日志（`cRam0000000143634cd8` demo 门 + 日志级 `0x14362c3a8 > 2`）。
- 条件快照应用：满足拾取条件（`param_7+0xf5/0xf0` 标志）时 `FUN_1416f24f0(plStack_c0, plVar30)` 应用快照；否则（`param_6` 非空）构造 `0x188B` 消息经 `FUN_140adb2f0/FUN_140adb610` 发送（8 槽载荷：auStack_91 判别、+6/+7/+8/+9/+3/+self/cursor 指针等）。
- 位置/朝向写回：命中路径 `*(lVar31+0x10)=uVar38; *(lVar31+0x18)=uVar36`（Transform 写回）；`uStack_100 = CONCAT71(...,1)`。
- 事件计数收尾：`*puVar4 = uVar26 − (uVar24+uVar22)`（Events\<T> 双游标差分），无事件时置 `i64::MIN` 哨兵。

### SheepControl::new @0x1416f0ea0（O）

692B 构造：初始化 SheepControl 组件（PickingMode 判别 + 快照 Option 字段；`0xfffffffffffffffe` = None 哨兵 @+0x40 区）。

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::debug::sheep_controls::PickingMode::is_some`（行 51）— 谓词/校验
- `country_core::systems::debug::sheep_controls::handle_mouse_picking`（行 267）— 系统:事件响应（拾取羊实体）

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 4 项
