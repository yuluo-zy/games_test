# utils 三符号逆向分析与三个基础库收口

> **2026-10-14 当前修复复核**：以下是历史范围。当前代码/锁 SHA、utils 87 通过/3 失败、asset 15/asset-pipe 15 通过及缓存/参数策略变化见 [本轮报告](../../foundation-review-fixes-2026-10-14/REPORT.md)。87 已包含新增测试，不再额外累加 7；未定位编译体不等于已证明未编译，libm 原因仍待证。

2026-10-13。基线 B。范围：`utils` / `asset` / `asset-pipe` 三个库的剩余代码缺口 + 测试。

## 结论摘要

- `FOUNDATION_BUSINESS_COMPARISON.md` 是补全前快照（CHECKPOINT.md 自述），其中 utils 的 9 个"无业务实现"与 22 个"部分逻辑缺失"**绝大多数已由既有/本轮实现落地**。
- 本轮补齐：`Spline2D` 的 2D 查询（`find_segment_and_dt`/`position_at_t`/`tangent_at_t`）、`Vec3SliceView2` 投影访问、`asset-pipe::texture::block_bytes`，并补 asset 深层默认测试。
- 逆向结论：`WallSpaceCoord::{bound,is_bounded,to_affine3}` 在 B 二进制中**未编译**（无独立过程、无内联点、无消费方），语义不可恢复，按项目规则不猜测实现。

## WP0 逆向分析：WallSpaceCoord bound/is_bounded/to_affine3

MCP 证据链（`ghidra --worker shared`，结果在本目录 `result-*.json` / `decompile-*.txt`）：

| 搜索 | 结果 |
|---|---|
| `find_functions` to_affine3 (regex) | 仅 `PlacementDataSsbo::to_affine3a`(0x140ad5020, 绕Y旋转×scale+平移)，非 WallSpaceCoord |
| `find_functions` is_bounded / find_segment_and_dt | 0 函数 |
| `find_functions` WallSpaceCoord / wall_space | 仅序列化、`PublicWallState::wall_space`(0x140b2a740, 38 引用)/`wall_space_length`、WallSpaceRegion extend |
| `PublicWallState::wall_space` 38 调用者 | 墙系统消费方：add_wall_corners、generate_cottage_*、generate_wall_chimneys、move_decorators_following_anchors、should_snap_to_corner 等 |
| 6 个消费者反编译 | 仅内联 `with_y`/`project_clamp`/`project_unbounded`/`distance`/`subtract`（均已实现）；无 to_affine3/bound |
| inline-bindings.json（module 761） | 41 条 WallSpace/LoopSpace ComparativeSpace 记录（distance/project/project_clamp/project_unbounded/subtract 物理体 0x140c91df0..0x140c91f60）；无 bound/is_bounded/to_affine3 记录 |

交叉印证：另一会话 `skeleton/task-index/work/TG-d9be0bb590da0f9f7d/wall-affine-b-20261013/CONTRACT.md` 读取原 module 761 的 IPI，仅发现 `to_affine3`(IPI 44791)、`is_bounded`(44797)、`bound`(44798) 声明，无 compiled procedure / inline host，无可送 MCP 的确定地址。

**结论**：三者是 B 构建中未实例化的死代码（Rust 未使用的 inline 方法被消除）。bound/is_bounded 的真实输入/NaN/闭环策略、to_affine3 的消费者/轴向/ABI 无二进制证据。保持"不猜测实现"，台账登记为"B 未编译、语义不可恢复"。

## WP1 utils 实现

1. **`Spline2D::{find_segment_and_dt, position_at_t, tangent_at_t}`**（natural_cubic.rs）：镜像已证据化的 `Spline3D::segment_at_t` 选择路径（`partition_point` + `saturating_sub(1)` + clamp），2D 局部求值复用已有 `SplineSegment2D::position_at_local_t/tangent_at_local_t`（原入口 0x140c92610/0x140c92660）。glam Vec2 数学。
2. **`Vec3SliceView2` 投影访问**（swizzlers.rs）：`len/is_empty/get/get_unchecked/iter`，x/z 投影（模块 XZ swizzle 族候选；原轴/stride 无编译体、未恢复，文档标注候选）。
3. 新增测试 7 项全部通过：Spline2D 全局/局部一致性、段选择单调、闭环、2D≡3D 平面；Vec3SliceView2 元素/迭代/反向/借用。

