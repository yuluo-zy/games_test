# B 组 input SPEC（2026-10-15 第二轮追加）

> 组：B（input 并入）。文件所有权：`crates/country-core/src/systems/backend/input/*`。
> 现状：`calculate_input_delta`（6 模拟 + 40 数字槽，0.1/0.05 repeat 计时）与 `translate_input_to_cursor_moved`
> （事件折叠/窗口钳制）已实现。顶层 `input/*`（action_set/input_stream/mod/panic_repro_state）也已恢复。
> 本 SPEC 覆盖 contracts.json 剩余缺口：**键盘/Steam 映射、reset/replay 集成、update_cursor_position、原全局调度、OS 事件生产者**。
> 证据：`evidence/core-recovery-20261015/input/{delta-first-slots,cursor,mod-key}.asm`、`signatures.json`、
> `cursor-update-procedures.json`、`evidence/country-core-deep/chapters/systems/backend/input/*.rs.md`、
> `ui-input/decomp/decomp-0x1417cd0a0.txt`、`input/ALGORITHMS.md`。
> O=机器码证实；I=推断；U=未知。

---

## 1. 剩余缺口清单

| # | 缺口 | 结论 | 见 § |
|---|---|---|---|
| 1 | 键盘/Steam 映射（translate_input 主生产者） | **O（结构/常量）** @0x1417cd0a0；逐槽 key→action 映射需 GameProfile key_bindings 表 | §2.1 |
| 2 | reset/replay 集成（reset_input_state + 录制回放机） | **O** @0x140942b00 / @0x1409423d0 / @0x140941a30 / @0x14083c620 | §2.2 |
| 3 | update_cursor_position | **O（签名）**；正文全内联，合并 InputCursorMoved→游标、写 CursorMovedScreenSpace | §2.3 |
| 4 | 原全局调度 | **O**：流水线序 translate_input→translate_input_to_cursor_moved→calculate_input_delta→reset_input_state；mod_key+update_cursor_position 成对 | §2.4 |
| 5 | OS 事件生产者 / UiHistoryButtonPressed | **O（签名）**：translate_input 读 ButtonInput<KeyCode/MouseButton>+MouseWheel+InputCursorDelta+UiHistoryButtonPressed；UiHistoryButtonPressed 事件**未在现有 calculate_input_delta.rs** | §2.1, §2.5 |

---

## 2. 原行为契约

### 2.1 translate_input @0x1417cd0a0（键盘/鼠标/Steam → ActionSetValues）

**权威签名（O，`signatures.json` into_configs）**：
```
(Res<GameTime>, Res<ButtonInput<KeyCode>>, Res<ButtonInput<MouseButton>>,
 Option<Res<SteamClientResource>>, Res<EscMenuState>, ResMut<ActionSetValues>,
 ResMut<ActiveInputModeTracker>, EventReader<MouseWheel>, Local<translate_input::MouseMoved>,
 EventReader<InputCursorDelta>, EventReader<UiHistoryButtonPressed>, Res<GameProfile>)
```

**数据流（O，decomp-0x1417cd0a0，34KB）**：
- **输入模式跟踪**：键盘/鼠标事件存在（`ButtonInput<KeyCode>` 队列非空）→ `on_mouse_keyboard_used`（内联）：
  写 `ActiveInputModeTracker.mouse_kb_deadline = now+250ms`（`now`=FUN_140d635e0，冷却常量 0x0ee6b280 ns）。
  Steam 分支（`Option<Res<SteamClientResource>>` 非空且初始化）走 Arc 计数 + 检测。
- **GameProfile 键位表**：`piVar5 = GameProfile 的键位数组`（u32/槽；**0xc3=195=未绑定哨兵**）。
  每个 digital 槽读对应 keycode → `FUN_14089aef0`（= `ButtonInput<KeyCode>::pressed`）→ OR 进
  **ActionSetValues.current digital bool 字节 @+0x88+k（k=0..39）**（+0x88 = current.digital[0]）。
  逐槽判定模式：`if (keycode != 0xc3) { if (pressed) { flags[offset] |= 1; } }`（大量重复；部分槽先
  `FUN_14089a890/FUN_14089ab50`（released/moving 谓词）再组合）。
- **相机模拟（键盘）**：`fVar36 = GameTime.delta(=*(param_1+0x28)) * 30.0`（常量 0x1429cd034）；
  写 current.analog：
  - camera_pan（slot2 @+0x68/+0x6c）±；camera_height（slot3 @+0x70）±；
  - camera_zoom（slot4 @+0x78）：`×1.5`（0x1429258d0）/`×-1.5`（0x1429df180）；
  - camera_rotate（slot5 @+0x80）：`×1800.0`（0x142ce2d30）/`×-1800.0`（0x142ce2d34）。
  该段受 EscMenuState 门控：`tag==2 || (tag==10 && payload!=1)`（O，`*(longlong *)*param_5`）。
