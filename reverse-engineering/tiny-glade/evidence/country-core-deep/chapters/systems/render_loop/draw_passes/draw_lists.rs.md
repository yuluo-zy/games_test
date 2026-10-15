# systems/render_loop/draw_passes/draw_lists.rs

> 文件作用：CPU 绘制列表容器与构建：`DrawLists`（21 个 `Vec<DrawEntry>`，按图层索引 DRAW_LIST_COUNT=21，wp5 O）、`DrawLists::new`（0x1409ba340）、`build_draw_lists`（Bevy 系统：puffin scope + World 查询 + 按材质 shader 名分派构造 `DrawData::new*` 项）、`DrawLists::sort`（逐图层排序）。大量胶水为按材质族分派的 `DrawData::new*` 单态闭包。

函数总计 70：业务 3 / 内联无独立符号 6 / 派生胶水 61

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists` | 0x1409ba540 | 0 | Bevy 系统（`::f` 名 = UNK_142abe3f3，puffin scope 取函数名尾段）：World（param_2+0x38 资源表，hash 0x8597ee0727b461b3）→ 遍历实体/绘制项 + 依材质 shader 名串（nani_solid_albedo/vertex/velocity_only、gothic_window、window_bricks、wooden_gate、garden_stone、canopy_flowers、leaf_particle、duck/wren/butterfly/firefly、ivy_branch/ivy_leaf/ivy_flower 等）经逐字节 trie 比较分派到 draw 函数（tree_canopy 0x1409ca350、butterfly 0x1415547e0、ivy_flower 0x1408dc1a0、ivy_leaf 0x1408dc180、ivy_branch 0x1408dbc50、duck 0x141555080、wren 0x141555ec0 等）→ 构造 DrawData 项 push 列表 | O | render-loop\decomp\decomp-0x1409ba540.txt；wp5 draw-passes §1 |
| `country_core::systems::render_loop::draw_passes::draw_lists::DrawLists::new` | 0x1409ba340 | 117 | 结构初始化（NotFound 手解机器码）：+0 `{cap=0,ptr=self+0x10,len=0}`、+0x20=1、+0x28 内联区、+0x38=1、+0x40 清零。**与 reconstruction 的 `DrawLists::new=default(21 空 Vec)` 不一致**，疑为 PDB 名错位（PLAYBOOK 事实 7），交中央裁决 | I（机器码证实初始化模式；名映射存疑） | `rne-read.py --hex 0x1409ba340 32`：`48 89 c8 48 c7 01 00 00 00 00 48 c7 41 08 10 00 00 00 0f 57 c0 …` |
| `country_core::systems::render_loop::draw_passes::draw_lists::DrawLists::sort` | 0x1409ba4b0 | 128 | 逐图层排序：param_1 含 21 个 `{ptr@+8,len@+0x10}`（0x18 步进至 0x1f8）；len>1 时 len<0x15 走小表插入排序（FUN_1413396b0）否则 std `sort_by_key`（FUN_140b4c720） | O | render-loop\decomp\decomp-0x1409ba4b0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `build_draw_lists::DrawData::new`（行 181）— 通用绘制项构造（3 单态，`impl$7::default`，wp5）
- `DrawData::new_main_pass`（行 189）— 主 pass 项（~71 单态，按材质 shader 分派）
- `DrawData::new_late_forward`（行 204）— 延迟光照项（1 单态）
- `DrawData::new_main_with_velocity`（行 212）— 含速度主 pass 项（5 单态）
- `DrawData::new_velocity_only`（行 227）— 仅速度项（1 单态）
- `apply_topology`（行 1452）— `RasterPipelineOpts::set_topology`（wp5 O）

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default`、`fmt` 等 61 项（`DrawData::new*` 闭包族 closure$0..closure$60 行 297–1313：按材质族绑定 `bind_common`/`bind_wall_color_customization`/`bind_brick_erase_highlight`/`bind_stone_floor_textures`/`bind_wooden_floor_textures` 等）

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### 1. `build_draw_lists`（0x1409ba540，14103B）—— 分发表（d_gap §1 完整解码）
- **查询**（O，PDB QueryState::new 实例名）：`(Entity, &Handle<Vao>, &Handle<RasterShaderProgram>, &Transform, Option<&DrawTopology>, Option<&DesignatedRenderPass>, Option<&UiPass>, Option<&WireframePass>, Option<&DontCastShadow>, Option<&OcclusionCulled>)`；各 Option 存在性 = 栈上 `{ptr, matches}` 对。
- **着色器名**：`AssetShaderLibrary::debug_raster_get_name`，None → `expect("Oops! This shader handle doesn't have a named shader")`。
- **分派**：编译器展开的 `match name`（按长度分支 + 8B movabs 异或 / 16B pcmpeqb 窗口 / bcmp）。**87 个名字 / 85 个叶子**（`distant_billboard`/`tree_billboard`、`ice_plane`/`water_plane` 各共用一个叶子），用 x86 子集模拟器（tools/country-core-audit/dg/emu.py）逐名验证全部命中。叶子写 `[rbp+0x380]` kind dword、`[rbp+0x384]` variant dword、`[rbp+0x388]` 回调指针、`[rbp+0x390]` 排序键 i32。**未命中 → `panic!("{name} doesn't have a draw function")`**（draw_lists.rs:1321:21）。
- **目标列表**（0x1409bc717..0x1409bcd14）：`DesignatedRenderPass(n)` → `[n]`（无阴影、无 OC/2）；`UiPass` → `[19]`；`WireframePass` → `[20]`；否则按 kind/variant 选主列表，`!DontCastShadow` 追加 `[0]`（kind 4 无阴影），`OcclusionCulled` 追加 OC/2 列表。21 列表下标：0 SHADOW、1..6 main、7..11 OC/2、12/13 Late、14 REFRACTION、15 WATER_SURFACE、16 PRE_TAA、17 POST_TAA、18 LOADING、19 UI、20 WIREFRAME。
- **模板参考值**（跳转表 0x142abf3f0）：kind 0/4 → None；1/2 → Some(variant)；3 → Some(variant|0x80)；5 → Some(4)；variant = MaterialId 数值。
- **回调**：82 个不同函数指针（24 个 draw_shaders 直接函数 + 58 个 closure_env$N）；d_cb_a 恢复 35 个、d_cb_b 恢复 34 个（详见两 B 报告；每个回调 = 资源读取 → bind_common/额外绑定 → state 位改写 → draw/nani_draw/discard 终结）。

