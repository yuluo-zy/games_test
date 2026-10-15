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

## 实现方式（算法级）（O，decomp-0x1408c0d90/0x14093fe50 全解）

**flush_error_log_to_screen @0x1408c0d90（系统）**：
1. `LOCK(); mutex_state==0 → 置 1; UNLOCK()`；未获取 → FUN_1428d8750（等待唤醒）。
2. `panic 中检查`：`(*0x14362c420 & 0x7fffffffffffffff) == 0 → uVar8=0`，否则
   `uVar8 = FUN_1428d86b0() ^ 1`（报告抑制语义）。
3. **取走全局队列**：`ptr = 0x14362bff8`、`len = 0x14362c000`（`len = 0` 复位）。
   `is_processing@0x14362bfe9 != 0 → panic "called Result::unwrap() on an Err value"`
   （0x142aa03b8，line 0x2b，Mutex 中毒）。
4. **逐条排空**：条目 stride 0x20 = `{cap@0, ptr@8, len@0x10, kind@0x18}`；
   `FUN_14093fe50(screen_log, text_ptr, text_len, 颜色表[kind])`，颜色表 0x142aa159c
   （0=0xff0000ff 红, 1=0xffff00ff 黄）→ 释放字符串 `func_0x000140613c20(ptr, cap, 1)`。
5. **尾部清理**：未消费条目逐个释放（cap-first 双指针）。
6. `uVar8==0 && panic 进行中 && !FUN_1428d86b0() → is_processing=1`。
7. `LOCK(); state=0; UNLOCK(); state 原为 2 → WakeByAddressSingle`（唤醒等待线程）。

**FUN_14093fe50 @0x14093fe50（ScreenLog::push_line，O）**：
- `len<0 / alloc 失败` → FUN_1428d8fe3（异常）；alloc+memcpy 文本（len==0 时 alloc 1B）。
- `count@+0xb0 < 5`：槽 = `screen_log + count*0x20`；drop 旧槽字符串（ptr≠0 才释放）；
  写 `{cap=len@+0x10, ptr@+0x18, len@+0x20, color u32@+0x28, 0x40a00000(5.0f)@+0x2c}`；
  `count = ((count+1)/5)*-5 + count + 1` = **(count+1) mod 5**（5 槽环）。
- `count >= 5 → FUN_1428d9518()`（panic_bounds_check，防御——正常被 mod 不变量阻止）。

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

（无）

## 派生胶水（derive/宏生成；不计独立业务）

（无）

## 备注

- 队列为进程级全局静态，`log_*_to_screen` 可被任意线程调用；`flush_error_log_to_screen` 在 ECS 主线程消费。
- 名称含 "error" 却处理两类日志，属于命名误导（见 DIGEST 错位/纠错）。
