# systems/audio/game_loading_audio.rs

> 文件作用：**加载界面音频：snapshot:/Loading 快照 + event:/UI/Loading。**

函数总计 3：业务 2 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::game_loading_audio::init_game_loading_audio | 0x140b1ae70 | 18 | 创建并立即 start snapshot:/Loading；状态装填 | O（round3 MCP 补证）：AudioCoreInner::instantiate_audio(0x140a54440,"snapshot:/Loading" 串@0x142ae94a8,len 0x11)+libfmod::EventInstance::start(0x1414372d0)+Vec 输出 {kind,ptr,cap,len} | round3 MCP 反编译 0x140b1ae70（tg_rev1） |
| country_core::systems::audio::game_loading_audio::loading_audio | 0x140b1b1b0 | 28 | 加载进行时播 event:/UI/Loading；完成→停快照(状态→2、清标志)+1s 定时器二次保护 | O | audio-misc\decomp\decomp-0x140b1b1b0.txt |

## 实现方式（算法级）（O，round3 + decomp-0x140b1b1b0）

**init_game_loading_audio @0x140b1ae70（round3 MCP）**：`AudioCoreInner::instantiate_audio`
（0x140a54440，`"snapshot:/Loading"`@0x142ae94a8，len 0x11）→ libfmod `start`
（0x1414372d0）→ 输出 Vec{kind,ptr,cap,len}。（O）
**loading_audio @0x140b1b1b0**：
- 加载进行中（load time 源 param_2 区）：若 AudioCore 非空 → instantiate_audio
  `event:/UI/Loading`（0x142ae9510，len 0x11）→ start（0x1414372d0）。I（内联路径结构）
- 完成分支：状态字节→2、清标志；并**1s 定时器二次保护**（时间累加@+0x34 与 1.0 常量
  0x142925904 比较）→ `stop(0)`（0x141437440）。O
- 失败链 unwrap panic `"called Result::unwrap() on an Err value"`（0x142ad0600 区，共用）。O

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 1 项