### 2. `DrawLists::sort`（0x1409ba4b0）
21 个 `{ptr@+8,len@+0x10}`（0x18 步进至 0x1f8）；len>1 时 **len<0x15 走小表插入排序（FUN_1413396b0）否则 std `sort_by_key`（FUN_140b4c720）**（driftsort，稳定，键 = i32@元素+0x60，见 draw_list 章）。
- **帧序 R2**：`DrawLists::new → build_draw_lists → DrawLists::sort`（render R2，CALL-CHAIN §2.1）。

### 3. 差异（待中央裁决）
`DrawLists::new`（0x1409ba340）机器码 = 小结构初始化（`{cap=0,ptr=self+0x10,len=0}`、+0x20=1、+0x38=1），与 reconstruction `default(21 空 Vec)` 不符——疑 PDB 名错位（PLAYBOOK 事实 7），render-loop DIGEST §5 同列。

## 关键事实

- 图层索引（wp5 O；reconstruction 行 90–104）：POST_TAA=17、LOADING_SCREEN=18、UI=19、WIREFRAME=20，共 21 层。
- dispatch 目标与 `draw_shaders/*` 一一对应（见上表）。
- 帧序 R2（frame-wiring）：`DrawLists::new → build_draw_lists → DrawLists::sort`。
- 差异：`DrawLists::new` 机器码与 reconstruction default() 不符 → 记 DIGEST"错位/纠错"节。
