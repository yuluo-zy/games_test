# resources/ui_settings.rs

> 文件作用：UI 设置：UiSettings 7 字段 {icon_size_mult f32@0, custom_cursor_size f32@4, toolbar_alignment u8@8, auto_hide_toolbar u8@9, input_lag_reduction u8@10, game_logo u8@11, custom_cursor u8@12}；ToolbarAlignment = Bottom/Left/Right/Top（串表 0x142acec66）；默认 {1.0, 24.0, 0x101, 0}（auto_hide=true）。pretty serde_json 持久化。

函数总计 20：业务 8 / 内联无独立符号 0 / 派生胶水 12

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::ui_settings::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >` | 0x1400ef110 | 5 | 序列化/serde | O：ron visit_str（ToolbarAlignment 变体名 Bottom/Left/Right/Top 解析） |  |
| `country_core::resources::ui_settings::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>` | 0x140a46c50 | 5 | 序列化/serde | O：serde_json visit_str（同变体名） |  |
| `country_core::resources::ui_settings::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > >` | 0x1400ee5c0 | 5 | 序列化/serde | O：ron Pretty serialize（UiSettings 7 字段，串 @0x142acec66） |  |
| `country_core::resources::ui_settings::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x1400ee430 | 5 | 序列化/serde | O：serde_json Compact serialize |  |
| `country_core::resources::ui_settings::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> > >` | 0x140a46ad0 | 5 | 序列化/serde | O：pretty serde_json 字段序 = 布局表（7 字段串 @0x142acec66 起） | ui-input\decomp\decomp-0x140a46ad0.txt |
| `country_core::resources::ui_settings::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> > >` | 0x140a46dd0 | 31 | 序列化/serde | O：serde_json deserialize（UiSettings） |  |
| `country_core::resources::ui_settings::_::impl$0::previous` | 0x140b12f30 | 31 | previous | U：NotFound（折叠）；语义待证 | ui-input\decomp\decomp-0x140b12f30.txt |
| `country_core::resources::ui_settings::ToolbarAlignment::is_horizontal` | 0x140b0e070 | 42 | ToolbarAlignment::is_horizontal | O（手解）：`test al;sete;cmp al,3;sete;or` = 变体 ∈ {0,3}——变体序 {Bottom=0,Left=1,Right=2,Top=3}，Bottom/Top 停靠 = 水平工具栏（重建代码的 Horizontal/Vertical/Bottom/Side 变体名错） | ui-input\decomp\decomp-0x140b12f30.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`default`、`deserialize`、`expecting`、`fmt`、`next`、`serialize`、`visit_enum`、`visit_map`、`visit_seq` 等 12 项
