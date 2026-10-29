# X1 static report (generated)

Pipelines: 404; lowered stages: 638; runtime-array limit as built.

## Capabilities in the lowered SPIR-V

| capability | stages | shaders | note |
| --- | ---: | ---: | --- |
| Shader | 638 | - |  |
| RuntimeDescriptorArray | 638 | - |  |
| StorageBufferArrayDynamicIndexing | 638 | - |  |
| SampledImageArrayDynamicIndexing | 638 | - |  |
| StorageImageArrayDynamicIndexing | 638 | - |  |
| StorageImageExtendedFormats | 28 | 24 | extended storage formats |
| GroupNonUniform | 18 | - |  |
| StorageImageWriteWithoutFormat | 17 | 13 | typeless storage writes |
| GroupNonUniformShuffle | 17 | 8 | simd shuffle |
| GroupNonUniformQuad | 15 | 8 | quad operations (incl. fragment stage) |
| ImageQuery | 13 | - |  |
| Int64 | 13 | 7 | 64-bit integers |
| PhysicalStorageBufferAddresses | 13 | 7 | buffer GPU addresses (D23); needs Metal 3 |
| GroupNonUniformVote | 11 | 6 | simd vote |
| DemoteToHelperInvocationEXT | 6 | 3 | discard semantics (demote vs terminate) |
| StorageImageReadWithoutFormat | 5 | 5 | typeless storage reads |
| GroupNonUniformArithmetic | 5 | 2 | simd reductions |
| ShaderNonUniform | 4 | 4 | non-uniform resource indexing |
| SampledImageArrayNonUniformIndexing | 4 | - |  |
| ComputeDerivativeGroupQuadsNV | 1 | 1 | compute-stage derivatives |
| GroupNonUniformBallot | 1 | 1 | simd ballot |

## Shaders using the watched capabilities

- **PhysicalStorageBufferAddresses** (7): nani/cull_and_bucket.cs.hlsl, nani/cull_and_bucket_plaster.cs.hlsl, nani/cull_and_bucket_smoke.cs.hlsl, nani/cull_and_bucket_stairs_floor.cs.hlsl, nani_plaster.raster.hlsl, nani_stairs_floor.raster.hlsl, terrain_editing/rasterize_terrain_stroke.cs.hlsl
- **Int64** (7): nani/cull_and_bucket.cs.hlsl, nani/cull_and_bucket_plaster.cs.hlsl, nani/cull_and_bucket_smoke.cs.hlsl, nani/cull_and_bucket_stairs_floor.cs.hlsl, nani_plaster.raster.hlsl, nani_stairs_floor.raster.hlsl, terrain_editing/rasterize_terrain_stroke.cs.hlsl
- **GroupNonUniformQuad** (8): deferred/canopy_light_sun_sky.raster.hlsl, deferred/grass_light_sun_sky.raster.hlsl, deferred/plant_light_sun_sky.raster.hlsl, deferred/solid_light_sun_sky.raster.hlsl, dof/analyze.cs.hlsl, ice_plane.raster.hlsl, water_plane.raster.hlsl, waterfall.raster.hlsl
- **GroupNonUniformShuffle** (8): dof/fullres.cs.hlsl, dof/fullres_all_lod0.cs.hlsl, dof/fullres_all_lod1.cs.hlsl, dof/halfres.cs.hlsl, generate_hzb.cs.hlsl, generate_mips.cs.hlsl, tgsr.cs.hlsl, tgsr_reproject.cs.hlsl
- **GroupNonUniformVote** (6): deferred/canopy_light_sun_sky.raster.hlsl, deferred/grass_light_sun_sky.raster.hlsl, deferred/plant_light_sun_sky.raster.hlsl, deferred/solid_light_sun_sky.raster.hlsl, final_post.raster.hlsl, mei_shadow.raster.hlsl
- **GroupNonUniformArithmetic** (2): dof/analyze.cs.hlsl, tgsr.cs.hlsl
- **GroupNonUniformBallot** (1): dof/analyze.cs.hlsl
- **DemoteToHelperInvocationEXT** (3): ice_plane.raster.hlsl, water_plane.raster.hlsl, waterfall.raster.hlsl
- **ComputeDerivativeGroupQuadsNV** (1): dof/analyze.cs.hlsl
- **StorageImageWriteWithoutFormat** (13): apply_volumetrics.cs.hlsl, blur_pyramid/blur.cs.hlsl, blur_pyramid/rev_blur.cs.hlsl, dof/fused.cs.hlsl, fxaa.cs.hlsl, primary_rt_debug.cs.hlsl, shader_refactoring.cs.hlsl, taa.cs.hlsl, terrain_lighting_texture.cs.hlsl, tgsr.cs.hlsl, water/deferred_rt.cs.hlsl, water/ice_refl_resolve.cs.hlsl, water/ice_refr_resolve.cs.hlsl
- **StorageImageReadWithoutFormat** (5): apply_volumetrics.cs.hlsl, primary_rt_debug.cs.hlsl, shader_refactoring.cs.hlsl, water/deferred_rt.cs.hlsl, water/ice_refl_resolve.cs.hlsl
- **StorageImageExtendedFormats** (24): dof/fused.cs.hlsl, dof/prepare.cs.hlsl, fsr_init_reactive.cs.hlsl, gauss_blur_building_mask.cs.hlsl, generate_hzb.cs.hlsl, generate_mips.cs.hlsl, gi/update_gi_radiance.cs.hlsl, lut/brdf_fg.cs.hlsl, painterly/acquire_target.cs.hlsl, paths/combine_path_exclusions.cs.hlsl, paths/directional_contract.cs.hlsl, paths/path_mask_visual.cs.hlsl, rasterize_buildings.cs.hlsl, rasterize_buildings_animate.cs.hlsl, rdi/temporal.cs.hlsl, terrain_editing/calc_highlight_hill_mask.cs.hlsl, terrain_editing/rasterize_terrain_stroke.cs.hlsl, tgsr.cs.hlsl, tgsr_blur_lum.cs.hlsl, tgsr_reproject.cs.hlsl, upload_cpu_raster.cs.hlsl, water/ice_deferred_rt.cs.hlsl, water/ice_refl_march.cs.hlsl, water/ice_refl_resolve.cs.hlsl
- **ShaderNonUniform** (4): nani_cloth.raster.hlsl, nani_solid_vertex_color_window_glass.raster.hlsl, nani_uber.raster.hlsl, text.raster.hlsl

