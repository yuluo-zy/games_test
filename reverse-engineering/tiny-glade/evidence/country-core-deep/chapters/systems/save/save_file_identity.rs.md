# systems/save/save_file_identity.rs

> 文件作用：存档文件身份识别：try_approx_parse 读文件搜字面量 "Identity" 后解析 JSON 值；derive_fallback 失败回退（文件元数据派生）。

函数总计 6：业务 2 / 内联无独立符号 0 / 派生胶水 4

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| ``country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity<std::fs::File>`` | 0x141e58f20 | 7 | 待推断 | O：try_approx_parse = 0x800B 缓冲→ReadFile→搜 "Identity"（串 0x142f92a40 len 10）→解析其后 JSON 为 SaveFileIdentity | session-save\decomp\decomp-0x141e58f20.txt, round2\session-save-CLOSURE.md  |
| ``country_core::systems::save::save_file_identity::derive_fallback_save_file_identity`` | 0x141673140 | 48 | 待推断 | O：derive_fallback = 解析失败回退：SystemTime + 二次尝试 + 元数据派生（字段 U） | session-save\decomp\decomp-0x141673140.txt  |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 4 项
