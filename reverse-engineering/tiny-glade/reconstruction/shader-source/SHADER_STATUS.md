# 着色器重建状态总表

由 `tools/render-passes/build_status_table.py` 从证据 JSON 汇总生成，勿手改。数据来源：`evidence/render-passes/integration/{fidelity-sweep,diff-exec,diff-exec-ps,diff-exec-vs}.json`。

## 总览

- HLSL 文件 415 个：头文件 155，手写 108，提升 122，混合 30
- 模块(bin×入口) 638 个，结构门禁通过 637；执行验证：EDGE 1，SAME 634，空壳 3

## 列说明

- **实现**：提升 = 由 spirv-cross 从原版 SPIR-V 提升（`batch_lift.py`，行为与原版一致，变量名是 SPIR-V 编号）；手写 = 人工重建，已通过执行验证；混合 = 一个文件里部分变体提升、部分手写；头文件 = 被包含的公共代码。
- **门禁**：结构门禁（入口/local_size/绑定一致，指令比 ≥ 0.8）。
- **执行**：在 lavapipe 上用同一批输入跑原版与重建版并比较所有输出。SAME 一致；EDGE 仅在极端输入不同（疑似 UB/浮点融合）；DIFF 正常输入也不同；空壳 = 原版几乎无指令；-= 未跑执行验证。
- 输入：计算 0–63 号（marching_squares 到 400），像素 0–15 号（含深度附件观察 discard），顶点 0–15 号。

