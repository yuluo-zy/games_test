# resources/walls/mod.rs

> 文件作用：墙资源族根模块：全局墙 ID（WallId u64 newtype）、Floor 楼板组件、WallHighlight 高亮资源、HighlightColor 高亮色枚举，以及模块级 split_height_into_skirts 墙裙边高度带计算（height>=5.9 起 4.5 每 +3.5）。

函数总计 20：业务 10 / 内联无独立符号 0 / 派生胶水 10

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::walls::HighlightColor::to_rgb` | 0x140b34f30 | 0 | HighlightColor::to_rgb（判别==1 → (0.1,0.03,0) 否则 (0.07,0.08,0.07)） | O | wall\decomp\decomp-0x140b34f30.txt |
| `country_core::resources::walls::split_height_into_skirts` | 0x140b34f80 | 0 | split_height_into_skirts（>=5.9 起 4.5 每 +3.5，条件 height>f+1.4） | O | wall\decomp\decomp-0x140b34f80.txt |
| `country_core::resources::time_of_day::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> > >` | 0x140080160 | 69 | 序列化 | I | wall\decomp\decomp-0x140080160.txt |
| `country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct<serde::__private::de::content::ContentRefDeserializer<serde_json::error::Error> >` | 0x141286860 | 69 | 序列化 | U |  |
| `country_core::resources::walls::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x140080160 | 69 | 序列化 | I | wall\decomp\decomp-0x140080160.txt |
| `country_core::resources::walls::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> > >` | 0x140b35980 | 69 | 序列化 | U |  |
| `country_core::systems::wall::shapes::circle::signifiers::edit_arrow::impl$0::clone_behavior` | 0x140002370 | 82 | edit_arrow clone_behavior（xor eax,eax 返回 0） | O | audio-misc\decomp\decomp-0x140002370.txt, camera-time\decomp\decomp-0x140002370.txt, collision-eco\decomp\decomp-0x140002370.txt |
| `country_core::resources::walls::WallId::get_inner` | 0x1400885d0 | 88 | WallId::get_inner（mov rax,[rcx] 返回内值 u64） | O | audio-misc\decomp\decomp-0x1400885d0.txt, wall\decomp\decomp-0x1400885d0.txt |
| `country_core::resources::walls::WallId::is_valid` | 0x140b34f20 | 94 | WallId::is_valid（u64!=0；PDB 索引名 get_inner 错位，真 get_inner 在 0x140885d0） | O | wall\decomp\decomp-0x140b34f20.txt |
| `country_core::resources::walls::impl$17::register_required_components` | 0x140b35ae0 | 147 | 构造/注册/生成 | U |  |

## 派生胶水（derive/宏生成；不计独立业务）

`default`、`deserialize`、`eq`、`fmt`、`hash`、`partial_cmp`、`serialize`、`visit_newtype_struct` 等 10 项
