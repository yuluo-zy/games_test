# input 域算法深挖（ALGORITHMS.md）

> 2026-10-15。范围：`input/{action_set, input_stream, mod, panic_repro_state}.rs`（顶层 input 模块，
> 与 `systems/backend/input/*` 相邻但独立）。目标：动作映射、录制/回放环、模式切换的
> 状态机与数据流（算法级：循环/分支/常量/调用链），供 reconstruction `todo!()` 缺口恢复。
> 证据：本轮新增反编译落盘 `input/decomp/decomp-0x{1400ed690,1416f8680,1416f92d0,1409499c0,
> 140949970,14094f2d0,140947c80,1413fe6f0,1402e0060}.txt` + wp15 b1_input 既有
> （`reconstruction/country-core-render/wp15/analysis/b1_input/*.c`）+ 原字节/串直读。
> O=机器码证实；I=推断；U=未知。**本轮不改 reconstruction 源码。**

---

## 1. 动作集（action_set.rs）：MainActionSet 双形态

### 1.1 两种内存形态（关键区分，O）

| 形态 | 结构 | 布局 | 使用方 |
|---|---|---|---|
| **运行时** `MainActionSet<Vec2, DigitalActionState>` | 0x170 B | digital `[8B;40]` @0x00..0x140 + analog `[Vec2;6]` @0x140..0x170 | deconstruct/reconstruct/just_pressed、InputFrame 打包、translate_input 产物 |
| **serde 形态** `MainActionSet<Vec2, bool>`（Compact/Pretty 用） | 0x58 B | analog `[Vec2;6]` @0x00..0x30 + digital `[bool;40]` @0x30..0x58 | serialize<Vec2,bool,Compact/Pretty> 0x1400ed690/0x1416f8680、deserialize visit_str 0x1416f92d0 |

- 运行时 digital 槽 = 8 B `{disc i32@0, val f32@4}`（O：deconstruct 机器码 `mov r14d,[rdi+8k] /
  movss xmm6,[rdi+8k+4]`；reconstruct 尾段写 80×i32 @0x00..0x140 + 12×i32 @0x140..0x170）。
- serde 形态 digital 槽 = 1 B（0=默认 false，跳过；≠0 写出），analog 槽 = Vec2 8 B。
- 两形态**字段序相反**（运行时 digital 在前、serde analog 在前）——两种实例化或独立
  serde 表示类型，登记为待整合裁决项（§6.1）。槽位语义（名字）两形态完全一致（46 槽）。

### 1.2 46 动作名清单（O：serialize<Compact> 0x1400ed690 逐字段名引用 + 长度 + 偏移）

串表 = 无 NUL 后缀共享融合表 @0x14293d9c1..（另一副本 @0x142c91cd0..，deserialize 比较窗
@0x142c91850..；两副本各自独立）。槽位 = `tag u16 = slot<<8 | kind`（kind0=analog、kind1=digital）。

