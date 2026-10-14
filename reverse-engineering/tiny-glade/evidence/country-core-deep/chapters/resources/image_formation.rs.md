# resources/image_formation.rs

> 文件作用：**画面形成配置：三档 0x40B config+当前档 kind@+0xc0（缺省档=1）。**

函数总计 22：业务 7 / 内联无独立符号 1 / 派生胶水 14

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::resources::image_formation::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > | 0x1400eea00 | 11 | serde json 序列化派生（ImageFormation） | O（serde 派生） |  |
| country_core::resources::image_formation::_::impl$0::serialize<serde_json::value::ser::Serializer> | 0x14085faa0 | 11 | serde value 序列化派生 | O（serde 派生） |  |
| country_core::resources::image_formation::ImageFormationConfigs::initial | 0x14067dd80 | 90 | PDB 名错位：该址=AppSteamIntegration::client（mov rax,rcx）；ImageFormationConfigs::initial 为内联（default 折叠），真实体无独立符号 | O（错位） | audio-misc\decomp\decomp-0x14067dd80.txt, collision-eco\decomp\decomp-0x14067dd80.txt, terrain-water\decomp\decomp-0x14067dd80.txt |
| country_core::resources::image_formation::ImageFormationConfigs::current_mut | 0x1408aea50 | 99 | 0x1408aea50：self+(kind<<6) | O | audio-misc\decomp\decomp-0x1408aea50.txt |
| country_core::resources::image_formation::ImageFormationConfigs::iter_non_default_mut | 0x1408aea60 | 106 | 0x1408aea60：{begin,begin+0xc0,idx=0} 三档遍历 | O | audio-misc\decomp\decomp-0x1408aea60.txt |
| country_core::resources::image_formation::ImageFormationConfigs::get | 0x1408aea80 | 114 | 0x1408aea80：self+(i<<6)（无边界检查） | O | audio-misc\decomp\decomp-0x1408aea80.txt |
| country_core::resources::image_formation::ImageFormationConfigs::switch_kind | 0x1408aea90 | 118 | 0x1408aea90：[+0xc0]=kind | O | audio-misc\decomp\decomp-0x1408aea90.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::image_formation::ImageFormationConfigs::current`（行 94）— 查询/计算

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`closure`、`default`、`deserialize`、`eq`、`expecting`、`visit_map`、`visit_seq`、`visit_str` 等 14 项
