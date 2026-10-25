# B 阶段 d_gap：d_main 遗留的「只恢复结构」部分

范围：`systems/render_loop/draw_passes/` 下除 `draw_shaders/` 外全部文件 + `render/pass_builder/draw_list.rs`。O = 机器码观测，I = 推断，U = 未知。
未 commit。验证：`/tmp/h_d_gap2`（i_api 的全树 harness 副本 + 本范围 17 个文件 + `d_main_tests.rs`，真实 `draw_shaders`/`shader_library`/`Transform`）：
`L0H_REAL=$PWD/reconstruction/country-core/src L0H_WORK=/tmp/w_d_gap2 CARGO_TARGET_DIR=/tmp/l0h-target /tmp/h_d_gap2/run.sh` → **569 passed; 0 failed**（含本范围 138 个）。
本范围文件与 `draw_list.rs` 已 `rustfmt --edition 2024 --config max_width=120`；`grep todo!/unimplemented!` 为空（`draw_lists.rs` 里两处是对原版 `todo!("TODO(vulkan)")` 文案的复现）。

## 完成度一览

| 任务 | 状态 |
|---|---|
| 1a `final_post`（0x14090d000 + 闭包 0x1409ce780）资源接线 | **完成** |
| 1b `deferred_raster`（0x1409d20b0）资源接线 | **完成** |
| 1c `water_surface_pass` / `ice_surface_pass` 资源接线 | **未完成**（只补了机器码事实，见 §4）|
| 2 `build_draw_lists` 分发表 | **完成**（87 个名字，85 个叶子，全部由机器码模拟器核对）；60 个回调闭包本体**部分**恢复（§2.3）|
| 3 `DrawItem`/`DrawListItem` 统一 | **完成**（以机器码布局为准，保留 `draw_list.rs` 一份）|
| 4 `object_grass_push` 半径选择 / `ui_2d` 组件名 | **完成** |

## 1. `build_draw_lists`（0x1409ba540，14103 B）

### 1.1 查询与逐实体逻辑（O）
- 查询类型取自 PDB `QueryState::new` 实例名：`(Entity, &Handle<Vao>, &Handle<RasterShaderProgram>, &Transform, Option<&DrawTopology>, Option<&DesignatedRenderPass>, Option<&UiPass>, Option<&WireframePass>, Option<&DontCastShadow>, Option<&OcclusionCulled>)`。
  各 `Option` 的存在性 = 栈上 `{ptr @X, matches @X+8}` 对（0x1a8/0x1b8/0x1c8/0x1d8/0x1e8/…）。
- 着色器名：`AssetShaderLibrary::debug_raster_get_name`，`None` → `expect("Oops! This shader handle doesn't have a named shader")`。
- 分发：编译器展开的 `match name`（按长度分支 + 8 字节 `movabs` 异或 / 16 字节 `pcmpeqb` 窗口 / `bcmp`）。叶子写 `[rbp+0x380]`（kind dword）、`[rbp+0x384]`（variant dword）、`[rbp+0x388]`（回调指针）、`[rbp+0x390]`（排序键 i32）。
  **未命中** → `panic!("{name} doesn't have a draw function")`（draw_lists.rs:1321:21）。
- 目标列表（`0x1409bc717..0x1409bcd14`）：`DesignatedRenderPass(n)` → `[n]`（无阴影、无 OC/2）；否则 `UiPass` → `[19]`；否则 `WireframePass` → `[20]`；否则按 kind/variant 选主列表，
  `!DontCastShadow` 追加 `0`（kind 4 无阴影），`OcclusionCulled` 追加 OC/2 列表。详表见 `draw_lists.rs` 模块头 / `PassKind` 文档。
- 模板参考值 `Option<u8>`（跳转表 `0x142abf3f0`）：kind 0/4 → `None`；kind 1/2 → `Some(variant)`；kind 3 → `Some(variant|0x80)`；kind 5 → `Some(4)`。`variant` = `MaterialId` 数值（Solid0/Grass1/Plant2/Canopy3/Forward4）。
- 元素字段（`0x1409bc8f0..0x1409bc97c`）：`+0 Option<DrawTopology>`（tag u32，None=2；载荷 f32 @+4）、`+8 Entity`、`+0x10 shader 名 &str`、`+0x20 变换 0x30B`、`+0x50 *Handle<Vao>`、`+0x58 *Handle<Shader>`、`+0x60 排序键`、`+0x64/0x65 Option<u8>`、`+0x70 回调`。
- 21 个列表的完整下标表（含新增的 0 SHADOW、14 REFRACTION、15 WATER_SURFACE、16 PRE_TAA、17 POST_TAA）在 `list` 模块。**列表 15** 由水/冰表面 pass 读取（`+0x170/+0x178`）= `DesignatedRenderPass(0xf)` 的实体。

