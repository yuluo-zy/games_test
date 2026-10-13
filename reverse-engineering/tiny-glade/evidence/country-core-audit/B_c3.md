# B 阶段报告 c3：compute_passes 的 19 个薄 pass + `indirect_params`

标注：O = 机器码观测，I = 推断，U = 未知（未编造）。工作根：`reverse-engineering/tiny-glade`，源码在 `reconstruction/country-core/src/systems/render_loop/`。
证据来源：`tools/country-core-audit/dis.py` 反汇编 + `evidence/render-recovery/producer-closure/**/0x*.c`（既有 Ghidra 反编译，逐函数对照反汇编复核）；
`primary_rt_debug` 无现成反编译，纯反汇编恢复。

## 1. 结果摘要
- 19 个 pass 文件 + `indirect_params.rs` 全部清除 `todo!/unimplemented!`，每个文件有 `#[cfg(test)]`。**86 个测试，隔离测试台全绿**（测试台总计 384 通过 / 0 失败）。
- 新发现并用于全组：**绑定名哈希 = FNV-1a-64(名字)**（O：`bool_render_shadows`、`shadow_map`、`blue_noise_texture`、`depth_tex`、`color_tex`、`history_valid`、`half_depth_tex`、`output_tex` 8 个名字的 EXE 立即数逐位吻合，另有约 60 个名字在各文件测试里逐个断言）。
  纹理库键/SSBO 键仍是 `NameHash`(xxh3_64)。`0x579f39ab91a01865 = xxh3_64("blue_noise")`（旧文档写作 water_noise_texture，**应更正**）。
  离线复算了 SsboId：`PebblePlacement`、`MouseTerrainRaycast`、`AtomicCounterBuffer`、`DrawIndirectBuffer`、`DispatchIndirectBuffer`、`DrawIndexedIndirectBuffer`、`MouseDepthValue`、`LanternSsbo` 的立即数 = xxh3_64(NAME)（脚本见 §7）。
- 这些 pass 的真实形状：`ComputePass::new(name, uri)` → 一串 `uniform/bind_texture/*_storage_buffer` → `dispatch_*`。重建用 A4 `ComputePass` 真实 API 逐条还原，不再是 `Gap` 存根。

