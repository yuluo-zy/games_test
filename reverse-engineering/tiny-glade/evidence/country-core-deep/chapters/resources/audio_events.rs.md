# resources/audio_events.rs

> 文件作用：**音频事件类型定义（16 个 bevy Event，mod.rs add_audio_events 依序注册）。事件槽布局由各消费系统反汇编钉死（wall stride 0x28/building 0x20/stairs 0x38）。**

函数总计 16：业务 16 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `PlayTerrainSound` | (数据型) | 1 | 地形编辑音效事件（register_event #1） | O | audio-events-analysis §1a |
| `SuppressPathSound` | (数据型) | 2 | 抑制路径音效事件（#2，terrain_stairs 消费） | O | 同上 |
| `PlayReplayAudio` | (数据型) | 3 | 撤销/重做音效事件（#3） | O | 同上 |
| `PlayDecoratorSoundCmd` | (数据型) | 4 | 装饰物音效命令（#4） | O | 同上 |
| `PlayStairsAudioCmd` | (数据型) | 5 | 楼梯音效命令（#5；槽 stride 0x38：kind@+8、f32×4@+c..0x18、pos u64/u32@+0x1c/+0x24、f32×2@+0x28/+0x2c） | O | audio-events-analysis §8 |
| `PlayWallUiSound` | (数据型) | 6 | 墙体 UI 音效（#6） | O | 同上 §1a |
| `PlayWallAudio` | (数据型) | 7 | 墙体音效（#7；槽 stride 0x28：kind@+8、f32@+c、pos@+0x10、音量 f32@+0x1c、byte@+0x20） | O | 同上 §8 |
| `RequestWallMaterialAudio` | (数据型) | 8 | 墙体材质音频协调请求（#8） | O | 同上 |
| `WallBendAudioParamReset` | (数据型) | 9 | 墙体弯曲音频参数复位（#9） | O | 同上 |
| `PlayTimberframeSound` | (数据型) | 10 | 木骨架音效（#10） | O | 同上 |
| `PlaySheepReactSound` | (数据型) | 11 | 绵羊反应音效（#11） | O | 同上 |
| `PlayMenuUiSound` | (数据型) | 12 | 菜单 UI 音效（#12） | O | 同上 |
| `PlayBuildingAudio` | (数据型) | 13 | 建筑音效（#13；槽 stride 0x20：id@+0、pos@+8、editing_speed@+0x14、kind@+0x18；payload 无 bevy 书签） | O | 同上 §8 |
| `PlayElevationSupportSoundCmd` | (数据型) | 14 | 抬升支撑音效命令（#14） | O | 同上 |
| `PlayClutterSoundCmd` | (数据型) | 15 | 杂物音效命令（#15；槽 stride 0x18） | O | 同上 §3f |
| `PlayTreeSoundCmd` | (数据型) | 16 | 树木音效命令（#16，tree-tool 写入） | O | 同上 |
