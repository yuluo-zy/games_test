# resources/terrain_stroke.rs

> 文件作用：**地形笔画：TerrainStroke 点列（Vec<Vec3>，上限 1024）+ 插入/形变末点规则（0.001 单点阈值 + spacing 比较）。terrain 编辑数据单元。**

函数总计 10：业务 3 / 内联无独立符号 1 / 派生胶水 6

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::terrain_stroke::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>` | 0x14157db20 | 9 | serde 反序列化 visitor（derive 胶水） | O | terrain-water/DIGEST.md
| `country_core::resources::terrain_stroke::_::impl$0::serialize<serde_json::value::ser::Serializer>` | 0x14157d970 | 9 | serde_json::Value 序列化（derive 胶水） | O | terrain-water/DIGEST.md
| `country_core::resources::terrain_stroke::TerrainStroke::insert_or_morph_last_point` | 0x140a97880 | 42 | len==0→push；len==1 且 dist>0.001→push；len≥2 且 dist(last,prev)≥spacing→push（<1024），否则覆盖 last（morph） | O | terrain-water\decomp\decomp-0x140a97880.txt

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::terrain_stroke::TerrainStroke::new`（行 23）— 构造空笔画

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`deserialize`、`expecting`、`visit_map`、`visit_seq` 等 6 项
