# Tiny Glade 原作 HLSL 源码恢复 — 主 agent 进度追踪

日期：2026-10-13。目标：从 B 发行 404 个 `.bin`（SPIR-V）恢复 `shaders/` 源码树并构建完整可编译工程。
数据源：`/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/compiled-shaders/`（404 .bin，只读 oracle）。
主线：骨架 → 逐族 HLSL 重建 → dxc 重编译结构指纹校验 → 大算法 shader 双人独立重建 + 交叉 review → 主 agent 居中判定/整合。

规则（沿用项目约束）：
- Observation/Inference/Unknown 分开标注；禁止编造名字/路径/哈希。
- 原始 .bin 与 evidence 只读；重建输出只写 `reconstruction/shader-source/` 与 `evidence/render-passes/recovery/`。
- 每个 agent 独立输出目录；交付必须是"重建的 .hlsl + 校验结果 + 变更记录"，不把"跑了工具"当交付。
- 校验锚点：dxc 重编译后 SPIR-V 结构指纹（entry/stage/localsize/bindings/opcode 直方图 + 归一化哈希）。注意 dxc `-Zi -Qembed_debug` 会嵌入源码路径/临时路径，字节哈希不可直接比对。
- A/C 版本未分析；本追踪仅 B。

## 阶段状态

| 阶段 | 状态 | 说明 |
|---|---|---|
| P0 骨架 + 验证管线 | complete | wave0：skeleton（413 占位 + include 图）+ pipeline（dxc v1.10.2605.37、spirv-cross、recompile_check 指纹校验） |
| P1a 共享 inc 层 | in_progress | wave1a：INC-A 主干 / INC-B 噪声数学 / INC-C 光照效果（并行） |
| P1b 小/中族重建 | done | wave1b：F-E/F-D/F-T/F-W/F-V 完成，F-R 部分+子 agent 补齐 |
| P1c 整合裁决 + inc 修复 | done | wave1c：13 项 inc 修复应用并验证；F-R 15 bin 补验 12 硬过+3 f0ad；260 main 编译 125 PASS/135 wave-2 占位 FAIL |
| P2 大算法族重建 | done（重建+review+合并） | wave2a 11 重建收齐；4 组 review 裁决（water-ice=A、gi-deferred=B、dof-aa=混合、nani-cull=B）；4 个合并全落：water-ice 22/22、gi-deferred 22/22、dof-aa 22/26（4 为 dxc 版本非失败）、nani-cull 44/44（含 -D 变体） |
| P2c 最终整合 | done | wave2c：4 孤儿重建、Affine3Packed 幂等守卫修复（2 nani main 复验硬过）、全树编译 258/260、硬门 235/260、404/404 bin 覆盖 |
| P3a 保真度门槛 + 批量提升 | done | 2026-10-13 `sweep_all.py`（指令量比 + opcode 直方图距离）+ `batch_lift.py`（spirv-cross 提升替换低保真手写）；638 模块中 637 硬门通过且比值 ≥ 0.8，唯一例外 final_post f0ad ps（dxc DCE，已注明）；见 integration/fidelity-sweep.md |
| P3b 与 rhapsody/shader-pipeline 工程接线 | todo | 编译/校验管线并入工程 |

## 族 ↔ agent 归属

