# systems/audio/decorator_audio.rs

> 文件作用：**装饰物音效：16 个循环 loop（窗/灯/烟囱/旗帜）+ 按距离衰减的瞬时播放。**

函数总计 5：业务 2 / 内联无独立符号 0 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::decorator_audio::decorator_audio_system | 0x14145dc20 | 0 | 事件→(f32,Vec3) 稳定排序距离衰减→循环/瞬发分派 | O | audio-misc\decomp\decomp-0x14145dc20.txt |
| country_core::systems::audio::decorator_audio::decorator_audio_init_system | 0x14145c6b0 | 51 | 16×instantiate_loop：WindowDrag/Place/PlacePreview/Remove/Combine/Hover、DecoratorMoveWoodenFloorTEMP、Lantern/MoveWall/MoveGround/Rotate/Amb、ChimneyChangeHeight/Drag、Flags/Rotate/Drag/DragGround | O | audio-misc\decomp\decomp-0x14145c6b0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`closure` 等 3 项
