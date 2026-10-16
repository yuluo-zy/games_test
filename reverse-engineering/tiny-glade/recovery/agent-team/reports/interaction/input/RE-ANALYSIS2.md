# RE-ANALYSIS2: systems/backend/input（二轮补挖 · input）

> 会话：interaction re-analyst（2026-10-15，二轮）。reviewer-interaction 4 项取证的第 1、2 项。
> 依据：B EXE 机器码 + 既有 decomp + 本轮 RNE 常量直读 + ghidra 定点（job-rot.json）。
> 记号：O=机器码证实；I=推断；U=未恢复。「旧结论 X → 新证据 Y」=修正。不改 reconstruction 源码。

## 1. 键绑定表布局（item 1，O）

### 1.1 KeyBindings 资源（game_key_bindings.rs 既有 + 本轮 translate_input 交叉）

- **KeyBindings = 40 动作槽 × 12B（stride 0xc）**，每槽 = `{key u32@0, value u64@4}`；
  未绑定哨兵 **0xc3（=195）**。
- `default` @0x14089b110 写 36 默认键（前 10：`0x2a,0x26,0x14,0x17,0x18,0x24,0xc3,0xc3,0xc3,0xc3`）。
- 鼠标键：u16 对 `{1,2,1}` @0x1bc/0x1c0/0x1c4（`assign_mouse_button` 写 `{key,mouse} u16 @0x1bc+action*4`）。
- ron serialize @0x140831740：40 字段名表（stride 0xc），动作名 = move_forward…mouse_focus_edit（§2.3 DIGEST）。
- **translate_input @0x1417cd0a0 逐槽读**（本轮交叉，O）：绑定槽 = `piVar5[3k]`（i32 索引步距 3 = 12B），
  槽号 k = 0,3,6,9,0xc,0xf,…,0x6f（38 槽）+ 尾 2 槽（0x70/0x71，u16 访问 = 鼠标绑定区），
  共 **40 槽**。每槽 `iStack_98 = *piVar5`（key）+ `uStack_94/90 = piVar5[1..2]`（value u64），
  调 `FUN_14089aef0 = ButtonInput<KeyCode>::pressed`（callee 名 get_inner<KeyCode>）。
- **旧结论 X → 新证据 Y**：章节「绑定平铺进前 30 槽（未取证）」→ 机器码读满 **40 槽**
  （0x00..0x6f 步 3 + 尾 0x70/0x71）；每槽 12B {key,value}，0xc3 哨兵。
- 值 u64 用途：同槽被 `calculate_input_delta` 系消费（analog 方向量）或直接判按下（digital 标志）。

### 1.2 Steam 输入源路径（item 1，O）

translate_input @0x1417cd0a0 的 Steam 段（callee 名本轮批确认）：
```
param_4 = Steam ClientManager（Arc 克隆：LOCK;inc，溢出检查）
uVar17 = SteamAPI_SteamUtils_v010 (0x142923570)      # utils 接口
cVar15 = IsSteamRunningOnSteamDeck(uVar17) (0x1429234f0)
drop 克隆（LOCK;dec，归零 FUN_140b85350 释放）
cVar15==0（非 Deck）→ 走 事件源(param_2) 检查路径
cVar15!=0（Deck）→ 走 Steam 输入驱动路径（translate 事件排空）
```
- 事件源双检：`param_2`（ButtonInput<KeyCode>）与 `param_3`（ButtonInput<MouseButton>）的
  Events 挂起计数决定是否进入逐槽翻译。
- **Deck 控制器→InputTranslationState 的实际翻译在 `systems::steam::update_steam_inputs`
  （ui-input/audio-misc 域，22 同构块，内联 ActiveInputModeTracker——input ALGORITHMS §3.1）**；
  translate_input 的 Deck 门 = 选择键盘/鼠标 vs Steam 事件驱动（I，跨域 ui-input/steam）。
- 结论：**Steam 路径不直接写 InputTranslationState（translate_input 内）；它门控事件源，
  控制器状态由 update_steam_inputs 写入**。restorer 接线：translate_input 保持 Deck 检查 +
  事件排空门；控制器翻译归 steam 系统。

## 2. InputFrame 精确布局（item 2，O 对账）

### 2.1 运行时帧（deque 元素，0xd0B，回放 case2 读 + 录制 default 写，O）