| 族（源路径） | wave | owner | 状态 | 校验 | 判定记录 |
|---|---|---|---|---|---|
| nani cull/bucket + draw lists + wall | P2 | nani-cull-{a,b} | 运行中 | 硬门+指纹 | — |
| gi/ssgi/light_binning/restir/deferred_light_sun_sky | P2 | gi-deferred-{a,b} | 运行中 | 同上 | — |
| water/ice surface + deferred RT | P2 | water-ice-{a,b} | 运行中 | 同上 | — |
| dof + taa/tgsr/fsr | P2 | dof-aa-{a,b} | 运行中 | 同上 | — |
| nani raster main + main_raster_simple | P2 | nani-raster | 运行中 | 同上 | — |
| grass + clearing/billboard | P2 | grass-clearing | 运行中 | 同上 | — |
| final_post + painterly/vermeer | P2 | final-post-painterly | done（13 文件；16/16 模块硬门过；仅直方图残差+系统性 OpString 漂移） | 硬门+指纹 | 见判定记录 inc 清单 |
| inc/ 主干（common/utils/math/math_const/hash/pack_unpack/samplers/frame_constants/consts/current_view/projection/uv/halfres/texture_utils/atomics/debug_utils/tracked_buffer/image/color/bicubic/bilinear/upsample/quad_mesh/texture_blend/parallax_mapping/stochastic_tiling） | P1a | INC-A | done（32 文件，编译自检 PASS；frame_constants 偏移 0..240 与 _apply_screen_shadows 结构逐字节一致） | 消费族指纹后门待验 | 函数名推断；consts/color{xyz,luv,kohlrausch}/lanczos2 体为 U（标准实现）；tracked_buffer/debug_utils 近无操作 |
| inc/ 噪声随机（blue_noise/hammersley/gaussian_rand/perlin/simplex_noise/fbm_noise/morton/hsv/sh/smooth_max/curve/find_root/scatter） | P1a | INC-B | done（13 文件，编译自检 PASS，22 常量存活核对） | 消费族指纹后门待验 | 函数名全部推断（dxc 内联丢 OpName）；sh band-2、scatter 邻居/slerp、curve 体为 Unknown |
| inc/ 光照效果（lighting/lighting_indirect/brdf/brdf_lut/shadow/cascades/lanterns/reservoir/rt/raymarch/wind/foliage_exclusion/lod/frustum/history_valid/unjitter_taa/coc/grading_lut/prefix_scan/bitonic_sort/draw_indirect*/sun.inc/terrain_heightmap.inc/rocky_terrain.inc/grass_blade_data/highlight_color/color_library/wall_color_*/window_color_customization/roof_colors_include/material_eval/material/） | P1a | INC-C | done（42 文件，编译自检 PASS） | 消费族指纹后门待验 | 函数名系推断（inc 全内联，OpName 被丢）；BVH 子节点解码、material 参数结构、color-id→palette 为 Unknown |
| blit/、skydome/cloud/star、ui_2d、custom_cursor、dashed_line、timer_circle、screenshot、session_selection/logo、lut_gallery、fxaa、capture_panic_screenshot | P1b | F-E | done（32 文件，53/53 绑定+入口+localsize 匹配，52/53 capabilities，4/53 全指纹；resolve_to_swapchain.cs 归属 F-E） | 绑定+入口+localsize 必配，opcode 指纹收敛 | fxaa StorageImageExtendedFormats 系 dxc 版本差异 |
| depth/reprojection/hzb/shadow/volumetrics/mips/prefix_scan/lut/blur/rdi/ssgi | P1b | F-D | done（31 源/35 模块；绑定+入口+localsize 35/35 匹配，1/35 全指纹；ssgi/ 含 XeGTAO.hlsl+vaShared.hlsl） | 绑定+入口+localsize 必配，opcode 指纹收敛 | RDA/码差系 dxc 版本差异 |
| terrain_editing/buildings/read/upload/terrain_lighting | P1b | F-T | done（21 文件；17/18 绑定+入口+localsize+capabilities 匹配，2/18 全指纹） | 绑定+入口+localsize 必配，opcode 指纹收敛 | floor ps 绑定差异；见判定记录 inc 清单 |
| rocky_terrain/paths/stairs | P1b | F-R | 部分（15 文件已写，6/15 bin 已校验；9 bin 待补验） | 绑定+入口+localsize 必配，opcode 指纹收敛 | 子 agent 产物已落盘但报告未生成；整合阶段统一补验 |
| water masks/spawn + garden | P1b | F-W | done（18 文件；18/18 绑定+入口+localsize+capabilities 匹配，1/20 全指纹 PASS，19 行为等价残差） | 绑定+入口+localsize 必配，opcode 指纹收敛 | 见判定记录 inc 修复清单 |
| critters/roof/leaf/particles/ivy/lilypad/reed | P1b | F-V | done（34 文件；42/42 raster + 3/3 compute 绑定+入口+localsize 匹配，0 编译失败） | 绑定+入口+localsize 必配，opcode 指纹收敛 | RDA 能力系 dxc 版本差异；见判定记录 |

## 交叉 review 协议（大算法 shader）

1. 两名 agent 独立重建同一族（各自输出目录），以 SPIR-V 结构指纹 + 帧/资源上下文为共同 oracle。
2. reviewer agent 对比两版 HLSL 与目标指纹，列出：分歧点、每一版的可信证据、哪个更接近原结构。
3. 主 agent 居中判定：采用/合并/重做；分歧记入 RECOVERY_TRACKER 判定记录。
4. 判定标准：结构指纹一致优先；名字/文件划分次之；禁止为凑指纹改语义。

