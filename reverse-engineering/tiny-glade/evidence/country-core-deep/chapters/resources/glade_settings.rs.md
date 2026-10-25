# resources/glade_settings.rs

> 文件作用：**谷地设置存档**——`GladeSettings` 默认布局 {+0x0 u32=0、+0x8 f32=0.01、
> +0xc f32=1.0、+0x10 u32=8、+0x18 f32=0.0、+0x1c f32=1.0、+0x20 u8=1}；`load` 从
> glade_settings.json serde_json Option 读取，失败回退 Default（字段语义 U）。

函数总计 9：业务 2 / 内联无独立符号 1 / 派生胶水 6

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `_::impl$0::deserialize::visit_str` | 0x140a468b0 | 4 | json 反序列化变体名匹配 | I（serde 胶水面） |  |
| `GladeSettings::load` | 0x1408bbbc0 | 49 | 见文件作用 | O：实证 | camera-time\decomp\decomp-0x1408bbbc0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `GladeSettings::try_load`（行 43）— Option 返回版（并入 load 的 unwrap_or_default 路径）

## 派生胶水（derive/宏生成；不计独立业务）

`default`、`deserialize`、`expecting`、`visit_map`、`visit_seq` 等 6 项

## 实现方式（算法级）

`GladeSettings::load`（0x1408bbbc0）：glade_settings.json serde_json Option → 失败回退 Default{+0 u32=0、+8 f32=0.01、+0xc f32=1.0、+0x10 u32=8、+0x18 f32=0.0、+0x1c f32=1.0、+0x20 u8=1}（Default 0x8c2240）。字段语义 U。
