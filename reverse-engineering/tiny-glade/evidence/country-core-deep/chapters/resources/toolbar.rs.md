# resources/toolbar.rs

> 文件作用：工具栏资源：Toolbar {categories: Vec<ToolbarCategoryModes> (cap,ptr,len), current u8@0x18}；ToolbarCategoryModes 0x20B {modes Vec, current u64@0x18}；ModeEntry 0x10B {cat u32, mode u8, pad, arg u64}。new 建 11 类（含 id0 类别；Roofs 为 Decor 0x38..0x3f 区间模式）；contains 的 装饰区间 = 0x38..=0x3f；switch_mode 入参 = (&mut Toolbar, &record 16B)；serde 按类别名 （'Path'…）序列化（input replay 持久化）。

函数总计 24：业务 7 / 内联无独立符号 4 / 派生胶水 13

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::toolbar::Toolbar::new` | 0x140af8c50 | 0 | 构造（Toolbar::new） | O(部分，round3 round2 复查修正)：PDB/S_PUB32 身份成立（Toolbar::new）；但行为断言未核——0x140af9000 实为该函数体内 +0x1b0（非独立函数），“不同单态/内联”说法可疑；alloc 0x30×8B 元素表待反汇编核验，未核前不升完整 O | PDB/S_PUB32 0x140af8c50 |
| `country_core::resources::toolbar::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x1404f5100 | 7 | 序列化/serde | I：serde_json Compact serialize（对称 Pretty） |  |
| `country_core::resources::toolbar::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> > >` | 0x140afaed0 | 7 | 序列化/serde | I：serde_json Pretty serialize（类别名写出；Compact 对应 0x1404f5100） |  |
| `country_core::resources::toolbar::ToolbarCategoryModes::contains` | 0x140af89d0 | 17 | ToolbarCategoryModes::contains | O（手解）：入参 &{cat u32, mode u8@4}；SIMD 16B 块遍历 0x10 步长；cat<=5 时 mode 区间 0x38..=0x3f（mode-0x38<8） |  |
| `country_core::resources::toolbar::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>` | 0x140afb5b0 | 34 | 序列化/serde | O：visit_str 比较 'Path'（类别名序列化）；toolbar 持久化供 input replay | ui-input\decomp\decomp-0x140afb5b0.txt |
| `country_core::resources::toolbar::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> > >` | 0x140afb280 | 34 | 序列化/serde | I：ToolbarCategoryModes deserialize（serde_json，细节并入 visit_str） |  |
| `country_core::resources::toolbar::Toolbar::switch_mode` | 0x140af9850 | 265 | Toolbar::switch_mode | O（手解）：(&mut Toolbar, &{cat u32, mode u8, arg u64}) 逐类 contains 后写 current；panic 串含 'mode but no toolbar'（0x142ae3cbc 区，完整原文 U） |  |

## 实现方式（算法级）（O，机器码手解）

**Toolbar::new（0x140af8c50 符号 / 函数体 0x140af9000 起，O 布局）**：
建 11 个 `ToolbarCategoryModes`（Vec cap-first `{cap@0,ptr@8,len@0x10}`），类别 id 集合
`{6,7,8,0xb,0xc,9,0x10,10,0,0xf,0x11,0x13}`（含 **id=0 类别**；重建代码 `ToolbarCategory::from_id`
未收录 0 与 0x12）；`Toolbar.current u8@+0x18 = 0`。各类别模式列表（非 demo/demo 对比，见 DIGEST §2.1；
Buildings 列表含 (0xb,0x3f) 与 (0x12,0) 两跨类条目，置信度中——Ghidra 类型混淆风险）。

**ToolbarCategoryModes::contains @0x140af89d0（O）**：入参 `&{cat u32@0, mode u8@4}`；SIMD 16B
块遍历 0x10 步长元素（lane 字节比较→位图→逐位扫描）；`cat <= 5` 时 mode 做区间匹配
`lea eax,[mode-0x38]; cmp al,8` → **0x38..=0x3f**（重建代码 0x38..=0x3d 错）；否则精确匹配；
命中返回 1。

**switch_mode @0x140af9850（O）**：入参 `(&mut Toolbar, &{cat u32@0, mode u8@4, arg u64@8})`
（16B 记录，**非** `(u32,u8)` 两参）；逐类 contains → 命中写 `Toolbar.current = 类别下标`；
无命中 → panic（串含 "mode but no toolbar"，0x142ae3cbc 区，完整原文 U）。

**serde（visit_str @0x140afb5b0 / deserialize @0x140afb280 / serialize @0x140afaed0/0x1404f5100）**：
类别按名序列化（`cmp dword [rdx],'Path'` = "Path" 类别名比较）——Toolbar 持久化供 input replay
流加载；`next_mode @0x140afb590` PDB 名错位 = serde unknown_variant 错误 thunk
（`&"variant identifier"` 0x142ae59e4）；真 next_mode U（ICF 折叠）。

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::toolbar::ToolbarCategoryModes::next_mode`（行 21）— I：类别内下一模式（(current+1)%len 逻辑已并入 check_if_can_switch 闭包；PDB 名错位——0x140afb590 实为 serde unknown_variant thunk）
- `country_core::resources::toolbar::ToolbarCategoryModes::last_selected_mode`（行 27）— I：读 categories[i].current（内联进闭包/switch_mode）
- `country_core::resources::toolbar::impl$1::from`（行 69）— I：ToolbarCategory 转换构造（id→枚举，From 派生）
- `country_core::resources::toolbar::Toolbar::switch_category`（行 243）— I：按类别 id 找 Vec 下标写 current（内联进闭包 'byte@0x18=N' 写）

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`closure`、`deserialize`、`eq`、`expecting`、`visit_enum`、`visit_map`、`visit_seq`、`visit_str` 等 13 项
