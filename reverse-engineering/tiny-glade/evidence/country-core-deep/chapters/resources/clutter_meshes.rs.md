# resources/clutter_meshes.rs

> 文件作用：**杂饰网格类型表：ClutterMeshTy 56 资产路径变体+删除标记组件**

函数总计 21：业务 11 / 内联无独立符号 0 / 派生胶水 10

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::clutter_meshes::_::impl$0::first` | 0x140002370 | 16 | 折叠 stub | O：ICF 折叠（Duck::being_petted 等多符号共享） | audio-misc\decomp\decomp-0x140002370.txt, camera-time\decomp\decomp-0x140002370.txt, collision-eco\decomp\decomp-0x140002370.txt |
| `country_core::resources::clutter_meshes::impl$1::clone_behavior` | 0x140002370 | 16 | 折叠 stub | O：ICF 折叠（Duck::being_petted 等多符号共享） | audio-misc\decomp\decomp-0x140002370.txt, camera-time\decomp\decomp-0x140002370.txt, collision-eco\decomp\decomp-0x140002370.txt |
| `country_core::resources::clutter_meshes::impl$1::register_required_components` | 0x1408b9170 | 16 | Bevy 注册胶水 | O：register_required_components（Bevy 组件注册） | collision-eco\decomp\decomp-0x1408b9170.txt |
| `country_core::resources::clutter_meshes::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x1404f39b0 | 133 | 派生胶水 | O：serde derive（复用依赖，不计业务） |  |
| `country_core::resources::clutter_meshes::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> > >` | 0x14179a940 | 133 | 派生胶水 | O：serde derive（复用依赖，不计业务） |  |
| `country_core::resources::clutter_meshes::_::impl$0::serialize<serde_json::value::ser::Serializer>` | 0x14179a560 | 133 | 派生胶水 | O：serde derive（复用依赖，不计业务） | collision-eco\decomp\decomp-0x14179a560.txt |
| `country_core::resources::clutter_meshes::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>` | 0x14179b870 | 134 | 派生胶水 | O：serde derive（复用依赖，不计业务） |  |
| `country_core::resources::clutter_meshes::_::impl$0::deserialize<enum2$<serde_json::value::Value> >` | 0x14179ad30 | 134 | 派生胶水 | O：serde derive（复用依赖，不计业务） |  |
| `country_core::resources::clutter_meshes::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> > >` | 0x14179b3e0 | 134 | 派生胶水 | O：serde derive（复用依赖，不计业务） | collision-eco\decomp\decomp-0x14179b3e0.txt |
| `country_core::resources::clutter_meshes::ClutterMeshTy::from` | 0x1408b83d0 | 199 | 转换 | O：ClutterMeshTy::from(&str)（68KB）——56 变体按资产路径匹配（shovel/bucket/fallen_leaves/bench/pumpkin2/anvil/door_bell…），未识别 panic | collision-eco\decomp\decomp-0x1408b83d0.txt |
| `country_core::resources::clutter_meshes::clutter_mesh_ty_data` | 0x1408b8e90 | 260 | 查询/计算 | O：clutter_mesh_ty_data——静态表 switch(0..0x37)→8B 条目（名+kind） | collision-eco\decomp\decomp-0x1408b8e90.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`closure`、`default`、`deserialize`、`expecting`、`fmt`、`next`、`visit_enum` 等 10 项
