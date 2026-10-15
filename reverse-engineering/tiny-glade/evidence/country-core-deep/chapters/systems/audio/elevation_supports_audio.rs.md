# systems/audio/elevation_supports_audio.rs

> 文件作用：**抬升支撑（木架/柱）出现音效。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::elevation_supports_audio::elevation_supports_audio | 0x140ad51c0 | 27 | kind→3 串 WoodenBracketsAppear/BrickColumnAppear/WoodenColumnAppear→instantiate_audio+start | O | audio-misc\decomp\decomp-0x140ad51c0.txt |

## 实现方式（算法级）（O，DIGEST §2.13 串级补全）

**elevation_supports_audio @0x140ad51c0**：PlayElevationSupportSoundCmd 的 kind 字节 →
3 个瞬发事件名三选一（O 串）：`WoodenBracketsAppear`/`BrickColumnAppear`/
`WoodenColumnAppear`（WP14 合同只列 1 个，本批补 2 个）→ `AudioCoreInner::instantiate_audio`
（0x140a54440）→ `start`。（O）
- 无循环 loop；纯 kind→事件瞬时播放。（O）
