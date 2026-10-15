# systems/ui/signifiers/signifier_create.rs

> 文件作用：UiSignifierStream：0x170B 无锁环形流（read cursor@+0x160、write counter@+0x168、0x28B 槽）；write 自旋追加；signifier_create 每帧排空并生成指示符实体。（重建代码写成 Vec<UiAppendSignifier>是错误的——真实是环形流。）

函数总计 4：业务 2 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write` | 0x140b2cf90 | 24 | UiSignifierStream::write | O（wp15 b7）：拷贝 0x28B 记录进槽 + 自旋推进游标（36 callers） | wp15/analysis/b7_signifiers/write_0x140b2cf90.c |
| `country_core::systems::ui::signifiers::signifier_create::signifier_create` | 0x140b2cfa0 | 29 | 系统:排空生成（signifier_create） | O（wp15 b7）：drain → spawn signifier 实体 | wp15/analysis/b7_signifiers/signifier_create_0x140b2cfa0.c |

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 2 项

## 实现方式（算法级）

- `UiSignifierStream` = 0x170B 无锁环形流（O）：`write` @0x140b2cf90 拷贝 0x28B `UiAppendSignifier` 记录进槽 + 自旋推进写游标（read cursor@+0x160、write counter@+0x168），36 callers（O wp15 b7）。
- `signifier_create` @0x140b2cfa0（O，wp15 b7）：每帧 drain 环形流 → 逐条 spawn signifier 实体。
- 重建代码写成 `Vec<UiAppendSignifier>` 是错的（O）：真实为无锁环形流（DB 布局证据），无锁写方与排空方并发安全。
- 处理链（O 组合）：widget 交互 → `write` 入流 → signifier_create 排空生成实体 → animation 推进 → mark_for_delete/despawn 回收。
