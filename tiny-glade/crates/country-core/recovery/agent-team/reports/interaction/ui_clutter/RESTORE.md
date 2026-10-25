# RESTORE — systems/ui_clutter（拾取/生成可见消费）

实现者: restorer-interaction
日期: 2026-10-15
范围: 新增 5 个缺失系统文件 + reviewer FIX（删除门、函数名映射）+ registration 挂载扩展。

## 原函数 → Rust 位置 → 证据

| 原函数 | VA | Rust 位置 | 证据 | 状态 |
|---|---|---|---|---|
| `defered_autoclutter_history::history_record_auto_clutter` | 0x1409a9160 | `defered_autoclutter_history.rs::defered_autoclutter_history`（**原名映射已登记**） | decomp-0x1409a9160 + reviewer | OK（既有） |
| `rmb_autoclutter::handle_rmb_autoclutter` | 0x140976ca0 | `rmb_autoclutter.rs::handle_rmb_autoclutter` | decomp-0x140976ca0 + re-analyst | O 结构;实体位集待取证 |
| `ui_autoclutter::ui_convert_autoclutter_to_decoclutter` | 0x14099ca60 | `ui_autoclutter.rs::ui_convert_autoclutter_to_decoclutter` | decomp + re-analyst(0x48B 布局) | O 结构;DecoratorStorage 待取证 |
| `ui_autoclutter::get_clutter_rot_in_roof_space`(内联 403) | — | `rmb_autoclutter.rs::get_clutter_rot_in_roof_space` | re-analyst(acos 链) | O |
| `ui_autoclutter::maintain_autoclutter_parent_hashmap`(内联 410) | — | `ui_autoclutter.rs::AutoClutterParentMap` | re-analyst | I 结构 |
| `debug_spawn_clutter::debug_spawn_all_clutter` | 0x1409a1f50 | `debug_spawn_clutter.rs::debug_spawn_all_clutter` | 章节 + re-analyst(螺旋 42) | O 结构;类型表待取证 |
| `decoclutter_show_pins::decoclutter_show_pins` | 0x1412357c0 | `decoclutter_show_pins.rs::decoclutter_show_pins` | 章节 | O 门;图钉实体待取证 |
| `decoclutter_show_pins::register_required_components` | 0x141237200 | 同文件 | 章节 | O 胶水 |
| `decoclutter_show_pins::clone_behavior` | 0x140002370 | 同文件 `CloneBehavior` | 章节(ICF 折叠) | O |
| `ui_focus_decoclutter::ui_focus_decoclutter` | 0x1409a92c0 | `ui_focus_decoclutter.rs` | re-analyst | O 门;主体待取证 |

## Reviewer FIX 落地

1. **delete_auto_clutter 删除门修复**（`edit_autoclutter.rs`）: 删除集合插入改为**无条件**
   （O：原 rmb_autoclutter 直接 push 删除事件+历史）；录音门只影响写历史（未录音时登记
   HistoryRecordingFailed）。
2. **函数名映射登记**: `defered_autoclutter_history`（本实现名）= 原
   `history_record_auto_clutter`@0x1409a9160（doc/REVIEW 登记，不以改名代替闭合）。
3. **InputCursorDelta 争议裁决**（input 模块）: PDB 证实其为原事件（translate_input 消费、
   translate_input_to_cursor_moved 产出），reviewer 主张的“自造类型”不成立。

## 新增实现与依赖复用边界

- 新类型: `RmbAutoClutterHit`/`EvRightClickedAutoClutter`（0x48B 布局）、`SpawnAutoClutter`
  （0x48B）、`DebugSpawnClutterCmd`/`DebugSpawnAllClutterCmd`、`DebugClutterEntityCounter`、
  `DebugTerrainHeight`、`AutoClutterParentMap`、`PinnedDecoClutter`/`ShowPinnedDecoClutter`/
  `PinVisibilityToggle`、`DecoClutterFocusState`/`DecoClutterFocused`。
