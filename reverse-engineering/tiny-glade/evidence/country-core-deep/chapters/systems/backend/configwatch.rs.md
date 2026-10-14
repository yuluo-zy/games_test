# systems/backend/configwatch.rs

> 文件作用：**配置热载系统。每帧检查 `ConfigWatch`（互斥队列）是否有待处理的配置变更；有则日志 `'Configwatch: detected changes: N'` 并调用配置重载助手 `FUN_14093b530`（读 `assets/gladestate`+`colors.json` 路径 → 重载 ColorLibrary / SSBO 库），随后重载生效。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::backend::configwatch::configwatch` | 0x14085b1c0 | 12 | Bevy 系统（含 ResMut，+0x1c tick 回写）。锁 ConfigWatch 互斥项（`*param_3+0x30` 处 +0x10 字节标志，LOCK/UNLOCK 保护）；非空闲则 panic `Result::unwrap()`（configwatch.rs:43）；空闲时按日志门槛（全局 `>2`）输出 `'Configwatch: detected changes: '` + 计数；调 `FUN_14093b530`（重载助手）后清理队列。 | O | decomp/decomp-0x14085b1c0.txt；rne-read --unks：`'Configwatch: detected changes: '` 0x142a95080、`'called Result::unwrap()...'` 0x142a949f0、源文件 0x142a95048/0x142a95090（configwatch.rs） |
| （助手）`FUN_14093b530` | 0x14093b530 | — | configwatch 的重载助手：拼 `assets/gladestate` + `colors.json` 路径（`FUN_140d62810`/`FUN_140d76ed0` 路径拼接/to_string），`FUN_140969720` 读 ColorLibrary，`func_0x000142923bf0` memcmp 62B，复制 0x250B 配置结构到目标，`FUN_140beefa0`/`FUN_1428cab41` 重载 SSBO 库（`UNK_142c2e180`→ssbo_library.rs）。跨域到 asset_libraries{color,ssbo}_library。 | I | decomp/decomp-0x14093b530.txt；rne-read --unks：`assets/colors.json` 0x142ab1a40、`assets/gladestate` 0x142aef51a、`ssbo_library.rs` 0x142c2e180 |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

（无）

## 派生胶水（derive/宏生成；不计独立业务）

（无）

## 备注

- configwatch.rs 与 meshwatch.rs / texture_watch.rs 结构同构（锁 → 日志变更数 → 逐项重载 → 清理）。
- 源码行 43 处 `Result::unwrap` panic 对应 ConfigWatch 锁/队列取值的 unwrap。
