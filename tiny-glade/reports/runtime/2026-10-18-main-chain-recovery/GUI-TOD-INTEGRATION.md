# TimeOfDayScreen 原接口整合

日期：2026-10-18。B 原版证据；共享生产整合者 root。

本次未修改或抢占旧 GUI family claim `TG-cfac1f498c427933d2/core-gui-20261015`，也未改其原证据。OS 子任务在 TOD 输入合同中交付字段候选，root 对实际原函数重新独立查询后，将限定的资源 schema 合入共享生产 `country-core/src/resources/gui.rs`。此项修复类型合同，不代表完整时段界面已恢复。

原只读结果：`gui-time-of-day-original.json`。程序为 B patched EXE，revision `1791719479858:b17d372d:3`；对应发行 RNE SHA-256 为 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。未修改分析数据库。

| 原偏移 | 共享字段 | 原生产者/消费者 |
|---|---|---|
| +00 | time_of_day f32 | circle 回调写目标；TOD 读 |
| +04 | sun_xz_rotation f32 | angle 回调写目标；TOD 读 |
| +08 | just_opened bool | `do_time_of_day_screen@1405cc7f0` 写 visible && !was_visible |
| +09 | was_visible bool | 同函数尾写 visible |
| +0a | visible bool | GUI、mode_manager、TOD 共用 |
| +0b | time_input_active bool | `closure_0@140505780` clock 交互置 true；TOD 消费 |
| +0c | angle_input_active bool | 同回调 angle 交互置 true；TOD 消费 |

`init_state<TimeOfDayScreen>@1401f2660` 注册 size 16、alignment 4；startup 在 `1405a1a36/41` 清十三个值字节。生产采用 repr(C)、Default 清零，并对尺寸及五个布尔槽偏移作编译期断言。两个现有构造调用由 boot owner 同步 Default，其余字段不再由独立 TodScreen 伪资源代替。

原同一回调含 GUI 声音、导航、hover 动画、两个圆的布局和目标重映射；本次没有按字段合同自行创造这些界面消费者。它们仍须按对应原函数恢复与验收。
