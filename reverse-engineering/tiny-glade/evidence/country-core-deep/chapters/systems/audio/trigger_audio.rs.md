# systems/audio/trigger_audio.rs

> 文件作用：**音频触发器：相机穿过树木触发盒的响度（4 实例 + audibility 平滑）。**

函数总计 3：业务 2 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::trigger_audio::trigger_audio_system | 0x1409eafa0 | 0 | 相机段×AudioTriggerLibrary 全部盒 Aabb3::intersect_segment：命中 audibility+0.5 否则-0.1 clamp[0,1](@+0x34)；<=0 停 High Wind；音量=位移/阈值×0.0125×位置衰减 lerp 平滑(@+0x35)；相机 z>=11 或 dist<=70 停 InForest 快照否则 start+set_parameter；set_volume | O | audio-misc\decomp\decomp-0x1409eafa0.txt |
| country_core::systems::audio::trigger_audio::trigger_audio_init_system | 0x1409ea8d0 | 34 | 4×instantiate_audio：Camera Tree Rustle/Amb/High Wind/Camera Whoosh/snapshot:/InForest | O | audio-misc\decomp\decomp-0x1409ea8d0.txt |

## 实现方式（算法级）（O，decomp-0x1409ea8d0/0x1409eafa0）

**trigger_audio_init_system @0x1409ea8d0**：4×instantiate_audio：`Camera Tree Rustle`/`Amb`/
`High Wind`/`Camera Whoosh` + `snapshot:/InForest`。（O）
**trigger_audio_system @0x1409eafa0**：
- 相机运动段 × `AudioTriggerLibrary`（asset_libraries 域）全部盒 `Aabb3::intersect_segment`：
  命中 → audibility `+0.5`，否则 `−0.1`，clamp[0,1]（@+0x34）。（O）
- audibility ≤ 0 → 停 `High Wind`；音量 = 位移/阈值 × 0.0125 × 位置衰减，`lerp` 平滑（@+0x35）。（O）
- 相机 z ≥ 11 或 dist ≤ 70 → 停 `snapshot:/InForest`；否则 start + set_parameter。（O）
- set_volume 收尾；无逐 kind 事件分派（触发来自体素/射线命中）。O

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项