| # | 槽 (slot,kind) | tag | 动作名 | 串地址(len) | serde 值偏移 |
|---|---|---|---|---|---|
| 1 | (0,0) analog | 0x000 | `input_space_cursor_move` | 0x14293d9c1 (23) | 0x00 Vec2 |
| 2 | (1,0) analog | 0x100 | `hardware_window_space_cursor_move` | 0x14293d9d8 (33) | 0x08 Vec2 |
| 3 | (2,0) analog | 0x200 | `camera_pan` | 0x14293d9f9 (10) | 0x10 Vec2 |
| 4 | (3,0) analog | 0x300 | `camera_height` | 0x14293da03 (13) | 0x18 Vec2 |
| 5 | (4,0) analog | 0x400 | `camera_zoom` | 0x14293da10 (11) | 0x20 Vec2 |
| 6 | (5,0) analog | 0x500 | `camera_rotate` | 0x14293da1b (13) | 0x28 Vec2 |
| 7 | (0,1) digital | 0x001 | `click` | 0x14293da28 (5) | 0x30 |
| 8 | (1,1) digital | 0x101 | `right_click` | 0x14293da2d (11) | 0x31 |
| 9 | (2,1) digital | 0x201 | `camera_rotate_mode` | 0x14293da38 (18) | 0x32 |
| 10 | (3,1) digital | 0x301 | `camera_pan_mode` | 0x14293da4a (15) | 0x33 |
| 11 | (4,1) digital | 0x401 | `tool_building` | 0x14293da59 (13) | 0x34 |
| 12 | (5,1) digital | 0x501 | `tool_path` | 0x14293da66 (9) | 0x35 |
| 13 | (6,1) digital | 0x601 | `tool_decorator` | 0x14293da6f (14) | 0x36 |
| 14 | (7,1) digital | 0x701 | `tool_customize` | 0x14293da7d (14) | 0x37 |
| 15 | (8,1) digital | 0x801 | `tool_terrain` | 0x14293da8b (12) | 0x38 |
| 16 | (9,1) digital | 0x901 | `tool_eraser` | 0x14293da97 (11) | 0x39 |
| 17 | (10,1) digital | 0xa01 | `tool_garden` | 0x14293daa2 (11) | 0x3a |
| 18 | (11,1) digital | 0xb01 | `tool_tree` | 0x14293daad (9) | 0x3b |
| 19 | (12,1) digital | 0xc01 | `tool_water` | 0x14293dab6 (10) | 0x3c |
| 20 | (13,1) digital | 0xd01 | `photomode` | 0x14293dac0 (9) | 0x3d |
| 21 | (14,1) digital | 0xe01 | `time_of_day` | 0x14293dac9 (11) | 0x3e |
| 22 | (15,1) digital | 0xf01 | `snapping_disable` | 0x14293dad4 (16) | 0x3f |
| 23 | (16,1) digital | 0x1001 | `undo` | 0x14293dae4 (4) | 0x40 |
| 24 | (17,1) digital | 0x1101 | `redo` | 0x14293dae8 (4) | 0x41 |
| 25 | (18,1) digital | 0x1201 | `ui_cancel` | 0x14293daec (9) | 0x42 |
| 26 | (19,1) digital | 0x1301 | `ui_confirm` | 0x14293daf5 (10) | 0x43 |
| 27 | (20,1) digital | 0x1401 | `ui_left` | 0x14293daff (7) | 0x44 |
| 28 | (21,1) digital | 0x1501 | `ui_right` | 0x14293db06 (8) | 0x45 |
| 29 | (22,1) digital | 0x1601 | `ui_down` | 0x14293db0e (7) | 0x46 |
| 30 | (23,1) digital | 0x1701 | `ui_up` | 0x14293db15 (5) | 0x47 |
| 31 | (24,1) digital | 0x1801 | `ui_tab_prev` | 0x14293db1a (11) | 0x48 |
| 32 | (25,1) digital | 0x1901 | `ui_tab_next` | 0x14293db25 (11) | 0x49 |
| 33 | (26,1) digital | 0x1a01 | `clear_canvas` | 0x14293db30 (12) | 0x4a |
| 34 | (27,1) digital | 0x1b01 | `main_menu` | 0x14293db3c (9) | 0x4b |
| 35 | (28,1) digital | 0x1c01 | `developer_menu` | 0x14293db45 (14) | 0x4c |
| 36 | (29,1) digital | 0x1d01 | `screenshot` | 0x14293db53 (10) | 0x4d |
| 37 | (30,1) digital | 0x1e01 | `screenshot_grading_reference` | 0x14293db5d (28) | 0x4e |
| 38 | (31,1) digital | 0x1f01 | `screenshot_feedback` | 0x14293db79 (19) | 0x4f |
| 39 | (32,1) digital | 0x2001 | `hide_ui_photomode` | 0x14293db8c (17) | 0x50 |
| 40 | (33,1) digital | 0x2101 | `cinematic_camera_ctrl` | 0x14293db9d (22) | 0x51 |
| 41 | (34,1) digital | 0x2201 | `human_scale_ref_toggle` | 0x14293dbb3 (22) | 0x52 |
| 42 | (35,1) digital | 0x2301 | `ivy_growth_toggle` | 0x14293dbc9 (17) | 0x53 |
| 43 | (36,1) digital | 0x2401 | `ivy_spawn` | 0x14293dbda (9) | 0x54 |
| 44 | (37,1) digital | 0x2501 | `camera_snapshot_add` | 0x14293dbe3 (19) | 0x55 |
| 45 | (38,1) digital | 0x2601 | `camera_snapshot_restore` | 0x14293dbf6 (23) | 0x56 |
| 46 | (39,1) digital | 0x2701 | `toggle_snapping` | 0x14293dc0d (15) | 0x57 |

- 运行时偏移换算：digital 槽 k → 字节 8k（disc）/8k+4（val）；analog 槽 k → 字节 0x140+8k。
- serde 偏移：analog 槽 k → 8k；digital 槽 k → 0x30+k。
- 串表后缀共享（融合表）实例：`input_space_cursor_move` 与 `hardware_window_space_cursor_move`
  共尾 `space_cursor_move`；`screenshot_grading_reference`/`screenshot_feedback` 共尾；无 NUL 分隔，
  serde 以 (ptr,len) 胖指针取子串。

