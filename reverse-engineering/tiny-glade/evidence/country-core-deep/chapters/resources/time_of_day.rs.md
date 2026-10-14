# resources/time_of_day.rs

> 文件作用：**昼夜数据资源**——`TodSequence`（entries: Vec<TodSequenceEntry>，0x50B 步长：
> {start_hour,end_hour,snapshot@+0xc}）与 `TodLightingSnapshot`（0x44B：sun_sky@+0x0 8×f32、
> fog_tint@+0x20 2×u64 打包雾色（**不插值**）、base_fog_thickness@+0x30 4×f32、speed_bias@+0x40）
> 的逐帧插值；会话序列化：TodSequence{entries}、TimeOfDay{time_of_day,snapshot,temp_ui_ids}；
> 目标态小结构 {time_of_day, sun_xz_rotation, latitude_degrees}；TodEasing 默认 fastrand 选段。

函数总计 24：业务 5 / 内联无独立符号 3 / 派生胶水 16

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `TodSequence::snapshot_at_tod` | 0x140b067d0 | 0 | `t=tod.rem_euclid(1)` 定段→clamp01 逆 lerp→TodLightingSnapshot::lerp（u64 对直接拷贝）；len==0 panic | O：实证（tod 经 sret 缓冲首 4B 传入） | camera-time\decomp\decomp-0x140b067d0.txt |
| `_::impl$0::serialize<serde_json::value::Serializer>` | 0x1400802c0 | 15 | 目标态小结构 {time_of_day, sun_xz_rotation, latitude_degrees} 序列化 | O：字段名串实证（round2） | round2\decomp\r2-0x1400802c0.txt |
| `_::impl$0::deserialize::visit_str` | 0x140080710 | 127 | 同结构反序列化变体名匹配 | I（serde 胶水面） |  |
| `_::impl$0::serialize<serde_json Pretty>` | 0x14007fb80 | 127 | TodSequence 序列化（单字段 "entries"） | O：实证（round2） | round2\decomp\r2-0x14007fb80.txt |
| `walls::_::impl$0::serialize<serde_json Compact>` | 0x140080160 | 179 | 数值字段序列化（u32/u64，9999 年份上限检查 + 数字 LUT "000102…99"）——std 胶水复用面 | O：实证（round2） | round2\decomp\r2-0x140080160.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `TimeOfDay::hour`（行 59）— 当前小时 getter（f32 直读）
- `TodLightingSnapshot::lerp`（行 145）— 快照插值（f32 组 lerp、fog_tint u64 对拷贝；0x140a97650）
- `snapshot_lerp`（行 234）— 两个快照间插值（并入 snapshot_at_tod 的分段插值调用）

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`default`、`deserialize`、`eq`、`expecting`、`hash`、`serialize`、`visit_map`、`visit_newtype_struct`、`visit_seq`、`visit_str` 等 16 项
