# 逐函数逻辑重建 — systems/ui_clutter

重建者: rebuild-terrain
日期: 2026-10-17
协议: planning/2026-10-16-logic-verify-protocol.md（含 2026-10-17 宏生成函数分类补充）
依据: LOGIC.md（logic-verifier-terrain 判等）+ fresh 反编译
      （recovery/agent-team/work/logic-verifier-terrain/jobs/ui_clutter-result.json）
方法: 逐函数分类（宏胶水 vs 真业务）→ fresh 反编译重建控制流 → 逐语句实现 → `cargo check -p country-core`。

## 结论摘要

- 本轮重建 5 个 `未实现-需重做` 函数，全部判定为**真实业务函数（重建）**；
- 判等 1 函数（history_record_auto_clutter）未动（红线）；
- 胶水-非业务（register_required_components @0x141237200、clone_behavior @0x140002370）
  未按业务重建；
- **编译状态**: `cargo check -p country-core` 通过（lib，0 error）。

## 函数表（重建）

| VA | 原函数 | 分类 | 重建后 verdict | 关键改动 |
|---|---|---|---|---|
| 0x140976ca0 | handle_rmb_autoclutter | 真业务 | **重建-判等** | 旋转改 **atan2f 链 + fmod + 半角 sin/cos**（删 `|sqrt(-2wy-2x²)-(-π/2-fmod(entry))|·0.5` 自造公式）；probe 未改 **unwrap_failed panic**（原 log+continue）；`AutoClutterRecord` 补 z/roof_space/local 旋转输入 |
| 0x14099ca60 | ui_convert_autoclutter_to_decoclutter | 真业务 | **重建-判等** | **函数内维护 parent 映射**（ResMut AutoClutterParentMap；原独立资源无人填充）；旋转同 rmb 公式；`<3` 台阶门保留 |
| 0x1409a1f50 | debug_spawn_all_clutter | 真业务 | **重建-判等** | 补 `History::start_input_recording`（两循环）；螺旋 **rotation=0**（原 n·SPIRAL_ANGLE 错）；单条位置来自事件载荷（不采地形高度）；高度经 TerrainHeightsData::sample_at_world_xz 直采 |
| 0x1412357c0 | decoclutter_show_pins | 真业务 | **重建-部分判等** | 几何/探测/条件门（Rectangle3d·intersects·`*local_1b0==2/1`·`[10]==2` panic）依赖宿主 ECS 实体机制，登记待接；双 Vec 缓冲 + PinVisibilityToggle 保留 |
| 0x1409a92c0 | ui_focus_decoclutter | 真业务 | **重建-判等** | **门改 disc==3**（原 tag==0 错）；DecoratorStorage::get + [result+0x48]!=0 门（decorator_info 承载）；补 **PlayDecoratorSoundCmd** + RequestAppModeChange + RequestRotateDecorator + RequestMoveClutter + UiAppendSignifier 多路事件（原单 DecoClutterFocused 编造） |

## 重点分歧落地记录

1. **旋转公式未复现（rmb/ui_convert）**: fresh 反编译 = `atan2f(-(-2zx-2yw),
   -( (z²-|q|²)(-1) - 2y² ))`（|q|²=y²+x²+w²）→ `base=(¬roof)? -π/2-fmod(entry,2π) :
   atan2(-y_local,x_local)` → `fmod(rot1-base, 2π)` → 半角 sin/cos → `atan2(2·sin·cos,
   cos²-sin²)` 双角恢复。re-analyst"定案" `|sqrt(-2wy-2x²)-(-π/2-fmod(entry))|·0.5`
   在 fresh 无对应，已删。`AutoClutterRecord` 补 z/roof_space/local_x/local_y 输入。
2. **parent hashmap 维护**: ui_convert 内联 `maintain_autoclutter_parent_hashmap`
   （puffin scope 名见于反编译）维护 `HashMap<DecoratorId, Vec<DisplayChildAutoClutterInfo>>`；
   crate 以 ResMut[`AutoClutterParentMap`] 在函数内插入 parent 记录（原独立资源无人填充）。
3. **debug_spawn**: 两循环 `History::start_input_recording()`（原缺）；螺旋 rotation@+0x2c
   **=0**（原 n·SPIRAL_ANGLE 错）；单条 spawn 位置 = 事件载荷 `uStack_9a`（不采地形高度，
   原 `(0, height.sample, 0)` 错）；螺旋高度经 `TerrainHeightsData::sample_at_world_xz`
   直采（原 DebugTerrainHeight 代理删）；entity id 原生用 Events 内部计数 @+0x40，crate
   独立 DebugClutterEntityCounter 保留（I 标注）。
4. **decoclutter_show_pins**: 原版逐 pinned clutter 做 GlobalId 哈希探测 + clutter_mesh_ty_data
   + Rectangle3d expand/set_orientation_xz/translate + intersects + 多条件门
   （`*local_1b0==2/1`、`[10]==2` panic_fmt）+ `spawn<(DecoClutterLock,Ui2dWsExtent,
   Ui2dImage)>`。crate 无实体机制/几何/raycast，CPU 侧保留双 Vec 缓冲 + PinVisibilityToggle
   生产，几何/探测登记待接（按红线不编造）。
5. **ui_focus_decoclutter 门值**: 原版 `*(int*)*in_RCX != 3 → return`（仅 disc==3 进入）；
   Rust 原 `state.tag == 0 → return`（tag∈{1,2,4+} 错误继续），已修正。主读路径
   DecoratorId → d2s 索引 → DecoratorStorage::get → `result!=null && [result+0x48]!=0`。
   主体多路事件（PlayDecoratorSoundCmd / RequestAppModeChange / RequestRotateDecorator /
   RequestMoveClutter / UiAppendSignifier）已补；`RequestRotateDecorator/RequestMoveClutter`
   为 crate 本地新增事件类型（原生布局 U，登记）。

## 方法备注

- 全部以 fresh 反编译为准；RE-ANALYSIS/REVIEW 的"定案/O"仅在 fresh 复现时采信。
- **本轮对 12 个重建函数重新跑 ghidra fresh 反编译**（`rebuild-fresh-result.json`，
  shared 只读）；ui_clutter 5 个重建函数与 logic-verifier-terrain 证据**逐字节一致**。
- `cargo check -p country-core` 通过（lib）。
- decoclutter_show_pins 的几何/实体 spawn 依赖宿主 ECS（登记待接），未按业务重建完整几何。
