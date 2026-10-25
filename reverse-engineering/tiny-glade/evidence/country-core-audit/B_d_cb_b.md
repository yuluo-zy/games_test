# B_d_cb_b：`build_draw_lists` 回调后一半（gap 序第 36..=69 个，共 34 个）

O = 机器码观测，I = 推断，U = 未知。未 commit。
文件：`systems/render_loop/draw_passes/draw_callbacks_b.rs`（新，含 42 个测试）；`draw_lists.rs`（只改这 34 行 `func()` 指向 + `is_recovered` 追加 34 项）；
`draw_passes/mod.rs`（追加 `pub mod draw_callbacks_b;`）；工具 `tools/country-core-audit/{typeid_resolve.py,cb_trace.py}`（见 §1）。

## 1. 取值路径（O）：TypeId 全部反查出真名，没有靠猜
闭包里的资源/组件访问是 `Components::get_resource_id/get_id(TypeId 两个 u64)`。同一对常量同时出现在 `init_state<T>` / `register_info<T>` /
`component_ids<T>` / `register_resource_with<..T..>` 等泛型实例体内，据此在 `procs.tsv` 反查 T（`typeid_resolve.py`：扫 EXE 里 8 字节常量 → 过程 → 取交集；
`cb_trace.py <addr…>` 用它标注每个回调的调用序列，含 `bind_texture`/`write_uniform<T>`/`read_storage_buffer<T>` 的泛型实参）。
- 资源：`get_resource_id` → `[world+0x210]` 稀疏表 → `[world+0x1e0]` 槽，缺失 → `Option::unwrap` panic。
- 组件：实体位置失败 → `Result::unwrap`；`get_id`+`Table::get_component` 缺失 → `Option::unwrap`；`get_id`+原型稀疏集 `cmp [+8*id],0; setne` = `EntityRef::contains`（Env18）。
- `nani_draw` 的 `rg`/`World` 在原版存在 builder 里；Rust 里 `nani_draw_world(b, kind, &World)` 取 `GpuWorld`、`AssetSsboLibrary`（均 `Resource`），`RenderGraph` 用一次性 `default()`（I）。

TypeId → 类型（O，括号内为两个 u64）：
`SemiTransparent`(0xe8fca894de8d2f65/0x7332eb25568966c0)、`FlatLighting`(0xc4c6de357615d70f/0x88fe3ad9437dd55c)、`MainCamera`(0xbba451ee6a08a3ff/0x8d008d15febcf29d)、
`InstancedDrawFromPlacementData`(0xed76c1eeabfaeba0/0x282debdb03116a9c)、`WallHighlight`(0x850dbd51eafef756/0x85782a4b39854c35)、`EditingHighlight`(0x6efcda1c77e106b4/0x7c999c10e7fa637f)、
`CursorTerrainRaycast`(0x062dfe8f62b6fcc5/0xf65008e548632ba7)、`EraserSizeMultiplier`(0x0ae77a86a2c6cefc/0xf5509a56b32e81d0)、`DebugGui`(0x48b321ef779dc301/0xbd65767194d9ddc8)、
`RoofFlagCloth`(0x983a5c6eded1c4e5/0x929a707bdcc6a79c)、`ColorId`(0x2ff65f567dea6ff9/0xbf26a3d6a6834edb)、`ActiveSession`(0x74430dfcc6d8719b/0xca5591f7e464509c)、
`LogoAnimation`(0x98df08dcfa7ea5ba/0x67b117a60e632bc3)、`AssetSsboLibrary`(0x8226cc786e679355/0x65cfda6a55a841ac)、`RoofHighlight`(0x9311cf2688353878/0xfd54118f9a9ee3d4)、
`GladeSettings`(0x3dce36237fb50dff/0x93a27cf4512d885c)、`CritterControl`(0xa2127765959c8b85/0xf078ccfbcec1f347)、`PrevTransform`(0xa0da022fc84ca3d9/0xfc62e38c26a7c4c9)、
`Duck`(0x272a1f294225ce7f/0xd79870fafff409bc)、`DuckState`(0x3a8f7d4d3ac26bd0/0xff6d570a464cbf09)、`Weather`(0xd2f519263066413b/0x5192b15f962559e4)、
`TreeCanopyMaxCullDistance`(0x1b4e385d376c1cc0/0x8c38d6ede2a437ec)、`RockyTerrainMeshVertCount`(0xfa44538b95ae76c6/0x12146c4ea814069b)。

