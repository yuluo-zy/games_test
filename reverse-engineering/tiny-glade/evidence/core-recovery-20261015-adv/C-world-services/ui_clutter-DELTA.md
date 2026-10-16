# ui_clutter-DELTA — 用户批改核对（2026-10-16，对照 ui_clutter-SPEC + 原机器码）

> 核对 agent（C 世界）。文件：`crates/country-core/src/systems/ui_clutter/*`。
> 原侧证据：`country-core-deep/ui-input/decomp/decomp-0x140976ca0.txt`（rmb handler，297 行）、`decomp-0x1412357c0.txt`（decoclutter_show_pins，1128 行）、`decomp-0x14099ca60.txt`（ui_convert）、`core-recovery-20261015/ui-clutter/{record,expand,replay}.asm`、RNE 常量直读（base=-π/2@0x142a10ff0、0.5@0x142925870、SPIRAL_ANGLE=2.3998277@0x142abbcec）。
> **生产接线**：经 `systems/history.rs:129,165` → `register_ui_clutter_resources` / `add_ui_clutter_cpu_systems` 挂载（history 链从 run_game.rs 注册）——**已接生产**。

## 1. handle_rmb_autoclutter 0x140976ca0

| SPEC 项 | 当前状态 | 证据 | 备注 |
|---|---|---|---|
| 原 RMB picking：hashbrown SIMD probe + 实体位集 + 组件表（+0x1a8）+ clutter 记录 0x58B 步长 | 仍缺 | rmb_autoclutter.rs:57-69；decomp 行 70-213 | 当前从 `RmbAutoClutterHit(u64)` 事件（宿主预解析）消费，无任何 picking 逻辑 |
| 旋转计算（三向量点积 + sqrt + CircularF32 + FUN_1408b83d0） | 待取证（死代码） | rmb_autoclutter.rs:39-54 `get_clutter_rot_in_roof_space` | 公式大体对应（base 常量 -π/2@0x142a10ff0 与 0.5@0x142925870 已核实），但 **无任何调用方**（ui_convert 用 `rotation=0.0`） |
| 推 0x48B `EvRightClickedAutoClutter` 事件（@+0x29=3、+0x2c 旋转、+0x30/+0x38、+0x3c uVar14、+0x40 entity） | 仍缺（事件无生产者） | rmb_autoclutter.rs:28-33 只定义 struct；decomp 行 254-272 | `EvRightClickedAutoClutter` 全文无写入方；ui_convert 只读 |
| History::add tag 0xd + 末尾 alloc 0x18B Arc | 有水分（间接化） | rmb_autoclutter.rs:67 → RecordAutoClutterHistory → defered_autoclutter_history → EditType::AutoClutter(tag 13) | 原版直接 `FUN_140a84ec0(..,{0xd,Arc},..)`；当前经事件链间接入 tag 13 |
| `FUN_140a86270`（History::start_input_recording 门控） | 仍缺 | — | decomp 行 273 末尾调用；当前未调 |
| **原版不触碰 DeletedAutoClutter 集合** | **有水分（错误触碰）** | rmb_autoclutter.rs:64 `deleted.0.insert(hit.0)` | SPEC §1.5 明确原版 RMB handler 不写删除集合；当前写入属错误语义 |

## 2. ui_convert_autoclutter_to_decoclutter 0x14099ca60

| SPEC 项 | 当前状态 | 证据 | 备注 |
|---|---|---|---|
| 消费 EvRightClickedAutoClutter → parent 解析 → InstaCreateDeco / InstaCreateStairPoint / 0x48B 事件 | 有水分（桩） | ui_autoclutter.rs:36-53 | `rotation=0.0` 硬编码；仅发 crate 自有 `SpawnAutoClutter`（无消费方）+ 计数 `parents.entries` |
| `get_clutter_rot_in_roof_space`（行 403） | 待取证（未接） | ui_autoclutter.rs 注释引用；decomp-0x14099ca60 | 未实现进转换路径 |
| `maintain_autoclutter_parent_hashmap`（行 410） | 有水分（计数器冒充） | ui_autoclutter.rs:30-33 `AutoClutterParentMap{entries}` | 原版维护真实 parent 映射表（布局 U）；当前仅递增计数 |

## 3. decoclutter_show_pins 0x1412357c0