## 2. 函数 → Rust 项
| 地址 | PDB 过程 | Rust 项（`compute_passes/…`） | 要点（O，除注明） |
|---|---|---|---|
| 0x1408a44c0 | `apply_volumetrics` | `apply_volumetrics::apply_volumetrics` | 相机模式 ∈{4,1} → (density 0.075, size 10.0) 否则 (1,1)；shadow_map 预设 0x11、blue_noise、color_tex 写 0x32、depth_tex；`current_view_index=0`；`dispatch_2d(color extent)` |
| 0x14098bf60 | `inclusive_prefix_scan_u32_1m` | `prefix_scan::inclusive_prefix_scan_u32_1m` | 三段（"_Prefix 1/2/merge"）；`assert_eq!(n & 0x3ff, 0)`；子和缓冲 `max(blocks,1)` 补到 1024 的倍数 ×4 B；dispatch `n-n/2`、`blocks-blocks/2`、`n-n/2` |
| 0x1409beca0 | `primary_rt_debug` | `primary_rt_debug::primary_rt_debug` | 仅反汇编；output 写、预设 9/10/8、blue_noise、`bool_render_shadows=true`、shadow_map、`bind_ray_tracing`、`current_view_index=0`、`dispatch_2d` |
| 0x140a0d190 | `calculate_reprojection_map` | `reprojection::calculate_reprojection_map` | 新建 "reprojection map"（深度 extent、格式 0x5c）；"Reprojection map" pass（`history_valid` = movzx 字节）+ "Copy depth" pass（深度拷进历史）|
| 0x1408743b0 | `calculate_ssgi` | `ssgi::calculate_ssgi` | 三 pass（SSGI → upsample → temporal_filter）；注意第一遍 `depth_tex` 绑**半分辨率**深度 |
| 0x140a44cd0 | `eval_gi_screen_radiance` | `eval_gi_screen_radiance::eval_gi_screen_radiance` | 辐射度纹理宽 ×3（"Screen indirect radiance" 格式 0x61）；Eval / Reproject / Filter 三 pass；`allow_non_determinism = (state==0)`；两处写访问码 **0x37** |
| 0x1408f6180 | `extract_half_res_depth_normals` | `half_res::extract_half_res_depth_normals` | `div_up_extent(depth.desc,[2,2,1])`；"half-res depth"(0x64)/"half-res normals"(0x26)；一 pass |
| 0x14090a870 (+0x14169a830 闭包) | `reset_indirect_args` | `reset_indirect_args::reset_indirect_args` | 非 ComputePass：4 块暂存区 0x80/0x1e0/0x10/8（初值 8×[0,1,1,0] / **无写入** / [0,1,0,0] / 0x0009000000000000），"Reset indirect args" pass 4 个 `add_resource_access(id,0x34,0)`，闭包 4 次 `copy_buffer` |
| 0x140a6fec0 | `restir_sample_direct_lights` | `restir_di::restir_sample_direct_lights` | 灯笼数 0 → 仅 1×1 占位纹理 "RDI reservoirs placeholder"，不声明 pass；否则 temporal "RDI reservoirs" + "RDI temporal" pass |
| 0x14098f9f0 | `render_shader_refactoring` | `shader_refactoring::render_shader_refactoring` | 两张跳转表还原的状态机（见 §4）；capture / compare 两种 pass；写回 `ResMut` 的 changed tick |
| 0x1408e4cc0 | `spawn_vegetation_around_buildings` | `spawn_vegetation_around_buildings::…` | 门控字节必须 ==1；灌木(90×90)+花(150×150)两 pass；花 pass 里地形预设 9/10 **重复绑定两次**（机器码如此） |
| 0x140a2f2e0 | `run_visibility_queries` | `visibility_queries::run_visibility_queries` | "Firefly vis query"；`dispatch_1d(0x60)` |
| 0x140a01620 | `mouse_depth_compute` | `mouse_depth_value::mouse_depth_compute` | 三个 world 资源→ 光标 Vec2、`filter_box_y_frac` = clamp(NaN→0)、WindowSize→Vec2；`dispatch_1d(1)` |
| 0x140866540 | `read_bush_positions_for_cpu` | `read_bush_positions_for_cpu::…` | cpu-mapped 写 + 两个读 + `draw_indirect_buffer_offset=10`；`dispatch_1d(0x2581)` |
| 0x140a58450 | `highlight_active_terrain_stroke` | `highlight_active_terrain_stroke::…` | 模式 5/10 才取事件（新缓冲最后一项优先，否则旧缓冲）；抑制字节 @+0xf9 强制 -1；idx≠-1 时两遍 `gauss_blur_r16f`（水平→垂直，sigma=1.0，"blur temp" 256²/0x4c） |
| 0x1409768b0 | `mouse_read_terrain_stroke_index` | `mouse_read_terrain_stroke_index::…` | `tag&1` 选 x/z 位模式，8 字节 push constants；`dispatch_1d(1)` |
| 0x140958f70 | `path_pebbles_compute` | `path_pebbles::path_pebbles_compute` | 5 个纹理 + 3 个 i32 uniform（3/0x240/0x18）；`dispatch_1d(0x240)` |
| 0x140a32770 (+0x1412fc510) | `rasterize_buildings` | `rasterize_buildings::{upload_buildings, declare_passes, rasterize_buildings}` | `filter_map` 闭包（进度>0.05 / kind>1 丢弃，圆/矩形两种 32 B 元素）；≥5000 个 → `"Exceeded maximum number of buildings per level"`；未注册缓冲 → 静默跳过上传；4 个 pass |
| 0x140a01cf0 | `ray_trace_gi` | `ray_trace_gi::ray_trace_gi` | 两 pass（Trace GI candidates / Update GI radiance）+ 三个 temporal；update 的 `history_valid` 需三个标志位都为 1 |
| — | `indirect_params`（0 个函数） | `indirect_params.rs` | 4 个真 SSBO 类型：NAME/缓冲规格见 §3 |