### 1.2 分发表获取方法（可复现）
`tools/country-core-audit/dg/{emu.py,run1.py,run2.py,run3.py}`：把 `dis.py 0x1409ba540` 的输出存为 `/tmp/bdl.txt`，用自写的小型 x86 子集模拟器（mov/lea/cmp/jcc/xor/or/and/pcmpeqb/pmovmskb/movdqu + `bcmp` 调用）
从 `0x1409bb30c` 起对候选名执行到 `0x1409bc717`，读叶子写入的四个字段；候选名由字符串池（`0x142abe0c0..` 的重叠 16 字节窗口、`0x142abe6a0..` 的 `bcmp` 池）还原，再用模拟器逐名验证（完全匹配才命中，所以不会误收）。
85 个叶子写点全部被命中（`distant_billboard_shader`/`tree_billboard_shader`、`ice_plane`/`water_plane` 各共用一个叶子）。

### 1.3 回调（`DrawFnId`，82 个不同的函数指针）
- 24 个是 `draw_shaders::*` 的直接函数，58 个是 `build_draw_lists::closure_env$N` 的 `FnOnce::call_once`（N = 0..60）。
- **已恢复本体（13 个）**：`closure_env$4`（`discard`）、`$35`（`DrawTopology` tag 1 → `state.byte0 = (b&0xfc)|2`、`blend = 载荷`，再 `draw`；用于 `vertex_color_shader`/`sparkles_shader`/`ui_2d`…）、`$42`（`draw`）、`$46`（`draw_floor_shader(b, false)`）、
  `$58`/`$59`（`todo!("TODO(vulkan)")`，draw_lists.rs:1294:25 / 1304:25）、`path_pebble_draw`、`draw_grass_shader`、`draw_grass_lod1_shader`、`lilypad_draw`、`lilypad_flower_draw`、`reed_draw`、`draw_instanced_butterfly_shader`。
- **typed gap（其余 69 个）**：`DrawFnId::func()` 返回 `gap::<ADDR>`（调用即 panic，文案含原地址）。每个变体的文档里列出了该闭包在机器码里的调用类别
  （如 `get_resource_id×3, bind_texture×5, read_storage_buffer×2, write_uniform×3, nani_draw`）。这些闭包要读 ECS 组件 / nani `GpuWorld` / `World` 资源，
  依赖 `World` 资源访问器与 nani `nani_draw(gw, lib, rg)` 的取值路径（A4 的 `NaniDrawExt::nani_draw` 需要显式 `&GpuWorld, &lib, &mut rg`，从 `World` 取的方式未定），不在本范围，**留给下一轮逐个恢复**。
  另：`draw_shaders::{stairs_draw, draw_bricks, ivy*, roof*, water_planes_draw, leaf_particle, tree_trunk/canopy, firefly*, duck, wren, rocky_terrain}` 的直接函数在 Rust 里签名带额外的数据参数（原版从 `EntityDrawParams.entity` + `World` 取），同样需要适配器。

## 2. `DrawItem` / `DrawListItem` 统一（O）
- 回调的 `&EntityDrawParams` 就是元素指针（`[rdx+8]` = 实体），故 `EntityDrawParams` 是 `DrawListItem` 的**首字段**（0x50 B：topology/entity/shader_name/transform）。
- `render/pass_builder/draw_list.rs` 现在是唯一定义：`DrawTopology`（`Component`；变体名 U，`Alt` = tag 1）、`EntityDrawParams`、`DrawListItem{params, vao, shader, sort_key:i32, stencil_ref}`、`DrawListFn`、`sort_draw_list`（`sort_by_key(i32)` 稳定）。
  `draw_lists.rs` 改为 `pub use … as DrawItem/DrawFn/DrawEntry`。旧 `DrawListItem{sort_key:u64, shader_id}` 与 `to_draw_packet` 的 `[0,0]` 头已按机器码改为 `[1, vao]`。
- 公共 API 变化：`DrawItem{transform, head, shader, sort_key, stencil_ref}` → `DrawListItem{params.transform, vao, …}`；回调参数 `&DrawItem` → `&EntityDrawParams`；
  `build_draw_lists(&[DrawListSource], &mut DrawLists)` → `build_draw_lists(&mut World, &dyn ShaderNames, &mut DrawLists)`（+ 纯函数 `push_row`/`select_lists`；`DrawListSource` 删除）。
- `Transform` 注意：机器码里 `Transform` 的内存布局是 `{rotation@0, translation@0x10, scale@0x1c}`（元素 `+0x20..` 整块 3×movaps 拷贝，`bind_common` 直接读），
  与 `utils/components/transform.rs` 文档里写的 `{translation@0, rotation@0xc, scale@0x1c}` 不一致（那份文档来自反编译字段推断）。本模块按字段转换（`draw_transform`），请该文件 owner 核对。

