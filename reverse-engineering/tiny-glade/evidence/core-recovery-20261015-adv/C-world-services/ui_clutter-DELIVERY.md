# ui_clutter-DELIVERY — 实现交付（2026-10-16，C 组实时复合 agent）

> 对照 ui_clutter-DELTA 的合成 agent 应关闭缺口逐项落地。文件：
> `crates/country-core/src/systems/ui_clutter/*`（组内）。生产接线已由
> `systems/history.rs:129,165` → `register_ui_clutter_resources`/`add_ui_clutter_cpu_systems`
> 挂载（DELTA §5 已接生产）。
> 逐项分离：**真实代码接线 / candidate 链测试 / 原侧验证 / 仍缺项**。

## 1. rmb_autoclutter 还原原 picking（DELTA #1）

- **真实代码接线**：`systems/ui_clutter/rmb_autoclutter.rs`：
  - **删除错误 `deleted.0.insert(hit.0)`**（SPEC §1.5：原版 RMB handler 不触碰
    `DeletedAutoClutter` 集合）；
  - **hashbrown entity probe**：`AutoClutterEntityIndex`（hashbrown `HashMap<u64,
    AutoClutterRecord>`，host 维护）`raw_entry` 键查找（与 FUN_1405b0d70 哈希 +
    control-byte probe 语义对应）；未命中 log + 跳过；
  - **旋转数学**：`get_clutter_rot_in_roof_space(record.w/x/y, record.entry_angle)`
    （原死代码已接入转换路径）；
  - **0x48B `EvRightClickedAutoClutter`**（@+0x29=3）生产：entity/ty/position/rotation；
  - **无条件历史 tag 0xd**：经 `RecordAutoClutterHistory` → `defered_autoclutter_history`
    → `History::add(EditType::AutoClutter)`（tag 13 = 0xd）；
  - **`History::start_input_recording` 门控**（O：FUN_140a86270 同名同址），AppMode tag
    传入（Replay 模式拒绝）。
- **原侧验证**：decomp-0x140976ca0.txt 行 70-213（probe/位集/组件表 0x1a8）、254-272
  （0x48B 事件 @+0x29=3）、273（FUN_140a86270）、274-281（alloc 0x18B Arc + tag 0xd）。
- **candidate 链测试**：`AutoClutterEntityIndex` raw_entry probe + `fmod_2pi` 为纯函数可测；
  rmb → record → history 链可经 World 注入验证（未及加单测，登记）。
- **仍缺项**：真实 ECS 实体/组件表机制（0x1a8 组件表 + 位集）为宿主机制，本域以实体索引
  承载；`AutoClutterRecord` 0x58B 布局字段名 U；`FUN_1408b83d0`（ClutterMeshTy::from）的
  精确旋转位换算待取证。

## 2. ui_convert 转换（DELTA #2）

- **真实代码接线**：`systems/ui_clutter/ui_autoclutter.rs`：
  - `AutoClutterParentMap` 从计数器 → **真实 `HashMap<u64, AutoClutterParent>`**（
    `maintain`/`remove`/`parent_of`）；
  - **rotation=0 桩删除**：`get_clutter_rot_in_roof_space` 接入转换路径；
  - **InstaCreateDeco / InstaCreateStairPoint** 生产（wp20 合同最小 struct）：按记录类型
    判别（decomp `if (*puVar22 < 3)` 台阶点分支）分流。
- **原侧验证**：decomp-0x14099ca60.txt 行 444-520（0xa0B 记录 + 0x60B parent 分配 +
  hashbrown insert）、497-505（`if (*puVar22 < 3)` + FUN_1408b83d0）。
- **仍缺项**：`maintain_autoclutter_parent_hashmap` 映射表槽位布局 U（0x60B 载荷字段名）；
  `FUN_1408b8e90`（clutter_mesh_ty_data）+ 完整变换矩阵（常量区 UNK_1429cd1/1429258）未
  复现；`StartPlaceDecoClutterInteraction` 联动未接；递归调 `FUN_140976ca0`（行 1130）
  的 parent 再右键路径未落地。

## 3. decoclutter_show_pins（DELTA #3）

- **真实代码接线**：`systems/ui_clutter/decoclutter_show_pins.rs`：
  - `ShowPinnedDecoClutter` → **56B union 语义**：双 `Vec<u64>` 缓冲（pinned + hits，
    O decomp uStack_1d0/lStack_1c8 + uStack_220/lStack_218）+ show 载荷槽；
  - `decoclutter_show_pins` 遍历 pinned 集合 → 维护 pinned 缓冲 + 发 `PinVisibilityToggle`
    （CPU 侧渲染消费）；
  - `register_required_components` **补真实**：注册 `DecoClutterLock` 组件（空桩修正）。
- **原侧验证**：decomp-0x1412357c0.txt 行 264-312（双 Vec 缓冲）、313-329（遍历 + 组件注册）；
  `register_required_components` @0x141237200。
- **仍缺项**：实体/组件遍历（hashbrown + clutter_mesh_ty_data + UiState::add +
  SignifierFactory + raycast FUN_1408ce180）依赖宿主 ECS；hits 缓冲填充方（宿主 raycast）；
  `DecoClutterLock` 组件在宿主实体上的实际挂载。

## 4. Deletion → suppression 接口（DELTA #4）

- **仍缺项**：generation/mesh suppression 接口未建（SPEC §4"先留真接口"未落地——无系统
  查询 `DeletedAutoClutter` 输出 suppression 集合；`is_deleted` 只读接口已存在）。渲染
  （GPU 上传/绘制）后置。登记待接。

## 5. 事件链生产/消费闭环（DELTA #5）

- **真实代码接线**：
  - `EvRightClickedAutoClutter`：**闭环**——rmb handler 生产 → ui_convert 消费（原"无
    生产者/消费空事件流"已闭合）；
  - `InstaCreateDeco`/`InstaCreateStairPoint` 事件注册 + ui_convert 生产（消费宿主接线）；
  - `RmbAutoClutterHit` 生产者 = 宿主输入域（cross-domain，登记）；`SpawnAutoClutter`
    消费方 = 宿主 deco-clutter 生成（GPU 侧，后置）。
- **仍缺项**：`RmbAutoClutterHit` 生产者（输入域）、`SpawnAutoClutter` 消费方（宿主）、
  `RequestMoveClutter/RequestRotateDecorator/LastClonedClutter`（ui_focus_decoclutter 消费
  面）未接。

## 汇总

- **真实接线**：rmb handler 还原（probe/旋转/0x48B/History tag 0xd/start_input_recording/
  不触碰 DeletedAutoClutter）、ui_convert parent map + 旋转 + InstaCreate 生产、
  decoclutter_show_pins 56B union + 真实组件注册、EvRightClickedAutoClutter 闭环。
- **仍缺项**：parent 映射表槽位 U、实体/组件机制宿主面、RmbAutoClutterHit 生产者、
  SpawnAutoClutter 消费方、suppression 接口（GPU 后置）。
- **原侧验证**：decomp-0x140976ca0/14099ca60/1412357c0 行号对照；常量 base=-π/2、
  0.5、SPIRAL_ANGLE=2.3998277 已 RNE 核实。
- **编译**：组内文件无编译错误；整 crate 被其它组在制改动阻塞。
