# RE-ANALYSIS: systems/backend/input（交互与世界组 · input）

> 会话：interaction re-analyst（2026-10-15）。范围：`systems/backend/input/*` 6 文件。
> 依据：B patched EXE 机器码 + 既有 decomp（ui-input/input 域）+ 本轮新反编译（
> `evidence/country-core-deep/interaction-group/decomp/decomp-0x1416feeb0.txt`（calculate_input_delta 全量）、
> `decomp-0x140942b00.txt`（reset_input_state 全量）、callees 批 `results/out-callees.json`）。
> 记号：O=机器码证实 / I=有界推断 / U=未恢复；「旧结论 X → 新证据 Y」=本章修正。
> 不改 reconstruction 源码。

## 1. 函数清单表

| 函数 | 地址 | 状态 | 行为一句话 |
|---|---|---|---|
| translate_input | 0x1417cd0a0 | O（34KB，12 参） | 输入映射主系统：键盘/鼠标/Steam/事件源 → InputTranslationState（analog 累加 + digital 标志） |
| input_record_replay | 0x1409423d0 | O（12KB） | 录制/回放主状态机（mode@+0x40 niche 0/2/3/4） |
| control_input_replay | 0x140941a30 | O | 键盘驱动录制/回放：事件排空 + sync point seek + try_load |
| advance_input_replay_frame | 0x140942280 | O | Replay 帧推进：index++，帧尽 → Finished |
| reset_input_state | 0x140942b00 | O（本轮全量 375B） | 11 qword 状态块下移重排 + 双队列按 sync point 修剪 + ButtonInput::reset_all |
| calculate_input_delta | 0x1416feeb0 | O（本轮全量 5.5KB） | **40 路 digital 通道增量累加器 → 运行时 MainActionSet 0x170B** |
| record_and_replay_input_for_trailers | 0x14083c620 | O | trailer 专用：'O' 键启停 + 回放键 seek（共用 InputStream 状态机） |
| do_input_replay_ui | 0x14083cdd0 | O | egui 面板（checkbox 写 @+0xfc、帧计数、保存 tmp/input_replay.json） |
| update_cursor_position | 内联（instances=5，无 VA） | I | 游标累加 + EventWriter 重发（全内联无独立符号） |
| normalize_cursor_pos / unnormalize_cursor_pos | 内联（行 223/228） | O（宿主内联） | 游标归一化/反归一化（见 §4） |
| digital_delta | 内联（calculate_input_delta 行 143） | O（宿主内联） | 末 3 通道的 0.2/0.1 阈值特殊 delta |

## 2. 每函数行为级结论

### 2.1 translate_input @0x1417cd0a0（输入映射主系统，O）

**参数面**（12 参，均为 Res/ResMut/事件）：param_1=Time 类、param_2=ButtonInput<KeyCode>、
param_3=ButtonInput<MouseButton>、param_4=Steam ClientManager、param_5=AppMode（{disc,sub}）、
param_6/param_7=ResMut（输出 InputTranslationState）、param_8/param_10/param_11=三个 Events 双缓冲、
param_9=f32 资源（游标 delta 累加器）、param_12=按键绑定表。

**输入源（重点）**：
1. **键盘**：`FUN_14089aef0 = ButtonInput<KeyCode>::pressed`（callee 名
   `get_inner<KeyCode>`，14089aef0）；按键绑定表（param_12）逐槽查询，0xc3(=195) 哨兵=未绑定槽。
   绑定表元素 = {key u32@0, value u64@4}（8B 对），约 40 个槽
   （0x00,0x03,0x06,0x09,0x0c,0x0f,0x12,0x15,0x18,0x1b,0x1e,0x21,0x24,0x27,0x2a,0x2d,0x30,
   0x33,0x36,0x39,0x3c,0x3f,0x42,0x45,0x48,0x4b,0x4e,0x51,0x54,0x57,0x5a,0x5d,0x60,0x63,
   0x66,0x69,0x6c,0x6f,0x70,0x71）。
2. **鼠标**：`FUN_14089a890=just_pressed<MouseButton>`、`FUN_14089ab50=pressed<MouseButton>`、
   `FUN_14089a9f0=just_released<MouseButton>`。
3. **Steam**：param_4=Steam ClientManager（Arc 克隆 `LOCK;inc`）；`SteamAPI_SteamUtils_v010`
   (142923570) → `IsSteamRunningOnSteamDeck` (1429234f0)；是 Deck 时走 Steam 输入路径
   （cVar15 判定分支）。
4. **事件流**：3 个 Events 双缓冲（param_8/param_10/param_11，0x10B 元素 {qword@0,char@8}），
   末尾消费：末事件 char@8==0 → state+0x98=1，否则 state+0x99=1。

