# R7：E 包其余部分（utils：JSON 加载器、geo、debug_viz 构造器、child_wall）

日期：2026-10-14。落点：`tiny-glade/crates/country-core/src/utils/{load_json,geo,debug_viz,child_wall}.rs`。每项用 EXE 汇编/反编译核对。

| 项 | 地址 | 级别 |
|---|---|---|
| `read_attribute_from_json` / `load_json_as_mesh` | 0x14083e210 / 0x140b0eeb0 等 | O：流程见 R6 末尾，现已落码；`serde_json` 紧凑文本匹配类型，`assert!(len as i32 > max_index)` |
| `mesh_from_pts(label, pts)` | 0x140a6c120 | O：`rcx`=sret，`rdx/r8`=label，`r9`+栈=点区间；`assert!(pts_len > 0)`（串 0x142ad5530 实读）；`Vertex_Position`=点拷贝，`Vertex_Color`=`vec![Vec3::ONE; n]`（`0x14292e6b0` 两个 1.0 + 栈上 `0x3f800000`），索引 `0..n`（SIMD 展开只是向量化） |
| `update_curve_mesh`（4 个单态化，体相同） | 0x14157f130 等 | O：先按句柄取网格，取不到 → panic `MEOW####`（0x142c77ab9，8 字节）；之后与 `mesh_from_pts` 同样改写三项，空点列由 `set_attribute` 的非空断言 panic |
| `rewire_enclosed_curve(curve, split)` | 0x140a6ba50 | O：`split==0.0` 或 `==1.0`（0x142925904 低 4 字节 = 1.0）→ `None`；`crashy_slice(0,split)` 与 `(split,1)`，任一长度 < 0.1（0x142925984 = `0x3dcccccd`）→ `None`；前段末两点距离 < 0.1 丢末点，后段首两点距离 < 0.1 丢首点，原曲线首尾点距离 < 0.1 再丢后段末点；结果 = 后段 + 前段，`try_new_from_points(.., false).unwrap()` |
| `ChildWall::slice` | 0x140a24000 | O：DIGEST 说的 "clamp" 是错的——helper 0x140c904d0 是 `a + (b-a)*t`（`subss/mulss/addss`），结果须在 [0,1]，否则 panic `Tried to init CurveU from an invalid u: {u}`（串 0x142aa5398）。返回 `{id, lerp(t1), lerp(t2)}`，布局 id@0、a@8、b@0xc |
| `DebugSphere` / `DebugRectangle3d` / `DebugLine` 构造器与 builder | 0x140b02490 … 0x140b04f40 | O：13 个短函数逐字节核对；0x28 / 0x38 / 0x48 字节布局与默认值见模块注释 |

## 复查补记
统一复查后改了一处顺序：`read_attribute_from_json`（0x14083e210）里 `["buffer"]` 的查找在每个类型分支内部（反编译第 71–75、102–106 行等各取一次），类型匹配在前，所以未知类型即使没有 buffer 也先报 `unrecognized attrib`。之前的实现先取 buffer，已改；测试 +1。

## 偏差与未做
* `FUN_141466bb0` 按工作区里的 `Curve::crashy_slice` 使用（它带 `get_coord_at_u` 0x141467ce0 调用，I）。
* 构造器只写 `color` 的首个 dword，其余 12 字节在 EXE 里未初始化，取 0（I）。`DebugRectangle3d::rect` 的 32 字节布局未解（U）。
* `child_wall` 没有复用 `utils::curve::CurveU::new`（panic 文本不同）；`utils` 是共享 crate，不在此改。
* **`get_last_event` / `drain_events`（0x1402b4910 / 0x1402b47f0）**：反编译里 `+0x38` 哈希表按 TypeId 取、`+0x1e0` 步长 0x60 的资源表、`+0x5b0` 变更 tick、`events_a/b.start = event_count` 并 `mem::take` 两个缓冲，是 bevy_ecs `World` 资源访问 + `Events<T>::drain`，不是 DIGEST 所说的“自定义 EventRegistry”。属于 ECS 胶水，归入 G/H 包。
* `audio_utils`（`AudioLoop` 等，依赖 kira）归入音频模块；`debug_viz::*::show` 及闭包（`Commands` + 资源库）归入 H 包。
* `debug_viz::DebugRectangle2d` 没有构造器符号。

测试：`load_json` 6、`geo` 6、`debug_viz` 2、`child_wall` 3，共 +17。
