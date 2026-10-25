# REVIEW — systems/backend/input

> ⚠️ 本文为**一轮(2026-10-15)**审查,其中对 `translate_input_to_cursor_moved` 的"事件流反转/
> 自造 InputCursorDelta/夹取 cursor-192"结论**已被二轮 `REVIEW2.md` 依据 PDB 类型签名 +
> 调用者签名 + 机器码推翻撤回**(方向正确:consumes InputCursorMoved → produces
> InputCursorDelta;夹取 = WindowSize;192 = i32→f64 魔数)。以 REVIEW2 为准。

审查员: reviewer-interaction (独立核对,以原证据为准)
日期: 2026-10-15
审查范围: 当前 src 已有实现 + 原函数清单逐项核对。restorer 尚未提交 RESTORE.md / 新增 5 文件(translate_input / input_record_replay / input_replay_for_trailers / reset_input_state / update_cursor_position),本次只审已有现状并给出新增工作的核对基线。

## 核对方法

- inventory.json 枚举原函数 + 章节 + 新鲜反编译复核:
  - `calculate_input_delta` @0x1416feeb0: fresh get_functions(shared 只读),30KB decomp 已逐槽比对;全文落盘 `evidence/decomp-0x1416feeb0-fresh-shared.txt`(1354 行)。
  - `translate_input_to_cursor_moved` @0x140b17a80: 既有 decomp(FUN_140b17a80)全文核对。
  - 原点常量 `0x1429e95b0` read_memory = `{0x43300000,0x43300000}` = Vec2(192.0,192.0)。
  - 8 个函数 signature 复核(均 `void __cdecl (void)`,PDB 占位,不能当真实 ABI)。
- 断言均以反编译/字节为准,不采信头注释"旧章节读写了反转"之类的自述。

## 原函数清单(no_formal_mapping ≠ 零实现)

| 原函数 | VA | 当前 src | verdict |
|---|---|---|---|
| `calculate_input_delta::calculate_input_delta` | 0x1416feeb0 | calculate_input_delta.rs | **FIX** |
| `calculate_input_delta::digital_delta`(inline) | — | 同文件 | **FIX** |
| `translate_input_to_cursor_moved` | 0x140b17a80 | translate_input_to_cursor_moved.rs | **FIX(重)**,见下 |
| `translate_input::translate_input` | 0x1417cd0a0 | 缺失 | GAP |
| `input_record_replay::control_input_replay` | 0x140941a30 | 缺失 | GAP |
| `input_record_replay::advance_input_replay_frame` | 0x140942280 | 缺失 | GAP |
| `input_record_replay::input_record_replay` | 0x1409423d0 | 缺失 | GAP |
| `input_replay_for_trailers::record_and_replay_input_for_trailers` | 0x14083c620 | 缺失 | GAP |
| `input_replay_for_trailers::do_input_replay_ui` | 0x14083cdd0 | 缺失 | GAP |
| `reset_input_state::reset_input_state` | 0x140942b00 | 缺失 | GAP |
| `update_cursor_position::update_cursor_position`(inline) | — | 缺失 | GAP |

## 逐函数核对

### 1. calculate_input_delta @0x1416feeb0 — **FIX**

fresh 反编译确认:
- 输入 `ActionSetValues` 布局: previous.digital[40]@+0x30、current.analog[6]@+0x58、current.digital[40]@+0x88(与 action_set.rs 的 `{previous, current}` 两个 MainActionSetSerde{analog@0,digital@0x30} 叠加一致,即 previous.digital@+0x30、current.analog@+0x58、current.digital@+0x88)。
- `fVar1 = time.delta @ Time+0x58` → 与 `Res<bevy_time::Time>.delta_secs()` 一致。
- analog 6 槽原样拷贝到输出 ActionSetStates.digital[40] 之后的 analog 区 → `states.analog = values.current.analog` ✓。
- 数字沿语义(previous@+0x30 / current@+0x88):
  - 空闲(prev false,cur false)→ disc 4 ✓;按下(prev false,cur true)→ disc 0 ✓;松开(prev true,cur false)→ disc 3、val 保留 state.val(disc==1) 否则 0 ✓;repeat(disc 2):`val<=0` 时 `val=(val+0.1).max(0.05)`,再 `val-=dt` ✓;held(disc 1):`val+=dt` ✓。
- **缺失(确认 bug)**:40 槽并非同一规则。反编译中槽 20..25(`DIGITAL_NAMES[20..26]` = ui_left/ui_right/ui_down/ui_up/ui_tab_prev/ui_tab_next)是另一种"held"分支(Shape B):
  ```
  fVar7 = fVar7 + fVar1;
  bVar5 = fVar7 < 0.2;          // 定时器>=0.2
  if (!bVar5) fVar7 = 0.1;      // 复位到 0.1
  iVar6 = 2 - bVar5;            // disc = 2(repeat),触发连续 just_pressed
  ```
  即 UI 导航键长按自动连发(0.2s 初时延,0.1s 间隔)。当前 `digital_delta` 对所有 40 槽套 Shape A,槽 20..25 长按只累加 val 保持 disc 1,永不转 repeat —— **与原生行为不符**。该 6 槽 repeat 分支在字节 0x44..0x49/0x9c..0xa1 处逐一可见;槽 0..19 与 26..39 为 Shape A。