## 3. `indirect_params`（O）
`new_storage_buffer<T>` 与 `get_rg_buffer<T>` 核对：缓冲字节 / 元素：`AtomicCounterBuffer` 8（8 B×1）、`DrawIndirectBuffer` 0x10（16×1）、`DispatchIndirectBuffer` 0x80（16×8，I）、
`DrawIndexedIndirectBuffer` 0x1e0（20×24）；用途 0x122、对齐 4、内存类型 1；SsboId = xxh3_64(类型名)。`DrawIndirectBucket/DrawIndexedIndirectBucket` 在 PDB 里没有任何库实例（O），保持旧占位（U）。
本文件还声明 `pub mod pass_env;`（见 §5）。

## 4. 关键观测 / 判断
1. **门控与分支全部保留**：spawn_veg 门控字节、ssgi/ray_trace_gi 的 temporal 标志、restir 灯笼数、highlight 模式与抑制、rasterize 的 5000 上限与"缓冲未注册则跳过"、
   reset 的暂存区长度不符 panic、prefix_scan 的元素数断言、各处 `Result::unwrap` / `Option::unwrap` 站点。
2. **shader_refactoring 状态机**（`ResMut<ShaderRefactoringState>`，布局 `{u32 disc, u32 payload}`，TypeId 0xc95ed87f413070ab/0xe67d7e3df983f8ce）：
   - 第一张跳转表（0x142ab9ff0）：disc 0→直接返回（不碰 tick）；1→kind 2；2→`payload&3==3 ? kind0 : 2`；3/4→`payload&3==3 ? kind1 : 2`；5→`unreachable!()`；6..9 并入别的分支（U，按 no-op）。
   - 第二张跳转表（0x142aba004）：1→{2,0}；2→`payload<4 ? {2,payload+1} : {3,0}`；3/4→`payload+1`（并把 feedback 纹理 desc 尾字拷进 frame 句柄 `+0x20`，U，只返回标志）。
   - kind 2 无 pass；kind 0 = capture 一遍；kind 1 = capture 到临时纹理后 compare。
3. `eval_gi_screen_radiance` 的 `ShaderRefactoringState` 判别式只取 `== 0`（`allow_non_determinism`）。
4. `rasterize_buildings` 元素 32 B：`kind==1`→`[w3c,w38,v34.0,v34.1,w44,w40,1,0]`；`kind==0`→`[w44,w40,w3c,w3c,0,0,0,0]`；字段名全部 U（只有偏移）。
5. `reset_indirect_args` 的 0x1e0 B 暂存区在机器码里**没有任何写入**（U：暂存区原内容，24×20 B DrawIndexedIndirectCommand）；重建里按分配返回值原样拷贝。

## 5. 共享/新增文件（均在我拥有的范围内）
- 新增 `indirect_params/pass_env.rs`（+`pass_env/test_support.rs`，仅测试）：`binding_hash`(FNV-1a)、`binding::*` 常量、`PassEnv`（世界/后端/uniform 池/图工厂打包）、`RgFactory`/`RgTextureSpec`/`TemporalPair`
  （`create_raw_resource`、`get_or_create_temporal_pair` 的接缝）、`FromDisk`（见 A4 缺口）、`BLUE_NOISE`。放在 `indirect_params` 下是因为**不能改 `compute_passes/mod.rs`/`lib.rs`**。