## 判定记录

- 2026-10-13 [resolved] **FrexpStructType 冲突**：真名只出现在 taa/tgsr×4/water_ice_refl_resolve（curve.hlsl float3/int3 用途），history_valid 的消费者 bin 无此名 → 原版 history_valid.hlsl 未定义它。裁决：保留 curve.hlsl 的 `FrexpStructType{float3 mantissa; int3 exponent}`；history_valid.hlsl 改为自包含 `FrexpScalarType` + `incb_frexp_scalar`（不依赖 curve.hlsl 的宏顺序）。已用 taa 风格同 TU（curve+history_valid）dxc 编译验证 PASS。
- 2026-10-13 [resolved] **hash(uint2) 重复定义风险**：INC-A `hash.hlsl` 只有 `hash(uint)`；INC-B `perlin.hlsl` 自带 `hash(uint2)`（include 顺序证据表明 perlin 引用 hash.hlsl）。当前签名不同 → 合法重载，无冲突；若以后 hash.hlsl 加 uint2 需归一到 hash.hlsl。
- 2026-10-13 [resolved] **dxc frexp 工作区**：dxc 1.10.2605.37 对 SPIR-V 的原生 frexp 损坏，curve.hlsl 用 `#define frexp incb_frexp` 绕过——工具限制非原作；不影响指纹（重编译用同 dxc）。
- 2026-10-13 [open] **inc include 漂移**（F-R 报）：`inc/tracked_buffer.hlsl→smooth_max.hlsl`、`inc/math.hlsl→bicubic/quad_mesh/unjitter_taa(+image/curve)` 不在原 include 闭包内，消费方凭空多出 OpSource/OpString/OpLine → 直方图不收敛。裁决方向：删除多余 include（INC 层原闭包为准），待 INC owner 确认后执行。
- 2026-10-13 [open] **StairsCounter 布局**（F-R 报）：`inc/draw_indirect_readonly.hlsl` 的 `StairsCounter{uint,pad*3}` 与原 bin 不符（bin 嵌入 `{int count}`，stride 4）。裁决方向：按 bin 证据改为 `{int count}`（INC-C owner 修）。
- 2026-10-13 [open] **dxc 版本码差**（F-R 报）：原构建 generator 16ceb560 vs 本机 1.10.2605.37——Load(int3(p,0)) 多出 CompositeConstruct/Extract/VectorShuffle/Bitcast，do-while(false)/switch/phi 展平；StructuredBuffer 需 `[[vk::ext_capability(5302)]]+[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]` 才匹配 RuntimeDescriptorArray，R16f 存储图需 `RWTexture2D<half>`。裁决方向：接受工具差异，记录为重建约定；若需严格匹配再找与原版匹配的 dxc 版本。
- 2026-10-13 [resolved] **Affine3Packed 重复定义**（wave-2c）：`inc/utils.hlsl` 与 `wall/inc/draw_lists.hlsl` 定义相同结构，draw_lists 先被包含时 utils 再定义 → 重定义错误。裁决：两处改用幂等守卫 `TINYGLADE_AFFINE3PACKED`；复验 nani_solid_vertex_color / nani_velocity_only 编译通过、硬门 PASS（仅 opcode 直方图残差）。
- 2026-10-13 [resolved] **孤儿文件清单**：`mei_shadow.raster.hlsl`、`instanced_roof_beams.raster.hlsl`、`instanced_roof_spire.raster.hlsl`、`water/water_object_push.inc.hlsl` 由 wave-2c 补齐并硬门验证；`nani/inc/user.hlsl` 由 nani-cull 合并补齐。
- 2026-10-13 [open] **inc 修复清单**（F-W 报，F-R 交叉确认）：
  1. `frame_constants.hlsl` 缺"死"无界数组声明 → 不发 RuntimeDescriptorArray+SPV_EXT_descriptor_indexing（307/307 bin 相关；F-W 本地用 `_rda_trigger[]` 复现）。裁决：在 frame_constants 加触发声明（不占新绑定），使能力集匹配。
  2. `current_view.hlsl` 把 current_view_index 声明成文件级 static，原版嵌入 $Globals member 0（set0/b13）→ 破坏 b13 绑定。裁决：改回非 static 文件级全局（进 $Globals）。
  3. `find_root.hlsl` 调 `terrain_height()`，`terrain_heightmap.inc.hlsl` 未定义。裁决：按 use-site 证据在 terrain_heightmap.inc 补 `terrain_height`（INC-C owner）。
  4. include 漂移汇总（consts→math_const、tracked_buffer→smooth_max、pack_unpack→bilinear、math→bicubic/quad_mesh/unjitter_taa）：与原闭包不符 → 按 include-graph 原闭包删多余 include。
  5. water_mask_sampler/path_mask_sampler/building_mask 采样器 inc 仍是骨架（属主未填）→ 整合阶段指定属主补齐。
  6. （F-E 增补）`$Globals` 绑定仅能经显式 `cbuffer $Globals : register(bNN,space0)` 复现；`inc/current_view.hlsl` 的普通全局会落到 b0 → 与 #2 同源，统一按 $Globals 显式声明修。
  7. （F-E 增补）`inc/color/srgb.hlsl` if/return 被 dxc 降成 OpSwitch，原版用 if/else 赋值 → 按原版写法改（INC-A owner）。
  8. （F-E 增补）`inc/blue_noise.hlsl` 的 blue_noise_texture 未带 register(t14)，text.raster 需重声明 → 补 register（INC-B owner）。
  9. （F-T 增补）传递 include 漂移（pack_unpack→bilinear→history_valid 亦在链中）——系统性直方图 FAIL 主源 → 按 include-graph 原闭包统一修剪 inc 文件 include 列表（INC owner 联合执行）。
  10. （F-T 增补）`inc/color_library.hlsl` cbuffer 名为 ColorLibraryData，原版为 `color_library` → 按原版名修（INC-C owner）。
  11. （F-V 增补）`inc/samplers.hlsl` 缺 s5..s8（原版 13 个 sampler）→ 补全（INC-A owner）。
  12. （F-V 增补）`inc/highlight_color.hlsl` 引用 color_library 但未 include → 补 include 或文档化包含顺序（INC-C owner）。
  13. （F-V 增补）`inc/ray_cone.hlsl`、`inc/wall_distortion.hlsl` 仍是空骨架 → 按 use-site 证据补（INC-A/C owner）。
  14. （F-V 增补）roof_material_id/wooden_floor/stone_floor 材质 dispatch 体、nani 物理寻址 helper 体（stairs_floor/plaster/wooden_plank/smoke）——wave-2 nani 耦合，待 wave2 闭合。
  15. （F-D 增补）sampler register 冲突：samplers.hlsl 把 sampler_lnc 钉在 s0，rev_blur 实际绑 s3（shader 自声明）→ 核对各 sampler 实际 register（INC-A owner）。
  16. （F-D 增补）history_valid.hlsl 的函数名 `history_valid` 与 $Globals 成员 history_valid 撞名 → 函数改名（INC-C owner）。
  17. （final-post 增补）`inc/blue_noise.hlsl` 的 blue_noise_texture 固定 register(t14) 与 final_post 的 t18 冲突 → 从 blue_noise.hlsl 移除全局声明，消费方按各自 register 重声明（INC-B owner）。
  18. （final-post 增补）`inc/coc.hlsl` 的 `static const float gaussian_weights[16]` 与 painterly 的 `gaussian_weights[4]` 撞名 → 局部遮蔽已绕过，记录待归一（INC-C owner）。
  19. （final-post 增补）`inc/bicubic.hlsl` 是 Catmull-Rom，final_post 实际用非 Catmull 5-tap 核 → 按 final_post 内联证据保留（INC-A owner 记录差异）。
  20. （final-post 增补）`inc/grading_lut.hlsl` 缺 color/srgb.hlsl include 边 → 补（INC-C owner）。