### 1.3 deconstruct（0x140946b80，O，算法）

```
输入: &MainActionSet<Vec2,DigitalActionState> (0x170B)；输出: Vec<[u8;12]>（cap-first {cap,ptr,len}）
算法:
  len=0
  # 6 个 analog 槽（字节 0x140+8k 读 Vec2）
  for k in 0..6:
    v = Vec2@(0x140+8k)
    if (v.x != 0.0f || v.y != 0.0f) || NaN(v.x) || NaN(v.y):   # NaN 也打包！
      grow_one() if 需要 (FUN_140562cf0 系, 12B 元素)
      elem = {u16 tag = k<<8|0 @+0, Vec2 @+4}（movlps 8B）
      len++
  # 40 个 digital 槽（字节 8k 读 {disc i32, val f32}）
  for k in 0..40:
    {disc, val} = @(8k, 8k+4)
    if disc != 4:                                            # 4 = 空闲/释放-none（不打包）
      elem = {u16 tag = k<<8|1 @+0, disc u32 @+4, val f32 @+8}
      len++
  写回输出 Vec len
常量: tag 序列 analog 0x0/0x100/0x200/0x300/0x400/0x500；digital 0x1/0x101/…/0x2701
```

### 1.4 reconstruct（0x140947c80，O，算法）

```
输入: &[[u8;12]]（stride 0xc 确认：循环步进 puVar6+3(uint) = +12B，终止 in_R8*0xc）
输出: MainActionSet<Vec2,DigitalActionState>（0x170B，全部槽先置默认 disc=4）
循环 per 记录（12B, 基 B）:
  byte@B+3 == 0 → analog 分支（switch byte@B+1? 实际按 kind 位+槽跳表 0x142ab…）:
    case k in 0..5: 写 analog 槽 k = Vec2 {dword@B+4, dword@B+8}
  byte@B+3 != 0 → digital 分支:
    case k in 0..0x28: 写 digital 槽 k = {disc = dword@B+4, val = dword@B+8}
Ghidra 注: 0x140948727 起跳表反汇编坏数据（case 0x19/0x20 halt 区），机器码为准；
尾段 92×i32 写回输出（80 digital + 12 analog = 0x170B ✓）
空输入 → 92 槽全 4（空闲态）
```

### 1.5 序列化数据流（serde 形态，O）

- **serialize<Vec2,bool,Compact> 0x1400ed690**：栈上临时 Vec（cap-first）→ 写 `{` (0x7b) →
  逐 46 槽：非默认才写 `"name" : value`（analog 槽 SIMD packssdw/packsswb 一次判 8 个 f32 非零
  位；digital 槽逐字节判 0）→ 全默认则空 `{}`；尾部 `}` (0x7d)。analog 值经 FUN_1404bd170
  （f32 对），digital 值经 FUN_1404bc390（单字节）。**默认判定**：analog=(x==0&&y==0)，digital=字节==0。
- **serialize<Vec2,bool,Pretty> 0x1416f8680**：同 46 槽同偏移，Pretty 写深度缩进（写 LF 0x0a + 缩进
  空格循环 func_0x142923c00）；名字串表副本 @0x142c91cd0..。
- **deserialize::visit_str<serde_json> 0x1416f92d0**（43KB）：JSON map 键名匹配。按长度分派
  （顶层 0x17=23/0xe=14/0x10=16 三支），对每键做 SIMD 16B 窗口逐字符比较（比较常量窗
  UNK_142c91870..，输入串偏移 P 对表偏移 P-7 等后缀共享对齐），命中写槽索引到 visitor
  字段槽（param_1[1] = 索引 0/6/2/4/0xc/0xd/0xf/0x10/0x15/0x18/0x19/0x1b…）；未知名 → 0x2e（46=unknown 哨兵）。
- 值写入（visit_map/visit_seq 侧）写 0x58B serde 形态；**与运行时 0x170B 形态的互相转换点
  本轮未闭合**（疑在 translate_input 与 deconstruct 之间的消费链，见 §5）。

### 1.6 动作状态判别（O 原字节）

- `DigitalActionState`（8B，disc i32 + val f32）：`pressed` 0x14093fa70 = idx<3；
  `released` 0x14093fa80 = (idx-3)<2；`just_pressed` 0x14093fa90 = 跳表（UNK_142ab2c3c）判别；
  `just_released` 0x14093fac0 = idx==3。disc 4 = 空闲（不打包、不触发）。