**输出（写 InputTranslationState = param_7 数据）**：
- `+0x10` = now() 帧时间（u64，FUN_140d635e0）；`+0x18` = tick（i32）。
- **Analog 累加区 `+0x68..0x80`**：每绑定槽按下则加减 `fVar36 = dt * _UNK_1429cd034`
  （camera 高度/平移/缩放方向）；`+0x78` 另乘 `_UNK_1429258d0/_UNK_1429df180`；
  `+0x80` 用 `_UNK_142ce2d30/_UNK_142ce2d34`。
- **Digital 标志区 `+0x88..0xa1`**：逐字节 `|=` 按键按下结果（`+0x88` 先 `|=` "任意键按下"位）。
- 附加区 `+0xa2..0xae`：仅全局 `0x143634cd8`（demo 门）非零时写入（0x35/0x3d/0x40/0xab/0x25/
  0x33/0x23/0x29/0xa6/0xa7 键）。
- 鼠标 delta：`*param_9`（f32 资源）累加 + `_UNK_1429cd078` 阈值门控写 `+0x89`。

**AppMode 门（重点，输入源接线）**：analog 相机翻译仅在
`(AppMode==2 || (AppMode==10 && sub != 1))` 时运行（`lVar28 == 2 || ((int)lVar28==10 && [sub]!=1)`）。
**AppMode==10 的语义名与 ui-input §3.3「10+5k=Tool(视觉序 Buildings…Terrain)」冲突**
（terrain 域 ui_draw/ui_edit 均以 ==10 为门）——见 §5 未决。

**下游**：`calculate_input_delta`（§2.6）读 state+0x30..0x57（kind）与 +0x88..0xaf（active）。

### 2.2 input_record_replay @0x1409423d0（录制/回放主状态机，O）

已由 input 域深挖（ALGORITHMS §2.6）。**本轮机器码复核要点**：
- mode@+0x40（u64，niche `^0x8000000000000000`）：default 分支=0 AutoRecordPanicRepro、
  case 2=Replay、case 3=DisableInput、case 4=None。
- 录制条件（default 分支）：`(事件未消费 || param_11(外部开关)!=0 || state字节@+0xa3 bit0)`。
- **旧结论 X → 新证据 Y（命名纠偏）**：input_record_replay.rs 章节表头
  「case 0/2/3/4 = Idle/Replaying/Recording/Finished」为旧标注；Display 串权威名 =
  **AutoRecordPanicRepro/Records/Replay/DisableInput/None**（input/ALGORITHMS §6.4 已纠偏，本章节待同步）。
- 帧数据 0xd0B；回放读取 +0x30 u64/+0x38 u32/+0x3c u8 → state+0xe8/+0xf0/+0xf4；+0x40 双 u32
  （游标，f32→u32 clamp [0,u32max] NaN 安全，_UNK_1429a07d0 上限）写 CursorPosition(param_6)；
  +0x44 f32 除数；+0x68..+0xb8 11 qword → state+0x58..0xa8；+0xc0 Vec2 → 归一化反写。
- `*puVar26 = CONCAT44(fVar32 + (u64>>0x20)*fVar31, fVar29 + (float)uVar1*fVar31)` =
  反归一化游标增量写 param_7（_UNK_1429258e0/e4 为 scale）。

### 2.3 control_input_replay @0x140941a30（O）

callees（本轮批）：`clone<ToolbarCategoryModes>`(1412f7fa0)、`drop_in_place<InputRecordReplayState>`
(140602160)、`MainCamera::immediately_finish_interpolating`(140911b50)/`load_parameters`
(140911520)/`store_parameters`(140911710)、`InputStream::on_history_sync`(1409494c0)、
`try_load`(1409499c0)、log/panic_fmt。
- 事件排空 → mode>0 时重建流（clone toolbar + store camera + on_history_sync）→ try_load。
- **seek_to_sync_point 内联**：sync_points 空 → panic "No sync points found in replay state"
  (0x142ab2740)；mode!=Replay → panic "Can only seek to sync point if replaying but mode is "
  (0x142ab3658，格式化 panic 带 mode)；成功 index=sync.frame_index-1 + 恢复
  （游标 2×u32 + clone ToolbarCategoryModes 覆盖 + camera load_parameters）。
- 不足 → panic "Not enough player inputs to rewind " (0x142ab27b0)。

### 2.4 advance_input_replay_frame @0x140942280（O）

`mode==MIN+2(Replay)` → index=index+1；`index > frames.len(+0x38)` → verbosity>2 log +
`mode = MIN+4(None)`。callees：`as_inner_mut`(14067dd80)、log、__rust_dealloc。

