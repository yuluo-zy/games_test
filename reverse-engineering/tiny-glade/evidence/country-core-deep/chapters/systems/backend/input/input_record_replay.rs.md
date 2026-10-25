# systems/backend/input/input_record_replay.rs

> 文件作用：输入录制/回放状态机：mode@+0x40（^0x8000000000000000 后 case 0/2/3/4 = Idle/Replaying/Recording/Finished）。Idle 满足条件即 store camera + 克隆 ToolbarCategoryModes + 重建流；Replaying 按 0xd0B InputFrame 写回状态（@+0x30 u64/@+0x38 u32/@+0x3c u8/@+0x40 双 f32/@+0x44 除数/@+0x58..0xc0 十 qword）；越界 panic 'Out of bounds access'。

函数总计 8：业务 3 / 内联无独立符号 2 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::backend::input::input_record_replay::control_input_replay` | 0x140941a30 | 45 | 键盘控制录制/回放 | O：control_input_replay（键控 mode 切换 + seek sync point + camera 参数存取） | ui-input\decomp\decomp-0x140941a30.txt |
| `country_core::systems::backend::input::input_record_replay::advance_input_replay_frame` | 0x140942280 | 109 | 推进回放帧 | O：advance_input_replay_frame：Replaying 时 index@+0x48++，帧尽→Finished | ui-input\decomp\decomp-0x140942280.txt |
| `country_core::systems::backend::input::input_record_replay::input_record_replay` | 0x1409423d0 | 123 | 系统:输入处理 | O：完整状态机（见上）；流重建（frames 容量 8 + 1 sync point）；delta 经 2×2 矩阵 _UNK_142a171f0 + 偏移变换；advance 帧尽置 Finished(0x8000000000000004) | ui-input\decomp\decomp-0x1409423d0.txt |

## 实现方式（算法级）（O，decomp-0x1409423d0/0x140941a30/0x140942280）

**input_record_replay（系统主状态机，@0x1409423d0）**：
1. **状态编码**：mode u64@+0x40，存储 = `0x8000000000000000 + 变体`（niche）；`^0x8000000000000000`
   后 case 0=Idle/2=Replaying/3=Recording/4=Finished。index u64@+0x48。
2. **事件排空**：param_10 = Events 双 VecDeque（counter@+0x40 - head - len 语义，与 mode_manager 同款）；
   有未消费事件才进入分支。
3. **Idle（default）**：条件 = `(队列差非零 || *param_11 != 0 || 字节@+0xa3 bit0)`（param_11 = 外部
   录制开关，字节@+0xa3 = 工具/画布状态）→ 开始录制：
   - `clone ToolbarCategoryModes`（FUN_1412f7fa0，Location toolbar.rs 0x142ab2ba0）——起始快照；
   - `MainCamera::store_parameters`（FUN_140911710）——相机参数入栈；
   - 重建 InputStream：旧流 drop（FUN_140602160），新流 `{frames VecDeque cap=8, 1 sync point}`
     （FUN_141301640 构造，Location input_record_replay.rs 0x142ab3340）；
   - `InputStream::add_frame`（FUN_140949190）压入当前游标 delta 帧：`delta = (cursor - 原点)`
     （原点 0x1429e95b0）→ 2×2 矩阵 _UNK_142a171f0 + 偏移 `*param_7` 变换 → `divps` 归一。
4. **Replaying（case 2）**：越界检查 `head@+0x38 <= index` → panic `"Out of bounds access"`
   （0x142ab3224，line 0x14）；否则元素基址 `frames_ptr + 0x68 + ((index+len) clamp cap)*0xd0`
   （VecDeque 回绕数学）读 InputFrame 0xd0B：
   - `+0x30 u64 / +0x38 u32 / +0x3c u8` → 输出状态 `@+0xe8 / +0xf0 / +0xf4`；
   - `+0x40 双 f32` → 游标位置（f32→u32，NaN 安全 clamp 0..u32::MAX，比较 _UNK_1429a07d0）；
   - `+0x44 f32 缩放除数` → `norm = (cursor - 原点) / 除数`；
   - `+0x58..0xa8 11 qword` → 恢复 InputTranslationState 快照；
   - `+0xc0 u64` → 变换 `pos' = pos + frame * scale(_UNK_1429258e0/e4)` 写回游标 delta。
5. **Recording（case 3）**：清零 20 qword（0xa0B，InputTranslationState 复位）。
6. **Finished（case 4）**：no-op。

**control_input_replay（键控驱动，@0x140941a30）**：
- param_1/2/3 = ResMut（三处 tick 回写 `+0x1c`），param_4 = ResMut<流>，param_5 = Events
  （元素 stride 0x30=48B）。
- 排空事件（bevy Events 双缓冲）；`mode > 0` 时重建流：clone toolbar + store camera +
  `InputStream::on_history_sync`（FUN_1409494c0）；verbosity>2 log `"Starting input replay"`
  （0x142ab2650/2678）；`try_load`（FUN_1409499c0）。
- **sync point seek**：`sync_points.len==0` → panic `"No sync points found in replay state"`
  （0x142ab2740/2750）；`mode != MIN+2(Replaying)` → panic
  `"Can only seek to sync point if replaying but mode is "`（0x142ab3658/3668，格式化 panic 带 mode）；
  否则读 sync point（stride 0x60）：`index@+0x48 = sync.frame_index(+0x58) - 1`；
  恢复：游标（2×u32）+ clone ToolbarCategoryModes（sync+0x20）覆盖 Toolbar categories Vec +
  current 字节（sync+0x18）；`MainCamera::load_parameters`（FUN_140911520，数据 @sync+0x30）；
  不足 → panic `"Not enough player inputs to rewind "`（0x142ab27b0/27e0，带 map 闭包）。

**advance_input_replay_frame（@0x140942280）**：`mode==MIN+2` → `index = index+1`；`index >
frames.len(+0x38)` → verbosity>2 log + `mode = MIN+4(Finished)`（0x8000000000000004）。

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::backend::input::input_record_replay::normalize_cursor_pos`（行 223）— 待推断
- `country_core::systems::backend::input::input_record_replay::unnormalize_cursor_pos`（行 228）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 3 项