- `ActionSetAction`（tag u8@0 + 载荷）：tag 0=Analog(Vec2@+4)，tag 1=Digital(disc byte@+1)。
  `just_pressed` 0x1409487b0 = tag==1 && 内态 just-pressed（跳表 UNK_142ab30f0，按 u32@+4 判）；
  `pressed` 0x1409487e0 = tag==1 && 内态<3；`as_analog` 0x140948800 = tag==0 && +1==0 → Some(Vec2@+4)。
- `MainActionSet::just_pressed` 0x140946620：入参动作槽号（u8 in_DL）→ switch 46 槽 → 每槽
  读 disc（digital 槽 @8k）→ 槽专属跳表（UNK_142ab2c78..，46 张）→ 返回该槽 just-pressed。

---

## 2. 录制环（input_stream.rs）

### 2.1 InputStream（64 B，O）

```
{ sync_points: VecDeque<InputStreamSyncPoint> @0x00 {cap@0, ptr@8, len@0x10, head@0x18},
  frames:     VecDeque<InputFrame>          @0x20 {cap@0x20, ptr@0x28, len@0x30, head@0x38} }
元素: InputStreamSyncPoint = 0x60 B；InputFrame = 0xd0 B
```

### 2.2 同步点与帧布局

**InputStreamSyncPoint = 0x60 B**（O：on_history_sync 0x1409494c0 推入 12 qword）：
- @+0x00..+0x50：InputTranslationState 11 qword（qword0 @0、qword1 @8、…qword10 @0x50）
- @+0x58：frame_index u64 = 推入时 frames.len（帧环坐标锚）

**InputFrame = 0xd0 B**（O 两视图，差异登记 §6.2）：
- **serde 字段序（O：visit_str 0x1413fe800 variant 序 = serialize<Pretty> 0x1413fe6f0 写序）**：
  0=translation_state、1=prev_normalized_cursor_pos、2=time_delta_seconds、3=main_action_set_value。
  serde 值偏移（serialize<Pretty>）：`translation_state` @0x00（11 qword 88 B）→
  `main_action_set_value` @0x68（Vec<[u8;12]> cap-first 24 B = deconstruct 打包产物）→
  `prev_normalized_cursor_pos` @0xc0（Vec2，**实际串名 26 字符 = `prev_normalized_cursor_pos`**，
  见 §6.3）→ `time_delta_seconds` @0xc8（f32）。键串：
  translation_state@0x142c5b3d0(17)、prev_normalized_cursor_pos@0x142c5b3e1(26)、
  time_delta_seconds@0x142c5b3fb(18)、main_action_set_value@0x142c5b40d(21)。
  **注**：旧写法「translation_state→main_action_set_value→prev_normalized_cursor_pos→time_delta_seconds」
  混入内存偏移；真字段序为上述 [0,1,2,3]（repr(Rust) 内存重排导致字段序≠内存偏移递增序）。
- 回放视图（input_record_replay 0x1409423d0 case 2 读取 + 录制 default 分支写，O）：@+0x30 u64→state+0xe8、
  @+0x38 u32→state+0xf0、@+0x3c u8→state+0xf4、@+0x40 Vec2（游标 delta，clamp 后 u32 对写入
  游标）、@+0x44 f32 除数（time_delta 用，`delta/divisor` 归一）、@+0x68..+0xb8 状态块
  11 qword→state+0x58..0xa8、@+0xc0 Vec2（prev pos，lerp 用）。**录制分支把当前状态 11 qword
  （state+0x58..0xb0 读）写入帧 @+0x68..0xb8，与回放读一致。**
- 帧头两 Vec（cap-first @+0x00/@+0x18，空载 {0,4,0}）在剪帧 drop 时按指针释放（见 2.5）。

### 2.2b InputFrame `deserialize::visit_str` 0x1413fe800（O，本轮闭合，机器码手解）

RE-MCP NotFound（函数未建）→ capstone 手解（落盘 `input/decomp/hand-0x1413fe800-visitstr.txt`）。
身份 = InputFrame Deserialize visitor 的键名→字段序号匹配（与 serialize<Pretty> 0x1413fe6f0 逆）。

