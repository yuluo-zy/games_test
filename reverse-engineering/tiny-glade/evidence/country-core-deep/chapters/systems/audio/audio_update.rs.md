# systems/audio/audio_update.rs

> 文件作用：**每帧/启动期音频更新：把 AudioSettings 音量推给 5 个 FMOD VCA，并把相机（或默认）位姿作为 3D 听者推给 Studio。**

函数总计 2：业务 2 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::audio_update::audio_update_system | 0x1409c8dd0 | 11 | 5×set_vca_volume(Master@+0xf8/Music@+0xfc/Music-DawnJingle/SFX@+0x100/Amb@+0x104) + 相机资源(+0xc0 3×3+0xf0 平移)算听者 + AudioCore::update；puffin scope | O | audio-misc\decomp\decomp-0x1409c8dd0.txt |
| country_core::systems::audio::audio_update::boot_audio_update_system | 0x1409c9320 | 37 | 同 VCA 段；听者用固定默认位姿（不读相机）；启动期系统 | O | audio-misc\decomp\decomp-0x1409c9320.txt |

## 实现方式（算法级）（O，multi-baseline §2 全线性反汇编）

**audio_update_system @0x1409c8dd0（每帧）**：
1. puffin scope 包帧；`[AudioCore+0x10]` 非空 → 5×`set_vca_volume`（0x140a54810）：
   Master@+0xf8（`vca:/Master`）、Music@+0xfc（`vca:/Music`）、Music-DawnJingle@+0xfc、
   SFX@+0x100（`vca:/SFX`）、Amb@+0x104（`vca:/Amb`）——音量均取自 AudioSettings 偏移。
2. 听者位姿：相机资源 +0xc0 3×3 旋转 + +0xf0 平移 → SIMD 位置/朝向算到栈区。
3. `AudioCore::update`（0x140a53a00=AudioCoreInner::update）：听者三断言
   （|forward|−1<0.001 / |up|−1<0.001 / forward·up<0.001）→ set_listener_attributes(0)
   → Studio::update，失败 log `"FMOD update error: "`。

**boot_audio_update_system @0x1409c9320（启动期）**：VCA 段逐条同构；听者不读相机，
构造固定默认位姿（−1.0 等常量）后走同一 AudioCore::update 链。
