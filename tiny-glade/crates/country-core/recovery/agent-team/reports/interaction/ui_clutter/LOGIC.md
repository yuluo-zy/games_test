# 逐函数逻辑判等 — systems/ui_clutter

审计员: logic-verifier-terrain
日期: 2026-10-17
协议: planning/2026-10-16-logic-verify-protocol.md
方法: fresh 反编译（ghidra shared 只读，fields=signature,decompiled_code,callees）→ 重建控制流 → 逐语句对照 Rust body。
fresh 反编译证据: recovery/agent-team/work/logic-verifier-terrain/jobs/ui_clutter-result.json（decomp/0x1409*.txt）。

## 结论摘要

- 已审计业务函数 6：**判等 1**（附注）、**未实现-需重做 5**。
- 胶水-非业务（不计审计）: `register_required_components`@0x141237200（bevy register_component_inner 机器，Rust `world.register_component::<DecoClutterLock>()` 委托等价）、`clone_behavior`@0x140002370（bevy Component::clone_behavior 空实现 `return;`，Rust 空标记 struct）。
- 所有可见消费函数（rmb/ui_convert/debug_spawn/decoclutter_show_pins/ui_focus）**均未判等**，内部逻辑与 Rust 大相径庭，与 RESTORE 的"O 结构"登记不符。

## 函数表

| VA | 原函数 | Rust 位置 | Verdict | 分歧点 |
|---|---|---|---|---|
| 0x1409a9160 | history_record_auto_clutter | defered_autoclutter_history.rs | **判等**（附注） | 逐事件（stride 0x10）alloc 0x18 记录 {tag 1,1, id u64} → History::add。主路径吻合。分歧: Rust 加 `if let Err → HistoryRecordingFailed` 错误分支，原版忽略 History::add 返回值（无失败事件） |
| 0x140976ca0 | handle_rmb_autoclutter | rmb_autoclutter.rs | **未实现-需重做** | ①**旋转公式不吻合**: fresh 反编译是 `atan2f(2·ea·x+2·w·y, 2y²-w²-x²+ea²)` 链 + `fmod(atan2f−(−π/2−fmod(entry)),2π)` + 半角 sin/cos 输出；Rust `get_clutter_rot_in_roof_space = \|sqrt(−2wy−2x²)−(−π/2−fmod(entry))\|·0.5` 与 fresh 不一致（re-analyst"定案"未在 fresh 复现）；②**probe 未中原版 `unwrap_failed` panic**（LAB_140976f23），Rust `log::warn + continue`；③原版在 found 路径**直接 History::add**（tag 0xd 记录 {1,1,id}）且**无条件**（无录音门），Rust 走 RecordAutoClutterHistory 事件 → defered 链再 add（机制不同，结局近似）；④原版入口另有 `if *(in_RDX+0x18)!=0` 直达路径（Rust 无此门）；⑤0x48B 事件（subtype=3@+0x29、rotation@+0x2c、type@+0x3c、entity@+0x40）布局吻合；delete 无条件化（删除事件+历史无录音门）**证实成立** |
| 0x14099ca60 | ui_convert_autoclutter_to_decoclutter | ui_autoclutter.rs | **未实现-需重做** | ①**parent hashmap 维护缺失**: 原版内联 `maintain_autoclutter_parent_hashmap`（puffin scope 名见于反编译）+ `HashMap<DecoratorId, Vec<DisplayChildAutoClutterInfo>>`（reserve_rehash<…>、grow_one<DisplayChildAutoClutterInfo>）在函数内维护；Rust 的 AutoClutterParentMap 是**独立资源且无人填充**；②事件类型门 `*puVar19 < 3` 走台阶分支吻合（Rust `(record.ty as u8)<3`）；③0x48B 事件、ClutterMeshTy::from/clutter_mesh_ty_data 调用存在；Rust 的 InstaCreateDeco/InstaCreateStairPoint 为 wp20 合同 I 编造 struct；④旋转同 rmb（公式不吻合） |
| 0x1409a1f50 | debug_spawn_all_clutter | debug_spawn_clutter.rs | **未实现-需重做** | 螺旋位置/类型表吻合: `r=sqrt(n)`、`θ=n·2.3998277`、cos/sin 定位、`&AVAILABLE_DECOCLUTTER[42]` 类型字节、TerrainHeightsData::sample_at_world_xz 高度。**分歧**: ①两循环都调 `History::start_input_recording()`（Rust 完全缺）；②**螺旋 rotation 原版写 0**（`*(+0x2c)=0`），Rust `rotation: theta_for_spiral(n)`=n·SPIRAL_ANGLE；③单条 spawn 位置来自事件载荷（uStack_9a），**不采地形高度**，Rust 用 `(0, height.sample, 0)`；④entity id 原版用 Events 内部计数（+0x40），Rust 用独立 DebugClutterEntityCounter |
| 0x1412357c0 | decoclutter_show_pins | decoclutter_show_pins.rs | **未实现-需重做** | 原版逐 pinned clutter（hashbrown 0x98B 记录）:**GlobalId 哈希探测 + clutter_mesh_ty_data + Rectangle3d::expand/set_orientation_xz/translate + intersects + 多条件门（`*local_1b0==2/1`、`[10]==2`）+ panic_fmt**，并维护 56B union 双 Vec。Rust 仅 `state.pinned.clear(); for &entity in pinned { push; toggled.write(PinVisibilityToggle{visible: show.show}) }`——无几何/无探测/无条件门，纯自造近似 |
| 0x1409a92c0 | ui_focus_decoclutter | ui_focus_decoclutter.rs | **未实现-需重做** | ①**门值错**: 原版 `*(int*)*in_RCX != 3 → return`（仅 disc==3 进入）；Rust `state.tag == 0 → return`（tag∈{1,2,4+} 错误继续）；②原版哈希探测 decorator_id → `DecoratorStorage::get` + 门 `result!=null && result[+0x48]!=0`；③**主体大段未实现**: 原版经几何运算/事件写生产 **PlayDecoratorSoundCmd** 等多路事件；Rust 只写单个 DecoClutterFocused（编造） |
| 0x141237200 | register_required_components | decoclutter_show_pins.rs | 胶水-非业务 | bevy register_component_inner 全机器（TypeId 0x1ddeca6f5ff4d997/0x1f83c18c67995024、ComponentInfo::new、RequiredComponent 表）；Rust 委托 world.register_component 等价 |
| 0x140002370 | clone_behavior | decoclutter_show_pins.rs | 胶水-非业务 | `return;` 空实现（bevy Component::clone_behavior 折叠） |

