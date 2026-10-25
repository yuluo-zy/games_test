# startup/startup_critters.rs

> 文件作用：**小动物（critters）资源载入：butterfly/duck/splash 网格 + shader + 音效。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::startup_critters::startup_critters` | 0x1409d49a0 | 30 | `startup_critters`：载入 butterfly, duck_flying, duck_heart, splash 网格；shader — firefly_glow, duck_animated, splash_particle；音效 `event:/SFX/Butterfly/Shoosh`。 | **O** | decomp-0x1409d49a0.txt |

## 内联/无独立符号函数

- （无）

## 派生胶水

- 0 项： 等（derive 直接复用，不计业务）。

## 实现方式（算法级，2026-10-15 深挖）

- **startup_critters 0x1409d49a0（21KB，O）**：6x 名式网格（butterfly、duck_flying、duck_heart、
  splash 等）+ 5x shader（firefly_glow、duck_animated、splash_particle 等）+ 4 个 spawn 记录
  + FMOD `event:/SFX/Butterfly/Shoosh` + 6x unwrap panic。
