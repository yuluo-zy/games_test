# systems/audio/sheep_audio.rs

> 文件作用：**绵羊反应音效（kind 0-7 + 参数 FirstPersonOff）。**

函数总计 2：业务 1 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::sheep_audio::sheep_audio_system | 0x1409d5ea0 | 49 | kind 表 0x142ac2cc8(0..7)：Aww/Think/So/Angry/Idea/Eyes/Confused→instantiate+set_parameter+start；kind4=帧尾 start/stop 分支；参数 FirstPersonOff | O | audio-misc\decomp\decomp-0x1409d5ea0.txt |

## 实现方式（算法级）（O，multi-baseline §3e 反汇编）

**sheep_audio_system @0x1409d5ea0**：
- PlaySheepReactSound 槽 stride 0x20，kind@+0x18；跳表 **0x142ac2cc8**（kind 0–7，8+ 越界）；
  每 case 设 (name_ptr,len) → 公共块 0x1409d6060：instantiate_audio（0x140a54440）→
  set_parameter → start_if_not_playing（0x140a57300）。（O）
- 事件名（串表 0x142ac23b0）：0 `event:/SFX/Sheep/Aww`、1 `Think`、2 `So`、3 `Angry`、
  5 `Idea`、6 `Eyes`、7 `Confused`；**kind4 → 0x1409d61b0（帧尾 start/stop 分支，无瞬发）**。（O）
- 帧尾按应播标志 `start_if_not_playing`（0x140a596b0）/`stop`（0x140a59430）；
  参数含 `FirstPersonOff`。（O）
- init（行 26）无独立符号（sheep_audio_init_system 内联）。I

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::audio::sheep_audio::sheep_audio_init_system`（行 26）— 待推断
