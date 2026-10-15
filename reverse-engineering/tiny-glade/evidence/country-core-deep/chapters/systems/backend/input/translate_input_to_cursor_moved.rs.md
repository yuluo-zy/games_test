# systems/backend/input/translate_input_to_cursor_moved.rs

> 文件作用：录制事件双队列（stride 0x18，tag@8：0=mouse_moved/1=moved/2=跳过）→ 累加游标 delta，NaN 安全clamp，推 InputCursorMoved（0x10B {id, Vec2}，bevy 0.16 EventInstance {id@0, event@8} 布局）并累加进输出状态 @+0x58/+0x60。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::backend::input::translate_input_to_cursor_moved::translate_input_to_cursor_moved` | 0x140b17a80 | 0 | 系统:输入处理 | O：wp20 合同复核 + 本域 decomp：{计数@0, Vec2@8} push 证实官方 EventInstance 布局 | ui-input\decomp\decomp-0x140b17a80.txt |

## 实现方式（算法级）（O，decomp-0x140b17a80 全解）

**translate_input_to_cursor_moved @0x140b17a80（录制事件 → InputCursorMoved 合成）**：
1. **遍历录制事件双队列**（param_5 = Events，元素 stride 0x18 = 24B）：tag@+8，
   载荷 Vec2（2×f32）@+0xc；bevy Events 双缓冲排空（counter - head - len）。
2. **tag 分派**：
   - **tag == 0（mouse_moved 相对）**：`cursor = 当前游标资源(param_2) - 原点(0x1429e95b0)`；
     `acc = maxps(0, acc + 事件delta)`（下界 0）→ `acc = minps(cursor, acc)`（上界 = 游标-原点）
     —— **clamp 到 [0, cursor-原点]**，全程 NaN 掩码安全（auVar31/27/32 三掩码）；
     `sum += 事件delta`。
   - **tag == 1（moved 绝对）**：`sum += (事件值 - acc)`（差分）。
   - **tag == 2**：跳过（continue 循环，不消费计数）。
3. **事件尽** → `Events<InputCursorMoved> push 0x10B`：`{id = 计数, Vec2 = {sum.x, sum.y}}`
   （元素 stride 0x10，符合 bevy 0.16 EventInstance {id@0, event@8}）。
4. **折叠进翻译状态**（param_4 = ResMut）：`状态@+0x58 += sum`（累计 delta）；
   `状态@+0x60 += (最后事件游标 - 当前游标)`。