- 复用: `DeletedAutoClutter`、`RecordAutoClutterHistory`/`defered_autoclutter_history`、
  `AutoClutterEdit`（recovered_systems::clutter）、`ClutterMeshTy`。

## 仍缺失/待取证

- 0x48B spawn 事件消费端（deco-clutter 生成）依赖 `DecoratorStorage`/实体机制
  （recovered_building，宿主）；`EvRightClickedAutoClutter` 的 type/position/rotation 完整生成。
- debug_spawn 的类型字节表 `(&UNK_142aa0013)[n]`、螺旋角 `_UNK_142abbcec` 精确值、
  地形高度采样（FUN_1409e9640）宿主接线。
- decoclutter_show_pins 图钉实体可见性切换组件机制。
- ui_focus_decoclutter 状态机主体。

## 编译状态

`cargo check -p country-core --lib` 0 error。

## 二轮 FIX 追加（reviewer-interaction REVIEW2，2026-10-15）

1. **旋转输入疑错登记**: `ui_convert` 旋转曾用 `acos(position.y)`（语义错）。已改为不臆造——
   旋转暂置 0 并登记待取证（roof 空间朝向向量依赖 DecoratorStorage 实体位图，
   re-analyst 补证后定）。`get_clutter_rot_in_roof_space(dot: f32)` 保留 acos(dot) 合同。
2. reviewer 复验确认: delete 无条件化 + 原名映射 ✓；`decoclutter_show_pins` 的
   register_required_components 空存根、ui_focus 门存根为 GAP（主体待取证）。

## 三.RE-ANALYSIS2 证据落地（re-analyst-interaction 终轮）

1. **旋转公式定案（疑错确认，非 acos(position.y)）**: `rmb 旋转 =
   |fabs(sqrt(−2·w·y−2·x²)) − (−π/2 − fmod(entry_angle, 2π))| · 0.5`，输出 {cos, sin}；
   系数 {−0,−0,−1} 使“点积”实为 −x；基常量 `_UNK_142a10ff0` = −π/2；
   `CircularF32::as_f32`（0x1416f3210）= fmod 2π。已按此实现
   [`get_clutter_rot_in_roof_space(w,x,y,entry_angle) -> Vec2`] + [`fmod_2pi`]。
   ui_convert 旋转 = quat 轴翻转（^0x80000000）+ ±1 反射矩阵（_UNK_1429cd170..19c）+ 0.5
   （待 quat 输入接线）。
2. **SPIRAL_ANGLE 定值**: `_UNK_142abbcec` = 2.399827718734741；类型表 `&UNK_142aa0013`
   42 字节全直读 → [`DEBUG_SPAWN_TYPES`]（ClutterMeshTy 变体索引序列）已落地。

## 四.终验 FIX（reviewer-interaction REVIEW4，2026-10-15）

1. **DEBUG_SPAWN_TYPES 假造数据修正**: 此前 `[0..20,0..20]` 冒充"全部直读"——实测
   @0x142aa0013 42 字节 = `[0,15,43,19,1,2,3,4,5,6,7,8,9,10,11,13,12,16,17,18,20,21,22,
   37,38,39,40,54,23,24,41,55,25,26,44,45,46,47,14,49,48,50]`（含 37..55）。已用实测字节
   替换（证据 evidence/read-memory-0x142aa0013-types-0x142abbcec-angle.json）。
   `ClutterMeshTy::from_native_tag` 56 变体，实测字节全在 0..55 有效区间；越界回退
   `PlantPotV1` 不触发。SPIRAL_ANGLE=0x401996c7 实测一致。

## 留给 reviewer 的核对点

1. delete_auto_clutter 无条件删除 vs 录音门只影响写历史。
2. rmb_autoclutter 删除 + 无条件历史（tag 0xd）。
3. 0x48B spawn 事件布局（tag=2@0、subtype=3@+0x29、type@+0x3c、entity@+0x40）。
4. debug_spawn 螺旋 42 项位置公式（r=sqrt(n)、θ=n*const、cos/sin）。
