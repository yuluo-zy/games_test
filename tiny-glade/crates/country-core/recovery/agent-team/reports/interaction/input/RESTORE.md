# RESTORE — systems/backend/input

实现者: restorer-interaction
日期: 2026-10-15
范围: 本批新建 5 文件 + 2 处 reviewer FIX 纳入;实现与原函数/事件方向以 PDB 签名与机器码裁决。

## 原函数 → Rust 位置 → 证据

| 原函数 | VA | Rust 位置 | 证据 | 状态 |
|---|---|---|---|---|
| `translate_input` | 0x1417cd0a0 | `systems/backend/input/translate_input.rs::translate_input` | 反编译 + PDB 签名 | 结构已实现,键绑定/Steam 待取证 |
| `input_record_replay` | 0x1409423d0 | `input_record_replay.rs::input_record_replay` | decomp-0x1409423d0 + ALGORITHMS §2.6 | O 闭合(重建流为近似) |
| `control_input_replay` | 0x140941a30 | `input_record_replay.rs::control_input_replay` | decomp-0x140941a30 | O 闭合(try_load 待取证) |
| `advance_input_replay_frame` | 0x140942280 | `input_record_replay.rs::advance_input_replay_frame` | decomp-0x140942280 | O 闭合 |
| `reset_input_state` | 0x140942b00 | `reset_input_state.rs::reset_input_state` | wp20 分析 + re-analyst | O 闭合(键盘复位抽象化) |
| `record_and_replay_input_for_trailers` | 0x14083c620 | `input_replay_for_trailers.rs::record_and_replay_input_for_trailers` | decomp-0x14083c620 | O 闭合(同步点子字段待取证) |
| `do_input_replay_ui` | 0x14083cdd0 | `input_replay_for_trailers.rs::do_input_replay_ui` | decomp-0x14083cdd0 | O 串/面板已实现 |
| `update_cursor_position`(inline) | — | `update_cursor_position.rs::update_cursor_position` | 章节(I/O) | 实现(I,低优先级) |
| `normalize_cursor_pos`(inline 223) | — | `input_record_replay.rs::normalize_cursor_pos` | 0x1409423d0 | O |
| `unnormalize_cursor_pos`(inline 228) | — | `input_record_replay.rs::unnormalize_cursor_pos` | 0x1409423d0 | O |
| `calculate_input_delta` | 0x1416feeb0 | `calculate_input_delta.rs`(已有) | fresh 反编译(reviewer 证据) | **FIX 纳入:槽 20..25 Shape B** |
| `translate_input_to_cursor_moved` | 0x140b17a80 | `translate_input_to_cursor_moved.rs`(已有) | PDB + fresh 反编译 | **FIX 纳入:捕获路径 tag 2 跳过** |

## 新增实现与依赖复用边界

- 新类型(均在 `systems/backend/input/`):
  - `InputFrame`(0xd0 B 回放视图)、`InputRecordReplayState`(流+mode+index)、
    `InputTranslationStateResource`(current@0x00+buffer@0x58)、`InputReplayOutputState`、
    `CursorPosition{x,y}`、`InputReplayCommand` 事件、`UiHistoryButtonPressed` 事件、
    `SteamInputActive`、`InputReplayForTrailers`、`KeyButtonInput`/`MouseButtonInput`。
- 复用: `crate::input::{InputStream, InputTranslationState, InputRecordReplayMode}`、
  `ActionSetValues/States`、`CursorPositionSS`、`ToolbarCategoryModes`、`MainCamera`、
  `GameTime`、`EscMenuState`。
- **跨模块最小改动(编译必需,证据支撑原版为 ECS 资源)**: 给 `ToolbarCategoryModes`、
  `MainCamera`、`ActiveInputModeTracker` 补 `#[derive(Resource)]`(原函数以 Res/ResMut
  消费,decomp 证实)。

## Reviewer FIX 裁决

1. **FIX-1(采纳)**: `calculate_input_delta` 槽 20..25(ui_left/right/down/up/tab_prev/tab_next)
   held 分支为 Shape B(`timer<0.2 → disc1;else disc2,val=0.1`),其余槽 Shape A。
   `digital_delta` 增加 `auto_repeat` 参数,`update_action_states` 按槽号分派。
   测试补自动连发用例。
2. **FIX-2(部分采纳)**: reviewer 主张“事件流反转,产出 InputCursorMoved、InputCursorDelta
   为自造”。**PDB 签名否定该主张**:`translate_input_to_cursor_moved` 的签名
   `(Res<CursorPositionSS>, Res<WindowSize>, Res<CursorCapturedPosition>, ResMut<ActionSetValues>,
   EventReader<InputCursorMoved>, EventWriter<InputCursorDelta>)` 明确“消费 InputCursorMoved、
   产出 InputCursorDelta”,方向与既有实现一致。采纳子项:捕获路径只累加 tag 0 相对事件,
   tag 1/2(绝对/RelativeWhileCaptured)跳过(0x140b17a80 机器码)。夹取上界 = WindowSize
   (decomp param_2=WindowSize,非 reviewer 主张的“cursor-192”;192 是 u32→f32 转换魔数
   `0x4330000000000000` 的 f32 值,非夹取界)。
3. **FIX-3(无需改)**: `register_input_events` 注册 `Events<InputCursorDelta>` 正确
   (PDB 证实 InputCursorDelta 是原事件,被 translate_input/steam 消费)。

