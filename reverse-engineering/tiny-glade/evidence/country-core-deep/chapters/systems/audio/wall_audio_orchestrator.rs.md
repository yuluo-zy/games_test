# systems/audio/wall_audio_orchestrator.rs

> 文件作用：**墙体材质音频协调器（RequestWallMaterialAudio）。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator | 0x141234bf0 | 0 | RequestWallMaterialAudio→hashbrown/atomic_float 协调参数更新（0x141234bf0） | O | audio-misc\decomp\decomp-0x141234bf0.txt |

## 实现方式（算法级）（O 结构，decomp-0x141234bf0；细节 U）

**wall_audio_orchestrator @0x141234bf0**：
- RequestWallMaterialAudio 事件 → `hash_one<RandomState,GlobalId>`（0x1405b0d70）查墙状态表
  （hashbrown）→ 按墙状态字段协调参数更新。（O）
- 读多个全局常量（0x142c1e598/0x142c1be34/0x1429cd078/0x1429cd02c/0x14292e6f0/0x142925904）
  计算材质参数；含跳表（`WARNING: jumptable` 截断）按状态分支。（O）
- **断言 panic**：`assertion failed: w.simplified_curve_2d.is_valid()`（0x142ada193，与
  debug_viz_threshold_curve 共用串）。（O）
- 参数值来源含 `[+0x58]` 缩放与原子浮点（atomic_float 域）；逐字段语义 U。
