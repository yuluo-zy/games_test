# resources/toolbar.rs

> 文件作用：工具栏资源：Toolbar {categories: Vec<ToolbarCategoryModes> (cap,ptr,len), current u8@0x18}；ToolbarCategoryModes 0x20B {modes Vec, current u64@0x18}；ModeEntry 0x10B {cat u32, mode u8, pad, arg u64}。new 建 11 类（含 id0 类别；Roofs 为 Decor 0x38..0x3f 区间模式）；contains 的 装饰区间 = 0x38..=0x3f；switch_mode 入参 = (&mut Toolbar, &record 16B)；serde 按类别名 （'Path'…）序列化（input replay 持久化）。

函数总计 24：业务 7 / 内联无独立符号 4 / 派生胶水 13

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::toolbar::Toolbar::new` | 0x140af8c50 | 0 | 构造（Toolbar::new） | O(部分)：SOURCE_MAP 地址；此变体 alloc 0x30×8B 元素表（与 0x140af9000 的 11 类 0x10B 表不同——两地址为 new 的不同单态/内联，关系待核 I） |  |
| `country_core::resources::toolbar::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x1404f5100 | 7 | 序列化/serde | I：serde_json Compact serialize（对称 Pretty） |  |
| `country_core::resources::toolbar::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> > >` | 0x140afaed0 | 7 | 序列化/serde | I：serde_json Pretty serialize（类别名写出；Compact 对应 0x1404f5100） |  |
| `country_core::resources::toolbar::ToolbarCategoryModes::contains` | 0x140af89d0 | 17 | ToolbarCategoryModes::contains | O（手解）：入参 &{cat u32, mode u8@4}；SIMD 16B 块遍历 0x10 步长；cat<=5 时 mode 区间 0x38..=0x3f（mode-0x38<8） |  |
| `country_core::resources::toolbar::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>` | 0x140afb5b0 | 34 | 序列化/serde | O：visit_str 比较 'Path'（类别名序列化）；toolbar 持久化供 input replay | ui-input\decomp\decomp-0x140afb5b0.txt |
| `country_core::resources::toolbar::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> > >` | 0x140afb280 | 34 | 序列化/serde | I：ToolbarCategoryModes deserialize（serde_json，细节并入 visit_str） |  |
| `country_core::resources::toolbar::Toolbar::switch_mode` | 0x140af9850 | 265 | Toolbar::switch_mode | O（手解）：(&mut Toolbar, &{cat u32, mode u8, arg u64}) 逐类 contains 后写 current；panic 串含 'mode but no toolbar'（0x142ae3cbc 区，完整原文 U） |  |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::toolbar::ToolbarCategoryModes::next_mode`（行 21）— I：类别内下一模式（(current+1)%len 逻辑已并入 check_if_can_switch 闭包；PDB 名错位——0x140afb590 实为 serde unknown_variant thunk）
- `country_core::resources::toolbar::ToolbarCategoryModes::last_selected_mode`（行 27）— I：读 categories[i].current（内联进闭包/switch_mode）
- `country_core::resources::toolbar::impl$1::from`（行 69）— I：ToolbarCategory 转换构造（id→枚举，From 派生）
- `country_core::resources::toolbar::Toolbar::switch_category`（行 243）— I：按类别 id 找 Vec 下标写 current（内联进闭包 'byte@0x18=N' 写）

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`closure`、`deserialize`、`eq`、`expecting`、`visit_enum`、`visit_map`、`visit_seq`、`visit_str` 等 13 项
