# systems/audio/decorator_audio.rs

> 文件作用：**装饰物音效：16 个循环 loop（窗/灯/烟囱/旗帜）+ 按距离衰减的瞬时播放。**

函数总计 5：业务 2 / 内联无独立符号 0 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::decorator_audio::decorator_audio_system | 0x14145dc20 | 0 | 事件→(f32,Vec3) 稳定排序距离衰减→循环/瞬发分派 | O | audio-misc\decomp\decomp-0x14145dc20.txt |
| country_core::systems::audio::decorator_audio::decorator_audio_init_system | 0x14145c6b0 | 51 | 16×instantiate_loop：WindowDrag/Place/PlacePreview/Remove/Combine/Hover、DecoratorMoveWoodenFloorTEMP、Lantern/MoveWall/MoveGround/Rotate/Amb、ChimneyChangeHeight/Drag、Flags/Rotate/Drag/DragGround | O | audio-misc\decomp\decomp-0x14145c6b0.txt |

## 实现方式（算法级）（O，decomp-0x14145dc20/0x14145c6b0）

**decorator_audio_init_system @0x14145c6b0**：16×`instantiate_loop`（WindowDrag/Place/
PlacePreview/Remove/Combine/Hover、DecoratorMoveWoodenFloorTEMP、Lantern/MoveWall/MoveGround/
Rotate/Amb、ChimneyChangeHeight/Drag、Flags/Rotate/Drag/DragGround）。（O）

**decorator_audio_system @0x14145dc20**：
- 事件→(距离 f32,位置 Vec3) 稳定排序后按距离衰减分派；对每个 loop 播放器按距离/应播标志
  做 `stop`（0x140a59430）/`start_if_not_playing`（0x140a596b0）成对切换（decomp 见多组
  `FUN_140a59430(param)`/`FUN_140a596b0(param)` 与常量 0x142a7fa30/0x142a7b658/0x142925904）。I（结构）

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`closure` 等 3 项
