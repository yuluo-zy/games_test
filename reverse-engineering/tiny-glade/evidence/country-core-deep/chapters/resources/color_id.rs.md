# resources/color_id.rs

> 文件作用：**颜色索引资源族：ColorId(u32)+墙/门/半木三表+SSBO 载体。**

函数总计 19：业务 9 / 内联无独立符号 3 / 派生胶水 7

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::resources::color_id::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > | 0x1400e7480 | 26 | serde json 序列化派生（ColorId/terrain_editing 共享实例） | O（serde 派生） |  |
| country_core::resources::color_id::impl$12::clone_behavior | 0x140b1d670 | 26 | ColorId 组件 clone_behavior 派生 | O（派生） |  |
| country_core::resources::color_id::impl$12::register_required_components | 0x140b1cf80 | 26 | bevy 组件注册（0x140b1cf80） | O（胶水） | audio-misc\decomp\decomp-0x140b1cf80.txt |
| country_core::resources::terrain_editing::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > | 0x1400e7480 | 26 | serde json 序列化派生（ColorId/terrain_editing 共享实例） | O（serde 派生） |  |
| country_core::resources::terrain_editing::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> > > | 0x1408e5bc0 | 26 | serde json 序列化派生（terrain_editing Pretty） | O（serde 派生） | terrain-water\decomp\decomp-0x1408e5bc0.txt |
| country_core::resources::color_id::WallColorIds::new | 0x140b19890 | 79 | 建空表（0x140b19890） | O |  |
| country_core::resources::color_id::impl$3::default_value | 0x14099ca50 | 127 | 注意：0x14099ca50 属 color_id_roof 且返回 1（非本文件 ColorId(0)） | U（归属纠错） | audio-misc\decomp\decomp-0x14099ca50.txt |
| country_core::resources::color_id::GateColorIds::new | 0x140b19900 | 147 | 建空表（0x140b19900） | O |  |
| country_core::resources::color_id::HalftimberColorIds::new | 0x140b19970 | 216 | 建空表（0x140b19970） | O |  |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::color_id::WallColorIds::iter_nondefault`（行 85）— 迭代/变换
- `country_core::resources::color_id::GateColorIds::iter_nondefault`（行 153）— 迭代/变换
- `country_core::resources::color_id::HalftimberColorIds::iter_nondefault`（行 222）— 迭代/变换

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`deserialize`、`eq`、`fmt`、`serialize`、`visit_newtype_struct` 等 7 项
