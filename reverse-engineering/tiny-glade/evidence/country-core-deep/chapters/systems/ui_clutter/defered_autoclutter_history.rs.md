# systems/ui_clutter/defered_autoclutter_history.rs

> 文件作用：延迟记录 auto-clutter 历史（RecordAutoClutterHistory 事件 → history）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ui_clutter::defered_autoclutter_history::history_record_auto_clutter` | 0x1409a9160 | 16 | history_record_auto_clutter | O（wp20 既有；346B） |  |

## 实现方式（算法级）

- `history_record_auto_clutter` @0x1409a9160（O，346B，本域手解）：延迟记录 auto-clutter 历史。
  - 排空 `RecordAutoClutterHistory` 事件队列（bevy VecDeque，元素 stride 0x10，读 `u64@+8` 载荷）。
  - 每条事件：alloc 0x18B ArcInner `{strong=1, weak=1, value}` → `FUN_140a84ec0(writer, tag 0xd, UNK_142abd990)` 入历史记录队列（tag 0xd = AppMode Replay 判别值，历史回放联动，I）。
  - `FUN_140a84ec0` 与 rmb_autoclutter 共现 = 历史记录推送统一入口（I）。
