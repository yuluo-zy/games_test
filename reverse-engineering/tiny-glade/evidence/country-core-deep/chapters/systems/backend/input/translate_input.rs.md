# systems/backend/input/translate_input.rs

> 文件作用：输入映射主系统：Analog→ActionSetValues、Digital→ActionSetStates、MouseMoved 事件消费（wp20 合同复核，34KB 反编译已落盘）。

函数总计 5：业务 1 / 内联无独立符号 0 / 派生胶水 4

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::backend::input::translate_input::translate_input` | 0x1417cd0a0 | 28 | 系统:输入处理 | O：输入映射主系统（wp20 合同 + 本域 34KB decomp 复核一致） | ui-input\decomp\decomp-0x1417cd0a0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 4 项