- 2026-10-13 [resolved] **保真度门槛**：先前的"硬门"只比绑定/入口/localsize/capabilities，编造的实现也能通过。新增指令量比与 opcode 直方图距离；扫描发现 42 个模块比值 < 0.8（deferred 光照 ×4、nani cull/raster、wall_brick、terrain stroke 等均为空壳或编造）。统一用 spirv-cross 提升 oracle bin 替换（行为精确，名字为 SPIR-V id）；物理寻址模块（PhysicalStorageBuffer64）先改 Logical 再提升，指针解引用改写为 vk::RawBufferLoad。
- 2026-10-13 [resolved] **变体宏**：提升块以 `TGV_<id>` 保护（见 fidelity-sweep.md）；`variant_defines.json` 仅保留旧手写变体宏。
- 2026-10-13 [open] **可读性**：提升代码的局部名为 SPIR-V id，待回填命名；inc 层 24 个文件不再被任何 main 引用（推断内容，见 fidelity-sweep.md）。
- 2026-10-13 [resolved] **指令量门槛不足以证明算法**：差分执行发现 22 个手写 compute 输出与原作不同（见 validation-pipeline.md「差分执行」），已提升替换，复跑 0 DIFF。
- 2026-10-13 [open] **raster 无执行级验证**：468 个 VS/PS 模块仍只有结构指纹；需 PS（常量 varying 全屏绘制）与 VS（SPIR-V 改写输出到 SSBO）的差分运行器。compute 侧另有 14 SKIP / 12 VACUOUS / 4 ERROR 未覆盖。
- 2026-10-13 [resolved] **PS 执行级验证**：见 validation-pipeline.md「画面程序差分执行」；5 个手写 PS 文件差异已提升。ice_plane 仍有输入伪影待关注。VS（234 模块）尚无执行级验证。
- 2026-10-13 [open] **差分执行终稿**：638 模块中 488 SAME；1 DIFF（light_binning/bin_lights_to_cells n_14dca8ba…）、8 EDGE（ice_plane×2、lilypad×2、reed×2、dof/analyze、path_pebbles）待人工判定；16 个（SKIP/CRASH/ERROR）因工具/lavapipe 限制未覆盖；125 个原版为无输出空壳。
- 2026-10-13 [resolved] **差分执行收尾**：638 模块 505 SAME / 125 空壳 / 7 EDGE / 1 DIFF；EDGE 与 DIFF 已逐个判定（见 validation-pipeline.md「最终结论」）。6 个 SAME 在正常数据档全部崩溃，仅边界档验证。dof/analyze 的差异来自 dxc 1.9 的 DerivativeGroupQuads 模式，非源码问题。