```
visit_str(out: &mut [u8;2], key: &str)   # out[0]=0 固定；out[1]=字段序号 0..3，4=unknown
r8 = key.len - 0x11                       # 有效长度 17/18/21/26（r8 = 0/1/4/9）
if (u64)r8 > 9 → default（out[1]=4）
jump table @0x142c5baa0（10 项 rel32，基 rcx = lea [rip+0x185d288]）分派：
  len 17 → 0x1413fe822: pcmpeqb [rdx] vs "translation_stat"(0x142c5af00) +
                        movzx byte [rdx+0x10] vs 'e'(0x142937ee0)      → 命中 out[1]=0
  len 21 → 0x1413fe85a: pcmpeqb [rdx] vs "main_action_set_"(0x142c5aec0) +
                        pcmpeqb [rdx+5] vs "action_set_value"(0x142c5aeb0) → 命中 out[1]=3
  len 18 → 0x1413fe88e: pcmpeqb [rdx] vs "time_delta_secon"(0x142c5aed0) +
                        movzx word [rdx+0x10] vs "ds"(0x14295db20)     → 命中 out[1]=2
  len 26 → 0x1413fe8c2: pcmpeqb [rdx] vs "prev_normalized_"(0x142c5aef0) +
                        pcmpeqb [rdx+0xa] vs "lized_cursor_pos"(0x142c5aee0) → 命中 out[1]=1
  其余长度/未命中 → out[1]=4（unknown 哨兵）
```
要点：
- **后缀共享 SIMD 双窗**：输入串错位读（[rdx]、[rdx+5]、[rdx+0xa]、[rdx+0x10]）对融合串池 16B 常量窗
  （@0x142c5aeae..0x142c5af10：`action_set_value`+`main_action_set_`+`time_delta_secon`+
  `lized_cursor_pos`+`prev_normalized_`+`translation_stat`+`ightType`）——与 action_set visit_str
  0x1416f92d0 同族，但 InputFrame 侧更紧凑（单/双窗+长度跳表，无 46 槽）。
- 4 键确认 InputFrame serde 字段序 [translation_state, prev_normalized_cursor_pos,
  time_delta_seconds, main_action_set_value]（variant 序 = 字段序，见 §2.2 修正）。
- pmovmskb 全 0xffff（16/16 位全等）才判命中；len17 用 byte、len18 用 word 补尾窗。
- 函数边界：0x1413fe800..0x1413fe905（0x105B），0x1413fe910 起为 InputReplayInitialState
  serialize<Pretty>（键 @0x142c5b422..0x142c5b46d，0xb/0xc/0xd/0xd/0xb/0xf/0xf 长度 =
  window_size/cursor_depth/cursor_pos_ws/center_pos_ws/ui_settings/sheep_positions/sheep_rotations）。

### 2.3 add_frame（0x140949190，O，算法）

```
add_frame(&mut self, frame: &InputFrame 0xd0B, record: bool)
if record && frames.len >= 9999:                    # (len-9999) 无符号回绕判 ≥
    if sync_points.len < 2:
        if frames.len >= 50000:                     # (len-49999) 回绕判 ≥
            pop_front(frames): head'=(head+1)-(cap<=head+1?cap:0), len-1,
                           释放被弹帧内层两堆指针（qword@+0, qword@+0x18 非零则 dealloc）
        # 否则什么都不做（保留帧，等同步点裁剪路径）
    else:
        drop_oldest_sync_point()                    # 同步点存在时走裁剪
push 尾: 栈拷贝 0xd0B → 满则 grow → slot=((len+head)-(cap<=len+head?cap:0))*0xd0+ptr
         memmove → len++
限频日志: 帧计数 × 0x5d4e8fb00bcbe61d 乘散列后取低…< 0x14f8b588e3690 才打
         （≈ 每 ~1/30 帧？）→ TweakerView::as_inner_mut 经 log crate（串 "Stream frame count: "，
         行 33 = input_stream.rs）
常量: 裁剪阈 9999（帧数）/ 50000（帧数，sync<2 分支）/ 9（同步点数）/ 10000（帧数，on_history_sync）
```

### 2.4 on_history_sync（0x1409494c0，O，算法）

```
on_history_sync(&mut self, state: &InputTranslationState, record: bool)
if !record:
    truncate(frames)=0（释放全部帧内层 Vec）；truncate(sync_points)=0
else:
    if sync_points.len >= 9 || frames.len >= 10000:   # 任一超限 → 丢最老同步点
        drop_oldest_sync_point()
push 尾: 12 qword = {state.q0@0, q1@8, q2@0x10, …, q10@0x50, frames.len@0x58}（0x60B）
常量: 9 / 10000
```

### 2.5 drop_oldest_sync_point（0x140949660，O，算法）

```
pop_front(sync_points): head' 回绕、len-1、按需释放被弹槽内容
prune: target = 被弹同步点.frame_index（@+0x58）
  while target != 0:
    if frames.len 下溢（-1 判）→ panic（"assertion failed: self.frames.pop_front().is_some()"
      路径，0x140949940 折叠处，panic 串见 panic 清单）
    pop_front(frames)（释放内层 Vec 两指针）
    target--
re-base: 对每个剩余同步点:
    if f.frame_index < target（=被删帧数）→ unwrap_failed panic（O：core::option::unwrap_failed）
    f.frame_index -= target
```

