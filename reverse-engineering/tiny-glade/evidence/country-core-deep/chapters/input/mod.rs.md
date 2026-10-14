# input/mod.rs

> 文件作用：**活跃输入模式跟踪**。`ActiveInputModeTracker` = 0x20 B `{u64 @0, u32 @8, u64 @0x10,
> u32 @0x18}`，跟踪最后使用的输入模式（控制器 vs 鼠标键盘）；`mode()` 比较两半返回判别；
> `ActiveInputMode` 2 变体（Controller=1/MouseKeyboard=0），`is_controller` 直接返回判别字节。
> 证据：new_default_controller/new_default_mouse_keyboard 反编译 + mode/is_controller 原字节。

函数总计 7：业务 4 / 内联无独立符号 2 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `ActiveInputModeTracker::new_default_controller` | 0x140958eb0 | 15 | 默认控制器模式跟踪器 | **O** | 构造 0x20 B，常量 0x0ee6b280(2.5e8) |
| `ActiveInputModeTracker::new_default_mouse_keyboard` | 0x140958ef0 | 23 | 默认鼠标键盘跟踪器 | **O** | 同构（默认鼠标键盘） |
| `ActiveInputModeTracker::mode` | 0x140958f30 | 32 | 当前模式判别 | **I** | 原字节：比较两半 + 0xff 哨兵 |
| `ActiveInputMode::is_controller` | 0x1408c5680 | 71 | 是否控制器模式 | **O** | 原字节：返回判别字节（Controller=1） |

## 内联/无独立符号函数

- `ActiveInputModeTracker::on_controller_used`（行 39）— **I**（标记控制器使用，名称推断）
- `ActiveInputModeTracker::on_mouse_keyboard_used`（行 46）— **I**（标记鼠标键盘使用，名称推断）

## 派生胶水（1 项：clone 等）
