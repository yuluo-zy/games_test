# systems/backend/flush_screen_log.rs

> 文件作用：**屏幕日志队列。全局静态 `Mutex<Vec<(String, LogType)>>`（0x14362bfe8 起），`log_error_to_screen`/`log_warning_to_screen` 把消息压入队列，`flush_error_log_to_screen` 系统排空队列并按类型着色写入 `ScreenLog` 资源（最近 5 条环）上屏。**

函数总计 3：业务 3 / 内联无独立符号 0 / 派生胶水 0

## 布局表（全局静态队列 0x14362bfe0 区）

| 偏移 | 大小 | 含义 | 证据 |
|---|---|---|---|
| 0x14362bfe8 | 1B | Mutex 状态字节（LOCK/UNLOCK，0/1/2） | flush/log_* 反编译 |
| 0x14362bfe9 | 1B | "处理中"标志（is_processing） | 同上 |
| 0x14362bff0 | 8B | Vec cap（cap-first） | 同上 |
| 0x14362bff8 | 8B | Vec ptr | 同上 |
| 0x14362c000 | 8B | Vec len | 同上 |
| 队列条目 | 0x20B | {len@0x00, ptr@0x08, len@0x10, kind@0x18}（kind: 0=Error, 1=Warning） | log_error 写入位 |

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::backend::flush_screen_log::log_error_to_screen` | 0x1408c10a0 | 30 | 自由函数 `fn log_error_to_screen(msg: &str/String)`：clone 字符串（`func_0x000140613c10`+`func_0x000142923c00`），按需 `FUN_140562c30` 扩容队列，写入条目 kind=0（Error），len++。**不是 `log::error!`**（重构版本误写）。 | O | decomp/decomp-0x1408c10a0.txt |
| `country_core::systems::backend::flush_screen_log::log_warning_to_screen` | 0x1408c1310 | 38 | 同上，条目 kind=1（Warning）。 | O | decomp/decomp-0x1408c1310.txt |
| `country_core::systems::backend::flush_screen_log::flush_error_log_to_screen` | 0x1408c0d90 | 16 | Bevy 系统 `fn flush_error_log_to_screen(mut screen_log: ResMut<ScreenLog>)`：取走全局队列（len 置 0），逐条调 `FUN_14093fe50(screen_log, text, color)`；颜色按 kind 查表 0x142aa159c（0=0xff0000ff 红, 1=0xffff00ff 黄）。**虽名 flush_error 实际排空全部错误+警告条目**。 | O | decomp/decomp-0x1408c0d90.txt；rne-read --hex 0x142aa159c（颜色表） |
| （助手）`FUN_14093fe50` | 0x14093fe50 | — | `ScreenLog::push_line(text: String, color: u32)`：5 槽环缓冲（`+0xb0` 计数 `(n+1)%5`），每槽 0x20B {String, color@0x28, 5.0f32@0x2c}，超 5 条 panic。 | O | decomp/decomp-0x14093fe50.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

（无）

## 派生胶水（derive/宏生成；不计独立业务）

（无）

## 备注

- 队列为进程级全局静态，`log_*_to_screen` 可被任意线程调用；`flush_error_log_to_screen` 在 ECS 主线程消费。
- 名称含 "error" 却处理两类日志，属于命名误导（见 DIGEST 错位/纠错）。