### 2.5 reset_input_state @0x140942b00（O，本轮全量）

参数面：in_RCX=ResMut（InputTranslationState 所在资源，tick 回写 @[2]=+0x1c）、
in_RDX/in_R8=两个 InputStream 形态资源、in_R9=ButtonInput<KeyCode>。
1. **11 qword 状态块下移**：读 data+0x58..0xa8（qword 索引 0xb..0x15），清零原区，写 data+0x00..0x50
   （**straight copy，dest[i]=src[0xb+i]**；Ghidra 呈现的乱序是寄存器分配伪影）。
2. **双队列修剪**：对两个 deques（字段 +0x10 sync_len、+0x18 sync_head、+0x30 frames_len、
   +0x38 frames_head、+0x40 mode），按 `counter - head` 计算剩余/已耗；写 `*counter = mode - 已耗`，
   空时回写 `*counter = mode`。语义=按最后 sync point 裁掉已消费帧段。
3. **门控 ButtonInput::reset_all<KeyCode>()**（callee 14089acb0）：仅当两队列非全空才复位
   键盘沿状态；全空直接 return。

**旧结论 X → 新证据 Y**：章节「11 qword 置换」——实为整体下移 + 原区清零（无置换序）；
「按最后 sync point 修剪」——双队列计数回写机器码确认。

### 2.6 calculate_input_delta @0x1416feeb0（O，本轮全量 5.5KB）

**★ 主要纠错：40 路 digital 通道，非章节「三通道 x/y/z」★**
```
入参: in_RCX = Res<InputTranslationState>（lVar3=数据）
      in_RDX = Res<帧源>（base = *(float*)(data+0x58)）
      in_R8  = ResMut<输出 MainActionSet 运行时形态 0x170B>（tick 回写）
输出: piVar4 = 输出数据
      digital 40×8B {disc i32@0, val f32@4} @0x00..0x140
      + 6 qword（state+0x58..0x80 → +0x140..0x170，= MainActionSet analog 半区）
逐通道 i (0..40):
  kind   = byte@(src+0x30+i)
  active = byte@(src+0x88+i)
  prev   = {disc,val}@(out+8i)   # 读改写累积
  if kind == 0:                      # released
     disc = (active^1)<<2;  val 保持 prev
  else:
     if active == 0:                 # 松手
        disc = 3;  val = (prev.disc==1) ? prev.val : 0
     elif prev.disc == 2:            # deadzone
        v = prev.val + 0.1;  if v <= 0.05: v = 0.05
        disc = 2;  val = v - base
     elif prev.disc == 1:            # moving
        disc = 1;  val = prev.val + base
     else: disc = 1;  val = 0
```
- **digital_delta（内联，行 143）**：末 3 通道（0x44..0x46）用不同公式：
  `prev.disc==1 → v = prev.val+base; disc = (v<0.2)? 1:2; v = (v<0.2)? v:0.1`（0.2/0.1 阈值）。
- 常量：base=帧 delta（param_2 data+0x58）、死区 0.1/0.05、0.2/0.1（digital 特例）。
- 产出 = 运行时 MainActionSet（digital 前、analog 后），被 deconstruct 0x140946b80
  打包进 InputFrame 的 main_action_set_value。

**旧结论 X → 新证据 Y**：
- 「三通道独立 @+0x30/0x31/0x32（游标 x/y/z）」→ **40 通道**（+0x30..0x57 kind、+0x88..0xaf active）。
- 「输出区 @+0xa0 起 10 qword 原始镜像」→ 输出 0x140 通道区 + 6 qword state 拷贝（0x170B）。

### 2.7 record_and_replay_input_for_trailers @0x14083c620（O）

- 配置字节@+0xfc==1 才启用；命中按键表（`HashMap<KeyCode>`，get_inner 14089aef0）。
- mode 0/4 → log "Start recording"(0x142a93740) + MainCamera::store_parameters + 重建流 + mode=0；
  mode 1 → log "Finished recording"(0x142a93678) + mode=4；mode 2/3 → 回放键 log "Replay..."
  (0x142a93778) + index=-1 + 恢复 InputTranslationState + MainCamera::load_parameters。
- **trailer vs 普通回放差异**：共用 InputStream 状态机；trailer 由
  `InputReplayInitialState`（录制文件头，10 字段）驱动 + do_input_replay_ui 保存
  `tmp/input_replay.json`；普通回放由 `History::finish_input_recording` 事件驱动
  （control_input_replay）。

### 2.8 do_input_replay_ui @0x14083cdd0（O）

