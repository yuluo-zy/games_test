# systems/audio/building_audio.rs

> 文件作用：**建筑编辑音效：PlayBuildingAudio 事件（瞬时 detach/attach/peek）与每帧 7 个循环参数的 FMOD 驱动。**

函数总计 7：业务 3 / 内联无独立符号 1 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::building_audio::building_audio | 0x1417d2ad0 | 0 | 7×instantiate_loop：BuildingResize/RotateRectangle/RotateCircle/ChangeHeight/Move/PlasterChangeLoop/BuildingOverlapMove；参数 editing_speed | O | audio-misc\decomp\decomp-0x1417d2ad0.txt |
| country_core::systems::audio::building_audio::building_audio_init_system | 0x1417d21a0 | 34 | 事件 kind6/7/8→DetachFromGround/AttachToGround/PeakReached 瞬发+set_parameter(editing_speed,BuildingSize=inverse_lerp(area))；帧尾 7 kind(0,1,2,3,4,9,5)循环参数 ifWindow/BuildingHeight/isHalfTimbered/IsOnGround/loop | O | audio-misc\decomp\decomp-0x1417d21a0.txt |
| country_core::systems::audio::building_audio::get_building_size_01 | 0x1417d3f10 | 271 | 构建面积→[0,1]（BuildingSize 参数源） | I（内联，名推断） |  |

## 实现方式（算法级）（O，multi-baseline §4 + audio-extra §2 反汇编）

**building_audio_init_system @0x1417d21a0**：7×`instantiate_loop`（desc{count=3,max=50}）：
BuildingResize/RotateRectangle/RotateCircle/ChangeHeight/Move/PlasterChangeLoop/
BuildingOverlapMove → `insert_resource<BuildingAudioState>`。（O）

**building_audio @0x1417d2ad0（事件路径）**：
- PlayBuildingAudio 槽 stride 0x20，payload 从 @+0 起（无 bevy 书签）：id u64@+0（hash 键
  查 0x2a8 构建条目）、position@+8、editing_speed f32@+0x14、kind u8@+0x18。（O）
- kind 分派 `kind−6`：**只处理 kind 6/7/8**（DetachFromGround/AttachToGround/PeakReached，
  表 0x142ce4c88/0x142ce4ca0），其余 kind 事件被跳过。FMOD 序列 = instantiate_audio
  （0x140a54440）→ set_position（0x140a57da0）→ set_parameter(editing_speed / BuildingSize
  =inverse_lerp(area)，area 由条目[-0x48]形态 0/1 选圆/矩形）→ start_if_not_playing。（O）
- **帧尾默认路径**：对 kind {0,1,2,3,4,9,5} 每帧驱动各 loop（跳表 0x142ce4d30，
  BuildingResize/RotateRectangle/RotateCircle/ChangeHeight/Move/PlasterChangeLoop/
  BuildingOverlapMove），参数 editing_speed/BuildingSize/ifWindow/BuildingHeight/
  isHalfTimbered/IsOnGround/loop；kind 6/7/8 走 0x1417d386e panic/不支持。（O）
- `get_building_size_01 @0x1417d3f10`：面积→[0,1]（BuildingSize 源，I 内联）。

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::audio::building_audio::set_parameters`（行 214）— 设置/写入

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`eq`、`fmt` 等 3 项