- 覆盖导向输入搜索（compute）：9 个"输入没触发写入"模块中 8 个已验证 SAME（5 个手写版先被查出 DIFF 并改为提升版），1 个（spawn_waterwheels）为真空壳。详见 validation/validation-pipeline.md。

- [PS discard 验证] 给像素差分加深度附件后，34 个"只有 discard"的 PS 全部可观察；其中 6 个手写 PS 的 discard 逻辑是错的，已改为 lifted。PS 现为 232 SAME / 2 VACUOUS（真空壳）。

## 最终总账（合并 spv-diff 后全量重跑）
- 结构门禁 637/638（唯一例外 final_post f0ad，结果空壳 stub，dxc 会 DCE）。
- 像素 234：SAME 232 / VACUOUS 2（painterly/debug_target，原版空壳）；含深度附件，34 个 discard-only 全部可观察。
- 顶点 234：SAME 230 / EDGE 4（lilypad/reed 的 FMA 融合，profile 14）。
- 计算 170（输入 0–63；marching_squares 到 400）：SAME 161 / VACUOUS 1（spawn_waterwheels 空壳）/ DIFF 8。
  新增混合 profile 暴露 6 个手写 DIFF（spawn_bushes_garden、spawn_flowers_garden、terrain_lighting_texture、deferred_rt、ice_deferred_rt、ice_refr_resolve），已换成提升版并复测 SAME。
  剩余 8 个 DIFF 均为已提升（行为精确）的模块：bin_lights_to_cells n_14dca8ba、path_pebbles、taa、tgsr×4、ice_refl_resolve；差异量极小（几十个字、两边写入量接近），疑似浮点融合/NaN/越界等 UB，尚未逐个证明。

## 8 个差异调查结果（计算，输入 0–63）
结论：8 个里 7 个已变 SAME，1 个（ice_refl_resolve）定为 EDGE（仅驱动 FMA 融合差异）；仍 DIFF 为 0。计算总计 170：SAME 168 / EDGE 1 / VACUOUS 1。

