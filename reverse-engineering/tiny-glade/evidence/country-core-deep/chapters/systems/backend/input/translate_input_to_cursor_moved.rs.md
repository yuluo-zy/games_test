# systems/backend/input/translate_input_to_cursor_moved.rs

> 文件作用：录制事件双队列（stride 0x18，tag@8：0=mouse_moved/1=moved/2=跳过）→ 累加游标 delta，NaN 安全clamp，推 InputCursorMoved（0x10B {id, Vec2}，bevy 0.16 EventInstance {id@0, event@8} 布局）并累加进输出状态 @+0x58/+0x60。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::backend::input::translate_input_to_cursor_moved::translate_input_to_cursor_moved` | 0x140b17a80 | 0 | 系统:输入处理 | O：wp20 合同复核 + 本域 decomp：{计数@0, Vec2@8} push 证实官方 EventInstance 布局 | ui-input\decomp\decomp-0x140b17a80.txt |