## Binding aliasing after the production lowering

The lowering puts every descriptor array of one Vulkan descriptor type on one binding, so a binding carrying more than one resource type is an *aliased array*. SPIRV-Cross has to turn each such binding into valid MSL, which is the highest-risk step of decision D8.

- stages with at least one aliased binding: **486 / 638** (76%)
- pipelines with at least one aliased stage: 386 / 404
- aliased bindings by (set, binding): (0, 2): 486, (0, 0): 39, (0, 1): 16
- distinct types on one aliased binding -> occurrences: 2: 163, 3: 167, 4: 153, 5: 43, 6: 13, 7: 2

Widest aliasing (first 8):

- wall/generate_draw_lists.cs.hlsl [Compute] binding 2: 7 types
  - `struct%25`
  - `struct%28`
  - `struct%30`
  - `struct%33`
  - `struct%37`
  - `struct%39`
  - `struct%402`
- wall/generate_draw_lists.cs.hlsl [Compute] binding 2: 7 types
  - `struct%26`
  - `struct%29`
  - `struct%31`
  - `struct%34`
  - `struct%38`
  - `struct%40`
  - `struct%518`
- critters/firefly_glow.raster.hlsl [Vertex] binding 2: 6 types
  - `struct%24`
  - `struct%257`
  - `struct%27`
  - `struct%30`
  - `struct%33`
  - `struct%35`
- critters/firefly_glow.raster.hlsl [Vertex] binding 2: 6 types
  - `struct%24`
  - `struct%257`
  - `struct%27`
  - `struct%30`
  - `struct%33`
  - `struct%35`
- deferred/canopy_light_sun_sky.raster.hlsl [Fragment] binding 2: 6 types
  - `struct%2680`
  - `struct%50`
  - `struct%58`
  - `struct%61`
  - `struct%63`
  - `struct%71`
- deferred/canopy_light_sun_sky.raster.hlsl [Fragment] binding 2: 6 types
  - `struct%2680`
  - `struct%50`
  - `struct%58`
  - `struct%61`
  - `struct%63`
  - `struct%71`
- deferred/grass_light_sun_sky.raster.hlsl [Fragment] binding 2: 6 types
  - `struct%2516`
  - `struct%50`
  - `struct%59`
  - `struct%62`
  - `struct%64`
  - `struct%71`
- deferred/grass_light_sun_sky.raster.hlsl [Fragment] binding 2: 6 types
  - `struct%2516`
  - `struct%50`
  - `struct%59`
  - `struct%62`
  - `struct%64`
  - `struct%71`
