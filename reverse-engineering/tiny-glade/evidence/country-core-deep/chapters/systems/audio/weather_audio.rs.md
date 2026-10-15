# systems/audio/weather_audio.rs

> 文件作用：**天气/季节全局音频参数（Weather→Studio 全局参数）。**

函数总计 1：业务 0 / 内联无独立符号 1 / 派生胶水 0

## 实现方式（算法级）（O 串级证据 + U 参数名）

**weather_audio_system（行 5，无独立符号，逻辑并入调用点）**：从 Weather 状态经
`AudioCore::set_parameter` 驱动季节/天气全局参数（O，fill_chapters 串级）；参数名本身
**U**（未反编译钉死，音频事件系统 batch 未覆盖该模块独立函数）。I

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::audio::weather_audio::weather_audio_system`（行 5）— 待推断
