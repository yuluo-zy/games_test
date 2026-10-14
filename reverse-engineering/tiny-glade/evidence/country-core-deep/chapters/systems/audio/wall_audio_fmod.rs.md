# systems/audio/wall_audio_fmod.rs

> 文件作用：**墙体音频主系统：10 循环 + kind 0-23 分派 + 帧尾 start/stop。**

函数总计 2：业务 2 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::wall_audio_fmod::wall_audio_fmod | 0x140950920 | 0 | arg1=AppMode(==13 音量 1.0) arg2=AudioCore arg3=EventReader<PlayWallAudio> arg4=ResMut<WallAudioState>+栈参音量(+0x1c/60.0)；kind 表 0x142ab4998(0..23)：循环类 request_play+set_parameter(WallHeight/FenceLength/HasBricks)，瞬发类 instantiate_audio_w_position_and_volume(FenceAppear/Merge/Rearrange/Lintel*/StoneArch*/Pillar*/Gate*/PlatformStairs*/WallSplit/SewerGate*)，帧尾 10 loop 应播 start/stop+[+0x280] 音量×powf(0.1,arg+0x58×5.0) | O | audio-misc\decomp\decomp-0x140950920.txt |
| country_core::systems::audio::wall_audio_fmod::wall_audio_init_system | 0x14094fb80 | 40 | 10×instantiate_loop(desc{name,count=3,max=50})：Wall Draw/Wall Change Height/FenceExtend/LintelExtend/StoneArchExtend/WideningGate/PlatformStairsExtend/SewerGateWiden/FenceHeight/FenceMove；WallAudioState=固定 10×stride0x40 数组+参数 Vec@+0x48/0x88/0xc8/0x148+音量@+0x280 | O | audio-misc\decomp\decomp-0x14094fb80.txt |
