# startup/load_glade_audio.rs

> 文件作用：**glade 音频载入：按 glade theme 载入对应 FMOD 音乐事件与音效。**

函数总计 11：业务 2 / 内联无独立符号 2 / 派生胶水 7

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::load_glade_audio::load_glade_audio` | 0x1408be360 | 11 | `load_glade_audio`：读 gladestate 配置（theme byte@+0x58 ×8 索引），载入 GladeAudio；构造 4 个音频对象，事件路径 `event:/Music/Halloween/{Single Leaf,Tiny Olden Glade,Forgotten Ruins,Enchanted Forest}`（对应 4 theme）。 | **O** | decomp-0x1408be360.txt |
| `country_core::startup::load_glade_audio::GladeAudio::load` | 0x1408bf460 | 125 | `GladeAudio::load`：载入单个 GladeAudio（音频事件配置）。 | **O** | decomp-0x1408bf460.txt |

## 内联/无独立符号函数

- `country_core::startup::load_glade_audio::stop_and_play_new`（行 81）— **I**：`stop_and_play_new` — 停止旧音频并播放新（内联）。
- `country_core::startup::load_glade_audio::GladeAudio::try_load`（行 119）— **I**：`GladeAudio::try_load` — 尝试载入（内联）。

## 派生胶水

- 7 项：default, deserialize, expecting, visit_map, visit_seq, visit_str 等（derive 直接复用，不计业务）。

## 实现方式（算法级，2026-10-15 深挖）

- **load_glade_audio 0x1408be360（O）**：theme byte @ gladestate+0x58 x8 索引 →
  GladeAudio::load（0x1408bf460，4 主题各一次）→ 每主题：
  'Reading {path}/audio.json' 日志（'audio.json' 0x142aa0d44）→ 4 事件
  `event:/Music/Halloween/{Single Leaf, Tiny Olden Glade, Forgotten Ruins, Enchanted Forest}`
  （串 0x142aa0c33 区）→ FUN_140a54440 事件句柄 x2 + FUN_14089a280（容器注册）
  → FMOD 参数 `GladeTypeIsHalloween`(9, 0x142aa0ce8) / `IsHalloween`(0xb, 0x142aa0cf1)
  （FUN_140a53db0 设置，行 371/386）→ stop_and_play_new（内联，行 81）。
  5x unwrap panic 行 43（0x142ad0600/0x142ad05c0/0x142ad2bc0/0x142ad2c20）。
- **GladeAudio::load 0x1408bf460（O）**：单主题音频事件配置载入（见上）。
- 契约缺口：reconstruction GladeAudio 无 4 主题音乐事件表（HIGHLIGHTS 7 既有结论保持）。
