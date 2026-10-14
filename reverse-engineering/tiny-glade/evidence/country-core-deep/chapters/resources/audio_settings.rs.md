# resources/audio_settings.rs

> 文件作用：**音频音量设置（GameConfig 内嵌，RON serde；4 个 f32 音量字段 O）。**

函数总计 7：业务 1 / 内联无独立符号 0 / 派生胶水 6

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::resources::audio_settings::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > > | 0x140168850 | 4 | serde ron 序列化派生（AudioSettings） | O（serde 派生） |  |

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`default`、`deserialize`、`visit_map`、`visit_str` 等 6 项
