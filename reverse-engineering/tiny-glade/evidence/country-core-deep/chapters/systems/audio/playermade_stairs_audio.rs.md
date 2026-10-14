# systems/audio/playermade_stairs_audio.rs

> 文件作用：**玩家自制楼梯音效（6 循环 + kind≤18 分派）。**

函数总计 2：业务 2 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::playermade_stairs_audio::stairs_audio_system | 0x140f880a0 | 0 | 帧首 6 loop stop；kind 表 0x142bad9f0(≤18)：瞬发 StairPlace/StairDelete/StairsMerge/StairsMergeConfirm/LadderAppear/LadderPlace/StepsAppear/WalkwayAppear/StairSupportAppear/StairPillarAppear + 参数表 stair_width/size/editing_speed/extend_amount/h_offset/avg_arch_supports/height_above_ground/in_water/stair_edge_length/is_stub_preview/move_all | O | audio-misc\decomp\decomp-0x140f880a0.txt |
| country_core::systems::audio::playermade_stairs_audio::stairs_audio_init_system | 0x140f878d0 | 28 | 6×instantiate_loop：StairMove/StairChangeHeight/StairChangeWidth/LadderMove/LadderChangeHeight/LadderChangeWidth | O | audio-misc\decomp\decomp-0x140f878d0.txt |
