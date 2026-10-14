# systems/audio/building_audio.rs

> 文件作用：**建筑编辑音效：PlayBuildingAudio 事件（瞬时 detach/attach/peek）与每帧 7 个循环参数的 FMOD 驱动。**

函数总计 7：业务 3 / 内联无独立符号 1 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::building_audio::building_audio | 0x1417d2ad0 | 0 | 7×instantiate_loop：BuildingResize/RotateRectangle/RotateCircle/ChangeHeight/Move/PlasterChangeLoop/BuildingOverlapMove；参数 editing_speed | O | audio-misc\decomp\decomp-0x1417d2ad0.txt |
| country_core::systems::audio::building_audio::building_audio_init_system | 0x1417d21a0 | 34 | 事件 kind6/7/8→DetachFromGround/AttachToGround/PeakReached 瞬发+set_parameter(editing_speed,BuildingSize=inverse_lerp(area))；帧尾 7 kind(0,1,2,3,4,9,5)循环参数 ifWindow/BuildingHeight/isHalfTimbered/IsOnGround/loop | O | audio-misc\decomp\decomp-0x1417d21a0.txt |
| country_core::systems::audio::building_audio::get_building_size_01 | 0x1417d3f10 | 271 | 构建面积→[0,1]（BuildingSize 参数源） | I（内联，名推断） |  |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::audio::building_audio::set_parameters`（行 214）— 设置/写入

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`eq`、`fmt` 等 3 项
