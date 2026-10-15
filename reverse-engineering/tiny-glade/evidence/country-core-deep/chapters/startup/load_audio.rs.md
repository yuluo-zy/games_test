# startup/load_audio.rs

> 文件作用：**音频系统启动：注册 audio 源/资源容器。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::load_audio::load_audio` | 0x140b0e0f0 | 9 | `load_audio`：构造 3 个音频对象（0x1c/0xc/8B 记录）注册进 audio 资源；读配置（全局 gate `*puRam+0x...c3a8`）。细节 U（音频句柄类型）。 | **O** | decomp-0x140b0e0f0.txt |

## 内联/无独立符号函数

- （无）

## 派生胶水

- 0 项： 等（derive 直接复用，不计业务）。

## 实现方式（算法级，2026-10-15 深挖）

- **load_audio 0x140b0e0f0（O）**：FUN_140d635e0 取全局音频系统句柄 → FUN_140a57100 注册
  → 构造 3 个音频对象（0x1c/0xc/8B 记录）入 audio 资源容器（cap-first Vec push x2）；
  `if (2 < *puRam000000014362c3a8)` 门控 info 日志（FUN_140d63910/FUN_140d8a550，'Reading …' 族）。
  细节 U：音频句柄类型（audio-misc 域覆盖）。