### 2.6 模式机（InputRecordReplayMode，5 变体）

- niche 存储：变体 i = `0x8000000000000000 + i`（i64::MIN 基址，PLAYBOOK §2.3）。
- Display（fmt 0x14094f2d0）变体名 = 权威：
  | 判别值 | 名称 | Display 串地址(len) |
  |---|---|---|
  | 0 | AutoRecordPanicRepro | 0x142ab3ad0 (20) |
  | 1 | Records | 0x142ab3b08 (7) |
  | 2 | Replay | 0x142ab3b38 (6) |
  | 3 | DisableInput | 0x142ab3b44 (12) |
  | 4 | None | 0x142ab3b50 (4) |
- **转移表**（O：input_record_replay 0x1409423d0 + record_and_replay_input_for_trailers
  0x14083c620 两系统）：

| 当前模式 | 事件 | 动作 | 新模式 | 证据 |
|---|---|---|---|---|
| 0 AutoRecordPanicRepro | 每帧（record=true） | 事件/标志触发时（`有未消费事件 || flag`）：store camera + 克隆 ToolbarCategoryModes + 重建流（frames cap 8、1 同步点）；然后 add_frame | 0 | 0x1409423d0 default 分支 |
| 1 Records | 每帧（record=false） | add_frame（不裁剪） | 1 | 同上（flag=(mode!=1)&&mode<5） |
| 2 Replay | 每帧 | index++；取帧写回 InputTranslationState/游标/prev_pos；帧尽 → 4 | 2→4 | case 2 |
| 3 DisableInput | 每帧 | 清零输入状态 qwords | 3 | case 3 |
| 4 None | 每帧 | 无 | 4 | case 4 |
| 0 或 4 | 'O' 键（配置字节@+0xfc==1） | log "Start recording" + store camera + 重建流 + mode=0 | 0 | 0x14083c620 case 0/4 |
| 1 | 'O' 键 | log "Finished recording" + 释放流 + mode=4 | 4 | 0x14083c620 default |
| 2 或 3 | 'O' 键 | 无 | 2/3 | case 2/3 |
| 任意 | 第二键（0x1c） | log "Replay..." + mode=2 + index=-1 + seek_to_sync_point(0) | 2 | 0x14083c620 尾部 |

- try_load 成功 → mode = 2（Replay），index = -1（O：`param_1[8]=-0x7ffffffffffffffe; param_1[9]=-1`）。
- 回放 seek（内联于 control_input_replay 0x140941a30，O）：sync_points 空 → panic
  "No sync points found in replay state"；mode != 2 → panic
  "Can only seek to sync point if replaying but mode is " + fmt(mode)（input_stream.rs）；
  成功：index = sync[..].frame_index - 1，恢复该同步点状态（游标 qwords、ToolbarCategoryModes
  克隆、camera 参数）。

### 2.7 try_load（0x1409499c0，O）

```
try_load(&mut self, path) -> Result<(), ()>
读串（FUN_140d59140）失败 → Err（哨兵 i64::MIN，串 "Failed to read the file" @0x142ab35f0 23）
serde_json::from_str（FUN_1414053b0）失败 → Err（"Failed to deserialize" @0x142ab3607 21）
成功 → 写 8 qword（两 VecDeque）+ mode=0x8000000000000002(Replay) + index=-1
```

### 2.8 索引与长度（O 原字节）

- `sync_point_count` 0x140949170 = `mov rax,[rcx+0x18]`；`frame_count` 0x140949180 = `[rcx+0x38]`。
- `last_frame_mut` 0x140949480 = `((len+head-1)-(>=cap?cap:0))*0xd0 + ptr`。
- `impl$2::index` 0x140949970：`index < len → ((index+head)%cap 等价式)*0xd0 + ptr`，
  越界 panic "Out of bounds access"（@0x142ab3224，20 字符）。

### 2.9 顶层文件 serde（O）

- 录制文件 = `{replay_initial_state, first_frame}`（serialize<Compact> 0x1402e0060 两键，
  串 0x1429b3d4e/0x1429b3d62）。
- InputReplayInitialState 10 字段（串表 0x142c5b3d0 区，供 trailer 初始态恢复）：
  `window_size, cursor_depth, cursor_pos_ws, center_pos_ws, ui_settings, sheep_positions,
  sheep_rotations, app_mode, toolbar, camera`。
- InputStream serde 键 = `sync_points, frames`（同表区）。

---

## 3. 模式跟踪（mod.rs）：ActiveInputModeTracker

### 3.1 布局与算法（O：本轮反汇编 0x140958eb0/ef0/f30）

