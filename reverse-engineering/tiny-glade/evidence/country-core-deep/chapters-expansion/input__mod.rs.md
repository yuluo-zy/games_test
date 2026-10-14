# input/mod.rs

> 文件作用：**<待填>**

函数总计 7：业务 4 / 内联无独立符号 2 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `input::ActiveInputModeTracker::new_default_controller` | 0x140958eb0 | 15 | <待填> | U | — |
| `input::ActiveInputModeTracker::new_default_mouse_keyboard` | 0x140958ef0 | 23 | <待填> | U | — |
| `input::ActiveInputModeTracker::mode` | 0x140958f30 | 32 | <待填> | U | — |
| `input::ActiveInputMode::is_controller` | 0x1408c5680 | 71 | <待填> | U | — |

## 内联函数名称推断

- `input::ActiveInputModeTracker::on_controller_used` (L39, inline_instance, inlinee_declaration)
- `input::ActiveInputModeTracker::on_mouse_keyboard_used` (L46, inline_instance, inlinee_declaration)

## 派生胶水分组

- `input::impl$3::clone` (L7)
