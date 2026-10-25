# resources/placement_data.rs

> 文件作用：**放置数据 SSBO 元素（position/scale/angle→Affine3A）。**

函数总计 11：业务 3 / 内联无独立符号 2 / 派生胶水 6

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::resources::placement_data::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error> | 0x140b3b960 | 8 | serde 反序列化派生（PlacementDataSsbo visit_str） | O（serde 派生） |  |
| country_core::resources::placement_data::PlacementDataSsbo::to_affine3a | 0x140ad5020 | 84 | 0xad5020：绕 Y 旋转×scale+平移；4×movaps 写 4 行（末行 xmm1=position 含 z——w 行=pos.z 为原版怪癖，忠实转录） | O |  |
| country_core::resources::placement_data::PlacementDataSsbo::position | 0x140ad51a0 | 88 | 0xad51a0 | O |  |

## 实现方式（算法级）（O + 原版怪癖 I）

**PlacementDataSsbo::to_affine3a @0x140ad5020**：绕 Y 旋转 × scale + 平移；4×movaps 写 4 行
（末条 `[rsi+0x30]=xmm1`，xmm1=movups [rdi]=position 含 z）。（O）
- **原版怪癖**：16 元素第 16 位（w 行第 4 元素=m33）确实写 `position.z`（现有代码忠实转录
  而非笔误）；w 行未被着色器/矩阵消费（消费面未定位），m33≠1 属原版，暂记 I。（I）
- `position @0x140ad51a0`：字段读取（O）。`to_transform`（行 75）内联。I

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::placement_data::default_anim_t`（行 43）— 构造
- `country_core::resources::placement_data::PlacementDataSsbo::to_transform`（行 75）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`default`、`deserialize`、`expecting`、`visit_map`、`visit_seq` 等 6 项