## 仍缺失/待取证

- `translate_input` 的逐动作键码绑定表(`game_key_bindings` 模块,0xc3 哨兵已知,动作↔槽
  映射 U);Steam 输入路径(`systems::steam` 缺失,`SteamInputActive` 布尔登记);MouseWheel
  消费;UiHistoryButtonPressed 载荷语义。
- `InputReplayCommand` 变体名(0x30 B 事件布局已知,变体名 U);`try_load` 的录制文件 serde
  形态(`{replay_initial_state, first_frame}`);同步点状态块内 Toolbar/相机子字段布局。
- `InputReplayOutputState` 字段语义名(@+0xe8/+0xf0/+0xf4,机器码偏移名)。

## 编译状态

`cargo check -p country-core --lib` 0 error。测试二进制受其他 worker 在途错误
(`systems/ivy/ivy_grower.rs` EventWriter::new、save seek 导入)阻塞,本模块测试逻辑已
手写核验(Shape B 时序、捕获路径 tag 跳过)。

## 二轮 FIX 追加（reviewer-interaction REVIEW2，2026-10-15）

1. **FIX-2 方向裁决确认**: enriched PDB `compiled_instances[0].va = 0x140b17a80`（一轮"0x140b16a80"为十六进制笔误）；PDB 签名 + 两调用者一致 = consumes InputCursorMoved → produces InputCursorDelta；夹取 = WindowSize（`0x4330000000000000`×2 = 2^52 i32→f64 魔数）。一轮"cursor-192"已推翻。
2. **捕获路径 tag 2 修正**: 机器码 `cmp dword [r15+8],1; jne add` → 捕获时 tag 0 **与 tag 2（RelativeWhileCaptured）都累加**，仅 tag 1 跳过；非捕获时 tag 2 跳过。捕获分支改回 `Relative | RelativeWhileCaptured => delta += value`，测试同步改回（update_cursor_position 同理）。

## 三.RE-ANALYSIS2 证据落地（re-analyst-interaction 终轮）

1. **键绑定表定案**: KeyBindings = 40 动作槽 × 12 B `{key u32@0, value u64@4}`、0xc3 哨兵；
   translate_input 逐槽读满 40 槽（i32 索引步 3，0x00..0x6f + 尾 0x70/0x71 u16 = 鼠标区）。
   [`BindingsTable`] 文档已更新（纠正“前 30 槽未取证”）；Steam 门 = Arc 克隆 ClientManager →
   `IsSteamRunningOnSteamDeck`（0x1429234f0），控制器→InputTranslationState 在
   `systems::steam::update_steam_inputs`（跨域登记）。
2. **InputFrame 精确布局 + 对账**: 帧偏移实为状态块 @+0x68..+0xb8（= 状态回写目标 +0x58..0xa8
   + 0x10）；serde 视图字段序 [translation_state@0x00, prev@0xc0, time@0xc8,
   main_action_set_value@0x68]；SyncPoint 0x60B 子字段（+0x18 toolbar current、+0x20/+0x28
   Toolbar Vec、+0x30 camera、+0x58 frame_index）；seek 公式已登记，toolbar/camera 从状态块
   子字段类型化恢复仍待取证。

## 五.卡3 Steam 全链 + 卡4 录制往返（coordinator next-batch，2026-10-15）

**卡3 Steam**（`systems::steam` 不在本 crate，库外接线登记）：
- `SteamInputActive` 文档更新：translate_input 内 Arc 克隆 ClientManager →
  `SteamAPI_SteamUtils_v010`（0x142923570）→ `IsSteamRunningOnSteamDeck`（0x1429234f0）
  门控事件源；controllers→InputTranslationState 实际在 `systems::steam::update_steam_inputs`
  （22 同构块，跨域 ui-input/steam）。
- 键绑定表文档更新：KeyBindings = 40 动作槽 × 12 B `{key u32@0, value u64@4}`、0xc3 哨兵，
  逐槽读满 40 槽（动作↔槽精确映射仍 U）。

**卡4 录制往返**（新增）：
- `InputFrameSerde`（serde 视图字段序 [translation_state, prev_normalized_cursor_pos,
  time_delta_seconds, main_action_set_value]）+ `RecordingFile`
  （`{replay_initial_state, first_frame}`）+ `try_load`（读路径 → serde → 首帧入流，
  mode=Replay、index=-1）。
- `seek_to_sync_point`（index = frame_index−1 + clone toolbar + load_parameters +
  immediately_finish_interpolating；toolbar/camera 子字段类型化待取证）。
- `do_input_replay_ui` 既有；normalize/unnormalize 精确公式既有。
- 文件路径层与 save/session 组的格式约定待协调（本实现接受 JSON 路径）。

## 留给 reviewer 的核对点

1. `digital_delta` 槽 20..25 Shape B 与 fresh 0x1416feeb0 的 `fVar7<0.2/0.1` 分支逐一核对。
2. `translate_input_to_cursor_moved` 方向:核对 PDB 签名 vs reviewer 原主张;捕获路径 tag 2 跳过。
3. `input_record_replay` case 0/1/2/3/4 与 0x1409423d0 的帧打包/解包偏移(@0x30..0xc8)。
4. `reset_input_state` 的 11 qword 下移置换(@0x58..0xa8→@0x00..0x50)与事件排空。
