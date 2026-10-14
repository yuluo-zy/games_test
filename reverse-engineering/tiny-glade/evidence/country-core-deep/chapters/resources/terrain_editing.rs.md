# resources/terrain_editing.rs

> 文件作用：**地形编辑数据：TerrainEditingData {HashMap<TerrainStrokeId, TerrainStroke>（数据@+8/掩码@+0x10/计数@+0x20）, last_stroke_id u32@+0x38}；new_stroke 上限 0x3fffff。编辑笔画的生命周期管理。**

函数总计 22：业务 6 / 内联无独立符号 6 / 派生胶水 10

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::color_id::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x1400e7480 | 38 | ColorId serde 序列化（ICF 同址共享，derive 胶水） | O | terrain-water/DIGEST.md
| `country_core::resources::terrain_editing::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x1400e7480 | 38 | TerrainEditingData 存档序列化（CompactFormatter） | O | terrain-water\decomp\decomp-0x1408e5bc0.txt
| `…impl$0::serialize<serde_json::PrettyFormatter>` | 0x1408e5bc0 | 38 | 序列化/serde | O（胶水）：serde_json 派生 serialize（PrettyFormatter 变体） | terrain-water\decomp\decomp-0x1408e5bc0.txt |
| `country_core::resources::terrain_editing::_::impl$0::serialize<serde_json::value::ser::Serializer>` | 0x1408e6b70 | 43 | TerrainEditingData serde_json::Value 序列化 | O | terrain-water\decomp\decomp-0x1408e6b70.txt
| `country_core::resources::terrain_editing::TerrainEditingData::get_last_mut_stroke` | 0x140924900 | 83 | 按 last_stroke_id 查 HashMap 取末笔画可变引用 | O | terrain-water\decomp\decomp-0x140924900.txt
| `country_core::resources::terrain_editing::TerrainEditingData::new_stroke` | 0x140924a10 | 91 | last_stroke_id<0x3fffff 则 +1 并 HashMap::insert，否则 None | O | terrain-water\decomp\decomp-0x140924a10.txt

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::terrain_editing::TerrainEditingData::new`（行 64）— 构造空数据
- `country_core::resources::terrain_editing::TerrainEditingData::get_mut_stroke`（行 75）— 按 id 取可变引用
- `country_core::resources::terrain_editing::TerrainEditingData::get_stroke`（行 79）— 按 id 取只读引用
- `country_core::resources::terrain_editing::TerrainEditingData::last_stroke_id`（行 87）— 当前笔画 id
- `country_core::resources::terrain_editing::TerrainEditingData::set_stroke`（行 110）— 置入笔画
- `country_core::resources::terrain_editing::TerrainEditingData::clear`（行 120）— 清空 + id 归零

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`deserialize`、`eq`、`expecting`、`fmt`、`serialize`、`visit_map`、`visit_seq`、`visit_str` 等 10 项