## (include)

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `buildings/building_mask.inc.hlsl` | include-graph direct edge -> inc/foliage_exclusion.hlsl; consumed by rasterize_buildings / gauss_blur_building_mask / garden spawners; rasterize_build… | 头文件 | — |  |
| `buildings/curved_halftimber.inc.hlsl` | include-graph direct edge -> inc/material/solid_material.inc.hlsl; consumed by nani_curved_halftimber raster + nani cull/bucket (wave-2 family); curve… | 头文件 | — |  |
| `clearing/canopy_vert.inc.hlsl` | Evidence: embedded in the tree_canopy bins. Shared canopy vertex helpers used by tree_canopy.raster.hlsl (growth-scaled canopy placement and the canop… | 头文件 | — |  |
| `clearing/inc/canopy_common.inc.hlsl` | Evidence: embedded in 6 bins (tree_canopy, tree_trunk, deferred canopy light sun sky x2). Shared tree-canopy helpers: growth age from the placement an… | 头文件 | — |  |
| `clearing/inc/forest_floor.hlsl` | Evidence: embedded in the terrain_editing floor raster bins (2 variants). Forest-floor shading helpers (ground cover / leaf-litter blend). The terrain… | 头文件 | — |  |
| `clearing/inc/instanced_placement.hlsl` | Evidence: embedded in 10 bins (instanced_background_tree, meadow_flower, meadow_plant, tree_canopy, tree_trunk x2 variants). Provides the 48-byte Plac… | 头文件 | — |  |
| `deferred/light_sun_sky.inc.hlsl` | MERGE (gi-deferred review 2026-10-13): adopted candidate B (near-exact include closure; B's helpers match the bin). NOTE (review delta rejected): the … | 头文件 | — |  |
| `deferred/light_sun_sky_prelude.inc.hlsl` | MERGE (gi-deferred review 2026-10-13): adopted candidate B (include-umbrella shape matches the target: prelude included early, right after common, pul… | 头文件 | — |  |
| `dof/multires.inc.hlsl` | The ring-gather consumers inline a half2 pack/unpack (f32tof16 x2 / <<16 and the reverse) used to store the CoC ring taps in the groupshared ring_samp… | 头文件 | — |  |
| `dof/tiles.inc.hlsl` | Tile-list bookkeeping shared by the tile-based DoF passes. DofTileOffsets     - 8 uint offsets into tile_coords_buffer, one per dispatch group (analyz… | 头文件 | — |  |
| `dof/upsample_halfres.inc.hlsl` | fullres/fullres_upsample bins inline an 8-texel half-res gather: fullres_pixel/2 + spatial_resolve_offsets[quadrant*16+i].xy, averaged *1/8, then the … | 头文件 | — |  |
| `fullscreen_triangle_vert.inc.hlsl` | Fullscreen triangle vertex shader used by blit / resolve / deferred fullscreen passes. No vertex buffer: three vertices are synthesized from SV_Vertex… | 头文件 | — |  |
| `fxaa.inc.hlsl` | FXAA 3.11-style antialiasing core operating on the caller-provided texture. | 头文件 | — |  |
| `garden/defs.inc.hlsl` | raw-per-bin debug type/OpMemberName (_garden_spawn_* + _garden_read_bush_positions_for_cpu + _nani_bush/_nani_cull_and_bucket_*): BushXform{Affine3Pac… | 头文件 | — |  |
| `garden/placement_utils.inc.hlsl` | inlined use-sites in _garden_spawn_* + _water_spawn_* (world grid uv: ((float2(id.xy)+0.5)/grid_dims-0.5)*130; placement heightmap uv: world*0.0076923… | 头文件 | — |  |
| `gi/gi_settings.inc.hlsl` | _gi_* consumer debug names); I (settings constants) MERGE (gi-deferred review 2026-10-13): content adopted from candidate A (hi_px_subpixels pinned by… | 头文件 | — |  |
| `gi/restir.inc.hlsl` | _gi_update/_gi_eval disassembly); I (helper names) MERGE (gi-deferred review 2026-10-13): content adopted from candidate A (packing helpers; B kept th… | 头文件 | — |  |
| `grass/grass_common.inc.hlsl` | Evidence: embedded in 8 grass compute bins (generate_grass, bundles_prefix, calc_dispatch_args, calc_grasslet_bundle_ranges, calc_tiles, tile_bundle_c… | 头文件 | — |  |
| `inc/atomics.hlsl` | Atomic counter helpers for append/allocator buffers. | 头文件 | — |  |
| `inc/bicubic.hlsl` | Bicubic (Catmull-Rom) interpolation for texture sampling. | 头文件 | — |  |
| `inc/bilinear.hlsl` | 2x2 bilinear texel neighborhood helper used by explicit-LOD gather paths. | 头文件 | — |  |
| `inc/bitonic_sort.hlsl` | spirv-cross (_dof_autofocus/_read_mouse_depth_value) ascending 64-element bitonic sort over groupshared uint (k/j loop, pair indices a=m+i, b=j==k/2?(… | 头文件 | — |  |
| `inc/blue_noise.hlsl` | MERGE (water-ice review inc-fix #3, RECOVERY_TRACKER #17): the blue_noise_texture global declaration and its fixed register(t14) were removed so consu… | 头文件 | — |  |
| `inc/brdf.hlsl` | spirv-cross (_deferred_solid_light_sun_sky PS) exact opcodes — GGX D=alphaSq/(PI*(NdotH^2*(alphaSq-1)+1)^2) with alphaSq=roughness^4, Hammon correlate… | 头文件 | — |  |
| `inc/brdf_lut.hlsl` | spirv-cross (_deferred_*_light_sun_sky PS) brdf_fg_lut.SampleLevel((float2(NdotV, roughness^2)*0.984375+0.0078125),0).xy -> (F,G); include-graph edge … | 头文件 | — |  |
| `inc/cascades.hlsl` | spirv-cross (_apply_screen_shadows/_terrain_lighting_texture) cascade selection against frame_constants.view_constants[2..4] (slice=0/1/2 via world_to… | 头文件 | — |  |
| `inc/coc.hlsl` | MERGE (dof-aa review inc-fixes #1/#2, 2026-10-13): ring_samples + gaussian_weights removed (the bins embed ring_samples once per gather consumer as th… | 头文件 | — |  |
| `inc/color/helmholtz_kohlrausch.hlsl` | Helmholtz-Kohlrausch effect: saturated colors appear brighter than their luminance. Operates in CIELUV, adding a chroma-proportional lightness boost. | 头文件 | — |  |
| `inc/color/luv.hlsl` | CIE L*u*v* (CIELUV) conversion with D65 white point. | 头文件 | — |  |
| `inc/color/math.hlsl` | Small color-math helpers shared by the color conversion headers. | 头文件 | — |  |
| `inc/color/srgb.hlsl` | sRGB <-> linear conversion (per-channel and vector forms). | 头文件 | — |  |
| `inc/color/xyz.hlsl` | sRGB <-> CIE XYZ (D65) conversion. Input/output assumed linear sRGB. | 头文件 | — |  |
| `inc/color/ycbcr.hlsl` | Rec.709 YCbCr conversion with chroma in [-0.5, 0.5] (used by temporal accumulation shaders to keep luma/chroma separated). | 头文件 | — |  |
| `inc/color_library.hlsl` | raw-per-bin OpMemberName ColorLibrary 37 float4 (temp_color_1..night_tint) exactly as listed; cbuffer variable `color_library` bound at per-consumer s… | 头文件 | — |  |
| `inc/common.hlsl` | Umbrella header included by nearly every shader. Pulls in the shared sampler set; additional shared helpers belong in their specific headers. | 头文件 | — |  |
| `inc/consts.hlsl` | Shared game constants. Content is provisional: named constants are not recoverable from compiled SPIR-V (they fold into literals). Keep additions here… | 头文件 | — |  |
| `inc/current_view.hlsl` | Selects the active camera/view out of frame_constants.view_constants[5]. `current_view_index` is a shader-global (lands in the per-shader $Globals UBO… | 头文件 | — |  |
| `inc/curve.hlsl` | Observation (identical in _taa and _tgsr): FrexpStructType is a float3/int3 mantissa/exponent pair; frexp is C-style (x == mantissa * 2^exponent, mant… | 头文件 | — |  |
| `inc/debug_utils.hlsl` | Debug visualization helpers. Nearly always no-op'd at codegen (only one consumer produces code), but kept as a small utility surface. | 头文件 | — |  |
| `inc/draw_indirect.hlsl` | raw-per-bin (_generate_grass/_wall_generate_draw_lists) writes DrawIndexedIndirectCommand.instanceCount via InterlockedAdd((offset+cmd)*20+4); include… | 头文件 | — |  |
| `inc/draw_indirect_readonly.hlsl` | raw-per-bin (_garden_read_bush_positions_for_cpu/_rocky_terrain_stairs_stairs_counter) DrawIndexedIndirectCommand + StairsCounter names; include-graph… | 头文件 | — |  |
| `inc/draw_indirect_types.hlsl` | raw-per-bin (_generate_grass/_wall_generate_draw_lists/_garden_*/_nani_draw_indexed_indirect_to_dispatch) OpMemberName DrawIndexedIndirectCommand{coun… | 头文件 | — |  |
| `inc/fbm_noise.hlsl` | Observation: 2-octave fractal of perlin() with a fixed 36.87-deg rotation matrix (cos=0.8, sin=0.6) and frequency 1.9.  Function name not embedded -> … | 头文件 | — |  |
| `inc/find_root.hlsl` | Observation (from _rocky_terrain_stairs_stairs inlined use-site, called with the segment corners _149[0]/_149[3] and a target stair height): fa = heig… | 头文件 | — |  |
| `inc/foliage_exclusion.hlsl` | consumers _clearing_meadow_plant/_generate_grass/_buildings_building_mask sample foliage_exclusion_mask (rectangular mask of cleared/sculpted regions)… | 头文件 | — |  |
| `inc/frame_constants.hlsl` | Per-frame UBO, bound at set1/b0, named `frame_constants`. Members are declared flat (global-scope cbuffer members), so consumers reference e.g. `view_… | 头文件 | — |  |
| `inc/frustum.hlsl` | consumers _nani_cull_and_bucket/_generate_grass/_grass_calc_tiles cull against the main view; spirv-cross unavailable for the PhysicalStorageBufferAdd… | 头文件 | — |  |
| `inc/gaussian_rand.hlsl` | Observation (from _generate_grass inlined use-site, two independent hash seeds): u   = (rand01(hash(seed_b)) * 2) - 1                 // [-1,1] L   = … | 头文件 | — |  |
| `inc/grading_lut.hlsl` | raw-per-bin (_final_post/_lut_gallery_thumbnail) grading_lut_tex/icc_lut/tony_mc_mapface_lut (Texture3D) + shadow_mask_mean read in final grading; exa… | 头文件 | — |  |
| `inc/grass_blade_data.hlsl` | raw-per-bin + spirv-cross (_generate_grass/_grass.raster/_grass_lod1.raster) GrassBladeDataPacked{position,color,packed0,packed1,packed2,pad} 32B; byt… | 头文件 | — |  |
| `inc/halfres.hlsl` | Full-resolution <-> half-resolution coordinate helpers. | 头文件 | — |  |
| `inc/hammersley.hlsl` | Observation (from _lut_brdf_fg inlined use-site): u1 = float(i)/float(N)  (loop used sqrt(float(i)*0.0009765625) with N=1024) u2 = radicalInverse_VdC(… | 头文件 | — |  |
| `inc/hash.hlsl` | Integer + float hash helpers used for deterministic per-instance / per-texel randomness. `hash` is the classic 5-step integer hash; `rand01` bitcasts … | 头文件 | — |  |
| `inc/highlight_color.hlsl` | raw-per-bin (_deferred_*/_gi_*/_nani_*_bricks) uses color_library temp_color_1/2/3 + frame_constants.glade_theme_id for editing/highlight colors; exac… | 头文件 | — |  |
| `inc/history_valid.hlsl` | (_apply_screen_shadows/_calculate_reprojection_map/_ssgi_temporal_filter) scale-invariant temporal-history gating; history_valid.hlsl was NOT observed… | 头文件 | — |  |
| `inc/hsv.hlsl` | Observation (from _nani_cloth ps inlined use-site): rgb2hsv: p = lerp(c.zy,K.wz / c.yz,K.xy, step(c.z,c.y)); q = lerp(p.xyw,c.x / c.x,p.yzx, step(p.x,… | 头文件 | — |  |
| `inc/image.hlsl` | Image/texture helpers for common mask and border operations. | 头文件 | — |  |
| `inc/lanterns.hlsl` | raw-per-bin OpMemberName LanternPacked{position, intensity__color_id} 16B (lantern_buf stride 16: position@0, packed uint@12 = half16 intensity low / … | 头文件 | — |  |
| `inc/lighting.hlsl` | spirv-cross (_deferred_*_light_sun_sky/_billboard_*/_clearing_*) sun diffuse+specular = BRDF(NdotL,NdotV,NdotH,HdotL) * sun_color * NdotL * shadow, sk… | 头文件 | — |  |
| `inc/lighting_indirect.hlsl` | FIX (gi-deferred merge 2026-10-13, review inc-fix): the `#include "lanterns.hlsl"` edge is spurious — the original closure embeds lighting_indirect.hl… | 头文件 | — |  |
| `inc/lod.hlsl` | raw-per-bin (_nani_cull_and_bucket/_grass_calc_tiles) struct{first_index,index_count,bounding_sphere_radius,pad0}; consumers select between grass/stat… | 头文件 | — |  |
| `inc/material/canopy_material.inc.hlsl` | include-graph edge material/canopy_material.inc.hlsl -> material_eval.inc.hlsl; consumers _clearing_tree_canopy/_deferred_canopy sample canopy_color/c… | 头文件 | — |  |
| `inc/material/eval/canopy_material.inc.hlsl` | raw embedded path style "eval/../canopy_material.inc.hlsl" -> includes ../canopy_material.inc.hlsl; consumers _deferred_canopy evaluate tree crowns; e… | 头文件 | — |  |
| `inc/material/eval/grass_material.inc.hlsl` | raw embedded path style "eval/../grass_material.inc.hlsl" -> includes ../grass_material.inc.hlsl; consumers _deferred_grass/_terrain_lighting_texture … | 头文件 | — |  |
| `inc/material/eval/plant_material.inc.hlsl` | raw embedded path style "eval/../plant_material.inc.hlsl" -> includes ../plant_material.inc.hlsl; consumers _deferred_plant evaluate foliage/flowers w… | 头文件 | — |  |
| `inc/material/eval/solid_material.inc.hlsl` | raw embedded path style "eval/../solid_material.inc.hlsl" -> includes ../solid_material.inc.hlsl; consumers _deferred_*/_gi_*/_primary_rt_debug evalua… | 头文件 | — |  |
| `inc/material/grass_material.inc.hlsl` | include-graph edges material/grass_material.inc.hlsl -> material_eval.inc.hlsl and -> raymarch.hlsl (raw embedded style "eval/../grass_material.inc.hl… | 头文件 | — |  |
| `inc/material/plant_material.inc.hlsl` | include-graph edges material/plant_material.inc.hlsl -> material_eval.inc.hlsl and -> raymarch.hlsl (raw embedded style "eval/../plant_material.inc.hl… | 头文件 | — |  |
| `inc/material/solid_material.inc.hlsl` | include-graph edge material/solid_material.inc.hlsl -> brdf_lut.hlsl (raw embedded style "eval/../solid_material.inc.hlsl"); consumers _nani_*_bricks/… | 头文件 | — |  |
| `inc/material_eval.inc.hlsl` | include-graph edge material_eval.inc.hlsl -> brdf.hlsl; consumers _deferred_*/_billboard_*/_clearing_* decode gbuffer normal/roughness + BRDF lighting… | 头文件 | — |  |
| `inc/math.hlsl` | Shared math helpers (safe normalization, squared lengths, clamping and small geometric utilities). Uses HLSL intrinsics where available. | 头文件 | — |  |
| `inc/math_const.hlsl` | Standard mathematical constants. All are compile-time constants; dxc folds them into literals at use sites, so names are not recoverable from SPIR-V. | 头文件 | — |  |
| `inc/morton.hlsl` | Observation (from _dof_analyze inlined use-site, tiling a 8x8 workgroup by local index): x = m & 0x55555555u; x = (x ^ (x>>1)) & 0x33333333u; x = (x ^… | 头文件 | — |  |
| `inc/pack_unpack.hlsl` | 8-bit color/normal pack and unpack helpers for gbuffer / baked-color buffers. | 头文件 | — |  |
| `inc/parallax_mapping.hlsl` | Steep parallax mapping helper (depth-map-based UV offset). | 头文件 | — |  |
| `inc/perlin.hlsl` | MERGE (water-ice review inc-fix #4): perlin() rewritten to the explicit-arithmetic bilinear blend (multiply-add of the four lattice values, not lerp()… | 头文件 | — |  |
| `inc/prefix_scan.hlsl` | spirv-cross (_light_binning_cell_prefix_sum/_nani_histogram_prefix_sum/_grass_tiles_prefix) Hillis-Steele inclusive scan over groupshared uint (step s… | 头文件 | — |  |
| `inc/projection.hlsl` | Projection matrix helpers (perspective matrix construction and view-projection utilities). | 头文件 | — |  |
| `inc/quad_mesh.hlsl` | Fullscreen / unit quad geometry helpers driven by SV_VertexID. | 头文件 | — |  |
| `inc/ray_cone.hlsl` | behavior-consistent body, filled in P1c integration pass, adjudicated inc-fix #13). Used by water / GI reflection-cone and reprojection consumers (307… | 头文件 | — |  |
| `inc/raymarch.hlsl` | include-graph edges material/{grass,plant}_material -> raymarch.hlsl; consumers _deferred_*/_dof_*/_water_* march view rays against depth/height field… | 头文件 | — |  |
| `inc/reservoir.hlsl` | raw-per-bin OpMemberName Reservoir1spp{w_sum,payload,M,W} 16B + Reservoir1sppStreamState{p_q_sel,M_sum} 8B; spirv-cross (_deferred_*_light_sun_sky/_rd… | 头文件 | — |  |
| `inc/rocky_terrain.inc.hlsl` | consumers _clearing_*/_generate_grass/_path_pebbles/_rocky_terrain_displace sample rocky_terrain texture (rock displacement) alongside terrain_heightm… | 头文件 | — |  |
| `inc/roof_colors_include.hlsl` | consumers _nani_gothic_window_bricks/_nani_instanced_roof/_nani_solid_vertex_color_bricks/_wall_wall_brick_lod* read roof_color_ids + roof_colors (pal… | 头文件 | — |  |
| `inc/rt.hlsl` | MERGE (water-ice review): trace_software_bvh rewritten as the morton-coded uint2-stack traversal transcribed from the embedded _water_deferred_rt/_wat… | 头文件 | — |  |
| `inc/samplers.hlsl` | Static sampler set shared by all shaders. Each shader binds only the samplers it actually uses. Name convention: three letters = min-filter / mag-filt… | 头文件 | — |  |
| `inc/scatter.hlsl` | Observation: scatter_grid_dim is a $Globals cbuffer member declared by the consuming shaders; scatter.hlsl references it as a global (not declared her… | 头文件 | — |  |
| `inc/sh.hlsl` | Observation (from _gi_eval_screen_radiance inlined use-site): float4(0.2820949852466583251953125f, ray_dir.xyz * 0.48860299587249755859375f)  == Y00, … | 头文件 | — |  |
| `inc/shadow.hlsl` | spirv-cross (_apply_screen_shadows/_terrain_lighting_texture/_critters_*) PCSS shadow: cascade select -> 5-sample blocker search (shadow_map.SampleLev… | 头文件 | — |  |
| `inc/simplex_noise.hlsl` | Observation (from _generate_grass theme-4 inlined use-site): the Ashima/WebGL-noise 2D simplex "snoise" verbatim opcode pattern: i = floor(v + dot(v,C… | 头文件 | — |  |
| `inc/smooth_max.hlsl` | The single consumer only shows max(); the smooth_max body below is the standard IQ smax (max(a,b) + h^2/(4k), h = max(k-/a-b/,0)).  Function name not … | 头文件 | — |  |
| `inc/stochastic_tiling.hlsl` | Stochastic texture tiling: hash-driven tile selection so a repeating texture does not show visible periodicity. | 头文件 | — |  |
| `inc/sun.inc.hlsl` | raw-per-bin + spirv-cross consumers read frame_constants.sun_color/sky_low_color/sky_high_color and color_library day/dusk pairs for atmospheric light… | 头文件 | — |  |
| `inc/terrain_heightmap.inc.hlsl` | MERGE (water-ice review inc-fix): terrain_height() references a consumer-declared `terrain_heightmap` global texture. _water_plane_refraction_copy emb… | 头文件 | — |  |
| `inc/texture_blend.hlsl` | Texture blending helpers (mask-based layer blends). | 头文件 | — |  |
| `inc/texture_utils.hlsl` | Texture sampling helpers used by path/mask samplers. | 头文件 | — |  |
| `inc/tracked_buffer.hlsl` | Helpers for appending packed records into a byte-address buffer while tracking the written count with a GPU atomic. Consumers (stairs, mesh generators… | 头文件 | — |  |
| `inc/unjitter_taa.hlsl` | consumers _taa/_tgsr read reprojected history using the current frame's sample_offset_pixels (view_constants) to remove TAA jitter; include-graph edge… | 头文件 | — |  |
| `inc/upsample/easu.hlsl` | FidelityFX FSR 1.0 EASU (Edge Adaptive Spatial Upsampling) for final post upscaling. Standard FSR1.0 EASU path (12 taps, luma gradient clamp). | 头文件 | — |  |
| `inc/upsample/lanczos2.hlsl` | Lanczos2 (2-lobe) interpolation for final-post upscaling. | 头文件 | — |  |
| `inc/utils.hlsl` | Procedural-instance utilities: affine 3x4 transform helpers and the packed transform record used by spawner shaders. | 头文件 | — |  |
| `inc/uv.hlsl` | Coordinate-space helpers shared by frame_constants consumers. Matrix-based (no cbuffer dependency) so it compiles before frame_constants.hlsl declares… | 头文件 | — |  |
| `inc/wall_color_customization.hlsl` | consumers _nani_gothic_window_bricks/_nani_plaster/_nani_roof_floor/_nani_solid_vertex_color_bricks read a wall palette color id (from wall_color_ids … | 头文件 | — |  |
| `inc/wall_color_ids.hlsl` | spirv-cross (_nani_gothic_window_bricks VS/_gi_trace) ByteAddressBuffer wall_color_ids indexed by brick/triangle id (Load<uint>(id*4)); include-graph … | 头文件 | — |  |
| `inc/wall_distortion.hlsl` | behavior-consistent body, filled in P1c integration pass, adjudicated inc-fix #13). Embedded by 28 bins (nani roof/ivy/wall-brick surface shaders). Th… | 头文件 | — |  |
| `inc/wind.hlsl` | consumers _flag_*/_leaf_particles/_garden_lowpoly_flower/_lilypad bend vertices by gusting noise + time; include-graph edge clearing/inc/instanced_pla… | 头文件 | — |  |
| `inc/window_color_customization.hlsl` | consumers _nani_balcony_door/_nani_solid_vertex_color_window/_solid_vertex_color_wooden_gate sample window_colors + gate_color_ids; frame_constants + … | 头文件 | — |  |
| `ivy/arch_noise_discard.inc.hlsl` | SOURCE PATH : shaders/ivy/arch_noise_discard.inc.hlsl KIND        : include_only COMPILED AS MAIN IN: none (include-only file) ALSO EMBEDDED IN (as in… | 头文件 | — |  |
| `light_binning/grid.inc.hlsl` | MERGE (gi-deferred review 2026-10-13): content adopted from candidate A (cell-index + min/max-world + point-to-cell helpers match the bin; B was const… | 头文件 | — |  |
| `nani/atlas_part_info.inc.hlsl` | MeshAtlasPartInfo{first_index,index_count,bounding_sphere_radius,pad0} (16B); generate_draw_lists/ cull_and_bucket access atlas_part_info as Structure… | 头文件 | — |  |
| `nani/cull_and_bucket.inc.hlsl` | _nani_cull_and_bucket*.cs.hlsl bins: shared cull body (frustum side-plane test, next-visibility bitmask OR + filtered-instance append with allocator +… | 头文件 | — |  |
| `nani/draw_list_offsets.inc.hlsl` | _nani_clear_draw_lists / _nani_allocate_draw_commands / _nani_fill_* / _nani_generate_*: OpMemberName instance_list_offset_array (uint4[27], b13 UBO) … | 头文件 | — |  |
| `nani/draw_lists.inc.hlsl` | nani cull/fill bins: GpuFilteredInstance{raw}, GpuFilteredInstanceMeta{raw}, NaniDrawListInfo{global_instance_offset}; nani bins' DrawIndexedIndirectC… | 头文件 | — |  |
| `nani/inc/cull_and_bucket_owned_custom.hlsl` | _nani_cull_and_bucket_{plaster,smoke,stairs_floor,curved_halftimber}.cs.hlsl bins: the include appears FIRST in the embedded source order and carries … | 头文件 | — |  |
| `nani/inc/trimesh.hlsl` | nani_stairs_floor raster; provides the trimesh chunk decode for stairs-floor instances (NaniTrimeshChunk from stairs_floor.inc: gpu_address / vertex_i… | 头文件 | — |  |
| `nani/inc/user.hlsl` | stairs_floor.inc + wall/inc/brick_vs_out.hlsl; embedded in 52 nani raster/cull bins; role is the shared nani per-user shading/instance include / O (cl… | 头文件 | — |  |
| `painterly/stroke.inc.hlsl` | Shared painterly stroke record + packing helpers. A stroke packs its centered position as two halves in `pos`, its RGBA8 color in `color`, and its siz… | 头文件 | — |  |
| `paths/path_mask_sampler.inc.hlsl` | consumers _clearing_*/_generate_grass/_path_pebbles/_ivy_* inline `path_mask.SampleLevel(sampler_llb, world_xz*0.0076923077+0.5, 0).x` (uv from world … | 头文件 | — |  |
| `plaster.inc.hlsl` | Plaster-wall shared record type. Member layout/names recovered verbatim from the consumer bins' OpMemberName tables: PlasterInstanceData: bounding_sph… | 头文件 | — |  |
| `resolve_to_swapchain.inc.hlsl` | Shared logic for resolving the HDR framebuffer to the swapchain: coordinate transform flag + per-pixel sample coordinate (before MSAA resolve / encode… | 头文件 | — |  |
| `rocky_terrain/inc_mesh.inc.hlsl` | Shared rocky-terrain mesh record layouts. The displaced mesh is generated by displace_rocky_terrain.cs (one thread per triangle) and consumed by the r… | 头文件 | — |  |
| `rocky_terrain/stairs/isolines.inc.hlsl` | KIND: include_only / embedded only in _rocky_terrain_stairs_stairs.cs.hlsl_cs_main_0.bin spirv-cross (_rocky_terrain_stairs_stairs.cs.hlsl_cs_main_0_0… | 头文件 | — |  |
| `rocky_terrain/stairs/marching_squares.inc.hlsl` | KIND: include_only / embedded only in _rocky_terrain_stairs_stairs.cs.hlsl_cs_main_0.bin spirv-cross (_rocky_terrain_stairs_stairs.cs.hlsl_cs_main_0_0… | 头文件 | — |  |
| `rocky_terrain/stairs/place_stairs.inc.hlsl` | KIND: include_only / embedded only in _rocky_terrain_stairs_stairs.cs.hlsl_cs_main_0.bin spirv-cross (_rocky_terrain_stairs_stairs.cs.hlsl_cs_main_0_0… | 头文件 | — |  |
| `rocky_terrain/triangle_adjacency.inc.hlsl` | Per-triangle adjacency for crack fixing on the displaced rocky-terrain mesh. Each Triangle records, per edge, the index of the neighbouring triangle s… | 头文件 | — |  |
| `roof/dormer_hole.inc.hlsl` | Dormer-hole record + occlusion helper. DormerHole layout/names recovered verbatim from the consumer OpMemberName tables (position f3, wall_id u, size … | 头文件 | — |  |
| `roof/roof_tile.inc.hlsl` | Shared roof-tile record types + helpers used by the roof family (instanced roof raster and the gravity sim). Member layouts and names recovered verbat… | 头文件 | — |  |
| `roof/stone_floor.inc.hlsl` | Stone-floor procedural helpers. Texture bindings (stone_floor_normal/seed/height/roughness, x3 variants) and the heightmap ray-march + normal/seed/rou… | 头文件 | — |  |
| `roof/wooden_floor.inc.hlsl` | Wooden-floor procedural helpers. Texture bindings (wooden_floor_normal/height/seed/roughness, x3 variants) observed inlined in _nani_roof_floor ps_mai… | 头文件 | — |  |
| `roof_material_id.inc.hlsl` | Roof-floor shared record type + material-id dispatch. InstanceData layout/names recovered verbatim from the consumer bins' OpMemberName tables: positi… | 头文件 | — |  |
| `screenshot/capture_screenshot.inc.hlsl` | Shared screenshot capture helpers: a gaussian-weighted box downscale used when the source and destination sizes differ, plus the same-size direct path… | 头文件 | — |  |
| `smoke.inc.hlsl` | Smoke-instance shared record type. Member layout/names recovered verbatim from the consumer bins' OpMemberName tables: SmokeInstanceData: xform (Affin… | 头文件 | — |  |
| `ssgi/XeGTAO.hlsl` | Evidence: _ssgi_ssgi.cs.hlsl_cs_main_0.bin (embedded source path shaders/ssgi/XeGTAO.hlsl; inlined use-sites: the fast-acos horizon-angle reconstructi… | 头文件 | — |  |
| `ssgi/vaShared.hlsl` | Evidence: _ssgi_ssgi.cs.hlsl_cs_main_0.bin (embedded source path shaders/ssgi/vaShared.hlsl; inlined use-sites: the depth->view -Z reconstruction `-1.… | 头文件 | — |  |
| `stairs_floor.inc.hlsl` | Stairs-floor shared record types. Member layouts/names recovered verbatim from the consumer bins' OpMemberName tables (raw-per-bin.jsonl): NaniTrimesh… | 头文件 | — |  |
| `terrain_editing/inc_highlight_active_terrain_chunk.inc.hlsl` | no direct include edges; consumed by floor.raster.hlsl + deferred_*_light_sun_sky + rocky_terrain/stairs; highlights the actively-edited terrain chunk… | 头文件 | — |  |
| `terrain_editing/inc_terrain_stroke.inc.hlsl` | include-graph direct edge -> sdf_utils.inc.hlsl; consumed only by rasterize_terrain_stroke (stroke list rasterized into img_output / rocky_terrain / t… | 头文件 | — |  |
| `terrain_editing/path_cobblestone_surface.inc.hlsl` | consumed only by floor.raster.hlsl (path cobblestone shading over dirthpath_heightmap / dirt_1 / dirt_2 / brick_colors); no include edges / I (names/b… | 头文件 | — |  |
| `terrain_editing/sdf_utils.inc.hlsl` | include-graph (leaf, no includes); consumed by rasterize_terrain_stroke via inc_terrain_stroke; SDF building blocks for stroke rasterization / I (name… | 头文件 | — |  |
| `tgsr_common.inc.hlsl` | Status: U (helper division Inference). | 头文件 | — |  |
| `vermeer/stroke.inc.hlsl` | Shared vermeer stroke record + packing helpers (same layout as the painterly chain but RGBA8 keeps alpha, read back by the raster VS as the stroke alp… | 头文件 | — |  |
| `wall/inc/brick_frag.hlsl` | simplex_noise.hlsl; embedded in wall brick rasters; brick fragment-shader helpers (brick mortar / highlight modulation) / O (closure); I (body) | 头文件 | — |  |
| `wall/inc/brick_instance.hlsl` | draw_lists.hlsl; embedded in wall generate_draw_lists + brick rasters; provides per-brick instance decode on top of InstancedWallData / O (closure); I… | 头文件 | — |  |
| `wall/inc/brick_instance_flags.hlsl` | brick_instance.hlsl; embedded in wall generate_draw_lists + brick rasters; brick flag decode helpers (highlight / deletion flags on InstancedWallData.… | 头文件 | — |  |
| `wall/inc/brick_vert.hlsl` | current_view.hlsl; embedded in wall brick rasters; brick vertex-shader helper (wall-space -> world transform with distortion) / O (closure); I (body) | 头文件 | — |  |
| `wall/inc/brick_vs_out.hlsl` | brick_instance_flags.hlsl; embedded in wall brick rasters; vertex-shader output interface for brick fragments / O (closure); I (body) | 头文件 | — |  |
| `wall/inc/draw_lists.hlsl` | _wall_generate_draw_lists / _wall_wall_brick_lod{0,1} bins: InstancedWallData{transform (Affine3Packed), sin_deform_y (f4), global_arch_height_vals (f… | 头文件 | — |  |
| `wall/inc/highlight_mode.hlsl` | inc/material/solid_material.inc.hlsl; embedded in wall brick rasters; highlight-mode blending for the wall editing tools / O (closure); I (body) | 头文件 | — |  |
| `water/ice_shared.inc.hlsl` | SOURCE PATH : shaders/water/ice_shared.inc.hlsl KIND        : include_only COMPILED AS MAIN IN: none (include-only file) ALSO EMBEDDED IN (as include,… | 头文件 | — |  |
| `water/water_mask_sampler.inc.hlsl` | _path_pebbles/_rocky_terrain_stairs_stairs/_water_spawn_reeds/_water_spawn_lilypads/ _paths_path_mask_visual/_garden_lowpoly_flower inline `water_mask… | 头文件 | — |  |
| `water/water_object_push.inc.hlsl` | SOURCE PATH : shaders/water/water_object_push.inc.hlsl KIND        : include_only COMPILED AS MAIN IN: none (include-only file) ALSO EMBEDDED IN (as i… | 头文件 | — |  |
| `water/water_surface_frag.inc.hlsl` | SOURCE PATH : shaders/water/water_surface_frag.inc.hlsl KIND        : include_only COMPILED AS MAIN IN: none (include-only file) ALSO EMBEDDED IN: _wa… | 头文件 | — |  |
| `water/waterfall_vert.inc.hlsl` | SOURCE PATH : shaders/water/waterfall_vert.inc.hlsl KIND        : include_only COMPILED AS MAIN IN: none (include-only file) ALSO EMBEDDED IN: _waterf… | 头文件 | — |  |
| `water/waves.inc.hlsl` | SOURCE PATH : shaders/water/waves.inc.hlsl KIND        : include_only COMPILED AS MAIN IN: none (include-only file) ALSO EMBEDDED IN: _leaf_particles/… | 头文件 | — |  |
| `wooden_plank.inc.hlsl` | Wooden-plank shared record type. Member layout/names recovered verbatim from the consumer bins' OpMemberName tables: WoodenPlank: transform (Affine3Pa… | 头文件 | — |  |

## blit_resolve_swapchain

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `blit/blit_stretch_decode_srgb.raster.hlsl` | Fullscreen blit: sample the input texture (stretched) and decode sRGB -> linear. Evidence: ps inlined per-channel decode (0.04045/0.0773993805/0.94786… | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `blit/resolve_msaa_to_swapchain.raster.hlsl` | MSAA swapchain resolve: flip pixel (transform_image), average all MSAA samples, encode linear -> sRGB. vs is the shared fullscreen triangle. | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `blit/resolve_to_swapchain.raster.hlsl` | Non-MSAA swapchain resolve: flip pixel (transform_image), Load the texel, encode linear -> sRGB. vs is the shared fullscreen triangle. | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `final_post.raster.hlsl` | Final post-processing pass (blit_resolve_swapchain): input_tex is upsampled to the swapchain size, blurred pyramid blended in, tonemapped through the … | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✗/SAME<br>VS:f0adff76 ✓/SAME | f0ad 变体的 ps_main 是无结果占位，dxc 会整段删除，结构门禁无法匹配；文件头保留 KNOWN EXCEPTION 说明 |
| `resolve_to_swapchain.cs.hlsl` | Compute swapchain resolve (MSAA path): flip pixel (transform_image vs output_tex_dims), average MSAA samples, encode linear -> sRGB, write u15. | 手写 | CS:0 ✓/SAME |  |

## blur_mips

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `blur.cs.hlsl` | Evidence: _blur.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23). bindings: $Globals b13 (img_in_dims/img_out_dims), img_in t14, img_out u15, LocalSize [8,8,… | 手写 | CS:0 ✓/SAME |  |
| `blur_downsample.cs.hlsl` | Evidence: _blur_downsample.cs.hlsl_cs_main_0.bin (Texture2D input, entry cs_main) + _blur_downsample.cs.hlsl_cs_main_multisampled_0.bin (Texture2DMS i… | 手写 | CS:0 ✓/SAME<br>cs_main_multisampled:cs_main_ ✓/SAME |  |
| `blur_pyramid/blur.cs.hlsl` | Evidence: _blur_pyramid_blur.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23). bindings: $Globals b15 (input_mip_dims), input_tex t13, output_tex u14 (float3… | 手写 | CS:0 ✓/SAME |  |
| `blur_pyramid/rev_blur.cs.hlsl` | Evidence: _blur_pyramid_rev_blur.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23). bindings: $Globals b16 (output_extent/sample_self), sampler_lnc s3, input_… | 手写 | CS:0 ✓/SAME |  |
| `gauss_blur.cs.hlsl` | （原文件无说明；按路径：gauss_blur.cs.hlsl） | 提升 | CS:b9dd0465 ✓/SAME<br>CS:dc5fd7e4 ✓/SAME |  |
| `gauss_blur_building_mask.cs.hlsl` | Evidence: _gauss_blur_building_mask.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23). bindings: $Globals b13 (building_mask_dims/out_building_mask_dims/is_ho… | 手写 | CS:0 ✓/SAME |  |
| `gauss_blur_mask.cs.hlsl` | Evidence: _gauss_blur_mask.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23). bindings: $Globals b13 (water_mask_dims/out_water_mask_dims/is_horizontal/sigma/… | 手写 | CS:0 ✓/SAME |  |
| `gauss_blur_r16f.cs.hlsl` | Evidence: _gauss_blur_r16f.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23). bindings: $Globals b13 (img_in_dims/img_out_dims/is_horizontal/sigma), img_in t1… | 手写 | CS:0 ✓/SAME |  |
| `generate_mips.cs.hlsl` | Evidence: _generate_mips.cs.hlsl_cs_main_0.bin, entry cs_main, LocalSize [256,1,1]. FidelityFX Single Pass Downsampler variant (blur_mips / 4-channel)… | 手写 | CS:0 ✓/SAME |  |

## building_rasterize

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `rasterize_buildings.cs.hlsl` | spirv-cross of _rasterize_buildings.cs.hlsl_cs_main_0.bin (mask buildings: shape_id 0 = circle, 1 = oriented box, into out_mask.x + out_mask2) / O (bi… | 手写 | CS:0 ✓/SAME |  |
| `rasterize_buildings_animate.cs.hlsl` | spirv-cross of _rasterize_buildings_animate.cs.hlsl_cs_main_0.bin (fade building mask highlight: x = highlight, y lerps toward x) / O (bindings/entry/… | 手写 | CS:0 ✓/SAME |  |

## critters_particles

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `critters/butterfly.raster.hlsl` | entries vs:vs_main ps:ps_main / spirv-cross + SPIR-V access/type inspection (instance StructuredBuffer<Butterfly> 32B stride, wing flap, cascaded shad… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `critters/duck_animated.raster.hlsl` | entries vs:vs_main ps:ps_main / spirv-cross + SPIR-V access/type inspection (per-vertex duck animation: wing flap, landing/take-off pose blend, pet bl… | 手写 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `critters/firefly.raster.hlsl` | entries vs:vs_main ps:ps_main / spirv-cross + SPIR-V access/type inspection (instance StructuredBuffer<Firefly> 16B stride, terrain-heightmap snap to … | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `critters/firefly_glow.raster.hlsl` | entries vs:vs_main ps:ps_main / spirv-cross + SPIR-V access/type inspection (instance StructuredBuffer<Firefly> + float visibility + DofFocus autofocu… | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `critters/firefly_vis_query.cs.hlsl` | entry cs:cs_main LocalSize [32,1,1] / spirv-cross + SPIR-V access/type inspection (per-firefly depth-window visibility query, 5-sample +2/-2 grid, tem… | 手写 | CS:0 ✓/SAME |  |
| `critters/nani_wren.raster.hlsl` | entries vs:vs_main ps:ps_main / spirv-cross + SPIR-V access/type inspection (instance indirection StructuredBuffer<uint> -> InstanceData with two Affi… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `rain_particles.raster.hlsl` | Reconstructed from inlined vs/ps opcode patterns (spirv-cross scaffolding + target fingerprint). Model: rain streak quads. Per-instance seed hashed (i… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `sheep.raster.hlsl` | Sheep instance animation: per-vertex positions/normals come from a structured buffer (`animation`) holding 30-frame sweeps (positions[30] + normals[30… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `snow_particles.raster.hlsl` | Snow particles: camera-facing quads in a 100-unit repeating volume around the active camera. vs_main rejects (NaN position, undefined attributes) in t… | 手写 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `sparkles.raster.hlsl` | Sparkle particle instances (critters_particles). Per-instance StructuredBuffer (position/scale/time_offset/speed_mult/color_id). Each sparkle pulses o… | 手写 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `splash_particle.raster.hlsl` | Splash particle: per-vertex position/color/normal/uv transformed by a global model_to_world; ps applies a 2-octave value-noise ring pattern (drop + ri… | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |

## debug_dev

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `fill_buffer.cs.hlsl` | Evidence: _fill_buffer.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23). bindings: push_constants (PushConstant storage, cbuffer, members value+range), __ali… | 手写 | CS:0 ✓/SAME |  |
| `normal_debug.raster.hlsl` | Evidence: _normal_debug.raster.hlsl_b903e43ffb3da915.bin (vs_main + ps_main). bindings: $Globals b13 (current_view_index), debug_normals_buffer b14, f… | 手写 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `primary_rt_debug.cs.hlsl` | Evidence: _primary_rt_debug.cs.hlsl_cs_main_0.bin (SPIR-V, module 0). bindings: sampler_lnc s0, sampler_llc s3, sampler_default s9, sampler_shadow_nea… | 手写 | CS:0 ✓/SAME |  |
| `shader_refactoring.cs.hlsl` | Evidence: _shader_refactoring.cs.hlsl_capture_0.bin (entry `capture`, [8,8,1]) + _shader_refactoring.cs.hlsl_compare_0.bin (entry `compare`, [8,8,1]).… | 手写 | capture:capture_ ✓/SAME<br>compare:compare_ ✓/SAME |  |
| `texture_test.raster.hlsl` | Evidence: _texture_test.raster.hlsl_b903e43ffb3da915.bin (vs_main + ps_main). bindings: sampler_default s9, $Globals b13 (current_view_index/model_to_… | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |

## deferred_raster

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `deferred/canopy_light_sun_sky.raster.hlsl` | MERGE (gi-deferred review 2026-10-13): adopted candidate B as the integration base (both b903/f0ad variants via DEFERRED_FULL_LIGHTING). The review's … | 手写 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `deferred/grass_light_sun_sky.raster.hlsl` | MERGE (gi-deferred review 2026-10-13): adopted candidate B as the integration base (both b903/f0ad variants via DEFERRED_FULL_LIGHTING). The review's … | 手写 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `deferred/plant_light_sun_sky.raster.hlsl` | MERGE (gi-deferred review 2026-10-13): adopted candidate B as the integration base (both b903/f0ad variants via DEFERRED_FULL_LIGHTING). The review's … | 手写 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `deferred/solid_light_sun_sky.raster.hlsl` | MERGE (gi-deferred review 2026-10-13): adopted candidate B as the integration base (B is the only side covering both b903/f0ad variants via DEFERRED_F… | 手写 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |

## depth_normals

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `copy_depth_to_r.cs.hlsl` | Evidence: _copy_depth_to_r.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23). bindings: input_tex t14/space0 (SampledImage 2D), output_tex u15/space0 (Storage… | 手写 | CS:0 ✓/SAME |  |
| `extract_half_res_depth_normals.cs.hlsl` | Evidence: _extract_half_res_depth_normals.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23). bindings: stencil_tex t14 (uint4), depth_input_tex t15, gbuffer_i… | 手写 | CS:0 ✓/SAME |  |

## dof

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `dof/analyze.cs.hlsl` | Dilates the per-tile min/max CoC over a circular ring, classifies 8x8 tile quads into near/far/both buckets, and appends tile coords + dispatch counts… | 提升 | CS:0 ✓/SAME | dxc 会给 quad 操作加 DerivativeGroupQuadsKHR，由 spv_strip_dgq.py 去除后一致 |
| `dof/autofocus.cs.hlsl` | Autofocus: samples 64 depth points in the focus box, bitonic-sorts the view-space focus distances per focus plane, and blends focus0/focus1/bias towar… | 提升 | CS:0 ✓/SAME |  |
| `dof/clear_halfres.cs.hlsl` | Clears the half-res color output for the tile list (group offsets[6] from DofTileOffsets). | 手写 | CS:0 ✓/SAME |  |
| `dof/fullres.cs.hlsl` | Full-res bokeh: half-res spatial resolve (upsample_halfres.inc) + the dithered 2-ring x 17-tap CoC gather (multires.inc / shared with halfres), summed… | 提升 | CS:6f1dd5dd ✓/SAME<br>CS:72a2a95d ✓/SAME |  |
| `dof/fullres_all_lod0.cs.hlsl` | Full-res bokeh pass at LOD0 scale: 9-tap ring, 4 bins, no half-res upsample blend (tile list offsets[1]). | 提升 | CS:6f1dd5dd ✓/SAME<br>CS:72a2a95d ✓/SAME |  |
| `dof/fullres_all_lod1.cs.hlsl` | Full-res bokeh pass at LOD1 scale: 5-tap ring, 2 bins, no half-res upsample blend (tile list offsets[2]). | 提升 | CS:6f1dd5dd ✓/SAME<br>CS:72a2a95d ✓/SAME |  |
| `dof/fullres_upsample.cs.hlsl` | Full-res reconstruction of the half-res DoF output: per-tile 8-tap spatial resolve using the offsets table, alpha-normalized. | 手写 | CS:0 ✓/SAME |  |
| `dof/fused.cs.hlsl` | Fused full-res bokeh DoF: same ring gather as halfres but without tile indirection, using pixel-space Loads (21-tap ring, 6 bins, 6-tap disk scatter). | 手写 | CS:0 ✓/SAME |  |
| `dof/halfres.cs.hlsl` | Half-res bokeh: tile-dispatch 8x8, CoC limit from dilated tiles, blue-noise dithered 2-ring x 17-tap anisotropic CoC gather into a packed groupshared … | 提升 | cs_halfres:cs_halfr ✓/SAME<br>cs_halfres:cs_halfr ✓/SAME<br>cs_halfres_all:cs_halfr ✓/SAME<br>cs_halfres_all:cs_halfr ✓/SAME |  |
| `dof/init_buffers.cs.hlsl` | Seeds the per-tile dispatch-args and the initial DofTileOffsetsPacked values. | 手写 | CS:0 ✓/SAME |  |
| `dof/prepare.cs.hlsl` | Computes the per-pixel signed circle-of-confusion from focus depth and writes the tile min/max abs-CoC used by the analyze pass. | 手写 | CS:0 ✓/SAME |  |

## fsr_fxaa

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `fsr_copy_history.cs.hlsl` | A plain input->output copy: output_tex[id.xy] = input_tex[id.xy]. | 手写 | CS:0 ✓/SAME |  |
| `fsr_init_reactive.cs.hlsl` | Fills the FSR reactive-mask buffer with a constant value. | 手写 | CS:0 ✓/SAME |  |
| `fxaa.cs.hlsl` | LIFTED: body generated by tools/render-passes/batch_lift.py from the oracle bins (the hand-written version scored below the fidelity gate). Variants s… | 提升 | CS:0 ✓/SAME |  |

## gi

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `gi/blur_screen_radiance.cs.hlsl` | LIFTED: body generated by tools/render-passes/batch_lift.py from the oracle bins (the hand-written version scored below the fidelity gate). Variants s… | 提升 | CS:0 ✓/SAME |  |
| `gi/eval_screen_radiance.cs.hlsl` | MERGE (gi-deferred review 2026-10-13): content adopted from candidate B (marginal winner; structurally identical to A, better closure parity). hi_px_s… | 手写 | CS:0 ✓/SAME |  |
| `gi/reproject_screen_radiance.cs.hlsl` | MERGE (gi-deferred review 2026-10-13): content adopted from candidate B (tie on body; better OpSource parity 14 vs 15). Includes gi_settings.inc (A's … | 手写 | CS:0 ✓/SAME |  |
| `gi/trace_gi_candidates.cs.hlsl` | LIFTED: body generated by tools/render-passes/batch_lift.py from the oracle bins (the hand-written version scored below the fidelity gate). Variants s… | 提升 | CS:0 ✓/SAME |  |
| `gi/update_gi_radiance.cs.hlsl` | MERGE (gi-deferred review 2026-10-13): content adopted from candidate B (near-exact include closure). hi_px_subpixels now comes from gi_settings.inc.h… | 手写 | CS:0 ✓/SAME |  |

## hzb

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `generate_hzb.cs.hlsl` | Evidence: _generate_hzb.cs.hlsl_cs_main_0.bin, entry cs_main, LocalSize [256,1,1]. FidelityFX Single Pass Downsampler variant (hzb / depth chain): pus… | 手写 | CS:0 ✓/SAME |  |

## light_binning

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `light_binning/bin_lights_to_cells.cs.hlsl` | MERGE (gi-deferred review 2026-10-13): A body + B feature flag. A's RNG matches the bin exactly (wang_hash((idx*1664525 + hash(frame_index) + 10139042… | 提升 | CS:14dca8ba ✓/SAME<br>CS:8faee3cc ✓/SAME | dxc 把 (6a)*(6a) 重排为 36*(a*a) 使球/格子边界判断翻转；对相关值加 precise 后两个变体 SAME |
| `light_binning/cell_prefix_merge.cs.hlsl` | MERGE (gi-deferred review 2026-10-13): tie winner; adopted candidate A (structurally identical to B, histdiff 30, hard gate PASS). Evidence: _light_bi… | 手写 | CS:0 ✓/SAME |  |
| `light_binning/cell_prefix_sum.cs.hlsl` | MERGE (gi-deferred review 2026-10-13): tie winner; adopted candidate A (structurally identical to B, both histdiff 36 near-converged). Evidence: _ligh… | 手写 | CS:0 ✓/SAME |  |
| `light_binning/cell_subtotal_prefix_sum.cs.hlsl` | MERGE (gi-deferred review 2026-10-13): tie winner; adopted candidate A (structurally identical to B, hard gate PASS with push_constants + shared_data2… | 手写 | CS:0 ✓/SAME |  |

## lut

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `lut/brdf_fg.cs.hlsl` | LIFTED: body generated by tools/render-passes/batch_lift.py from the oracle bins (the hand-written version scored below the fidelity gate). Variants s… | 提升 | CS:0 ✓/SAME |  |

## main_raster_simple

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `solid_vertex_color.raster.hlsl` | Per-vertex-color solid raster with flat-lighting + snow-tinted albedo + semitransparent shadow dither. Packs (normal/flags, albedo) into the gbuffer. | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `solid_vertex_color_albedo_map.raster.hlsl` | Solid raster sampling an albedo map (terrain/building ground) with snow tint, roughness gbuffer top byte, and contact-shadow flag. | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `solid_vertex_color_boat.raster.hlsl` | （原文件无说明；按路径：solid_vertex_color_boat.raster.hlsl） | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `solid_vertex_color_garden_stone.raster.hlsl` | （原文件无说明；按路径：solid_vertex_color_garden_stone.raster.hlsl） | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `solid_vertex_color_waterwheel.raster.hlsl` | （原文件无说明；按路径：solid_vertex_color_waterwheel.raster.hlsl） | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `solid_vertex_color_wooden_gate.raster.hlsl` | Wooden gate: normal-mapped with a seed-hashed per-fragment brightness variation and a per-wall window color; packs (normal, albedo) into gbuffer. | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `vertex_color.raster.hlsl` | Trivial per-vertex-color raster: vertex position is transformed by the per-draw model_to_world then the active camera's world_to_sample; the fragment … | 手写 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |

## nani_cull_bucket

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `nani/allocate_draw_commands.cs.hlsl` | _nani_allocate_draw_commands.cs.hlsl_cs_main_0.bin: groupshared mesh_histogram[64]+group_draw_count, per instance bucket by (instance%64), compact his… | 手写 | CS:0 ✓/SAME |  |
| `nani/clear_draw_lists.cs.hlsl` | _nani_clear_draw_lists.cs.hlsl_cs_main_0.bin: instance_list_offset b13 UBO + 3 RWStructuredBuffer<uint> (draw_list_length u14, instance_indirection_al… | 手写 | CS:0 ✓/SAME |  |
| `nani/clear_histogram.cs.hlsl` | _nani_clear_histogram.cs.hlsl_cs_main_0.bin (clear_histogram family): body zeroes one histogram element per thread; local_size 64; binding u13 RWStruc… | 手写 | CS:0 ✓/SAME |  |
| `nani/clear_subtotals.cs.hlsl` | _nani_clear_subtotals.cs.hlsl_cs_main_0.bin: RWStructuredBuffer<uint> __alias_subtotals u13, push_constants.item_count, zero subtotals[x] for x<item_c… | 手写 | CS:0 ✓/SAME |  |
| `nani/cull_and_bucket.cs.hlsl` | LIFTED: body generated by tools/render-passes/batch_lift.py from the oracle bins (the hand-written version scored below the fidelity gate). Variants s… | 提升 | CS:8a7d2f81 ✓/SAME<br>CS:d8520a92 ✓/SAME |  |
| `nani/cull_and_bucket_bush.cs.hlsl` | （原文件无说明；按路径：nani/cull_and_bucket_bush.cs.hlsl） | 提升 | CS:8a7d2f81 ✓/SAME<br>CS:d8520a92 ✓/SAME |  |
| `nani/cull_and_bucket_curved_halftimber.cs.hlsl` | LIFTED: body generated by tools/render-passes/batch_lift.py from the oracle bins (the hand-written version scored below the fidelity gate). Variants s… | 提升 | CS:8a7d2f81 ✓/SAME<br>CS:d8520a92 ✓/SAME |  |
| `nani/cull_and_bucket_flowers.cs.hlsl` | （原文件无说明；按路径：nani/cull_and_bucket_flowers.cs.hlsl） | 提升 | CS:8a7d2f81 ✓/SAME<br>CS:d8520a92 ✓/SAME |  |
| `nani/cull_and_bucket_plaster.cs.hlsl` | LIFTED: body generated by tools/render-passes/batch_lift.py from the oracle bins (the hand-written version scored below the fidelity gate). Variants s… | 提升 | CS:8a7d2f81 ✓/SAME<br>CS:d8520a92 ✓/SAME |  |
| `nani/cull_and_bucket_roof_tile.cs.hlsl` | （原文件无说明；按路径：nani/cull_and_bucket_roof_tile.cs.hlsl） | 提升 | CS:8a7d2f81 ✓/SAME<br>CS:d8520a92 ✓/SAME |  |
| `nani/cull_and_bucket_smoke.cs.hlsl` | LIFTED: body generated by tools/render-passes/batch_lift.py from the oracle bins (the hand-written version scored below the fidelity gate). Variants s… | 提升 | CS:8a7d2f81 ✓/SAME<br>CS:d8520a92 ✓/SAME |  |
| `nani/cull_and_bucket_stairs_floor.cs.hlsl` | LIFTED: body generated by tools/render-passes/batch_lift.py from the oracle bins (the hand-written version scored below the fidelity gate). Variants s… | 提升 | CS:8a7d2f81 ✓/SAME<br>CS:d8520a92 ✓/SAME |  |
| `nani/cull_and_bucket_wooden_plank.cs.hlsl` | （原文件无说明；按路径：nani/cull_and_bucket_wooden_plank.cs.hlsl） | 提升 | CS:8a7d2f81 ✓/SAME<br>CS:d8520a92 ✓/SAME |  |
| `nani/draw_indexed_indirect_to_dispatch.cs.hlsl` | disasm of _nani_draw_indexed_indirect_to_dispatch.cs.hlsl_cs_main_0.bin: single thread turns a DrawIndexedIndirectCommand bucket into a uint4 dispatch… | 手写 | CS:0 ✓/SAME |  |
| `nani/fill_sorted_instance_list.cs.hlsl` | _nani_fill_sorted_instance_list.cs.hlsl_cs_main_0.bin: per-thread de-duplicate a filtered instance into the sorted buffer under a histogram slot; push… | 手写 | CS:0 ✓/SAME |  |
| `nani/fill_unsorted_instance_indirection.cs.hlsl` | disasm of _nani_fill_unsorted_instance_indirection.cs.hlsl_cs_main_0.bin: per instance write mesh id (instance/64) into the indirection buffer under a… | 手写 | CS:0 ✓/SAME |  |
| `nani/fill_unsorted_instance_list.cs.hlsl` | _nani_fill_unsorted_instance_list.cs.hlsl_cs_main_0.bin: per filtered instance, histogram slot then write packed instance into the sorted buffer and b… | 手写 | CS:0 ✓/SAME |  |
| `nani/generate_dispatch_args.cs.hlsl` | _nani_generate_dispatch_args.cs.hlsl_cs_main_0.bin: per draw list, clamp allocator count to the list offset and emit 4 dispatch args (64/32/32768/1 la… | 手写 | CS:0 ✓/SAME |  |
| `nani/generate_draw_lists.cs.hlsl` | _nani_generate_draw_lists.cs.hlsl_cs_main_0.bin: builds draw commands from per-bucket instance counts (mesh_histogram), emitting DrawIndexedIndirectCo… | 手写 | CS:0 ✓/SAME |  |
| `nani/group_draw_cmd_prefix_sum.cs.hlsl` | _nani_group_draw_cmd_prefix_sum.cs.hlsl_cs_main_0.bin: Hillis-Steele scan over a byte-address histogram range with end masking; block total added to d… | 手写 | CS:0 ✓/SAME |  |
| `nani/histogram_prefix_sum.cs.hlsl` | _nani_histogram_prefix_sum.cs.hlsl_cs_main_0.bin: unrolled Hillis-Steele inclusive scan over shared_data2x[1024] (2 elems/lane, 10 steps), exclusive p… | 手写 | CS:0 ✓/SAME |  |
| `nani/prefix_sum_unsorted_instances_per_atlas_entry.cs.hlsl` | raw disasm of _nani_prefix_sum_unsorted_instances_per_atlas_entry.cs.hlsl_cs_main_0.bin: per 64-draw-cmd group, exclusive scan of instance_count, then… | 手写 | CS:0 ✓/SAME |  |
| `nani/prepare_unsorted_draw_lists.cs.hlsl` | _nani_prepare_unsorted_draw_lists.cs.hlsl_cs_main_0.bin: writes one DrawIndexedIndirectCommand per atlas part (index_count/first_index from atlas, ins… | 手写 | CS:0 ✓/SAME |  |
| `nani/subtotal_prefix_sum.cs.hlsl` | _nani_subtotal_prefix_sum.cs.hlsl_cs_main_0.bin: unrolled Hillis-Steele scan (2 elems/lane, 1024/lane block) with range_start/range_end end masking; n… | 手写 | CS:0 ✓/SAME |  |

## nani_raster_main

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `nani_balcony_door.raster.hlsl` | Nani balcony door: VS jitters the door surface with a 3D value-noise lattice; PS dithers semitransparent doors, colors the door (window palette for th… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `nani_bush.raster.hlsl` | Nani bush: VS bends the bush with a 2D wind lattice + grass_object_push and forwards the part id + vertex height; PS alpha-discards via bush_alpha, cu… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `nani_cloth.raster.hlsl` | Nani cloth: VS flutters the cloth with two layered 2D wind lattices + a vertical sine wave, offsets the bottom hem per-instance; PS picks the cloth co… | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `nani_curved_halftimber.raster.hlsl` | Nani curved half-timber: VS builds the curved timber beam from instance control points; PS shades halftimber + wood roughness + plaster fill and packs… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `nani_flower.raster.hlsl` | Nani flower: VS selects the flower color by glade theme + flower type, sways the stem with a 2D wind value-noise lattice and applies grass_object_push… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `nani_gothic_window_bricks.raster.hlsl` | Nani gothic window brick wall: brick material + window frame colors, snow tint and semitransparent dither; packs the gbuffer. | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `nani_instanced_roof.raster.hlsl` | Nani instanced roof: per-tile transform from instanced_data; PS shades the roof tile palette with moss, damage and normal map and packs the gbuffer. | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `nani_lantern_emissive.raster.hlsl` | Nani lantern: VS forwards a half-packed lantern position/intensity + flags; PS dithers semitransparent lanterns, colors the glass by palette id, packs… | 手写 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `nani_plant.raster.hlsl` | Nani plant: VS bends the foliage with grass_object_push masks + a 2D wind value-noise lattice and per-instance color variation; PS packs (normal, albe… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `nani_plaster.raster.hlsl` | LIFTED: body generated by tools/render-passes/batch_lift.py from the oracle bins (the hand-written version scored below the fidelity gate). Variants s… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `nani_preplaced_flower.raster.hlsl` | Nani pre-placed flower: VS culls flowers inside building masks (NaN position), bends with a 2D wind lattice + grass_object_push; PS samples an albedo … | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `nani_roof_floor.raster.hlsl` | Nani roof/floor: per-instance roof-tile transform; PS selects a wooden or stone floor material by instance type, applies height-parallax + normal map,… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `nani_smoke.raster.hlsl` | LIFTED: body generated by tools/render-passes/batch_lift.py from the oracle bins (the hand-written version scored below the fidelity gate). Variants s… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `nani_solid_albedo_map.raster.hlsl` | Nani solid raster with a per-instance albedo-map texture layer: VS forwards the albedo texture index + vertex flags; PS samples the albedo map unless … | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `nani_solid_albedo_map_emissive.raster.hlsl` | Nani solid albedo-map raster with emissive markers: vertex-color magenta markers flag emissive windows that light up at night (discarded by day); PS e… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `nani_solid_vertex_color.raster.hlsl` | Nani-driven solid vertex-color raster. VS resolves the per-instance transform through instance_indirection_buffer -> nani_instance_data, forwards pack… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `nani_solid_vertex_color_bricks.raster.hlsl` | （原文件无说明；按路径：nani_solid_vertex_color_bricks.raster.hlsl） | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `nani_solid_vertex_color_window.raster.hlsl` | Nani window solid: VS jitters the facade vertices with a 3D value-noise lattice (per-window hash) and forwards the semitransparent flag + window color… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `nani_solid_vertex_color_window_glass.raster.hlsl` | LIFTED: body generated by tools/render-passes/batch_lift.py from the oracle bins (the hand-written version scored below the fidelity gate). Variants s… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `nani_stairs_floor.raster.hlsl` | （原文件无说明；按路径：nani_stairs_floor.raster.hlsl） | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `nani_uber.raster.hlsl` | Nani uber material raster: VS forwards instance flags + bindless albedo index; PS samples the bindless albedo, applies snow tint and packs the gbuffer… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `nani_velocity_only.raster.hlsl` | LIFTED: body generated by tools/render-passes/batch_lift.py from the oracle bins (the hand-written version scored below the fidelity gate). Variants s… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `nani_wooden_plank.raster.hlsl` | （原文件无说明；按路径：nani_wooden_plank.raster.hlsl） | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |

## painterly_vermeer

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `lut_gallery/thumbnail.raster.hlsl` | LUT gallery thumbnail tile: samples a preview, applies a 3D grading LUT (amount lerped), optionally overlays an icon, and draws a hover/active border. | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `painterly/acquire_target.cs.hlsl` | Painterly thumbnail chain: resolves the multisampled frame color into a non-MS target with a depth-aware edge-preserving box blur, writing the circle-… | 手写 | CS:0 ✓/SAME |  |
| `painterly/convolve_target.cs.hlsl` | Painterly thumbnail chain: separable-ish gaussian convolve of the acquired target. The blur radius shrinks toward the right side of the frame (photogr… | 手写 | CS:0 ✓/SAME |  |
| `painterly/debug_target.raster.hlsl` | Painterly debug-target pass: draws nothing (fragment is discarded). The vertex pass is the standard fullscreen triangle. | 手写 | PS:b903e43f ✓/空壳<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/空壳<br>VS:f0adff76 ✓/SAME |  |
| `painterly/generate_strokes.cs.hlsl` | Painterly thumbnail chain: procedural stroke generation. Each invocation fabricates one stroke whose color matches the target texture and whose rotati… | 手写 | CS:0 ✓/SAME |  |
| `painterly/raster_strokes.raster.hlsl` | Painterly thumbnail chain: rasterizes the generated strokes as textured quads (two triangles per stroke) over the brush texture. | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `vermeer/acquire_target.cs.hlsl` | Vermeer thumbnail chain: resolves the multisampled frame color into a non-MS target (the "acquire" step). Maps each target texel to the corresponding … | 手写 | CS:0 ✓/SAME |  |
| `vermeer/combine.raster.hlsl` | Vermeer thumbnail chain: final combine. In non-photomode the accumulated stroke alpha drives a progressive reveal and a soft vignette fades in. | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `vermeer/convolve_target.cs.hlsl` | Vermeer thumbnail chain: gaussian convolve of the acquired target with the same photograph-style horizontal falloff as the painterly chain. | 手写 | CS:0 ✓/SAME |  |
| `vermeer/generate_strokes.cs.hlsl` | LIFTED: body generated by tools/render-passes/batch_lift.py from the oracle bins (the hand-written version scored below the fidelity gate). Variants s… | 提升 | CS:2f7da8dc ✓/SAME<br>CS:f890205f ✓/SAME |  |
| `vermeer/raster_strokes.raster.hlsl` | Vermeer thumbnail chain: rasterizes generated strokes as textured quads. Photomode produces the clean "photograph" look; otherwise strokes get a luma-… | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |

## path

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `path_pebble.raster.hlsl` | Path pebble instancing pass. One instanced quad per pebble: the vertex shader reads the pebble's affine transform from pebble_placement_buffer (one Pe… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `path_pebbles.cs.hlsl` | spirv-cross + SPIR-V disasm (grid-cell scatter over scatter_grid_dim, 104-unit cell, hash jitter; height -> path_mask rejection <0.9; rocky/water in-b… | 提升 | CS:0 ✓/SAME | spirv-cross 把 OpSRem 展开为 a-b*(a/b)，除数为 0 时 UB；batch_lift.fix_srem 修复后 SAME |
| `paths/combine_path_exclusions.cs.hlsl` | spirv-cross + SPIR-V disasm (clamp of sum of 4 path-mask texel loads -> paths_exclusion; no samplers, texel Load only; gid is int3 -> uint2 bitcast, c… | 手写 | CS:0 ✓/SAME |  |
| `paths/directional_contract.cs.hlsl` | spirv-cross + SPIR-V disasm (directional contraction of the path mask along terrain_sdg gradient; if sdg==0 fallback to smoothstep(0.9,0.95,path_mask)… | 手写 | CS:0 ✓/SAME |  |
| `paths/path_mask_visual.cs.hlsl` | spirv-cross + SPIR-V disasm (world_xz from gid via paths_visual_dims*65; terrain_heightmap GatherRed slope -> offset_scale=lerp(1,0,clamp); 4x rotated… | 手写 | CS:0 ✓/SAME |  |

## prefix_scan

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `prefix_scan/inclusive_prefix_scan.cs.hlsl` | Evidence: _prefix_scan_inclusive_prefix_scan.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23). bindings: __alias_inout_buf u13 (RWByteAddressBuffer uint), gr… | 手写 | CS:0 ✓/SAME |  |
| `prefix_scan/inclusive_prefix_scan_merge.cs.hlsl` | Evidence: _prefix_scan_inclusive_prefix_scan_merge.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23). bindings: __alias_inout_buf u13 (RWByteAddressBuffer), s… | 手写 | CS:0 ✓/SAME |  |
| `prefix_scan/inclusive_prefix_scan_subtotals.cs.hlsl` | Evidence: _prefix_scan_inclusive_prefix_scan_subtotals.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23). bindings: input_buf t13 (ByteAddressBuffer), __alias… | 手写 | CS:0 ✓/SAME |  |

## reprojection

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `calculate_reprojection_map.cs.hlsl` | Evidence: _calculate_reprojection_map.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23). bindings: sampler_nnc s2, $Globals b13 (depth_tex_dims/prev_depth_tex… | 手写 | CS:0 ✓/SAME |  |
| `rdi/temporal.cs.hlsl` | Evidence: _rdi_temporal.cs.hlsl_cs_main_0.bin (SPIR-V, module 0). bindings: sampler_nnc s2, sampler_llc s3, lantern_buf t13 (StructuredBuffer LanternP… | 手写 | CS:0 ✓/SAME |  |

## rocky_terrain_stairs

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `rocky_terrain/displace_rocky_terrain.cs.hlsl` | Rocky-terrain vertex/triangle displacement. One thread owns one triangle (gid.x: vertices [3x, 3x+1, 3x+2], triangle x). When none of the 3 vertices s… | 手写 | CS:0 ✓/SAME |  |
| `rocky_terrain/rocky_terrain.raster.hlsl` | Rocky-terrain mesh raster pass. The vertex shader indexes the displaced mesh SSBO by gl_VertexIndex (3 vertices per triangle): emits the displaced pos… | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `rocky_terrain/stairs/stairs.cs.hlsl` | local_size [8,8,1] / gid uint3 (SV_DispatchThreadID) O (algorithm); residual do-while/phi/switch + int3-Load codegen = dxc-version artifacts; OpSource… | 提升 | CS:0 ✓/SAME |  |
| `rocky_terrain/stairs/stairs_counter.cs.hlsl` | SPIR-V disasm (StructuredBuffer<DrawIndexedIndirectCommand> draw_commands, AccessChain [0][idx][1] -> instanceCount; RWStructuredBuffer<StairsCounter>… | 手写 | CS:0 ✓/SAME |  |
| `rocky_terrain/stairs/stairs_indirect.raster.hlsl` | spirv-cross + SPIR-V disasm of both bins (vs: indirect stair instance draw — instance transform (StructuredBuffer<Stair> stairs_buffer t16, structured… | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |

## roof

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `flag_roof_cloth.raster.hlsl` | Roof-top flag cloth (roof family). The vs blends between a rest pose and an animated pose read from anim textures (positions + normals), keyed by a sw… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `flag_small.raster.hlsl` | Small wall flag (roof family). The vs bends the flag plane with a two-octave gust: a slow 2D value-noise field (origin-anchored, drifting in time) dri… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `flag_wall_cloth.raster.hlsl` | Wall banner cloth (roof family). The vs bends the flag with a gust envelope (origin-anchored 2D value noise) plus a faster flutter term; vertices are … | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `instanced_roof_beams.raster.hlsl` | （原文件无说明；按路径：instanced_roof_beams.raster.hlsl） | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `instanced_roof_spire.raster.hlsl` | SOURCE PATH : shaders/instanced_roof_spire.raster.hlsl KIND        : raster_main COMPILED AS MAIN IN (2 bins): _instanced_roof_spire.raster.hlsl_b903e… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `roof_snow.raster.hlsl` | Roof snow overlay (roof family). The vs displaces each roof vertex with a fine snow-noise jitter: a trilinear 3D-lattice float2 noise (custom 3D hash)… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `roof_tiles_gravity.cs.hlsl` | Roof tile gravity/placement simulation. Each thread updates one RoofShingle record (in RWStructuredBuffer<RoofShingle> __alias_instanced_data at set0/… | 提升 | CS:0 ✓/SAME |  |

## screenshot_capture

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `capture_panic_screenshot.cs.hlsl` | Panic-screen capture: writes dst_dims to the buffer header, then maps every thread to two dst pixels, encodes each to linear, converts to Rec.709 YCbC… | 手写 | CS:0 ✓/SAME |  |
| `screenshot/capture_screenshot_buffer.cs.hlsl` | Captures a screenshot into a byte-address buffer as RGBA8 rows. sample1 reads a non-MSAA source; sample4 averages/resamples a Texture2DMS source. When… | 手写 | sample1:sample1_ ✓/SAME<br>sample4:sample4_ ✓/SAME |  |
| `screenshot/capture_screenshot_texture.cs.hlsl` | Captures a screenshot into an unorm float4 RWTexture. sample1/sample4 handle non-MSAA and MSAA sources; same-size direct copy vs gaussian downscale. | 手写 | sample1:sample1_ ✓/SAME<br>sample4:sample4_ ✓/SAME |  |

## session_ui

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `loading_screen.raster.hlsl` | Loading-screen animation: a centered quad samples a Texture2DArray frame strip (8 layers driven by progress_time); black content (1 - sample.x) fades … | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `logo_animation.raster.hlsl` | Intro logo: a vertex-fed quad; the ps composites the logo texture with an animation-texture reveal sweep driven by anim_t, fading out with fade_out. | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `logo_branches_animation.raster.hlsl` | Logo "branch" intro: each of 4 branches grows from its start position with a per-branch angle and timing; the ps picks the branch texture by branch in… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `logo_overlay.raster.hlsl` | Overlay logo used on top of gameplay: aspect-corrected quad near the corner; ps samples input_tex with a warm tint. | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `session_selection/background.raster.hlsl` | Session-selection background: fullscreen triangle + a hex-grid dithered sampling of background_tex with a vignette curve. | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `session_selection/new_glade_thumbnail.raster.hlsl` | "New glade" thumbnail tile: quad positioned at first_thumbnail_center; ps brightens the preview slightly while hovered. | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `session_selection/thumbnail.raster.hlsl` | Session thumbnail tile: rounded-rect masked preview, hover highlight, and an optional black-and-white mode (e.g. for unavailable saves). | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `session_selection/thumbnail_painterly.raster.hlsl` | Painterly session thumbnails: instanced (6 verts/instance) tiles scattered around first_thumbnail_center, rotated and sized by hashed randomness; the … | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |

## shadow

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `apply_screen_shadows.cs.hlsl` | Evidence: _apply_screen_shadows.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23). bindings: sampler_lnc s0, sampler_shadow_nearest s11, sampler_shadow_linear… | 手写 | CS:0 ✓/SAME |  |
| `mei_shadow.raster.hlsl` | SOURCE PATH : shaders/mei_shadow.raster.hlsl KIND        : raster_main COMPILED AS MAIN IN (2 bins): _mei_shadow.raster.hlsl_b903e43ffb3da915.bin pass… | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `shadow_mask_mean.cs.hlsl` | Evidence: _shadow_mask_mean.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23). bindings: __alias_shadow_mask_histogram u13 (RWByteAddressBuffer), __alias_shad… | 手写 | CS:0 ✓/SAME |  |

## sky_cloud

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `cloud.raster.hlsl` | Clouds: instanced billboards on a shell around the camera. The ps samples one of 4 cloud textures (by instance quarter) at 7 texel offsets, resolves l… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `skydome.raster.hlsl` | Skydome: a dome mesh is transformed by model_to_world; the ps blends a sun-disc + glow, a moon sprite (projected onto the dome), and a day/night sky g… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `star.raster.hlsl` | Stars: instanced procedural billboards scattered on a large shell around the camera; the ps samples the star texture, gated by view direction and the … | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |

## ssgi

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `ssgi/ssgi.cs.hlsl` | Evidence: _ssgi_ssgi.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23; SHA-256 66e61114e7e782b18c01a857168ac52b587b1a063635a084e2af6b5caebef57d). bindings: sa… | 提升 | CS:0 ✓/SAME |  |
| `ssgi/temporal_filter.cs.hlsl` | Evidence: _ssgi_temporal_filter.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23; SHA-256 d31c87e724cf473fea5cc9cf65e66699177e57997eb8b8aaf9651482313920b1). b… | 提升 | CS:0 ✓/SAME |  |
| `ssgi/upsample.cs.hlsl` | Evidence: _ssgi_upsample.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23; SHA-256 2a466f7f8b21c2cedf861db078351469a1dc11eebc14e29de06359d8ecc077d6). bindings… | 提升 | CS:0 ✓/SAME |  |

## taa_tgsr

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `taa.cs.hlsl` | Temporal AA: reprojects + bicubic-cross filters the history, neighborhood-clamps in YCoCg, blends, curve-quantizes the new history, and applies a tile… | 提升 | CS:0 ✓/SAME | frexp 辅助函数未处理非规格化数（提升缺陷），已修复后 SAME |
| `tgsr.cs.hlsl` | TGSR (temporal Gaussian super-resolution) accumulation: reprojected-history + current-frame statistics drive an adaptive blend/clamp in YCoCg, with a … | 提升 | CS:3ee6a953 ✓/SAME<br>CS:7a0af184 ✓/SAME<br>CS:b303f36f ✓/SAME<br>CS:f7efabb8 ✓/SAME | frexp 辅助函数未处理非规格化数（提升缺陷），4 个变体已修复后 SAME |
| `tgsr_blur_lum.cs.hlsl` | TGSR blurred-luminance pass: 3x3 Gaussian (exp(-(i^2+j^2)*0.75)) over luminance, unjittered by half the current clip offset, written as sqrt(weighted … | 手写 | CS:0 ✓/SAME |  |
| `tgsr_reproject.cs.hlsl` | TGSR reprojection: fetches the 2x2 motion quad, on disocclusion re-searches the max-depth position, filters the history with a 5-tap bicubic cross, an… | 手写 | CS:0 ✓/SAME |  |

## terrain

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `demo_terrain_overlay.raster.hlsl` | spirv-cross of _demo_terrain_overlay.raster.hlsl_* (46x46 grid over the terrain; overlay tinted quad in sample space) / O (bindings/entry/flow); I (na… | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `mouse_terrain_raycast.cs.hlsl` | spirv-cross of _mouse_terrain_raycast.cs.hlsl_cs_main_0.bin (ray-march against terrain heightmap from the mouse ray; stores mouse_raycast_position + t… | 手写 | CS:0 ✓/SAME |  |
| `read_mouse_depth_value.cs.hlsl` | Evidence: _read_mouse_depth_value.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23). bindings: $Globals b13 (depth_map_dims/mouse_position_screenspace/window_… | 提升 | CS:0 ✓/SAME |  |
| `read_terrain_stroke.cs.hlsl` | spirv-cross of _read_terrain_stroke.cs.hlsl_cs_main_0.bin (query push constant -> stroke_indices sample -> terrain_stroke_index at byte 16 of out_buff… | 手写 | CS:0 ✓/SAME |  |
| `terrain_derivative.cs.hlsl` | spirv-cross of _terrain_derivative.cs.hlsl_cs_main_0.bin (rock steepness from central differences of the terrain heightmap, smoothstep(0.75,1.25) into… | 手写 | CS:0 ✓/SAME |  |
| `terrain_editing/cache_stroke.cs.hlsl` | spirv-cross of _terrain_editing_cache_stroke.cs.hlsl_cs_main_0.bin (accumulate max stroke into cache, clear stroke) / O (bindings/entry/LocalSize/flow… | 手写 | CS:0 ✓/SAME |  |
| `terrain_editing/calc_highlight_hill_mask.cs.hlsl` | spirv-cross of _terrain_editing_calc_highlight_hill_mask.cs.hlsl_cs_main_0.bin (fill mask where the highlight stroke passes) / O (bindings/entry/Local… | 手写 | CS:0 ✓/SAME |  |
| `terrain_editing/floor.raster.hlsl` | LIFTED: body generated by tools/render-passes/batch_lift.py from the oracle bins (the hand-written version scored below the fidelity gate). Variants s… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `terrain_editing/rasterize_terrain_stroke.cs.hlsl` | LIFTED: body generated by tools/render-passes/batch_lift.py from the oracle bins (the hand-written version scored below the fidelity gate). Variants s… | 提升 | CS:0 ✓/SAME |  |
| `terrain_editing/read_terrain_height.cs.hlsl` | spirv-cross of _terrain_editing_read_terrain_height.cs.hlsl_cs_main_0.bin (pairs of 16-bit packed heights into two alias buffers) / O (bindings/entry/… | 手写 | CS:0 ✓/SAME |  |
| `terrain_editing/read_terrain_rock_mask.cs.hlsl` | spirv-cross of _terrain_editing_read_terrain_rock_mask.cs.hlsl_cs_main_0.bin (pack 32 rock-presence bits per row) / O (bindings/entry/LocalSize/flow);… | 手写 | CS:0 ✓/SAME |  |
| `terrain_lighting_texture.cs.hlsl` | spirv-cross of _terrain_lighting_texture.cs.hlsl_cs_main_0.bin (terrain indirect light + soft shadow; grass/dirt color from height comparison, 5-tap d… | 提升 | CS:0 ✓/SAME |  |
| `timed_terrain_gizmo.raster.hlsl` | spirv-cross of _timed_terrain_gizmo.raster.hlsl_* (vs: arrow_index selects y/x axis stretch, world_to_sample output in sample space; ps: passthrough C… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `upload_cpu_raster.cs.hlsl` | spirv-cross of _upload_cpu_raster.cs.hlsl_{r16f,rgba16f}_0.bin (two entries; 64x64-tile RGBA8 CPU raster upload, uint word = 4 packed pixels; UAV form… | 手写 | r16f:r16f_0 ✓/SAME<br>rgba16f:rgba16f_ ✓/SAME |  |

## ui

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `custom_cursor.raster.hlsl` | Custom OS cursor quad driven by a CPU-written cursor_buf (position px, enabled flag, size px). Disabled cursor is moved off-screen via NaN position. | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `dashed_line.raster.hlsl` | Dashed line: per-vertex model_to_world transform; fragment discards every other dash cell (frac of the line-length parameter). | 手写 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `egui.raster.hlsl` | egui (immediate-mode UI) textured quad: a packed vertex buffer (20-byte stride: xy = pixel position, zw = uv, byte16 = RGBA8 color) is decoded in the … | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `text.raster.hlsl` | Text / vector-shape renderer. vs expands packed PrimPacked records into quads. ps has two paths: a shape-stream interpreter (per-primitive op list sam… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `timer_circle.raster.hlsl` | Circular countdown timer drawn as a billboard quad in world space. The ps renders a ring whose filled arc grows with timer_progress. | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `ui_2d.raster.hlsl` | Simple 2D UI quad (no per-camera transform): position/size in window pixels. vs emits the 6-vertex quad; ps samples ui_texture with -1 mip bias and ap… | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `ui_2d_color_button.raster.hlsl` | Color-picker button quad. Same 6-vertex quad as ui_2d; ps adds an animated circle-reveal: progress sweeps around, masked toward the original color whe… | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `ui_2d_ws.raster.hlsl` | World-space UI quad: positioned via ui_position (world) + ui_size, oriented along the camera right/up axes. ps premultiplies alpha with -1 mip bias. | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |

## vegetation_grass

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `billboard_forest.raster.hlsl` | Evidence: _billboard_forest.raster.hlsl_b903e43ffb3da915.bin / _f0adff76827e73ff.bin. bindings (vs): $Globals b13 (current_view_index/framebuffer_dims… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `billboard_plant.raster.hlsl` | Evidence: _billboard_plant.raster.hlsl_b903e43ffb3da915.bin / _f0adff76827e73ff.bin. bindings (vs): $Globals b13 (current_view_index/model_to_world/fl… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `clearing/distant_billboard.raster.hlsl` | Evidence: _clearing_distant_billboard.raster.hlsl_b903e43ffb3da915.bin / _f0adff76827e73ff.bin. bindings: sampler_default s9, distant_hills_texture t1… | 手写 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `clearing/instanced_background_tree.raster.hlsl` | Evidence: _clearing_instanced_background_tree.raster.hlsl_b903e43ffb3da915.bin / _f0adff76827e73ff.bin. bindings: $Globals b13 (current_view_index/mod… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `clearing/meadow_flower.raster.hlsl` | Evidence: _clearing_meadow_flower.raster.hlsl_b903e43ffb3da915.bin / _f0adff76827e73ff.bin. bindings (vs): sampler_llb s4, sampler_default s9, $Global… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `clearing/meadow_plant.raster.hlsl` | Evidence: _clearing_meadow_plant.raster.hlsl_b903e43ffb3da915.bin / _f0adff76827e73ff.bin. bindings (vs): sampler_llb s4, sampler_default s9, $Globals… | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `clearing/tree_canopy.raster.hlsl` | LIFTED: body generated by tools/render-passes/batch_lift.py from the oracle bins (the hand-written version scored below the fidelity gate). Variants s… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `clearing/tree_trunk.raster.hlsl` | Evidence: _clearing_tree_trunk.raster.hlsl_b903e43ffb3da915.bin / _f0adff76827e73ff.bin. bindings (vs): $Globals b13 (current_view_index/rocky_terrain… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `garden/exclude_other_masks.cs.hlsl` | spirv-cross + raw disasm of _garden_exclude_other_masks.cs.hlsl_cs_main_0.bin (int2 pixel = int2(id.xy); clear garden_mask.x where path.x>0.1 / rocky.… | 手写 | CS:0 ✓/SAME |  |
| `garden/marching_squares.cs.hlsl` | spirv-cross + raw disasm of _garden_marching_squares.cs.hlsl_cs_main_0.bin (per 8x8 group loads a 9x9 garden/path/discard sample grid into groupshared… | 提升 | CS:0 ✓/SAME | 需 64–400 号输入才触发写入 |
| `garden/read_bush_positions_for_cpu.cs.hlsl` | spirv-cross + raw disasm of _garden_read_bush_positions_for_cpu.cs.hlsl_cs_main_0.bin (thread0 writes float2(count,count); other threads read BushXfor… | 手写 | CS:0 ✓/SAME |  |
| `garden/read_garden_mask.cs.hlsl` | spirv-cross of _garden_read_garden_mask.cs.hlsl_cs_main_0.bin (garden_mask.y > 0.3 -> bits, 32 texels/thread in 512-wide row) / O (algorithm + thresho… | 手写 | CS:0 ✓/SAME |  |
| `garden/spawn_bushes_buildings.cs.hlsl` | spirv-cross of _garden_spawn_bushes_buildings.cs.hlsl_cs_main_0.bin (exclusion; garden_mask.y>0.3 skip; buildings_mask.x gate + perlin(world)>0.5; Bus… | 提升 | CS:0 ✓/SAME |  |
| `garden/spawn_bushes_garden.cs.hlsl` | spirv-cross of _garden_spawn_bushes_garden.cs.hlsl_cs_main_0.bin (exclusion masks; buildings_non_blurred<0.5; garden_mask.y+0.005>0.5 -> BushXform (cm… | 提升 | CS:0 ✓/SAME |  |
| `garden/spawn_flowers_buildings.cs.hlsl` | spirv-cross of _garden_spawn_flowers_buildings.cs.hlsl_cs_main_0.bin (two jittered sites; exclusion; garden_mask.y>0.3 skip; buildings_mask.y band 0.5… | 提升 | CS:0 ✓/SAME |  |
| `garden/spawn_flowers_garden.cs.hlsl` | spirv-cross of _garden_spawn_flowers_garden.cs.hlsl_cs_main_0.bin (two jittered candidate sites; exclusion; buildings<0.5; garden_mask.y band around 0… | 提升 | CS:0 ✓/SAME |  |
| `garden/spawn_small_flowers_garden.cs.hlsl` | spirv-cross of _garden_spawn_small_flowers_garden.cs.hlsl_cs_main_0.bin (jittered world pos seed+100; exclusion masks; garden_mask.y in (0.35,0.8) ban… | 提升 | CS:0 ✓/SAME |  |
| `garden_lowpoly_flower.raster.hlsl` | spirv-cross of _garden_lowpoly_flower.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin (identical VS both variants: Flower03Xform.position float4 =… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `generate_grass.cs.hlsl` | Evidence: _generate_grass.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23). bindings: sampler_llb s4, sampler_default s9, color_library b13, __alias_draw_com… | 提升 | CS:0 ✓/SAME |  |
| `grass/bundles_prefix.cs.hlsl` | Evidence: _grass_bundles_prefix.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23). bindings: __alias_tile_bundle_subtotal_buffer u13 (RWByteAddressBuffer). Lo… | 手写 | CS:0 ✓/SAME |  |
| `grass/calc_dispatch_args.cs.hlsl` | Evidence: _grass_calc_dispatch_args.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23). bindings: __alias_grass_dispatch_args_buffer u13 (RWStructuredBuffer<ui… | 手写 | CS:0 ✓/SAME |  |
| `grass/calc_grasslet_bundle_ranges.cs.hlsl` | Evidence: _grass_calc_grasslet_bundle_ranges.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23). bindings: __alias_grasslet_bundle_ranges u13 (RWStructuredBuff… | 手写 | CS:0 ✓/SAME |  |
| `grass/calc_tiles.cs.hlsl` | Evidence: _grass_calc_tiles.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23). bindings: $Globals b13 (terrain_heightmap_dims/visual_terrain_heightmap_dims/ h… | 手写 | CS:0 ✓/SAME |  |
| `grass/tile_bundle_count.cs.hlsl` | Evidence: _grass_tile_bundle_count.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23). bindings: __alias_tile_bundle_prefix_buffer u13, __alias_tile_bundle_sub… | 手写 | CS:0 ✓/SAME |  |
| `grass/tile_bundle_gen.cs.hlsl` | Evidence: _grass_tile_bundle_gen.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23). bindings: __alias_bundle_buffer u13 (RWStructuredBuffer<GrassBundlePacked>… | 手写 | CS:0 ✓/SAME |  |
| `grass/tiles_prefix.cs.hlsl` | Evidence: _grass_tiles_prefix.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23). bindings: __alias_blade_prefix_subtotal_buffer u13 (RWByteAddressBuffer). Loc… | 手写 | CS:0 ✓/SAME |  |
| `grass.raster.hlsl` | Evidence: _grass.raster.hlsl_b903e43ffb3da915.bin / _f0adff76827e73ff.bin (SPIR-V, offsets 23/34106). bindings: sampler_default s9, terrain_heightmap … | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `grass_lod1.raster.hlsl` | Evidence: _grass_lod1.raster.hlsl_b903e43ffb3da915.bin / _f0adff76827e73ff.bin (SPIR-V). Bindings identical to grass.raster. vs_main builds a 3-vertex… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `grass_object_push.raster.hlsl` | Evidence: _grass_object_push.raster.hlsl_b903e43ffb3da915.bin / _f0adff76827e73ff.bin (SPIR-V). bindings: $Globals b13 (obj_position_radius float4). v… | 手写 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `ivy/instanced_ivy_branch.raster.hlsl` | SOURCE PATH : shaders/ivy/instanced_ivy_branch.raster.hlsl KIND        : raster_main COMPILED AS MAIN IN (2 bins): _ivy_instanced_ivy_branch.raster.hl… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `ivy/instanced_ivy_leaf.raster.hlsl` | SOURCE PATH : shaders/ivy/instanced_ivy_leaf.raster.hlsl KIND        : raster_main COMPILED AS MAIN IN (2 bins): _ivy_instanced_ivy_leaf.raster.hlsl_b… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `ivy/ivy.raster.hlsl` | SOURCE PATH : shaders/ivy/ivy.raster.hlsl KIND        : raster_main COMPILED AS MAIN IN (2 bins): _ivy_ivy.raster.hlsl_b903e43ffb3da915.bin file_sha25… | 手写 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `leaf_particle.raster.hlsl` | Falling leaf particles: each instance carries a position / rotation / scale / up axis; the quad is oriented to the up axis with a horizontal alignment… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `leaf_particles.cs.hlsl` | Leaf particle simulation. Each thread owns one 32-byte LeafParticle (position f3 +0, rotation f +12, scale f +16, up_axis f3 +20) in RWByteAddressBuff… | 提升 | CS:0 ✓/SAME |  |
| `lilypad.raster.hlsl` | Lilypad: per-instance Reed affine xform from instance_buffer; the vertex is bent by a water-noise displacement (waves.inc), a value-noise wobble along… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME | VS 曾在 14 号输入出现差异，根因不是 FMA 而是 dxc 把 (x*20)-(z*20) 因式分解为 (x-z)*20 改变舍入；batch_lift 用 spv_fmsub(precise 临时量) 保持两次乘法后，各变体 SAME |
| `lilypad_flower.raster.hlsl` | Lilypad flower: Reed instance xform + water displacement; the vertex is offset by the grass_object_push height and the wave displacement. ps samples t… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `reed.raster.hlsl` | Reed: like lilypad (Reed instance xform + water displacement + value-noise wobble) but with a vertex normal passed to the fragment stage; ps packs the… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME | 同 lilypad：dxc 因式分解 (x*20)-(z*20) 导致差异；spv_fmsub 修复后各变体 SAME |
| `stairs_foliage_exclusion.raster.hlsl` | LIFTED: body generated by tools/render-passes/batch_lift.py from the oracle bins (the hand-written version scored below the fidelity gate). Variants s… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |

## volumetrics

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `apply_volumetrics.cs.hlsl` | Evidence: _apply_volumetrics.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23). bindings: sampler_shadow_linear s12, $Globals b13 (current_view_index/depth_te… | 手写 | CS:0 ✓/SAME |  |

## wall_draw_lists

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `wall/generate_draw_lists.cs.hlsl` | _wall_generate_draw_lists.cs.hlsl_cs_pass1_0.bin / cs_pass2_0.bin (LocalSize 64,1,1 both): per brick instance, build the wall-space transform, frustum… | 提升 | cs_pass1:cs_pass1 ✓/SAME<br>cs_pass2:cs_pass2 ✓/SAME |  |
| `wall/reset_pass2_inst_counts.cs.hlsl` | _wall_reset_pass2_inst_counts.cs.hlsl_cs_main_0.bin: zero instanceCount of the first 4 draw commands at draw_indirect_buffer_offset / O | 手写 | CS:0 ✓/SAME |  |

## wall_raster

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `wall/wall_brick_lod0.raster.hlsl` | （原文件无说明；按路径：wall/wall_brick_lod0.raster.hlsl） | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `wall/wall_brick_lod1.raster.hlsl` | （原文件无说明；按路径：wall/wall_brick_lod1.raster.hlsl） | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |

## water

| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |
|---|---|---|---|---|
| `ice_plane.raster.hlsl` | SOURCE PATH : shaders/ice_plane.raster.hlsl KIND        : raster_main COMPILED AS MAIN IN (2 bins): _ice_plane.raster.hlsl_{b903e43ffb3da915,f0adff768… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME | 曾因 half(NaN) 符号位(UB)出现差异，提升后各变体均 SAME |
| `lut/water_noise.cs.hlsl` | Evidence: _lut_water_noise.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23). bindings: sampler_llr s1, $Globals b13 (blue_noise_texture_dims/output_tex_dims)… | 手写 | CS:0 ✓/SAME |  |
| `water/calc_deferred_rt_args.cs.hlsl` | SOURCE PATH : shaders/water/calc_deferred_rt_args.cs.hlsl KIND        : compute_main COMPILED AS MAIN IN (1 bins): _water_calc_deferred_rt_args.cs.hls… | 手写 | CS:0 ✓/SAME |  |
| `water/deferred_rt.cs.hlsl` | SOURCE PATH : shaders/water/deferred_rt.cs.hlsl KIND        : compute_main COMPILED AS MAIN IN (1 bins): _water_deferred_rt.cs.hlsl_cs_main_0.bin entr… | 提升 | CS:0 ✓/SAME |  |
| `water/draw_water_mask.cs.hlsl` | spirv-cross + raw SPIR-V disasm of _water_draw_water_mask.cs.hlsl_cs_main_0.bin (input SV_DispatchThreadID is int2; $Globals{out_water_mask_dims,is_ad… | 手写 | CS:0 ✓/SAME |  |
| `water/ice_cracks.raster.hlsl` | SOURCE PATH : shaders/water/ice_cracks.raster.hlsl KIND        : raster_main COMPILED AS MAIN IN (2 bins): _water_ice_cracks.raster.hlsl_b903e43ffb3da… | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `water/ice_deferred_rt.cs.hlsl` | SOURCE PATH : shaders/water/ice_deferred_rt.cs.hlsl KIND        : compute_main COMPILED AS MAIN IN (1 bins): _water_ice_deferred_rt.cs.hlsl_cs_main_0.… | 提升 | CS:0 ✓/SAME |  |
| `water/ice_refl_march.cs.hlsl` | SOURCE PATH : shaders/water/ice_refl_march.cs.hlsl KIND        : compute_main COMPILED AS MAIN IN (1 bins): _water_ice_refl_march.cs.hlsl_cs_main_0.bi… | 提升 | CS:0 ✓/SAME |  |
| `water/ice_refl_resolve.cs.hlsl` | SOURCE PATH : shaders/water/ice_refl_resolve.cs.hlsl KIND        : compute_main COMPILED AS MAIN IN (1 bins): _water_ice_refl_resolve.cs.hlsl_cs_main_… | 提升 | CS:0 ✓/EDGE | 已逐值证明：重建版用 dxc -Gis(严格 IEEE)编译并给原版/重建版全部浮点运算注入 NoContraction 后，0–63 号输入逐位一致(strict 差异 0)；默认执行的差异仅来自 lavapipe 对两种指令顺序的乘加融合不同(对照：原版默认 vs 原版禁融合 差 29 字)。另证：同一份重建版以 -Gis 严格顺序编译、允许驱动融合时与原版默认执行逐位一致(0 差异)，说明唯一差别是 dxc 对表达式的重结合；仍记为 EDGE(已证明)，因为默认 dxc 编译无法在 HLSL 里保留严格顺序又允许融合 |
| `water/ice_refr_resolve.cs.hlsl` | SOURCE PATH : shaders/water/ice_refr_resolve.cs.hlsl KIND        : compute_main COMPILED AS MAIN IN (1 bins): _water_ice_refr_resolve.cs.hlsl_cs_main_… | 提升 | CS:0 ✓/SAME |  |
| `water/read_water_mask.cs.hlsl` | spirv-cross + raw disasm of _water_read_water_mask.cs.hlsl_cs_main_0.bin (water_mask.y > 0.5 -> water bits; > 0.2 -> clutter bits; 32 texels/thread in… | 手写 | CS:0 ✓/SAME |  |
| `water/spawn_boats.cs.hlsl` | spirv-cross + raw disasm of _water_spawn_boats.cs.hlsl_cs_main_0.bin (groupshared best_choice: per-thread score from water-depth min across a boat-siz… | 提升 | CS:0 ✓/SAME |  |
| `water/spawn_lilypads.cs.hlsl` | spirv-cross of _water_spawn_lilypads.cs.hlsl_cs_main_0.bin (animation storage image eased toward garden-clear flag; water band (theme2 -> perlin-warpe… | 提升 | CS:0 ✓/SAME |  |
| `water/spawn_reeds.cs.hlsl` | spirv-cross + raw disasm of _water_spawn_reeds.cs.hlsl_cs_main_0.bin (jittered world pos; water-mask height bounds-checked sample; water band falloff … | 提升 | CS:0 ✓/SAME |  |
| `water/spawn_waterwheels.cs.hlsl` | spirv-cross + raw disasm of _water_spawn_waterwheels.cs.hlsl_cs_main_0.bin — entry is EMPTY (only LocalSize 64,1,1 + cs_main; no bindings, no body ops… | 手写 | CS:0 ✓/空壳 | 原版只有约 3 条指令(空壳) |
| `water_plane.raster.hlsl` | LIFTED: body generated by tools/render-passes/batch_lift.py from the oracle bins (the hand-written version scored below the fidelity gate). Variants s… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `water_plane_refraction_copy.raster.hlsl` | SOURCE PATH : shaders/water_plane_refraction_copy.raster.hlsl KIND        : raster_main COMPILED AS MAIN IN (2 bins): _water_plane_refraction_copy.ras… | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `water_snow_plane.raster.hlsl` | SOURCE PATH : shaders/water_snow_plane.raster.hlsl KIND        : raster_main COMPILED AS MAIN IN (2 bins): _water_snow_plane.raster.hlsl_b903e43ffb3da… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `water_x_paths.raster.hlsl` | SOURCE PATH : shaders/water_x_paths.raster.hlsl KIND        : raster_main COMPILED AS MAIN IN (2 bins): _water_x_paths.raster.hlsl_b903e43ffb3da915.bi… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `waterfall.raster.hlsl` | LIFTED: body generated by tools/render-passes/batch_lift.py from the oracle bins (the hand-written version scored below the fidelity gate). Variants s… | 提升 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
| `waterfall_refraction_copy.raster.hlsl` | SOURCE PATH : shaders/waterfall_refraction_copy.raster.hlsl KIND        : raster_main COMPILED AS MAIN IN (2 bins): _waterfall_refraction_copy.raster.… | 混合 | PS:b903e43f ✓/SAME<br>VS:b903e43f ✓/SAME<br>PS:f0adff76 ✓/SAME<br>VS:f0adff76 ✓/SAME |  |