## 3. `deferred_raster`（0x1409d20b0，6203 B）—— 完整接线
- ABI 表在 `deferred_raster.rs` 模块头：`rcx` rg、`rdx` world、`r8` = `[Handle<Texture>;2]`（`[1]` = 颜色附件 tag1、`[0]` = 视口来源）、`r9` = 深度（tag0 可写、无清除）、栈参 5..13（逐个对应 binding hash）。
- 5 个条目 `ENTRIES = [(1,F),(3,F),(0,F),(0,T),(2,F)]` × 着色器数组（资源 `0x78cac557…`，`None` → `Option::unwrap` panic）；`flag` 决定 `byte3 = (b3&0x9f)|0x20` 与模板 `|0x80`；
  收尾状态 `byte0&=0x7f, byte1|=1, byte2=(b2&3)|0xe8, byte3=(b3&0xe0)|0x10`；`draw_no_vao(3)`。
- 绑定序列（33 步，逐条 hash/来源）见 `bind_all`；`Time` = `ctx.elapsed.as_secs_f32()`；Lantern uniform = `PassHost::ssbo_buffer_word(LanternSsbo)`；`bind_ray_tracing` 用 i_api 的 `DrawBuilder::bind_ray_tracing::<Tri,Meta,Bvh,Wall>`（本函数泛型）。
- 旧的 `DEFERRED_OPS`/`storage_buffer_reads` 已删除（被真实实现取代）。

## 4. `final_post`（0x14090d000 + 闭包 0x1409ce780）—— 完整接线
- 旧版 `FINAL_POST_OPS` **漏了** 动态名绑定（`format!("grading_lut:{}")` + `NameHash::compute`）与 `ShadowMaskMean` 缓冲读取；已更正。
- `FinalPostEnv`（0x98 B，字段偏移 = `rbp-0x40+k`）+ `final_blit_body`（闭包逐调用序列，12 次 `write_uniform` + `optional_uniform<[u32;2]>`）+ `build_env`（主函数打包逻辑）：
  缩放比分类（`ratio < 0.95 → 4`，`< 1.05 → 1`，否则 `2`）、`LutSelect{Custom/Keep/Default}`（模式 0/1/3，niche `i64::MIN`）、`GladeTheme == 0x13`、自定义 LUT 纹理 `Option` 等。
- 颜色附件 `ColorAttachment::Tag2`（i_api 已提供）+ `raster_single_object::<1>("Final blit", …)`。"msaa color out" 由调用方经 `PassHost::rg_resources().create_raw_texture` 创建（`msaa_color_desc` 给出描述：format 0x2b、sample_count 4）。
- 资源 TypeId 角色：`0x7e3bf548…` = `Option<Handle<Shader>>`；`0x4a142e09…` = 当前档配置数组（stride 0x40，选择字节 `+0xc0`）；`0xc2b8e941…/0x59f3c6d1…` = `GladeTheme`；`0x5f06b298…` = 自定义 LUT。
  **与既有文档的冲突**：`resources/image_formation.rs` 说每档 config 是 0xc0 B，机器码是 3 档 × 0x40 B（总 0xc0，`+0xc0` 为 kind 字节）；`final_post.rs` 旧文档把 TypeId 角色标反。请资源 owner 核对。

## 5. 水 / 冰表面（未完成）
未做的原因：两者混合了 `RenderGraph::create_raw_resource`（现有 `rg_resources()`）、`clear_rg_buffer`、`ComputePass`（需要 `ComputePassInit` 与 c2 风格的 `PassCx`）、列表驱动的 `bind_common` 绘制；接线需要逐个对照约 100 个调用点，本轮时间不够。
本轮补进 `transparency_pass.rs`（有测试）的机器码事实：
- 两者读取列表 **15**（`SURFACE_LIST`，d_main 遗留的“列表 15 无人读取”已解决）；
- `water_surface_pass` 栈布局：`rcx` rg、`rdx` world、`r8` 颜色、`r9` 深度（视口 = `extent_2d(r9+4)`）、`[0xc10]` `&Handle<Texture>`（决定射线缓冲大小）、`[0xc20]` u8（`bind_common` 标志）、`[0xc38]` `&DrawLists`；
- 缓冲：`water_rt_count_buffer`（`new_gpu_only(0x10, 0x20)`，`clear_rg_buffer`）、`water_rt_ray_buffer`（`size = (n + (n>>4)) << 4`，`n = w*h` 32 位）；
- 计算 pass：`Water deferred rt args` `/water/calc_deferred_rt_args.cs.hlsl`、`Water deferred rt` `/water/deferred_rt.cs.hlsl`（`dispatch_indirect_raw`）；
- 冰：13 个 RG 资源/pass 名与 4 个计算 pass（`Ice refl march/trace`、`Ice refr/refl resolve` + URI）。
- 旧计划表里“冰 pass 的 RG 资源名顺序是 I”现在有字符串证据（顺序仍按池里出现序）。
`WATER_OPS`/`ICE_OPS`/`surface_plan_for` 保持不变（d_main 的测试仍引用）。

