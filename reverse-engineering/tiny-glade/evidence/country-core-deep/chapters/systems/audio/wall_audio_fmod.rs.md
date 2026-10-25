# systems/audio/wall_audio_fmod.rs

> 文件作用：**墙体音频主系统：10 循环 + kind 0-23 分派 + 帧尾 start/stop。**

函数总计 2：业务 2 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::wall_audio_fmod::wall_audio_fmod | 0x140950920 | 0 | arg1=AppMode(==13 音量 1.0) arg2=AudioCore arg3=EventReader<PlayWallAudio> arg4=ResMut<WallAudioState>+栈参音量(+0x1c/60.0)；kind 表 0x142ab4998(0..23)：循环类 request_play+set_parameter(WallHeight/FenceLength/HasBricks)，瞬发类 instantiate_audio_w_position_and_volume(FenceAppear/Merge/Rearrange/Lintel*/StoneArch*/Pillar*/Gate*/PlatformStairs*/WallSplit/SewerGate*)，帧尾 10 loop 应播 start/stop+[+0x280] 音量×powf(0.1,arg+0x58×5.0) | O | audio-misc\decomp\decomp-0x140950920.txt |
| country_core::systems::audio::wall_audio_fmod::wall_audio_init_system | 0x14094fb80 | 40 | 10×instantiate_loop(desc{name,count=3,max=50})：Wall Draw/Wall Change Height/FenceExtend/LintelExtend/StoneArchExtend/WideningGate/PlatformStairsExtend/SewerGateWiden/FenceHeight/FenceMove；WallAudioState=固定 10×stride0x40 数组+参数 Vec@+0x48/0x88/0xc8/0x148+音量@+0x280 | O | audio-misc\decomp\decomp-0x14094fb80.txt |

## 实现方式（算法级）（O，multi-baseline §3 + audio-extra §1 全线性反汇编）

**wall_audio_init_system @0x14094fb80**：10×`instantiate_loop`（desc{count=3,max=50}）按序：
Wall Draw/Wall Change Height/FenceExtend/LintelExtend/StoneArchExtend/WideningGate/
PlatformStairsExtend/SewerGateWiden/FenceHeight/FenceMove；结果装入固定
WallAudioState=10×AudioLoopPlayer（stride 0x40：+0x0/+0x40/+0x80/+0xc0/+0x100/+0x140/
+0x180/+0x1c0/+0x200/+0x240），参数 Vec@+0x48/+0x88/+0xc8/+0x148，音量@+0x280。（O）
**wall_audio_fmod @0x140950920（8 参系统）**：
- arg1=AppMode（==0xd 时音量常量 1.0）、arg2=AudioCore、arg3=EventReader<PlayWallAudio>、
  arg4=ResMut<WallAudioState>；音量源=栈参 +0x1c f32 `/60.0`（0x1429d18cc）归一化。（O）
- PlayWallAudio 槽 stride 0x28，kind@+8；跳表 **0x142ab4998**（kind 0–23，24 项）。（O）
- **循环类（kind 0/1/2/6/7/10/13/18/20/23）** `request_play`（0x1409285a0）+
  参数：1 `WallHeight`=inverse_lerp(0.5,14,事件+0xc)；2/7 `FenceLength`=inverse_lerp(8,50)/
  inverse_lerp(3,15)；10 `HasBricks`=事件+0x9非零?1:0。（O）
- **瞬发类（kind 3/4/5/8/9/11/12/14/15/16/17/19/21/22）**
  `instantiate_audio_w_position_and_volume`（0x140aa3590）：FenceAppear/Merge/Rearrange、
  LintelAppear/LintelRearrange/LintelMerge、StoneArchAppear、PillarStone/WoodAppear、
  GateAppear、PlatformStairsAppear、WallSplit、SewerGateAppear；kind 9/16 同名 LintelRearrange。（O）
- **帧尾**：10 loop 按应播标志 start_if_not_playing/stop（0x140a596b0/0x140a59430）并清标志；
  `[state+0x280]` 音量 `×= powf(0.1, 栈参资源+0x58 × 5.0)`。（O）
