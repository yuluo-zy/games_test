# systems/history/signifier_replay_active.rs

> 文件作用：回放激活指示器系统：ReplayModeElapsedTime（enum2$ 判别）活跃时累加，对每个带 SignifierReplayIndex 组件的实体把 Transform 朝光标目标做弹性抖动（ease_out_elastic_jiggle=0.5*exp2(-10t)*sin(7.853982t)）；register_required_components 注册必需组件。

函数总计 3：业务 2 / 内联无独立符号 1 / 派生胶水 0（已并入业务表）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（有依据推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::history::signifier_replay_active::impl$0::register_required_components` | 0x140b1d680 | 13 | 注册 SignifierReplayIndex 的必需组件 | O | decomp-0x140b1d680.txt |
| `country_core::systems::history::signifier_replay_active::signifier_replay_active` | 0x140b1b860 | 21 | 见文件作用 | O | decomp-0x140b1b860.txt |

## 内联/无独立符号函数（名称揭示意图；逻辑并入调用点）

- `country_core::systems::history::signifier_replay_active::ease_out_elastic_jiggle`（行 75）— 缓出函数（系数歧义 U）（U；内联；reconstruction 用 1-(1-t)^3）

<!-- 未覆盖函数数：0 -->