- 按下/空闲的 val 反编译为寄存器残留(非显式 0),但 disc 语义(just_pressed 只看 disc)不受影响,现值 0 可接受,不作 FIX。

**FIX 建议**:`update_action_states`/`digital_delta` 需按槽号分派:槽 20..25 用 Shape B(held 时 `if val < 0.2 {disc 1, val+=dt} else {disc 2, val=0.1}`),其余槽用 Shape A。测试需补 UI 长按自动连发用例(如 `ui_left` 槽 20 连续 0.25s → 第 3 帧 disc==2 && val==0.1)。

### 2. translate_input_to_cursor_moved @0x140b17a80 — **FIX(重)**

新鲜/既有反编译(FUN_140b17a80)显示原函数合同与当前实现**方向相反**:
- **原函数**:param_5 = EventReader 录制事件双队列(stride 0x18,tag@+8: 0=相对/1=绝对/2=跳过),逐事件折叠 → 生成**一个** `Events<InputCursorMoved>{id, Vec2=sum}`(param_6,stride 0x10 的 EventInstance),并累加 `ActionSetValues.current.analog[0] += sum`、`analog[1] += (最后 acc - 起始光标)`(param_4,state@+0x58/+0x60)。相对事件夹取:**`acc = clamp(acc + delta, 0, cursor - 原点)`**,原点 = `0x1429e95b0 = Vec2(192,192)`(本次 read_memory 证实)。
- **当前实现**:consumes `Events<InputCursorMoved>`(stride 0x10 枚举),produces `Events<InputCursorDelta>`(原二进制无此事件类型),夹取用 `window.as_vec2()`,并引入原函数没有的 `captured` 分支。
- 差异点:
  1. **事件流反转**:原函数是"录制事件 → InputCursorMoved 的**生产者**";当前实现把 InputCursorMoved 当**输入**、InputCursorDelta 当输出。头注释"旧章节把读写搞反了"与反编译相反 —— 章节是对的。
  2. **夹取上界错误**:原 `[0, cursor-192]`,当前 `[0, window_size]`。
  3. `InputCursorDelta` 是自造类型,原 inventory/章节无此事件。
  4. `captured`(iVar2==0/非0)在原函数里控制折叠路径(tag 0 夹取 vs 只加 tag≠1 原始值),当前实现的 captured 分支语义与 tag 1/2 处理不匹配。

**FIX 建议**:重新实现为原合同:consumes 录制帧事件(stride 0x18,tag@+8),输出 `Events<InputCursorMoved>`(保留原事件类型),夹取上界用 `cursor - Vec2::splat(192.0)`(或原常量地址的投影),analog[0]/[1] 累加与当前一致(那部分是吻合的)。若必须保留当前"CPU 适配器"形态(consumes InputCursorMoved),应明确改名为非原函数(如 `apply_cursor_moved_deltas`)并移除 `B 0x140b17a80` 标签,同时把真正 @0x140b17a80 放到录制链里。

### 3. mod.rs — PARTIAL

`register_input_events` 注册的 `Events<InputCursorDelta>` 非原事件;`add_input_cpu_consumers` 只链 translate_input_to_cursor_moved→calculate_input_delta,translate_input@0x1417cd0a0(analog/digital 主映射)缺失。属适配层,可保留但须与 FIX-2 一并改。

## 待取证

- 原 `Events<EditTerrainStrokeCmd>` 类事件是否带 bevy EventInstance id 前缀:translate_input_to_cursor_moved 原函数输出 stride 0x10 `{id, Vec2}` 证实 EventInstance 存在,但 InputCursorMoved 之外的其他事件(输入录制帧)布局依章节为准,新增函数实现时需按 0x140b17a80/0x1409423d0 的 stride 0x18/0xd0 复核。

## 对新增工作(restorer 批次)的核对基线

restorer 将新增 translate_input/input_record_replay/input_replay_for_trailers/reset_input_state/update_cursor_position。核对要点(章节+decomp 已备):
- `translate_input`@0x1417cd0a0: analog 动作经 FUN_14089aef0 族翻译进 ActionSetValues;digital 沿进 ActionSetStates;MouseMoved 双缓冲排空。
- `input_record_replay`@0x1409423d0: mode@+0x40(niche 0x8000000000000000+变体),Idle 建流 {frames cap 8 + 1 sync},Replaying 按 0xd0B InputFrame 写回,Recording 清零,Finished no-op;越界 panic "Out of bounds access"。
- `control_input_replay`@0x140941a30 / `advance_input_replay_frame`@0x140942280 / `reset_input_state`@0x140942b00 均按章节/既有 decomp 闭合。
- 这些函数目前全部缺失,审查时逐项 fresh 反编译。

## 结论

当前已有实现中 calculate_input_delta 有 1 处确认缺失(UI 6 槽自动连发),translate_input_to_cursor_moved 与原合同方向相反(事件流/夹取/自造类型)。input 模块**不能**依据现状宣布完成;restorer 新增批次前应先把这两处 FIX 纳入验收门槛。
