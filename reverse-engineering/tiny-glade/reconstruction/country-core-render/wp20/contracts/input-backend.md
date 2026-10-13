# WP20 Contract — `systems::backend::input/*`（backend-input 系统）

Source: `skeleton/B/crates/country-core/src/systems/backend/input/{calculate_input_delta,
input_record_replay,input_replay_for_trailers,reset_input_state,translate_input,
translate_input_to_cursor_moved,update_cursor_position}.rs`
Evidence: Ghidra decompiles in `analysis/b1_input/`；PDB bundles 同名。与 wp15 `input/` 类型
（`InputRecordReplayState`/`InputStream`/`InputFrame`/`ActionSetStates` 等）对接，不重复定义。

标注：**O**=反编译/PDB 直接支持；**I**=推断；**U**=未恢复（typed gap）。

## 模块与业务逻辑

- **input_record_replay.rs**（O）：
  - `input_record_replay` @0x1409423d0：按 `mode@+0x40`（`mode ^ 0x8000000000000000` 的 case
    0/2/3/4）分发：
    - case 0（Idle）：满足条件（工具/画布字节 @+0xa3 bit0 等）→ 开始录制：`MainCamera::store_
      parameters()`、克隆 `ToolbarCategoryModes`、用 1 个 sync point 重建 `InputStream`
      （`puVar5[0..7]` 写流，`[5]=8` 容量），随后 `InputStream::add_frame()` 用当前游标
      delta（`add_frame` 的 0xd0 B 帧）推帧。
    - case 2（Replaying）：读 `frames` 队列第 `index` 帧（stride **0xd0**）：帧 @+0x30 起
      （u64+u32+u8）写回输出 @+0xe8/+0xf0/+0xf4，@+0x40 双 u32→游标 delta，@+0x44 除数 →
      缩放，@+0x58..+0xc0 十 qword → 各输出。
    - case 3（Recording）：清零 21 qword（`InputTranslationState` 区段）。
    - case 4（Finished）：空。
  - `advance_input_replay_frame` @0x140942280：mode==Replaying 时 `index@+0x48 += 1`；当
    `frames 计数@+0x38 <= index` → 置 mode = `0x8000000000000004`（Finished）。
  - `control_input_replay` @0x140941a30（键盘控制录制/重放，O 结构见
    `record_and_replay_input_for_trailers`）。
- **input_replay_for_trailers.rs**（O）：
  - `record_and_replay_input_for_trailers` @0x14083c620：`配置字节 @+0xfc == 1` 且按键命中
    `HashMap<KeyCode,()>` 时：Idle/Finished → 日志 "Start recording"、`MainCamera::store_
    parameters()`、重建流 + mode=Idle(`0x8000000000000000`)；Recording(default) → 日志
    "Finished recording"、mode=`0x8000000000000004`；Replaying 键 → 日志 "Replay..."、
    mode=`0x8000000000000002`、index=-1、seek 最后 sync point 恢复 `InputTranslationState`、
    `MainCamera::load_parameters()` + `immediately_finish_interpolating()`。
  - `do_input_replay_ui` @0x14083cdd0：egui 面板——"Enable input recording" checkbox（写
    @+0xfc）、"Stream frame count: {}" 标签、"`O` - start/pause recording"、"`E` - replay"、
    "Replay sheep" 按钮（egui 复用）。
- **reset_input_state.rs**（O）：@0x140942b00——清零并重排 `InputTranslationState`（11 qword
  置换）、按最后 sync point 修剪两个输入 VecDeque（frames/sync_points，`count@+0x40`），然后
  `bevy_input::ButtonInput::reset_all<KeyCode>()`（清空键状态）。
- **calculate_input_delta.rs**（O）：@0x1416feeb0——对每个 analog action（活动字节 @+0x30..，
  值字节 @+0x88..，源值 @+0x58..）计算 `{tag:i32, value:i32}` 对：未活动 → `(kind^1)<<2`；
  活动按 kind 0/1/2（released/moving/deadzone）：kind2 走 `+0.1/下限 0.05` 死区、
  kind1 累加 `value+prev_delta`（`delta@+0x58`），kind0 保留 prev。输出区 @+0xa0 起为
  10 qword 原始值镜像。
- **translate_input.rs**（O）：@0x1417cd0a0——输入映射主系统（Analog→`ActionSetValues`，
  digital→`ActionSetStates`，`MouseMoved` 事件消费）。
- **translate_input_to_cursor_moved.rs**（O）：@0x140b17a80——遍历录制事件双队列（stride
  0x18，`tag@+8`：0=mouse_moved/1=moved/2=跳过），累加游标 delta，NaN 安全 clamp，推
  `InputCursorMoved` 事件（stride 0x10 `{tag, Vec2}`）并累加进输出状态 @+0x58/+0x60。
- **update_cursor_position.rs**（U 内联）：PDB 无独立过程（全内联）；负责把 `InputCursorMoved`
  合并进游标位置资源。

## Unknown (U / typed gaps)
- `InputFrame` 0xd0 各字段语义（@+0x30..+0xc0 与 ActionSet 值映射）；`InputTranslationState`
  11 qword 置换表；`MouseMoved`/`InputCursorMoved`/`InputCursorDelta` 精确布局；egui 面板
  精确控件；键盘绑定码（属 game_key_bindings）。

Reused deps：bevy_ecs、bevy_input（ButtonInput<KeyCode>，离线缺失 → 门面）、egui、serde_json。
