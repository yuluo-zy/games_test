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
| `country_core::resources::ui_settings::_::impl$0::previous` | 0x140b12f30 | 31 | **enum_iterator Sequence::previous**（依赖 trait 默认方法；ICF 折叠 Ghidra NotFound，RNE 手解：`0x2010004 >> (t<<3)` 打包表 {0x04,0x00,0x01,0x02}[t]，0x04=None niche → previous(Bottom)=None、其余=前一变体；与 QualityPreset::previous @0x140968f40 同族） | 胶水排除（候选U收尾闭合） | ui-input\decomp\decomp-0x140b12f30.txt（rne-disasm 手解） |
| `country_core::resources::ui_settings::ToolbarAlignment::is_horizontal` | 0x140b0e070 | 42 | ToolbarAlignment::is_horizontal | O（手解）：`test al;sete;cmp al,3;sete;or` = 变体 ∈ {0,3}——变体序 {Bottom=0,Left=1,Right=2,Top=3}，Bottom/Top 停靠 = 水平工具栏（重建代码的 Horizontal/Vertical/Bottom/Side 变体名错） | ui-input\decomp\decomp-0x140b12f30.txt |

## 实现方式（算法级）（O）

**UiSettings 7 字段（布局 O，serialize 字段序）**：
`{icon_size_mult f32@0x00, custom_cursor_size f32@0x04, toolbar_alignment u8@0x08,
auto_hide_toolbar u8@0x09, input_lag_reduction u8@0x0a, game_logo u8@0x0b, custom_cursor u8@0x0c}`。
默认值（wp21 O）：`{1.0, 24.0, 0x00000101, 0}` → alignment=1、auto_hide_toolbar=true。

**ToolbarAlignment 变体序**：串表 0x142acec66 = **Bottom/Left/Right/Top**
（重建代码 Horizontal/Vertical/Bottom/Side 错）；`is_horizontal @0x140b0e070`（O 手解）：
`test al; sete; cmp al,3; sete; or` = 变体 ∈ {0,3} → **Bottom=0/Top=3 = 水平停靠**，
Left=1/Right=2 = 垂直停靠。

**持久化**：`serialize @0x140a46ad0` pretty serde_json（7 字段串）；ron 变体
（0x1400ee5c0/0x1400ee430）；`visit_str @0x140a46c50`（serde_json）/
`@0x1400ef110`（ron）解析变体名。`previous @0x140b12f30` = enum_iterator
`Sequence::previous` 依赖方法（ICF 折叠，RNE 手解表 {0x04,0x00,0x01,0x02}[t]，
候选U收尾闭合；详见 round3/WALL-CANDIDATE-U-CLOSURE.md）。

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`default`、`deserialize`、`expecting`、`fmt`、`next`、`serialize`、`visit_enum`、`visit_map`、`visit_seq` 等 12 项
