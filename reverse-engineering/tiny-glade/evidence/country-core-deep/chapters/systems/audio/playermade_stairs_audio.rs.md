# systems/audio/playermade_stairs_audio.rs

> 文件作用：**玩家自制楼梯音效（6 循环 + kind≤18 分派）。**

函数总计 2：业务 2 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::playermade_stairs_audio::stairs_audio_system | 0x140f880a0 | 0 | 帧首 6 loop stop；kind 表 0x142bad9f0(≤18)：瞬发 StairPlace/StairDelete/StairsMerge/StairsMergeConfirm/LadderAppear/LadderPlace/StepsAppear/WalkwayAppear/StairSupportAppear/StairPillarAppear + 参数表 stair_width/size/editing_speed/extend_amount/h_offset/avg_arch_supports/height_above_ground/in_water/stair_edge_length/is_stub_preview/move_all | O | audio-misc\decomp\decomp-0x140f880a0.txt |
| country_core::systems::audio::playermade_stairs_audio::stairs_audio_init_system | 0x140f878d0 | 28 | 6×instantiate_loop：StairMove/StairChangeHeight/StairChangeWidth/LadderMove/LadderChangeHeight/LadderChangeWidth | O | audio-misc\decomp\decomp-0x140f878d0.txt |

## 实现方式（算法级）（O，multi-baseline §5 + audio-extra §3a 全线性反汇编）

**stairs_audio_init_system @0x140f878d0**：6×`instantiate_loop`（desc{count=3,max=50}）：
StairMove/StairChangeHeight/StairChangeWidth/LadderMove/LadderChangeHeight/LadderChangeWidth
→ `insert_resource<StairsAudioState>`（6 loop stride 0x38）。（O）
**stairs_audio_system @0x140f880a0**：
- **帧首**：6 个 loop 全部 `stop`（0x140a59430）；PlayStairsAudioCmd 槽 stride 0x38，
  kind@+0x8；`kind>0x12` 跳过。（O）
- 跳表 **0x142bad9f0**（kind 0–18）：循环类（0/1/2/7/9/10）`start_if_not_playing(loop)` +
  set_parameter(stair_width)；瞬发类（3/4/5/6/8/11/12/13/14/15/17/18）→ 公共块 0x140f883d0：
  instantiate_audio → set_position（事件+0x1c/+0x24）→ set_parameter("stair_width") →
  start_if_not_playing → 二次参数分派（跳表 0x142bada3c kind≤0xa / 0x142bada68 kind≤9）：
  size/extend_amount/h_offset/avg_arch_supports/height_above_ground/in_water/
  stair_edge_length/is_stub_preview/move_all。（O）
- 瞬发事件名表（0x142bad6c8 起）：StairPlace/StairDelete/StairsMerge/StairsMergeConfirm/
  LadderAppear/LadderPlace/StepsAppear/WalkwayAppear/StairSupportAppear/StairPillarAppear。（O）
- kind 16=忽略；17/18 复用 StairSupportAppear（同 14）。（O）