- **MouseWheel 折叠**（param_8 Events<MouseWheel>）：双缓冲排空，`(delta - 原点)` 累加，常量
  `+8.0`（0x142934604）/`+1.0`（0x142925904），写 +0x78（zoom）。
- **InputCursorDelta 折叠**（param_10）：同 translate_input_to_cursor_moved 的 `{x u32,y u32}` 语义，
  归一/平方长（SQRT）判定；Local<MouseMoved>（param_9）作 moving 判定的累计。
- **UiHistoryButtonPressed 读**（param_11）：`+0x98`/`+0x99` 字节 |= （undo/redo UI 按钮按下）。

**待取证（不落地）**：每个 digital 槽的**具体 GameProfile 键位索引 ↔ 动作槽映射表**——需 `GameProfile.key_bindings`
（ron 6 字段之一，`game_profile.rs`，0x208 B）的键位表逐项与 `piVar5[k]` 下标对齐；当前 game_profile 未在本工作区恢复。

### 2.2 reset_input_state + 录制/回放集成

**`reset_input_state` @0x140942b00（O，章节复核 + ALGORITHMS §5）**：
1. 清零并重排 `InputTranslationState` 11 qword（0x58..0xa8 区，按录制帧布局）。
2. 按最后 sync point 修剪 InputStream 双 VecDeque（drop 前段）。
3. `ButtonInput<KeyCode>::reset_all`。
4. 保留 InputStream 结构（mode/index 不动）。

**`input_record_replay` @0x1409423d0（O，模式机主系统）**：
- 状态编码：`mode u64 @+0x40`，存储 niche `0x8000000000000000 + 变体`；`^0x8000000000000000` 后
  case 0=AutoRecordPanicRepro/2=Replay/3=DisableInput/4=None；index u64 @+0x48。
- 有未消费事件才进入分支。0（default）：条件 `(队列差非零 || 录制开关 || 工具字节@+0xa3 bit0)` →
  clone ToolbarCategoryModes（FUN_1412f7fa0）+ MainCamera::store_parameters（FUN_140911710）+ 重建流
  （frames cap 8 + 1 sync point）+ `add_frame`（`delta=(cursor-原点)` → 2×2 矩阵 0x142a171f0 + 偏移 → divps 归一）。
- 2（Replay）：index 越界 panic `"Out of bounds access"`；否则读 0xd0B InputFrame →
  `+0x30 u64/+0x38 u32/+0x3c u8`→state+0xe8/f0/f4；`+0x40 双 f32`→游标（f32→u32 clamp）；
  `+0x44` 除数→`norm=(cursor-原点)/除数`；`+0x58..0xa8 11 qword`→InputTranslationState；
  `+0xc0`→`pos'=pos+frame*scale(0x1429258e0/e4=0.5)`。
- 3（DisableInput）：清零 20 qword（InputTranslationState 复位）。4（None）：no-op。

**`advance_input_replay_frame` @0x140942280（O）**：`mode==MIN+2` → `index=index+1`；
`index>frames.len(+0x38)` → log + `mode=MIN+4`（0x8000000000000004）。

**`control_input_replay` @0x140941a30（O）**：键控 mode 切换 + seek sync point（`sync_points.len==0`→panic
`"No sync points found in replay state"`；`mode!=MIN+2`→panic `"Can only seek to sync point if replaying but mode is "`
带 mode fmt）；成功 `index=sync.frame_index-1` + 恢复游标/Toolbar 克隆/MainCamera::load_parameters。
`try_load`（0x1409499c0）：读文件失败→Err（`"Failed to read the file"`）、serde_json 失败→Err
（`"Failed to deserialize"`）、成功→写两 VecDeque + `mode=MIN+2` + `index=-1`。

**`record_and_replay_input_for_trailers` @0x14083c620（O）**：配置字节 @+0xfc==1 才启用 + 命中
`HashMap<KeyCode>` 才动作：Idle/Finished→log `"Start recording"`+store camera+重建流+mode=Recording；
Recording→log `"Finished recording"`+mode=Finished；Replaying→log `"Replay..."`+index=-1+恢复状态。
`do_input_replay_ui` @0x14083cdd0：egui 面板（checkbox 写 @+0xfc、`"Stream frame count: "`、`O/E` 键提示、
`tmp/input_replay.json` 保存、Replay sheep 按钮）。

### 2.3 update_cursor_position（全内联）

**权威签名（O，`cursor-update-procedures.json` into_configs @0x141740630）**：
```
(Res<ActionSetStates>, Res<CursorCapturedPosition>, ResMut<CursorPositionSS>,
 EventWriter<window::CursorMovedScreenSpace>)
```
- 行为（I/章节）：把 `Events<InputCursorMoved>` 折叠进游标位置资源；非负 clamp 写回；`EventWriter<CursorMovedScreenSpace>`
  产出窗口空间游标事件。与 `translate_input_to_cursor_moved` 同帧不同 stage 消费同一事件流（顺序依赖）。
