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
