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
| P3 整合 + 完整工程 | todo | shader 源码树完整；编译/校验管线 + 与 rhapsody/shader-pipeline 工程接线 |
| P2 大算法族重建 | todo | nani cull、GI/ReSTIR、水/冰 RT、DoF/AA、grass、nani raster（双人+review） |
| P3 整合 + 完整工程 | todo | shader 源码树 + 编译/校验管线并入工程 |

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