## 严重分歧汇总

1. **旋转公式未复现**: rmb/ui_convert 的 roof-space 旋转 fresh 反编译 = atan2f 链 + fmod + 半角 sin/cos，Rust `get_clutter_rot_in_roof_space` 的 `|sqrt(−2wy−2x²)−(−π/2−fmod(entry))|·0.5` 公式在 fresh 中无对应——re-analyst"定案"未通过 fresh 复核。
2. **parent hashmap 维护缺失**: ui_convert 内联的 maintain_autoclutter_parent_hashmap 是原版真实逻辑，Rust 用未填充的独立资源代替。
3. **debug_spawn 行为错**: start_input_recording 缺失、螺旋 rotation 应为 0（Rust 写 n·angle）、单条 spawn 不采地形高度、entity 计数机制不同。
4. **decoclutter_show_pins / ui_focus_decoclutter 自造近似**: 原版几何/探测/多事件生产 vs Rust 单事件骨架；ui_focus 门值 3 vs 0。
5. **delete 无条件化证实**: handle_rmb_autoclutter 的删除事件 + History::add 均无条件（无录音门），与 RESTORE FIX-1 一致。

## 方法备注

- 全部以 fresh 反编译为准；RE-ANALYSIS/REVIEW 的"定案/O"仅在 fresh 复现时采信。
- 判等附注 = 错误路径适配或已登记 I 项，不影响主路径。
- edit_autoclutter.rs 的 `delete_auto_clutter` 为 crate 适配器（非原函数），其"删除无条件插入 + 录音门只影响写历史"与 handle_rmb 原版无条件删除/历史一致。