```
ActiveInputModeTracker = 0x20 B
  @0x00: controller_deadline: Instant {secs u64@0, nanos u32@8}   # 控制器"最后使用+冷却"截止
  @0x10: mouse_kb_deadline:  Instant {secs u64@0x10, nanos u32@0x18}
new_default_controller:  {controller=(now+250ms), mk=now}
new_default_mouse_keyboard: {controller=now, mk=(now+250ms)}
  # Instant::now() @0x140d635e0；now+250ms @0x140d63a20（r8=0, r9=0xee6b280=2.5e8 ns 冷却常量）
mode() = bool = (controller_deadline > mouse_kb_deadline)
  算法（O 全字节）：(secs@0 > secs@0x10) || (secs@0 == secs@0x10 && nanos@8 > nanos@0x18)
  实现：cmp+setne/cmovb 双段比较，`cmp r9b,1; sete al` 归一 bool
语义（I）：截止更晚的一方 = 更近使用 → mode=true=控制器活跃；两半相等/都过期 → false=鼠标键盘
on_controller_used / on_mouse_keyboard_used（I，无独立符号）：重置对应半 = now+250ms
  （wp15 证据：内联进 systems::steam::update_steam_inputs）
```

### 3.2 ActiveInputMode（O）

- u8 判别枚举：`MouseKeyboard=0 / Controller=1`；`is_controller` 0x1408c5680 =
  `movzx eax, byte [rcx]; ret`（返回判别字节即 bool，31 调用点；与 StructureType::is_fence
  折叠同址 0x1408c5680——语义巧合，两侧判别不同）。

---

## 4. PanicReproState（panic_repro_state.rs）

- `is_inactive` 0x140833230 = `cmp byte [rcx],0; sete al; ret`：首字节判别 == 0 → inactive。
- 17 调用点（DIGEST 登记），gates 调试 panic 复现输入路径（I）。

---

## 5. 与 backend/input 系统的调用链（数据流）

