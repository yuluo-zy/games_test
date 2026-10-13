// ==RECONSTRUCTED== shaders/waterfall.raster.hlsl
// SOURCE PATH : shaders/waterfall.raster.hlsl
// KIND        : raster_main
// COMPILED AS MAIN IN (2 bins):
//   _waterfall.raster.hlsl_b903e43ffb3da915.bin  (full waterfall surface render)
//   _waterfall.raster.hlsl_f0adff76827e73ff.bin  (simplified: wave normal + march + RT rays)
// entries: vs:vs_main  ps:ps_main
// Status: O (bindings/entry; vs + ps transcribed from spirv-cross)
// Evidence: spirv-cross vs (waterfall_vert.inc per-instance displacement + 3D
//   value-noise; C0=vertex normal, C1=world) and ps (same water-surface shading
//   structure as water_plane with waterfall bindings; the bin PS reads only TEXCOORD1).
// MERGE (water-ice review waterfall.raster.hlsl): A base; vs_main moved into
//   water/waterfall_vert.inc.hlsl (B's hash3 lattice + layout — the bin's VS entry
//   lives in that inc), PS lives in the shared water/water_surface_frag.inc.hlsl.
// Note: two 16-hex variants are compile-time feature variants — gated on
//   WATER_SURFACE_FULL (b903e = full render, f0adff = RT-ray source).

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/consts.hlsl"
#include "inc/perlin.hlsl"
#include "inc/hash.hlsl"
// inc/blue_noise.hlsl no longer declares the texture (per-review register fix).
#include "inc/utils.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/math.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/fbm_noise.hlsl"
#include "inc/raymarch.hlsl"
#include "inc/halfres.hlsl"
#include "inc/coc.hlsl"
#include "inc/history_valid.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/rt.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/shadow.hlsl"
#include "inc/cascades.hlsl"
// terrain_heightmap.inc omitted (reconstructed inc references a consumer-declared
// terrain_heightmap texture this shader never binds — see decisions.md).
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sh.hlsl"
#include "inc/lighting_indirect.hlsl"
#include "inc/material/eval/solid_material.inc.hlsl"
#include "inc/brdf_lut.hlsl"
#include "inc/highlight_color.hlsl"
#include "inc/wall_color_ids.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint history_valid;
    uint2 terrain_lighting_texture_dims;
    uint2 blue_noise_texture_dims;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 brdf_fg_lut_dims;
    uint rt_bvh_node_count;
    uint rt_triangle_count;
    uint2 water_noise_texture_dims;
    uint2 framebuffer_dims;
    uint2 water_mask_dims;
    uint2 half_depth_tex_dims;
    uint2 refraction_tex_dims;
    uint2 taa_history_tex_dims;
    uint2 reprojection_tex_dims;
    float3 water_color;
};

ByteAddressBuffer ripple_source_buffer : register(t26, space0);
ByteAddressBuffer autofocus_buf : register(t33, space0);
RWByteAddressBuffer __alias_water_rt_count_buffer : register(u34, space0);
RWByteAddressBuffer __alias_water_rt_ray_buffer : register(u35, space0);
Texture2DArray<float4> shadow_map : register(t18, space0);
Texture2D<float4> water_noise_texture : register(t27, space0);
Texture2D<float4> water_mask : register(t28, space0);
Texture2D<float4> half_depth_tex : register(t29, space0);
Texture2D<float4> refraction_tex : register(t30, space0);
Texture2D<float4> taa_history_tex : register(t31, space0);
Texture2D<float4> reprojection_tex : register(t32, space0);

// vs_main (uses $Globals.current_view_index + instance_buffer) lives here.
#include "water/waterfall_vert.inc.hlsl"
#include "water/water_surface_frag.inc.hlsl"
