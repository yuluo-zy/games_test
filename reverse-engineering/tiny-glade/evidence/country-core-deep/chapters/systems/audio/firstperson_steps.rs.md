# systems/audio/firstperson_steps.rs

> 文件作用：**第一人称脚步声：GroundType→FMOD 参数 + FirstPersonOff 全局参数。**

函数总计 3：业务 1 / 内联无独立符号 2 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::firstperson_steps::firstperson_steps | 0x140a889b0 | 39 | GroundType::to_fmod_param(行128)+全局参数 FirstPersonOff；GroundType==3 且相机运动时双线性采样地形栅格定材质；GroundType==0 播放行走 loop 否则停 | O | audio-misc\decomp\decomp-0x140a889b0.txt |

## 实现方式（算法级）（O，decomp-0x140a889b0）

**firstperson_steps @0x140a889b0**：
- GroundType 枚举 → `to_fmod_param`（内联，行 128）映射材质参数；另设全局参数
  `FirstPersonOff`（O 串/参数面）。（O）
- 材质判定：`GroundType==3` 且相机运动时 → 对地形栅格**双线性采样**定行走材质。（O）
- 播放控制：`GroundType==0` → 行走 loop `start_if_not_playing`；否则 loop `stop`。（O）
- init（行 29）与 GroundType::to_fmod_param（行 128）无独立符号，逻辑并入调用点。I

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::audio::firstperson_steps::firstperson_steps_init_system`（行 29）— 待推断
- `country_core::systems::audio::firstperson_steps::GroundType::to_fmod_param`（行 128）— 待推断