egui 面板：checkbox "Enable input recording" 写配置@+0xfc；"Stream frame count: " 标签 +
帧计数；按键提示 'O'/'E'；Save 按钮（串 "tmp/input_replay"+"json"）；Replay sheep 按钮。

### 2.9 update_cursor_position（内联，I）

- PDB 记录：`inline_instance; instances=5`（行 10）；内联依赖 = EventWriter::write、
  Vec2::add_assign、Vec::push、set_changed、deref_mut（模块 1611 的 5 个宿主）。
- 结论：消费 InputCursorMoved 类事件 → Vec2 累加 → 游标位置资源 + 经 EventWriter 重发。
  因无独立符号、宿主待定位，维持 I；restorer 按简单系统实现即可（行为面与
  translate_input_to_cursor_moved 的 InputCursorMoved 合成对称）。

## 3. 输入数据流总图（供 restorer 接线）

```
bevy 键盘/鼠标 → translate_input 0x1417cd0a0（SteamDeck 门 + 绑定表 0xc3 哨兵）
  → InputTranslationState（analog +0x68..0x80 / digital 标志 +0x88..0xae）
  → calculate_input_delta 0x1416feeb0 → MainActionSet 运行时 0x170B
  → deconstruct 0x140946b80 → InputFrame.main_action_set_value (Vec<[u8;12]>)
  → input_record_replay 0x1409423d0 add_frame 0x140949190（frames cap 8/9999/50000 裁剪）
录制文件 = {replay_initial_state, first_frame}（serialize<Compact> 0x1402e0060）
trailer: record_and_replay_input_for_trailers（'O' 键）↔ do_input_replay_ui
重置: reset_input_state 0x140942b00（状态块下移 + 修剪 + ButtonInput reset_all）
```

## 4. 内联游标归一化（normalize/unnormalize_cursor_pos，O 宿主内联）

- **normalize（录制分支）**：`n = ((cursor - origin) · M + win) / divisor`
  - origin = `_UNK_1429e95b0`（Vec2 常量）；M = 2×2 矩阵 `_UNK_142a171f0`；
    win = param_7（窗口尺寸 2×u32）；divisor = {Δy, Δy}（`divps`，Ghidra 的 OR 是 SIMD 装载伪影）。
  - 结果存帧 +0x40（u64 = 2×f32）。
- **unnormalize（回放分支）**：读帧 +0x40 拆 2×u32 → f32 → clamp[0,u32max]（_UNK_1429a07d0）
  写 CursorPosition；再 `pos' = pos + prev_pos * (cursor.y-origin.y)`（_UNK_1429258e0/e4 scale）。

## 5. 未决 / 待裁决（不臆造）

1. **AppMode==10 语义名**：已裁决（`reports/interaction/terrain/RE-APP-MODE.md`）——10 = **Terrain 工具**
   （行为仲裁：terrain 域 ui_draw/ui_edit 门 ==10 + is_tool_selected 工具区 {6..12,15..18} 含 10）。translate_input 的
   `(mode==10 && byte1!=1)` 门：byte1=disc u32 bits8..15，纯 disc10 恒真 ⇒ 相机平移实际运行在 {2,10}。
   ui-input §3.3「10+5k=Tool(视觉序 Buildings…/Terrain)」作废（与机器码行为冲突）。
2. InputTranslationState 布局部分映射：+0x30..0x57 kind 数组、+0x58..0xa8 11 qword 状态块、
   +0xe8/+0xf0/+0xf4 回放附加字段；+0x00..0x2f 与 +0xb0..0xe7 未定（U）。
3. reset_input_state 双队列修剪的 counter 语义（`mode - 已耗` 回写）——与 InputStream
   index 的关系待机器码进一步核对。
4. update_cursor_position 的 5 个宿主（模块 1611 编译实例）未逐一定位（I）。
5. AutoClutterEdit serde 字段名（见 ui_clutter 报告）——serialize 0x14003d300 写单字段
   `{"<name>": {"<key>": u64}}`，字段名串未取（U，低优先）。

## 6. 证据落盘（本轮新增）

- `evidence/country-core-deep/interaction-group/decomp/decomp-0x1416feeb0.txt`（全量 5.5KB）
- `evidence/country-core-deep/interaction-group/decomp/decomp-0x140942b00.txt`（全量 375B）
- `evidence/country-core-deep/interaction-group/decomp/decomp-0x140a86b60.txt`（mode_manager 入口 405B）
- `evidence/country-core-deep/interaction-group/results/out-callees.json`（12 系统 callees 批）
- `evidence/country-core-deep/interaction-group/results/out-missing-decomp.json`（10 函数补采批）
- `evidence/country-core-deep/interaction-group/results/out-mode-manager.json`