## 2. 34 个回调 → Rust 项（全部 O，除 §4 的 I/U）
| DrawFnId / 地址 | Rust | 要点 |
|---|---|---|
| Env1 0x1409b5c70 | `env1` | `BoatSsbo`(0x6d92…) → `multi_draw_indexed_indirect(0xc,1,0)` |
| Env7 0x1409b6150 | `env7` | 磁盘 + 预设 0xf/5/0xe/0x13/9/0xa + `Flower03Ssbo` → `multi_draw_indexed_indirect(0x10,1,0)` |
| Env26 0x1409b7d40 | `env26` | 预设 9/0xa + `GardenStoneSsbo` → `multi_draw_indexed_indirect(0xe,1,0)` |
| Env54 0x1409b7110 | `env54` | 磁盘 → `draw_instanced_w_gl_draw_flag(0x1770)`（旧文档标成 `draw`，以机器码为准） |
| Env51 / Env60 | `env51` / `env60` | 磁盘 → `draw`；Env60 先 `state.byte0=(b&0xf3)\|8`（`and al,-0xd; or al,8`，调用序列抽取看不到） |
| Env25 / 49 / 24 / 44 / 31 / 9 / 29 / 15 | `env25…env15`(+`_body`) | 纯 `nani_draw(kind)`：SolidVertexColor / VelocityOnly / Smoke / Plant / SolidVertexColorWindow / SolidAlbedoMap / …WindowGlass / CurvedHalfTimber；Env24/29 `state\|4`，Env15 `state\|8` |
| Env12 / Env13 | `env12`/`env13` | `uniform<i32>(0x052e…,0)`、`HalftimberColorIdSsbo`、磁盘、预设 5/9/0xa、`bind_wall_color_customization`、屋顶砖磁盘、`RoofColorIdSsbo` → `nani_draw(6)`；Env13 再 `bind_brick_erase_highlight` → `nani_draw(7)` |
| Env34 0x1409b7560 | `env34` | 先读 4 个资源，`state\|8`，两磁盘、墙色定制，`bool(0x203f…)=EditingHighlight[+0xc]`、`i32(0x5add…)`/`i32(0x052e…)`=墙高亮 id/tag、`Vec3(0x06bd…)`=光标、`f32(0xb585…)`=橡皮半径 → `nani_draw(Plaster)` |
| Env18 0x1409b5cf0 | `env18` | `contains::<SemiTransparent>`/`<FlatLighting>` → 两个 bool uniform → `draw` |
| Env5 0x1409b68e0 | `env5` | 磁盘 → `MainCamera.eye_position()` Vec3 → 8 个预设 → 组件 `InstancedDrawFromPlacementData`(u64) → `draw_instanced_from_placement_data` |
| Env21 0x1409b8680 | `env21` | `RoofFlagCloth`(u32@0)、`ColorId` 先读；bool×2=false → 4 磁盘 → `i32×2` → 磁盘 → `ActiveSession` 字节选末张磁盘（任一字节 ≤1 → 0x4c58…，否则 0xe523…） → `draw` |
| Env38 0x1409b9080 | `env38` | `LogoAnimation`(f32@+8、f32@+0x10) → 4 磁盘 → 两 f32 uniform → `draw` |
| Env36 0x1409b7fa0 | `env36` | `AssetSsboLibrary` → `SparkleSsbo` 状态（缺 → unwrap panic；名字 0x2d B 比对 `assert_eq!`）→ `to_draw_multi_instanced` → `read_storage_buffer<SparkleSsbo>(0xc348…)` → `draw_multi_instanced` |
| Env43 0x1409b8620 | `env43` | `rocky_terrain_shader(b, false, RockyTerrainMeshVertCount)` |
| TreeTrunk / TreeCanopy | `tree_trunk_cb` / `tree_canopy_cb` | 取 `Weather`、`MainCamera`、`TreeCanopyMaxCullDistance` 后调既有 `tree_trunk/tree_canopy` |
| 屋顶 ×3 | `roof_nani_cb`(`RoofHighlight`)、`roof_spire_cb`、`roof_support_beam_cb`(`AssetSsboLibrary`→`LibView`) | |
| WaterPlanesDraw | `water_planes_cb` | 进程布尔 `WATER_PLANES_DISABLED`、`ActiveSession`（变体字节、字节序列），`bind_ray_tracing<DrawBuilder>` 用 A4 真实实现 |
| LeafParticle / Firefly / Glow / Duck / Wren | `leaf_particle_cb` / `firefly_cb` / `firefly_glow_cb` / `duck_cb` / `wren_cb` | `GladeSettings`、`CritterControl`、`PrevTransform+Duck+DuckState`、`nani_draw(Wren)` |

SSBO 标签（本文件内，槽名/SsboId 由 `dsh_ssbo_info.py` 从 EXE 抽取，测试用 xxh3 复算）：`BoatTag`("Boats")、`Flower03Tag`("Small lowpoly flowers")、`GardenStoneTag`("Garden Stone")、`HalftimberColorIdTag`、`SparkleTag`。

