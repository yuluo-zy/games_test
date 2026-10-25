# systems/audio/clutter_audio.rs

> 文件作用：**杂物摆放音效：PlayClutterSoundCmd kind(0..0x26=38)→event:/SFX/Cosmetics/* 瞬时播放。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::clutter_audio::clutter_audio_system | 0x1408c02f0 | 66 | kind 表 0x142aa1688(39 项)→instantiate_audio+set_position+set_parameter+start_if_not_playing（串表 0x142aa0e4c：BirdsNest/Box/DoorBell/BoxFish/RackBrooms…） | O | audio-misc\decomp\decomp-0x1408c02f0.txt |

## 实现方式（算法级）（O，multi-baseline §3f 跳表全解）

**clutter_audio_system @0x1408c02f0**：
- PlayClutterSoundCmd 事件槽 stride 0x18，kind u8@+0x14；`kind>0x26` 越界跳过。（O）
- 跳表 **0x142aa1688**（39 项，kind 0–0x26）：每 case 设 (name_ptr,len) → 公共块 0x1408c0990：
  `instantiate_audio(core+0x10, name, len)`（0x140a54440）→ set_position（事件位置@+0/+8，
  0x140a57da0）→ set_parameter → start_if_not_playing（0x140a57300）。（O）
- 事件名全部 `event:/SFX/Cosmetics/…`（串表 0x142aa0e4c）；抽查 kind→名：
  0x1d→DoorBell/Shutters/Firewood、0x1c→BoxFish/ClayPot/Pumpkin/BoxSack、0x1f→RackBrooms、
  0x20→Veggiestall/BoxWaterCan、0x2e→ClothlineConnectionAppear、0x1a→Anvil、0x1b→Bucket/
  Ladder/Basket、0x24→ClothlineAppear、0x26→Box；其余逐 kind 未钉死（模式一致）。（O）
- 全 FMOD 调用内联（无外部独立调用），每事件瞬时一次播。（O）
