# systems/audio/audio_update.rs

> 文件作用：**每帧/启动期音频更新：把 AudioSettings 音量推给 5 个 FMOD VCA，并把相机（或默认）位姿作为 3D 听者推给 Studio。**

函数总计 2：业务 2 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::audio_update::audio_update_system | 0x1409c8dd0 | 11 | 5×set_vca_volume(Master@+0xf8/Music@+0xfc/Music-DawnJingle/SFX@+0x100/Amb@+0x104) + 相机资源(+0xc0 3×3+0xf0 平移)算听者 + AudioCore::update；puffin scope | O | audio-misc\decomp\decomp-0x1409c8dd0.txt |
| country_core::systems::audio::audio_update::boot_audio_update_system | 0x1409c9320 | 37 | 同 VCA 段；听者用固定默认位姿（不读相机）；启动期系统 | O | audio-misc\decomp\decomp-0x1409c9320.txt |
