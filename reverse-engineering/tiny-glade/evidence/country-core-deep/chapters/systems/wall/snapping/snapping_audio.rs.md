# systems/wall/snapping/snapping_audio.rs

> 文件作用：吸附音效系统：mode==2 且声音状态字节(+0x20)!=2，或 mode==3 且 <=1 到写 PlayWallUiSound。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::snapping::snapping_audio::snapping_audio` | 0x140a6a480 | 0 | snapping_audio（mode==2 且 +0x20 状态字节!=2 / mode==3 且 <=1 → PlayWallUiSound） | O | audio-misc\decomp\decomp-0x140a6a480.txt, wall\decomp\decomp-0x140a6a480.txt |