- 与 mod_key 成对进入调度（§2.4）。

### 2.4 原全局调度（O，ALGORITHMS §5 + signatures 链）

每帧输入流水线**顺序**：
```
translate_input @0x1417cd0a0
  → translate_input_to_cursor_moved @0x140b17a80（已实现）
  → calculate_input_delta @0x1416feeb0（已实现）
  → reset_input_state @0x140942b00
mod_key（update_mod_key_state @0x140af9c00）+ update_cursor_position 成对进调度
（into_configs @0x141764a40：tuple<mod_key_system, update_cursor_position>；@0x141740630：update_cursor_position 单入）
录制/回放：control_input_replay / input_record_replay / advance_input_replay_frame / record_and_replay_input_for_trailers
```
当前 `add_input_cpu_consumers` 只链了 `translate_input_to_cursor_moved, calculate_input_delta` —— 需补齐
translate_input、reset_input_state 与 replay 系统，并按上述顺序。

### 2.5 UiHistoryButtonPressed（事件，现有缺口）

- 定义在 `calculate_input_delta` 模块（`signatures.json`：`EventRegistry::register_event<…::calculate_input_delta::UiHistoryButtonPressed>`）。
- **生产者**：UI 历史按钮（undo/redo 键位？）按下写事件；**消费者**：translate_input 的 EventReader（§2.1）。
- 现有 `calculate_input_delta.rs` **未定义该事件** —— 落地项：定义事件 + 生产者（UI 侧，组外）+ 消费（translate_input）。

---

## 3. 可直接落地项（实现 agent 优先）

1. **`reset_input_state`**：`(ResMut<InputTranslationState/流>, ButtonInput<KeyCode> reset_all)` —— 按 §2.2 三项。
2. **`input_record_replay` + `advance_input_replay_frame` + `control_input_replay` + `record_and_replay_input_for_trailers`**
   （`input_record_replay.rs`/`input_replay_for_trailers.rs`）：复用已恢复的 `InputStream`（`input/input_stream.rs` 泛型帧），
   把 `InputFrame`（0xd0B 回放视图，见 ALGORITHMS §2.2）与 `InputReplayCommand` 事件补上。
3. **`update_cursor_position`**：按签名实现折叠 + `CursorMovedScreenSpace` 事件写。
4. **`mod_key` 模块**（`systems/mod_key.rs`，组外目录但属输入链）：`Modifiers` 资源 + `update_mod_key_state` @0x140af9c00
   + `is_disable_snapping_just_pressed` @0x140af9bf0；disc=15(snapping_disable) 派生 disable_snapping 等 3 字节（0x1408c5680 折叠）。
5. **`translate_input` 最小落地**：按 §2.1 结构（模式跟踪 + GameProfile 键位表 + digital bool 写入 + 相机模拟 + EscMenu 门）。
   逐槽 key→slot 映射待 GameProfile 表取证，可先实现"读键位表、0xc3 未绑定、pressed→bool"框架。
6. **全局调度**：`add_input_cpu_consumers` 补齐 translate_input→…→reset_input_state 顺序 + mod_key/update_cursor_position 对。
7. **`UiHistoryButtonPressed` 事件**：定义于 calculate_input_delta，接 UI 生产者（组外仲裁）与 translate_input 消费者。

---

## 4. 需先取证项（不落地、登记）

- **T1 GameProfile.key_bindings 表**：`piVar5[k]`（digital 槽 k 的 u32 keycode）的具体默认绑定值；game_profile.rs 恢复后对齐。
- **T2 InputFrame 完整 serde 字段序**：`[translation_state, prev_normalized_cursor_pos, time_delta_seconds, main_action_set_value]`
  （ALGORITHMS §2.2b，0x1413fe800 visit_str）；回放视图偏移与 serde 视图偏移不同（0xd0B 两视图，见 ALGORITHMS §6.2）。
- **T3 translate_input 逐槽分支细节**：`FUN_14089a890/ab50/a9f0` 三谓词族的确切语义（released/moving/deadzone）与每槽用到哪一个。
- **T4 `Local<translate_input::MouseMoved>` 状态**：原始累积语义（I）。
- **T5 mod_key 的 `ToggleSnapping` Local 与 DecoratorOpEvent 交互**（O 签名，分支细节待取证）。

---

## 5. 排除项（明确不做/后置）

- OS/设备层原始事件生产者（winit/窗口回调）——country-core 以 Bevy `ButtonInput`/`Events<MouseWheel>` 为入口，不重建设备层。
- Steam 工作坊/成就等非输入部分；`update_steam_inputs` @0x140ae9740 的 Steam 控制器专用映射（登记接口，细节待取证）。
- UI（`systems/ui/*`）内写 UiHistoryButtonPressed 的生产者（组外）。
