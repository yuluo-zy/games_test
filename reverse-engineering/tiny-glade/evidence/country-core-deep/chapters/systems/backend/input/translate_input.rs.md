# systems/backend/input/translate_input.rs

> 文件作用：输入映射主系统：Analog→ActionSetValues、Digital→ActionSetStates、MouseMoved 事件消费（wp20 合同复核，34KB 反编译已落盘）。

函数总计 5：业务 1 / 内联无独立符号 0 / 派生胶水 4

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::backend::input::translate_input::translate_input` | 0x1417cd0a0 | 28 | 系统:输入处理 | O：输入映射主系统（wp20 合同 + 本域 34KB decomp 复核一致） | ui-input\decomp\decomp-0x1417cd0a0.txt |

## 实现方式（算法级）（O，decomp-0x1417cd0a0 + wp20 复核）

**translate_input @0x1417cd0a0（输入映射主系统，34KB/多阶段）**：
1. **Analog 动作**：每个 analog 动作（CameraPan/cursor_move/camera_height 等）取值 →
   `FUN_14089aef0`（~50+ 调用点）翻译为 `ActionSetValues`；谓词族
   `FUN_14089a890/FUN_14089ab50/FUN_14089a9f0`（released/moving/deadzone 判定）。
2. **Digital 动作**：每个 digital 动作（left_click/undo/redo/tool_* 等）→ `ActionSetStates`
   （按键按下/释放沿）。
3. **MouseMoved 事件消费**：bevy `Events<MouseMoved>` 双缓冲排空 → 进翻译状态。
4. **输出**：写 `ActionSetValues/States` 资源（cap-first Vec 布局），联动
   `calculate_input_delta`（kind2 死区 0.1/0.05、kind1 累加、kind0 保留 prev）。
帧顺序：`FUN_140d635e0`（帧计数）开头取 time；主翻译段在 0x140b85350 后的逐动作循环。

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 4 项