- **未改** `lib.rs`、`mod.rs`、`rhapsody_types.rs`、`rhapsody_contract`、`compute_passes/mod.rs`（所有 19 个模块原本就已声明）。
- 提示集成者：c1 的 `compute_passes/pass_kit.rs`（也叫 `PassEnv`/`binding_hash`/`RgCreator`）与 c2 的 `c2_support.rs`（`PassCx`/`PassFrame`/`bind_garden_exclusion`）与本组 `pass_env` **三份同构基础设施**，
  应合并成一份。`spawn_vegetation_around_buildings.rs` 内联了一份 `bind_garden_exclusion<ComputePass>`（预设 0xe/0x13/5/0x10，O 0x14160e630），与 c2 的同名函数等价、与 `draw_shaders/common_includes.rs` 里的 `unimplemented!` 桩重复。

## 6. 对公共 API 的改动（旧 → 新，原因）
均无外部调用者（grep 确认）。
- 全部 pass 函数签名从 `(world: &mut Gap, …)` 改为 `(env: &mut PassEnv, 具体输入…) -> RecordedComputePass / 结构体`；world 资源查找（128 位 TypeId、`unwrap_failed`）由调用者在接缝外完成，
  每个文件导出对应 `*_TYPE_ID: TypeId128` 常量（O）。原因：原签名无法表达输入输出句柄。
- `highlight_active_terrain_stroke::AddTerrainHighlightCmd(pub Gap)` → `(pub i32)`（O：事件载荷 i32 @+8）。
- `path_pebbles::PebblePlacementSsbo(pub Gap)` → `(pub [u32;12])`（48 B，O）；`rasterize_buildings::GroundedBuildingsSsbo(pub Gap)` → `(pub [u32;8])`（32 B，O）；
  `read_bush_positions_for_cpu::BushCpuSsbo`、`mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo` 仍 `(pub Gap)`（布局 U），均实现 A4 的 `SsboType`/`StorageElement`。
- `indirect_params` 四个类型 `(pub Gap)` → `(pub [u32;N])` 真元素类型并实现 `SsboType`；**与 `resources::render_ssbo` / `consume::buffers` 里同名旧占位是两份类型**，
  集成者应把后者改为 `pub use crate::systems::render_loop::indirect_params::*`（`grass_shader`、`gpu_generate_draw_lists`、`ssbo_tags` 等现用旧占位）。

## 7. Stub / 接缝清单（不发明行为）
- 世界资源：`ResourceLookup`（TypeId 查找，A4 已是 typed gap）；各 pass 的世界输入以参数形式给出，TypeId 常量 O。
- `RgFactory`：`RenderGraph::create_raw_resource`、`get_or_create_temporal[_pair]`（rhapsody，外部库）。shader_refactoring 里原版是**单个** `Temporal<Texture>`，用 `temporal_pair().current` 近似。
- `reset_indirect_args`：`IndirectResetGraph`（在 `rg_helpers::rg_boundary::RenderGraph` 上补"多资源访问登记"与导入图）；复用 `RenderPassApi::copy_buffer`/`StagingAlloc`/`CopyRegion`。
- 泛型占位的别模块 SSBO（其 `SsboType` 尚未恢复，NAME U）：`FireflySsbo`、`FireflyVisibilitySsbo`、`BushSsbo`、`FlowerSsbo`、`LanternSsbo`、`MouseDepthValueSsbo`。
  SsboId 立即数（O）：Firefly 0x29d4a64d5ea39871、FireflyVisibility 0x9248897309ea4fea、Bush 0x87df9c07cbe7e324、Flower 0xb9559015b7cea7e6（= xxh3("FlowerSsbo")）、
  Lantern 0x8b2da1ef1bff9619（= xxh3("LanternSsbo")）、MouseDepth 0x75bd131453883cb7（= xxh3("MouseDepthValue")）。
- 离线 SsboId 复算：`evidence/country-core-audit/c3_tools/xxh3.py`（另有 `mk.sh` 测试台生成脚本、`ann.py` 反编译常量标注脚本）（用 EXE 里的 XXH3 secret 0x142b2ebb0；已用 `Cobblestone3RasterStagingSsbo` 与 `ColorLibrary` 自检）。可并入 `tools/country-core-audit`（我没改 tools）。

