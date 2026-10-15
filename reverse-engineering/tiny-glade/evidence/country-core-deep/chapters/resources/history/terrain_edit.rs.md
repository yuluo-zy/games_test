# resources/history/terrain_edit.rs

> 文件作用：地形编辑负载：TerrainEdit（判别 0/1/2：New/Edit/Move stroke；duration=clamp(Σ点距*0.01,0.4,3.0) 内联、revert 0.99 进度阈值+stroke 表清 active、reapply、update_terrain_curve 0x140859740 hash 查表+点集拷贝+fold+全局时间 lerp）手写 serde（serialize 0x1404f41b0/0x1417c1590、deserialize 0x1417c1ac0、visit_str 0x1417c23e0）。

函数总计 24：业务 7 / 内联无独立符号 17 / 派生胶水 0（已并入业务表）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（有依据推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::history::terrain_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >` | 0x1417c1ac0 | 20 | serde 主入口 monomorph（serde_json::Value / Serializer 实例） | O(入口) | 见文件作用所列 serde 地址 |
| `country_core::resources::history::terrain_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x1404f41b0 | 20 | serde 主入口 monomorph（serde_json::Value / Serializer 实例） | O(入口) | 见文件作用所列 serde 地址 |
| `country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>` | 0x1417c23e0 | 36 | serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口） | O(入口) | 见文件作用所列 serde 地址 |
| `country_core::resources::history::terrain_edit::impl$0::fmt` | 0x140858c80 | 62 | TerrainEdit Debug::fmt（niche 判别 0/1/2） | O | decomp-0x140858c80.txt |
| `country_core::resources::history::terrain_edit::TerrainEdit::revert` | 0x140858d30 | 102 | 0.99 进度阈值 panic + hashbrown 48B 表清 active 标志 | O | decomp-0x140858d30.txt |
| `country_core::resources::history::terrain_edit::TerrainEdit::reapply` | 0x1408592d0 | 184 | 对称重放应用 | O | decomp-0x1408592d0.txt |
| `country_core::resources::history::terrain_edit::TerrainEdit::update_terrain_curve` | 0x140859740 | 257 | hash 查 stroke 表+点集拷贝+fold+全局时间 lerp 进度回写 | O | history\decomp\decomp-0x140859740.txt |

## 内联/无独立符号函数（名称揭示意图；逻辑并入调用点）

- `country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$0::visit_str`（行 20）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$1::deserialize`（行 20）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::visit_enum`（行 20）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::terrain_edit::_::impl$0::deserialize`（行 28）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::expecting`（行 28）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::visit_map`（行 28）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::visit_seq`（行 28）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::terrain_edit::_::impl$0::serialize`（行 28）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::terrain_edit::TerrainEdit::duration`（行 78）— clamp(Σ点距*0.01,0.4,3.0)，空 0.3（内联进 TerrainReplayCmd::forward）（O；decomp-0x140994f30.txt；CLOSURE §1）
- `country_core::resources::history::terrain_edit::impl$1::duration::closure$0`（行 85）— 点距累加 fold 闭包（相邻点 distance，Σ*0.01 clamp(0.4,3.0)）（I；内联进 0x140994f30）
- `country_core::resources::history::terrain_edit::impl$1::duration::closure$0::closure$0`（行 88）— 点距累加 fold 闭包（相邻点 distance，Σ*0.01 clamp(0.4,3.0)）（I；内联进 0x140994f30）
- `country_core::resources::history::terrain_edit::impl$1::revert::closure$0`（行 125）— 撤销应用命令构造闭包（EditTerrainStrokeCmd 族）（I；内联进 0x140858d30）
- `country_core::resources::history::terrain_edit::impl$1::revert::closure$1`（行 131）— 撤销应用命令构造闭包（EditTerrainStrokeCmd 族）（I；内联进 0x140858d30）
- `country_core::resources::history::terrain_edit::impl$1::revert::closure$2`（行 169）— 撤销应用命令构造闭包（EditTerrainStrokeCmd 族）（I；内联进 0x140858d30）
- `country_core::resources::history::terrain_edit::impl$1::reapply::closure$0`（行 206）— 重放应用闭包（与 revert 对称）（I；内联进 0x1408592d0）
- `country_core::resources::history::terrain_edit::impl$1::reapply::closure$1`（行 241）— 重放应用闭包（与 revert 对称）（I；内联进 0x1408592d0）
- `country_core::resources::history::terrain_edit::impl$1::update_terrain_curve::closure$0`（行 265）— 进度 lerp 闭包（fold+全局时间，(1-t)*u+t）（I；宿主 0x140859740）

<!-- 未覆盖函数数：0 -->
## 实现方式（算法级，2026-10-15 深挖）

- **TerrainEdit 判别**（Debug::fmt 0x140858c80）：首字段 niche `x^0x8000000000000000
  < 2` → 变体 0/1/2（0=NewStroke(+4 处 stroke id)、1=+1 处 0x11B 载荷、2=+4 处
  0x10B 载荷）。
- **revert 0x140858d30 / reapply 0x1408592d0**（对称）：判别 0（NewStroke）且
  `param_6 < 0.99f`（静态 0x142a948d8 = 动画进度阈值）→ panic "Requested terrain
  stroke {id}"（terrain_edit.rs，@0x142a94cf8）；否则 hashbrown 查 48B/元素 stroke
  表（FUN_1405b0ed0），命中 → 清/置该 stroke active 标志（+0x28）。
- **duration**：无独立符号；公式内联于 `TerrainReplayCmd::forward 0x140994f30`：
  `clamp(Σ相邻点距*0.01, 0.4, 3.0)`（cm→m 换算），无点默认 0.3；"assertion failed:
  stroke_length.is_finite()"（terrain_edit.rs:43，@0x142a94c34）。
- **update_terrain_curve 0x140859740**（reapply 闭包宿主）：hash 查 48B stroke 表 →
  拷贝 12B 点集 → FUN_141469230 fold 求进度 → 与全局时间 FUN_140c95ef0
  (&0x142a94d48) 做 `(1-t)*u + t` lerp → 0..1 范围断言（复用 "Tried to init CurveU
  from an invalid u: "）→ 回写 stroke 记录 Vec。