## 3. 对前序文档/代码的纠正（O）
1. `draw_builder.rs::BrickEraseHighlightInputs` 文档里类型 U 的 A/C/D：A=`EditingHighlight`(byte+0xc)、C=`EraserSizeMultiplier`(f32)、D=`DebugGui`(byte+0xfe)（`bind_brick_erase_highlight` 读取序 A,B(=CursorTerrainRaycast),C,D）。
2. 树 shader 的“资源 A”= `Weather`(f32@+4)、“相机资源”= `MainCamera`、“资源 C”= `TreeCanopyMaxCullDistance`；`MainCamera` 文档称 `byte@+0x4d0` 是**模式字节**，tree shader 里叫 season（I，未改旧文件）。
3. 鸭 shader 第一个组件是 `PrevTransform`（Transform 布局），不是 `Transform`；落叶粒子数资源是 `GladeSettings`；萤火虫强度资源是 `CritterControl`。
4. `closure_env$54` 等的“调用类别”摘要里 `draw` 实际可能是 `draw_instanced_w_gl_draw_flag(N)`；以本文件为准。

## 4. I / U 清单（不编造）
- **U 类型布局**：`MainCamera.raw`、`ActiveSession`、`Weather`、`EditingHighlight`、`DebugGui`、`LogoAnimation`、`RoofFlagCloth`、`PrevTransform` 在 crate 里是 `Gap`/无 `Resource` 派生，本文件放只含被读字段的 `*View` 视图（偏移 O，名字后缀 View）；`Duck`/`DuckState`/`CritterControl`/`EraserSizeMultiplier`/`RoofHighlight`/`InstancedDrawFromPlacementData` crate 里不存在，按 PDB 名放 stub。**集成者**：真实类型落地后把 `res::<XView>` 换成真实类型即可。
- U `TreeCanopyMaxCullDistance`：真实类型是单字段 `f32`（无 `Resource`），机器码读 `f32@+4`，不一致；取其值，视图 `TreeCanopyMaxCullDistanceView`。
- I `WallHighlight`：`tag=ty`、`id=wall_id.0 as u32`（`tag!=0` 才取 id）与机器码 `byte@+0x10`/`dword@+8` 的对应关系未证实。
- I `GladeSettings.a`（u32@0）当作 `u64@+0` 粒子数（零扩展）。
- I `nani_draw_world` 用一次性 `RenderGraph::default()`（`RenderGraph` 在 crate 里不是 `Resource`）；`get_rg_buffer` 命中已导入句柄则不碰它。
- U Env36：`SparkleSsbo` 是 `Gap`，无 `SparseSsbo` 实现，稀疏区间由 `SparkleDrawRangesView` 携带（库状态存在/名字断言已按机器码做）。
- I Env43 读取顶点数的时机在调用前（原版在 shader 内最后读）；仅影响缺资源时的 panic 先后。
- I `WATER_PLANES_DISABLED` 是进程级 `static AtomicBool`（`.data 0x143634d90` 的运行期来源 U，默认 false）。
- U Env5 的 `PlacementData` 缓冲由宿主 `placement_buffer` 提供（测试里 Rig 无该缓冲，断言 `PlacementDataSSBO` 文案）。

## 5. 对公共 API / 共享文件的改动
- `draw_lists.rs`：34 个 `gap::<ADDR>` → `super::draw_callbacks_b::<fn>`；`is_recovered()` 的 `matches!` 追加 34 项（与 d_cb_a 会在同一处追加，合并时注意）。
- `draw_passes/mod.rs`：追加一行 `pub mod draw_callbacks_b;`。
- 无公共签名变化；`ssbo_tags.rs` 被 i_infra 并发改过（`DrawIndexedIndirectTag` 现为 `indirect_params::DrawIndexedIndirectBuffer` 别名），本文件按别名使用，已在该状态下通过测试。

## 6. 验证
测试台：`/tmp/h_d_gap2` 副本（已删），加 `draw_callbacks_b`、`indirect_params`、`components/color_id/cursor_terrain_raycast/glade_settings/walls` 替身，`nani::{GpuWorld,NaniDrawExt}` 用与真源逐字相同签名的替身（真实 nani 需要 half/bytemuck/整个 components 树），`common_includes_compute` 去掉了依赖 `compute_passes::pass_kit` 的 test 段。
全树 `cargo test --offline --lib` → **628 passed; 0 failed**（含本文件 42 个；原有测试全部保持通过）。`draw_callbacks_b.rs` 已 `rustfmt --edition 2024 --config max_width=120`；无 `todo!/unimplemented!`。
覆盖：每个回调的调用序列（预设/磁盘/SSBO 顺序）、nani kind 值、`state` 位改写(4/8)、uniform 值与顺序（含 Env13/34 的砖擦除量）、`contains` 四种组合、缺资源/缺组件/缺实体 panic 文案、Env21 末张纹理选择、适配器取值失败路径、SSBO `SsboId == xxh3(槽名)`。
**未验证**：真实 crate 整体编译（并发中）；真实 `nani_draw`（测试台用同签名替身）；`nani_draw_world` 与真实 `GpuWorld` 的端到端（缺 `half/bytemuck` 离线依赖于该测试台）。

## 7. 遗留
`*View` 视图与真实类型的收敛；Env36 稀疏缓冲；`RenderGraph` 作为 `Resource`/宿主来源；A 侧（前一半）由 d_cb_a 完成。