```
┌─ 每帧输入流水 ─────────────────────────────────────────────────────────────┐
│  bevy 事件（键盘/鼠标/硬件窗口）                                            │
│    → translate_input 0x1417cd0a0（Analog→ActionSetValues、                 │
│       Digital→ActionSetStates、MouseMoved 消费；wp20 合同）                 │
│    → translate_input_to_cursor_moved 0x140b17a80（事件双队列→游标 delta）    │
│    → calculate_input_delta 0x1416feeb0（analog 死区 0.1/0.05 累加）          │
│    → reset_input_state 0x140942b00（11 qword 置换清零 + 按最后同步点修剪     │
│       + ButtonInput reset_all）                                             │
│    → 下游消费: ActionSetValues/States → 46 动作（§1.2 槽位）                 │
│       → 相机/工具/UI 各系统                                                  │
└─────────────────────────────────────────────────────────────────────────────┘

┌─ 录制/回放流水 ────────────────────────────────────────────────────────────┐
│  History::finish_input_recording 0x1409dd790（history 域）                  │
│    └─ 推 0x30B 同步事件{路径+翻译态+flag}                                    │
│  input_record_replay 0x1409423d0（模式机主系统，§2.6）                       │
│    ├─ record: 条件重建流（store camera FUN_140911710 + Toolbar 克隆          │
│    │           FUN_1412f7fa0 + frames cap 8 + 1 同步点）                    │
│    │           + add_frame 0x140949190（帧含 MainActionSet 打包产物          │
│    │             Vec<[u8;12]>，deconstruct 0x140946b80 的产物消费点）        │
│    └─ replay: 逐帧写回 InputTranslationState/游标/prev_pos（§2.2 回放视图）  │
│  record_and_replay_input_for_trailers 0x14083c620（'O' 键启停 + 回放键 seek）│
│  control_input_replay 0x140941a30（消费 0x30B 同步事件）                     │
│    ├─ on_history_sync 0x1409494c0 × N                                       │
│    ├─ try_load 0x1409499c0（尾部事件路径）                                  │
│    └─ seek_to_sync_point（内联：mode 校验 + index=frame_index-1 + 恢复状态） │
│  串行化: InputStream/sync_points/frames → 录制文件                          │
│    {replay_initial_state, first_frame}（serialize<Compact> 0x1402e0060 /    │
│     Pretty 0x1413fe6f0；try_load 反向）                                     │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 6. 纠错 / 待定项登记（供集成 owner 裁决）

1. **MainActionSet 双形态字段序相反**（运行时 digital 前 @0x00 vs serde analog 前 @0x00）：
   两实例化或独立 serde 表示类型，机器码两侧都硬（deconstruct 0x140946b80 首读
   `movsd [rdx+0x140]` = analog@0x140、digital 8B 槽 @0x00..0x140 vs serialize 0x1400ed690
   `param_1[0..0xb]` 12 f32 = analog@0x00..0x30 + digital 40 字节 @0x30..0x58）。
   **本轮裁决**：`reconstruction/country-core/src/input/action_set.rs` `MainActionSet{analog,digital}`
   （analog 在前）只匹配 serde 形态，**不匹配运行时 digital-first 0x170B 形态**——恢复须分别建模
   （`MainActionSet<Vec2,DigitalActionState>` 0x170B + serde 专用 `<Vec2,bool>` 0x58B 表示）。
   wp15 合同「MainActionSet {analog, digital}」布局 U 的判定过时。
2. **InputFrame 两视图偏移冲突（本轮裁决：不同子结构，非同一字段不同偏移）**：
   serde translation_state@0x00（88B 平坦持久化字段）vs 回放读取状态块
   @0x68..0xb8（11 qword→ state+0x58..0xa8）。录制分支（input_record_replay default）把当前状态
   11 qword（state+0x58..0xb0 读）写入帧 @+0x68..0xb8，与回放读一致（O 两侧）。两视图仅在
   @0xc0（Vec2 prev_pos/prev_normalized_cursor_pos）一致；@0x68 在 serde 是 main_action_set_value
   Vec、在回放是状态块 qword0。**裁决：不同子结构/表示**，恢复时 serde 布局（translation_state@0x00、
   main_action_set_value@0x68、prev_normalized_cursor_pos@0xc0、time_delta_seconds@0xc8）与回放帧布局
   （状态块@0x68..0xb8）分别建模，勿以单一 0xd0B 结构同时满足两偏移。修正旧写「@0x68..0xc0」→
   实为 @0x68..0xb8（第 11 qword 在 +0xb8）。
3. **serde 串名 `prev_normalized_cursor_pos`（26 字符）**：非 `prev_normalized_cursor_position`
   （30 字符）。串表 0x142c5b3e1 起 26 字节即该名（融合表后缀共享，无 "ition" 续串）——
   JSON 键以此为准（reconstruction/文档中 30 字符写法需改）。
4. **DIGEST 旧标注纠偏**：input_record_replay 的 case 0/2/3/4 语义名
   （Idle/Replaying/Recording/Finished）实为 **AutoRecordPanicRepro/Replay/DisableInput/None**
   （Display 串权威）；「mode u32@0」实为控制器截止秒（mode() 比较派生，无存储判别位）。
5. try_load 写入序 `{ptr,cap,len,head}`（Ghidra 呈现）与运行时 cap-first `{cap,ptr,len,head}`
   不符，疑 Ghidra 重排或 serde 构造序差异——以 add_frame 机器码布局为运行时准。
6. reconstruct 反编译在 0x140948727 起有坏指令数据（case 0x19/0x20 halt 区），
   Ghidra 未建函数的 3 个 NotFound（0x1408c5680/0x140958f30/0x140946620）已原字节手解。

## 7. 本轮证据落盘

- 新反编译：`input/decomp/decomp-0x1400ed690.txt`（serialize Compact）、
  `decomp-0x1416f8680.txt`（Pretty）、`decomp-0x1416f92d0.txt`（visit_str）、
  `decomp-0x1409499c0.txt`（try_load）、`decomp-0x140949970.txt`（index）、
  `decomp-0x14094f2d0.txt`（fmt）、`decomp-0x140947c80.txt`（reconstruct 复采）、
  `decomp-0x1413fe6f0.txt`（InputFrame Pretty）、`decomp-0x1402e0060.txt`（录制文件 Compact）。
- **round3 新增手解**：`input/decomp/hand-0x1413fe800-visitstr.txt`（InputFrame visit_str 全函数，
  capstone + rip 相对常量解析；RE-MCP tg_ci1 NotFound → 机器码为准）。job/out 原样：
  `input/decomp/job-ci1.json`。
- job/out 原样：`input/decomp/job-tg_m5.json / out-tg_m5.json / job-xref.json / out-xref.json /
  job-serde.json / out-serde.json`。
- 既有复用：wp15 `analysis/b1_input/*.c`、ui-input `decomp/decomp-0x14083c620.txt /
  0x140941a30.txt / 0x1409423d0.txt / 0x1416feeb0.txt / 0x1417cd0a0.txt`、
  history `decomp/decomp-0x1409dd790.txt`。
- 手解工具：`evidence/country-core-deep/tools-extra/disasm-va.py`、`disasm-consts.py`（capstone）。