```
InputFrame = 0xd0B
+0x00..0x2f  两 Vec 头（cap-first {cap@0,ptr@8,len@0x10}×2 @+0x00/@+0x18；空载 {0,4,0}）
             —— 即「0x30 头」（add_frame 剪帧 drop 时按指针释放；main_action_set_value 系）
+0x30  u64  → state+0xe8
+0x38  u32  → state+0xf0
+0x3c  u8   → state+0xf4
+0x40  u64（游标 2×u32；f32→u32 clamp [0,u32max]，_UNK_1429a07d0=4.29497e9 上限）→ CursorPosition
+0x44  f32 除数（time_delta 归一，回放 `norm = (cursor-origin)/除数`）
+0x48..0x67 间隙/其余（U）
+0x68..0xb8  11 qword 状态块 → state+0x58..0xa8（录制写 state+0x58..0xb0 读 → 帧 +0x68..0xb8）
+0xc0  Vec2 prev_normalized_cursor_pos（回放 lerp 用）
+0xc8  f32 time_delta_seconds
```
- **旧结论 X → 新证据 Y**：input_record_replay 章节表头「+0x58..0xc0 十 qword」——
  **0x58..0xa8 是状态偏移（回写目标），帧偏移实为 +0x68..0xb8（11 qword）**；
  +0xc0 是 prev pos（非状态块内）。帧偏移 = 状态偏移 + 0x10。

### 2.2 serde 视图（serialize<Pretty> @0x1413fe6f0，O）

字段声明序（serialize 逐字段调用序，键串 @0x142c5b3d0/3e1/3fb/40d 已直读证实）：
```
1 translation_state         len 0x11(17)  ← 读 param_1+0x00
2 prev_normalized_cursor_pos len 0x1a(26) ← 读 param_1+0xc0
3 time_delta_seconds        len 0x12(18)  ← 读 param_1+0xc8(200)
4 main_action_set_value     len 0x15(21)  ← 读 param_1+0x68
```
- 运行时 repr 偏移：translation_state@+0x00、main_action_set_value@+0x68（Vec 24B）、
  prev@+0xc0、time@+0xc8（serialize 按字段访问器读）。
- **与回放视图对账**（ALGORITHMS §6.2 维持）：serde「translation_state@0x00」vs 回放
  「状态块@0x68..0xb8」是**不同表示**（serde 字段序 [translation_state, prev, time,
  main_action_set_value]；回放/录制帧 状态块在 +0x68）。恢复时分别建模，勿以单一 0xd0B
  结构同时满足两偏移。
- **0x30 头**：帧前 0x30B = 两 Vec 头（serde 的 main_action_set_value + 另一 Vec，空载 {0,4,0}）。

### 2.3 同步点 seek / try_load 结构（control_input_replay @0x140941a30，O）

```
InputStreamSyncPoint = 0x60B：
  +0x18  u8   Toolbar current 字节
  +0x20/+0x28  ToolbarCategoryModes Vec（clone 源，FUN_1412f7fa0）
  +0x30  camera 参数（MainCamera::load_parameters(camera, sync+0x30, 0)；store_parameters 反向）
  +0x58  u64 frame_index

seek_to_sync_point（内联）：
  sync_points.len == 0 → panic "No sync points found in replay state"(0x142ab2740/2750)
  mode != MIN+2(Replay) → panic "Can only seek to sync point if replaying but mode is "
                          (0x142ab3658，格式化 panic 带 mode)
  成功：index = sync[最后].frame_index(+0x58) - 1
        clone ToolbarCategoryModes(sync+0x20 区) 覆盖 Toolbar 资源
        写游标 2×u32（param_1 数据）
        MainCamera::load_parameters(sync+0x30, 0)；immediately_finish_interpolating

try_load（FUN_1409499c0）：
  读串失败 → Err（"Failed to read the file" @0x142ab35f0）
  serde_json::from_str 失败 → Err（"Failed to deserialize" @0x142ab3607）
  成功 → drop 旧 InputRecordReplayState（FUN_140602160）→ 重建流（mode=MIN+2 Replay, index=-1）
  → on_history_sync 推入新同步点
```
- 事件元素 0x30B（History::finish_input_recording 同步事件：路径+翻译态+flag）。
- 流字段写回（puStack_100[0..0xb]）= InputStream {sync_points VecDeque, frames VecDeque,
  mode@+0x40, index@+0x48}。

## 3. 二轮新证据落盘

- `interaction-group/decomp/decomp-0x1416f3210.txt`（**CircularF32::as_f32 = fmod 2π**，66B）
- RNE 常量直读：SPIRAL_ANGLE=2.39983、rmb 基 -π/2(-1.5708)、_UNK_142925980=f32MAX 等
- InputFrame serde 键串 @0x142c5b3d0..0x142c59a0d 直读（translation_state 等 4 键）