| SPEC 项 | 当前状态 | 证据 | 备注 |
|---|---|---|---|
| 遍历 pinned clutter 集合切换图钉实体可见性（1128 行 decomp：实体/组件表遍历 + hashbrown + clutter_mesh_ty_data + UiState::add + SignifierFactory + raycast FUN_1408ce180） | **有水分（桩）** | decoclutter_show_pins.rs:22-30 六行循环 | 只对 `PinnedDecoClutter` HashSet 逐项发 `PinVisibilityToggle` 事件；原版全部实体机制未复现 |
| `register_required_components` 0x141237200（bevy 组件注册） | 有水分（空桩） | decoclutter_show_pins.rs:40 `pub fn register_required_components(_world){ }` | 空函数冒充 O 注册 |
| `clone_behavior` 0x140002370 | 真实实现（ICF 单行） | decoclutter_show_pins.rs:44-45 | 行为标记，无独立逻辑，符合 |

## 4. ui_focus_decoclutter 0x1409a92c0 / debug_spawn_clutter 0x1409a1f50

| SPEC 项 | 当前状态 | 证据 | 备注 |
|---|---|---|---|
| DecoratorId → decorator_to_storage 索引 → DecoratorStorage::get → 内嵌 map → DecoratorInfo（RE §4 主读路径） | 部分（读路径真实） | ui_focus_decoclutter.rs:31-51 调 `storage.lookup_storage(id)` + `storage.decorator_info(id)`（decorator_storage.rs:417,427 真实方法） | 存储读路径真实；实体/相机联动（clutter_mesh_ty_data/UiState::add/SignifierFactory/EventWriter×4）未接，注释如实登记 |
| debug_spawn：螺旋 0x2a=42、r=sqrt(n)、θ=n*2.3998277、类型字节表 | 部分（O 部分真实） | debug_spawn_clutter.rs:58-63,28-31 | SPIRAL_ANGLE@0x142abbcec=2.3998277 ✓、类型表/计数 ✓；entity 计数/高度采样/0x48B 消费为 I 适配 |
| 消费删除 ID 的 generation/mesh owner（suppression，GPU 后置留真接口） | 部分 | mod.rs:26-33 `DeletedAutoClutter` + `is_deleted` | 权威删除集合存在；generation/mesh suppression 接口未建（SPEC §4"先留真接口"未落地——无系统查询 DeletedAutoClutter 输出 suppression） |

## 5. 接线汇总

| SPEC 项 | 当前状态 | 证据 | 备注 |
|---|---|---|---|
| ui_clutter 系统生产注册 | 真实实现 | history.rs:129,165 → register_ui_clutter_resources + add_ui_clutter_cpu_systems（6 系统） | 已接生产 Schedule（handle_rmb/ui_convert/debug_spawn/decoclutter_show_pins/ui_focus/delete_auto_clutter/defered） |
| `RmbAutoClutterHit` 生产者 | 仍缺 | — | 系统只读该事件；crate 内无生产者（宿主注入） |
| `EvRightClickedAutoClutter` 生产者 | 仍缺 | — | 无系统写；ui_convert 消费空事件流 |
| `SpawnAutoClutter` 消费方 | 仍缺 | — | 只写无读（deco-clutter 生成依赖宿主） |

## 合成 agent 应关闭的真实剩余缺口

1. **rmb_autoclutter 还原原 picking**：hashbrown probe + 组件表 + 0x58B clutter 记录 + 旋转数学 + 0x48B `EvRightClickedAutoClutter` 生产 + 直接 `History::add`(tag 0xd) + `start_input_recording`。删除当前 `deleted.0.insert`（原版不写 DeletedAutoClutter）。
2. **ui_convert**：实现真实 parent/roof transform 解析 + `get_clutter_rot_in_roof_space`（当前 rotation=0 桩）+ InstaCreateDeco/StairPoint 生产；替换 `AutoClutterParentMap` 计数器。
3. **decoclutter_show_pins**：1128 行 decomp 的实体遍历/可见性切换/`register_required_components` 实注册。
4. **Deletion → generation/mesh suppression 接口**：建真实 CPU 侧消费系统（查询 DeletedAutoClutter → 输出 suppression 集合）。
5. **`EvRightClickedAutoClutter`/`RmbAutoClutterHit`/`SpawnAutoClutter` 生产/消费闭环**：当前事件链无生产者或消费方。
6. **旋转公式常量取证**（相机方向 0x1429a9080..88=(0,0,-1) 已读）并把死代码 `get_clutter_rot_in_roof_space` 接入转换路径。
