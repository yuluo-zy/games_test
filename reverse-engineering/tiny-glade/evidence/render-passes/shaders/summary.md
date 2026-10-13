# Summary — GPU-side pass inventory from 404 compiled shaders

Static SPIR-V reflection of the distribution's compiled-shaders/ (404 .bin): **260 unique passes (117 raster + 143 compute)** grouped into **35 families**. Every raster .bin holds two modules (vs_main + ps_main); every compute .bin holds one (cs_main, plus a few cs_pass1/cs_pass2/sample1/sample4/r16f/rgba16f/cs_halfres_all). 638 modules total. All facts are from filenames + SPIR-V declarations; no game was run.

## Family list (compute / raster split)
- **GPU-driven scene**: nani_cull_bucket (24 compute: cull/bucket per instance type, histogram + prefix sums, draw-list generation, dispatch args, indirect-to-dispatch), wall_draw_lists (2 compute: cs_pass1/cs_pass2, pass2 reads hzb_tex), nani_raster_main (23 raster: uber, solid albedo/vertex-color variants, velocity_only), main_raster_simple (7 raster), deferred_raster (4 raster).
- **Shadow / HZB / depth**: shadow (2 compute apply_screen_shadows+shadow_mask_mean, 1 raster mei_shadow), hzb (1 compute generate_hzb), depth_normals (2 compute), reprojection (2 compute calculate_reprojection_map, rdi_temporal).
- **GI / lighting**: gi (5 compute: trace/update/eval/reproject/blur), ssgi (3 compute), light_binning (4 compute), volumetrics (1 compute).
- **Temporal / AA / upscale**: taa_tgsr (4 compute: taa, tgsr x4 variants + reproject/blur_lum), fsr_fxaa (3 compute).
- **Post / UI / present**: dof (11 compute), blur_mips (9 compute), prefix_scan (3 compute), lut (brdf_fg), blit_resolve_swapchain (1 compute + 4 raster incl. MSAA resolve + final_post), ui (8 raster incl. egui), painterly_vermeer (6 compute + 5 raster), session_ui (8 raster).
- **Content / procgen**: water (13 compute + 8 raster incl. software-BVH deferred-RT + refl/refr chains), terrain (11 compute + 3 raster), path (4 compute + 1 raster), rocky_terrain_stairs (3 compute + 2 raster), building_rasterize (2 compute), vegetation_grass (18 compute + 20 raster — largest), roof (1 compute + 6 raster), critters_particles (1 compute + 10 raster), sky_cloud (3 raster), screenshot_capture (3 compute), debug_dev (5).

## Strongest findings
1. **No hardware ray tracing**: no RayTracingKHR/RayQuery in any of the 638 modules; GI and water/ice "deferred RT" read the same software-BVH SSBOs (rt_bl_bvh, rt_triangles) → software ray marching.
2. **HZB/mips = FidelityFX SPD**: generate_hzb and generate_mips share the SPD signature (spdConstants push constant, imgSrc/imgDst/imgDst6, spdGlobalAtomic, LocalSize 256).
3. **Variant-pair structure**: every raster pass has two 16-hex variant compilations; for 97/138 two-variant passes one variant's bindings are a strict superset of the other's (0 mixed). E.g. cull with/without hzb_tex; nani_uber with/without bindless_textures_dyn@2.0; final_post full LUT chain vs minimal. Inference: shader-variants compile-time feature pairs, not separate passes.
4. **GPU-driven instancing**: nani vertex shaders read instance_indirection_buffer + nani_instance_data SSBOs; cull uses PhysicalStorageBufferAddresses, 64x1x1.
5. **G-buffer + deferred**: extract_half_res_depth_normals reads gbuffer_input_tex + stencil_tex; deferred passes read gbuffer/depth + light_indirection_grid + rdi_reservoir_tex + ssgi_tex + screen_shadows_tex.
6. **Descriptor conventions**: frame_constants UBO set1/b0 (occasionally an SSBO at 1.1); samplers set0/b0..12; $Globals UBO set0/b13; textures/SSBOs from b14.

## Open unknowns
- Exact feature flags behind the 16-hex variant pairs and when each runs.
- TAA vs TGSR vs FSR vs FXAA selection per frame.
- Software-BVH builder + traversal/sampling algorithms shared by GI and water.
- Real descriptor handles, formats/extents, execution order (needs CPU-side PDB/machine code or runtime evidence).

Deliverables: evidence/render-passes/shaders/catalog.json (machine-readable), catalog.md, method.md, this summary.md.