## 8. A4 缺口（需 A4/集成者处理）
1. `bind_texture<TextureFromDisk>`（0x1409056d0）：A4 报告已列 U。`pass_env::FromDisk` 临时实现 `ComputeTexture`（取库下标当资源号，多登记一条 RG 访问），**偏差**，原版不进 RG 访问表。涉及 apply_volumetrics、ray_trace_gi、eval_gi、restir_di、primary_rt_debug。
2. `half_res` 的 `stencil_tex` 用 `read_texture_view`（`TextureViewDesc`，机器码 `bd57a0/bd58a0(…,4)`），A4 `bind_texture` 不带视图描述 → 按普通深度绑定，aspect=4 记常量。
3. `BufferDesc::new_gpu_only` / `TextureDesc::new_2d/storage/sampled` / `div_up_extent`：`rhapsody_types::TextureDesc` 没有格式字段，用 `RgTextureSpec.format` 保留 O 格式码（0x4c/0x61/0x65/0x25/0x26/0x5c/0x64/0x6d/0x7a/9）。
4. `bind_texture` 的第 5 个实参（值 2，I：视图种类）A4 未建模。
5. `MappedStorageBuffer<'static,_>` 需要 `MappedBuffers` 设备，`dyn GraphicsDevice` 给不出：bush_cpu/mouse_terrain 缓冲退化为 `TypedStorageBuffer`（`BUFFER_LEN=1`），U。
6. `ComputePass::write_storage_buffer_concurrent` 的访问码 A4 里是 0x32（原版 `write_storage_buffer_concurrent` 无实例，I）；测试只断言"是写、同步组=实参"。

## 9. U / 未恢复
- 全部 world 资源的**类型名**（TypeId 128 位无法反查）：MainCamera 以外的光标/焦点/WindowSize/模式资源等只给 TypeId 与读取偏移。
- `0xf3aa86819743c1d8`、`0x633d7e34d2dd23f1`（spawn_veg 的 Flower 写缓冲绑定名）、`0xfb539f7243dbb832`（`[u32;2]` uniform 名）、`0xecf59a3a8eeaedc6`（prefix_scan 子和写缓冲名）不在重建 HLSL 标识符里，名字 U。
- `BushCpuSsbo`/`GroundedBuildingsSsbo` 的 `T::NAME`（试过多种候选与 xxh3 均不吻合）；`BushCpuSsbo`/`MouseTerrainRaycastSsbo` 元素布局。
- shader_refactoring：disc 6..=9 的归并分支；状态 3/4 的 `frame.desc +0x20` 拷贝语义；`rendered_tex` 等对拍纹理的用途位。
- 部分 `RenderGraph::create_raw_resource` 的 `from(desc)` 用途位；ssgi 第三张 `ssgi_temporal` 的 extent 来源（I：全分辨率）。

## 10. 验证
```
# 测试台：复制 /tmp/h_a4_compute（A4 的测试台）→ /tmp/h_c3，用符号链接把 indirect_params.rs + pass_env 重排成 mod-rs 形状，#[path] 挂载 19 个 pass，
# 并移除对他人在改文件的挂载（render/mesh_atlas、nani、resources::render）；xxh3 占位里只让 "blue_noise" 返回真实值。
/tmp/c3/mk.sh            # C3_MODS=<逗号分隔模块名>
test result: ok. 384 passed; 0 failed        # 其中本组 86 个（19 个 pass 文件 81 + indirect_params 3 + pass_env 2）
```
- `rustfmt --edition 2024 --config max_width=120` 已应用于全部 21 个文件。
- 真实 crate 无法离线 `cargo check`（缺 dolly 等），语法/类型由测试台编译覆盖（挂载的是真实源文件）。
- 测试覆盖：每个 pass 的名字/URI/入口（含 `:capture` 入口）、访问码 0x19/0x32/0x37/0x1a、uniform 取值、dispatch 维度、所有分支（门控、边界 4999/5000、NaN、模式 4/6/9/11、状态机转移）、`unwrap` panic 文案、哈希立即数。