| 模块 | 原因 | 证据 | 处置 | 判定 |
|---|---|---|---|---|
| taa、tgsr×4、ice_refl_resolve(部分) | **提升缺陷**：`incb_frexp` 辅助函数不处理非规格化数（denormal），原版 GLSL Frexp 会归一化 | 原版有 1 个 OpExtInst FrexpStruct，重建版用位运算模拟；补上 `x*2^24` 归一化后 taa/tgsr 的 0–63 号输入全部一致 | 修 batch_lift.py `FREXP_HELPER`，同步改 taa、tgsr(4 份)、ice_refl_resolve 里的拷贝 | taa SAME，tgsr×4 SAME |
| path_pebbles | **提升缺陷**：spirv-cross 把 OpSRem 展开成 `a - b*(a/b)`，除数为 0 时触发整数除零(UB)，原版是真正的取余 | 原版有 2 个 OpSRem，重建版 0 个、多 1 个 SDiv；改为 `%` 后（dxc 产出 OpSRem）0–63 号全部一致 | batch_lift.py 新增 `fix_srem`（仅非字面量除数），path_pebbles.cs.hlsl 已应用 | SAME |
| bin_lights_to_cells n_14dca8ba | **dxc 重结合浮点**：`(6a)*(6a)` 被 LLVM 重排成 `36*(a*a)`，球-格子边界判断翻转 1 次（多写 4 个字） | 重建 SPIR-V 出现 `FMul 36, r²` 而原版只有 `r*r`；给 `_130`、`_223` 加 `precise` 后两个变体 0–63 号全部一致；NoContraction 测试证明与 FMA 无关 | batch_lift.py `fix_lift_bugs` 对 `c*asfloat(532487669…)` 及其平方加 `precise`，文件已重生成 | SAME（两个变体） |
| water/ice_refl_resolve（剩余） | **驱动 FMA 融合**：输出对舍入极敏感（光线步进+阈值） | 原版自身加/不加 NoContraction 就有 3/11/15 个字不同；原版注入 NoContraction、重建版加 `precise` 后 profile 20/24/25/29 **逐位一致** | 保持原样（不加 precise，避免偏离原版融合行为）；diff_common.PROVEN_FMA 把它的 DIFF 记为 EDGE | EDGE |

改动文件：tools/render-passes/{batch_lift.py,diff_common.py,diff_exec.py}；shaders/{taa,tgsr,path_pebbles}.cs.hlsl、water/ice_refl_resolve.cs.hlsl、light_binning/bin_lights_to_cells.cs.hlsl；integration/diff-exec.json。结构门禁仍 637/638。

## 5 个 EDGE 的逐值证明（任务 D）
- 新增 `tools/render-passes/spv_nocontract.py`（给 SPIR-V 全部浮点算术结果注入 NoContraction）与 `diff_common.strict_variant / fma_recheck / apply_fma_verdict`；
  `diff_exec*.py` 对每个 DIFF/EDGE 自动复核（`--no-fma-check` 可关）：重建版用 `dxc -Gis`（严格 IEEE，去 float_controls 模式）+ NoContraction，原版也注入 NoContraction，
  逐位一致才算"已证明 FMA/重结合"（保持 EDGE，记 `fma_proven`）；不一致则升为 DIFF。硬编码的 PROVEN_FMA 名单已删除。
  负例：把 ice_refl_resolve 活路径上的常量 8.656→8.7，复核结果 DIFF（strict 差异 168 字）。
- lilypad / reed 顶点着色器（4 个变体）：**原先"FMA 融合"的推断是错的**。禁融合对照无变化，原版默认 vs 原版禁融合也无差异。
  真因：dxc 把 `(x*20)-(z*20)` 因式分解成 `(x-z)*20`（原版是 2 个 FMul，重建版 1 个），舍入不同，经哈希种子放大。
  修复：batch_lift 新增 `spv_fmsub`（precise 临时量保持两次乘法），两个文件刷新；0–15 号输入 4 个变体全部 SAME，结构门禁不变。
- water/ice_refl_resolve：保持 EDGE，但已逐值证明。重建版 -Gis + 两边 NoContraction：0–63 号输入 0 差异；对照 原版默认 vs 原版禁融合：29 字差异；
  重建版 -Gis 严格顺序但允许驱动融合 vs 原版默认：0 差异。即差别完全来自 dxc 默认编译对表达式的重结合 + lavapipe 乘加融合，算法相同。
  无法在 HLSL 层变 SAME（`precise` 同时禁止融合，原版默认执行里是被融合的）。
- 最新全量：像素 SAME 232/空壳 2；顶点 SAME 234；计算 SAME 168 / EDGE 1 / 空壳 1；结构门禁 637/638。