## 6. `object_grass_push` 半径选择 / `ui_2d` 组件名
- `GladeTheme` 首 u32 `t`（与 `compute_passes::grass::GladeThemeId` 同一资源）查 14 项跳转表（`0x142ac2dd8`）：t=6 → `sub==0 ? 1.5 : 0`；7/8 → `P.f0 + 6.0`；9 → `sub!=3 ? 7.5 : 0`；10 → `7.5`；11..16/范围外 → `0`；
  17 → `P.f8*0.75 + 6.0`；18 → `1.5`；19 → `CameraMatrices[+0x4d0]==3 ? 9.0 : 0`。半径 > 0 才产生额外项：`camera_mode==3` → `Vec4(eye_position, r)`，否则光标 `Vec4(hit ? pos : 0, r)`；
  查询项 = `Vec4(translation, ObjectGrassPush)`（`ObjectGrassPush` 是单个 f32，`compute_passes/grass.rs` 里写成 `Gap`，请 owner 改）。`push_radius` / `assemble_push_objects` 有测试。
- `ui_2d_pass` 查询（PDB）= `(Ui2dExtent, Ui2dImage, Option<CustomizationIcon>, Option<UiInteractionState>, Option<UiDrawOrder>, Option<UiInactive>)`；`Ui2dTint`→`CustomizationIconRead`、
  `Ui2dButton`→`UiInteractionStateRead`、`order`=`UiDrawOrder`、`extra`→`inactive`（`UiInactive` 存在性，uniform `U_INACTIVE`，旧名 `U_BUTTON_EXTRA` 删除）。组件字段真名仍 U。

## 7. 与 i_api 的对齐（已按其最新源码适配）
- `CommonDrawParams.flag: CommonDrawFlag`：`CommonDrawParams::new(Duration, impl Into<CommonDrawFlag>)`；`ItemMode::Shadow` 不再覆盖写 uniform，改为 `params.flag = from_byte(cascade+2)`；`BIND_COMMON_FLAG_INT_UNIFORM` 常量仍保留（测试引用）。
- `ColorAttachment::Tag2` 已由 i_api 提供：`final_post` 使用；`d_main_support::raster_pass` 里对 `ColorKind::Tag2` 的“建完再补丁”仍在，**待对齐**：可改为直接传 `ColorAttachment::Tag2` 后删补丁。
- `DrawBuilder::bind_ray_tracing<Tri,Meta,Bvh,Wall>`、`PassHost::ssbo_buffer_word`、`PassHost::rg_resources()` 已被 deferred/final_post 使用。
- 仍待 i_api：`RenderGraph::create_raw_resource` 的 `TextureDesc::new_2d(...).sample_count(..)` 在 `rg_resources` 里是 `TextureDesc`（rhapsody_types）而本模块用 `AttachmentTexture` 表示结果；调用方需要转换。

## 8. 需要集成者 / 其它 owner 处理
1. `lib.rs` 的 `consume::` 再导出若引用 `draw_lists::DrawData`、`DrawListSource`、`Ui2dTint`、`Ui2dButton`、`DEFERRED_OPS`、`FINAL_POST_OPS` 需删（本轮 grep 无外部引用）。
2. `components/drawable.rs` / `components/mod.rs` 若要成为组件的唯一定义处：`DrawTopology`（在 `draw_list.rs`）、`DesignatedRenderPass`/`UiPass`/`WireframePass`/`DontCastShadow`/`OcclusionCulled`（在 `draw_lists.rs`，均 `#[derive(Component)]`）。
3. `AssetShaderLibrary` 不是 `Resource`：`build_draw_lists` 通过 `ShaderNames` 取名（`AssetShaderLibrary` 已实现）。
4. 核对 `Transform` 布局（§2）、`ImageFormationConfigs` 结构（§4）、`ObjectGrassPush` 类型（§6）。
5. `tools/country-core-audit/dg/` 里的脚本依赖 `/tmp/bdl.txt` 与 `dis.py`，只作证据留存。

## 9. 遗留
1. 水/冰表面资源接线（§5）。
2. 58 个 `draw_lists` 闭包中尚未恢复的本体（共 69 个 gap 指针）（§1.3）：先做 nani_draw 小闭包（`$28/$25/$24/$49/$39/$9…`，各 76–474 B），需要 `World`→`GpuWorld/lib/rg` 取值路径。
3. `DrawTopology` 变体名、`GladeTheme[+4]`、`P`（TypeId -0x63e22d59…）资源的字段语义、`CustomizationIcon`/`UiInteractionState` 字段真名。