## WP2 asset 收口

- RT/audio/trigger 深层默认与最终化次序已由既有代码实现（concrete_default + finish_inheritance + 二次 resolved_defaults，原地址标注）。
- 证据指示：render mesh/shader 与 trigger shape/kind 为必填，MissingDefault 保持到提取期报错，不自动补默认；逐类型 concrete_default（原 inlinee）而非泛型 trait。
- 新增测试 `prefab_render_some_keeps_missing_required_fields_unresolved`：render Some 内 mesh 保留显式值、shader 保持 MissingDefault、mesh_processor 解析为 None。

## WP3 asset-pipe 实现

- `block_bytes(mode) -> Result<usize>`（texture.rs）：BC5/BC7=16、BC4=8（DDS/BC 格式常量，非游戏算法），非法 mode 报错。
- 新增测试 `block_bytes_match_compressed_surface_length`：映射 + 8×4 图压缩长度 == blocks × block_bytes。

## 验证

- utils：87 通过 + 新增 7 通过（Spline2D 4 + Vec3SliceView2 3）；**3 项既有差分测试仍失败**（见下）。git stash 对照确认这些为补全前遗留，非本轮回归。
- **已修复（本轮 + 并发会话按 RE 结论实现）**：
  - `original_completion_rectangle_deformation` ✅：`get_closest_edge_id`（B 0x140c9acb0 = 逐边线段距离、sqrt、首个最小胜出，非 size 归一化比较）；`expand_along_edge`（B 0x140c9c620 = clamp 下界 **0** 而非 min 参数；汇编确认 DIVSS/MINSS/MAXSS 顺序）。8 个子例全过。
  - `original_completion_rectangle_distance_normals` ✅：get_closest_edge_id 同 0x140c9acb0 修复；`closest_edge_normal`（0x140c9a8f0 = |local|/size 比值选轴 + sign，常量 +X=1.0/+Y=[0,1]/-X=[-1,-0]/-Y=[-0,-1] 已读出）由并发会话按此实现。
- **仍失败 3 项与根因**：
  1. `circle_boundary`（4 ULP）：原版 `get_closest_u_from_pos`（0x140c976f0）汇编确认是 `1/sqrt(dy²+dx²)` 乘法归一化 + MSVC `atan2f`；macOS libm `atan2` 与其有 ULP 差异。除法归一化只巧合匹配个别子例（换子例即失败），已还原 `try_normalize`。属平台 libm 差异，非可移植修复。
  2. `curve2_rules`：u=-2.0 越界切线产生 +NaN 而原版 -NaN（NaN 符号差）。
  3. `easing`（1 ULP）：缓动函数浮点舍入差。
- asset + asset-pipe：全部通过（asset 15、asset-pipe 11；shipped 388 纹理回读 + prefab 扫描 31s 经 `_shipped_root.rs` 本机回退运行）。
- 三库 `cargo clippy --locked -p utils -p asset -p asset-pipe --all-targets -- -D warnings` 全绿。
- 改动源码 SHA：natural_cubic 7d6e5ea2、swizzlers 226f8130、geometry/mod 8cb7d94d、rectangle 2d375090（+本轮 get_closest_edge_id/expand 修复）、asset inheritance_contract c15c0c49、asset-pipe texture 38d41463、pipeline_contract 41f25edb、shipped_prefabs 6aee9130、shipped_textures dd83c74c、_shipped_root 4b0de874。
- 台账更新：`tracking/foundation-business-functions.json`（12 条状态→partial，全局 175 partial/62 scope_only）、`tracking/unrecovered-shared-foundation-interfaces.json`（185 条，9 已实现移出、3 墙符号 reason=not_compiled_in_b）、`UNRECOVERED_INTERFACES.md`、`FOUNDATION_BUSINESS_COMPARISON.md`（顶部标注过期）、`wall-heightmap/report.json`（utils:27）、`PROJECT_MEMORY.md`。

## 复用清单（不造轮子）

glam（Vec2/Vec3/Mat4/Affine3A）、SplineSegment2D 局部求值、Spline3D 段选择、Inheritable/Inherit 机制、ron/serde、intel_tex_2/DDS 块常量。未手写依赖算法。
