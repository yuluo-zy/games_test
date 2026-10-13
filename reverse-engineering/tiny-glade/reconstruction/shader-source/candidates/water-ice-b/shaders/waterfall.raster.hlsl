// ==RECONSTRUCTED== shaders/waterfall.raster.hlsl
// Evidence: _waterfall.raster.hlsl_b903e43ffb3da915.bin (SPIR-V, offsets 23/50178).
//   VS (from water/waterfall_vert.inc.hlsl): $Globals b13 (17 members incl. current_view_index,
//   water_color), instance_buffer t14 (WaterfallMesh), frame_constants set1/b0; procedural waterfall
//   mesh — per-vertex UV grid, curved fall, 3D value-noise displacement.
//   PS (from water/water_surface_frag.inc.hlsl): same water-surface shading as water_plane but one
//   register higher (shadow_map t18, ripple t26, water_noise t27, water_mask t28, half_depth t29,
//   refraction t30, taa t31, reproj t32, autofocus t33, water_rt_count u34, water_rt_ray u35).
// Status: O (bindings from SPIR-V); shared incs O/I (condensed inner loops).
// NOTE: the f0ad PS is a reduced variant; single source cannot reproduce both — b903 is the target.

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/consts.hlsl"
#include "inc/math.hlsl"
#include "inc/utils.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/perlin.hlsl"

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

#include "water/waterfall_vert.inc.hlsl"

Texture2DArray<float4> shadow_map : register(t18, space0);
ByteAddressBuffer ripple_source_buffer : register(t26, space0);
ByteAddressBuffer autofocus_buf : register(t33, space0);
RWByteAddressBuffer __alias_water_rt_count_buffer : register(u34, space0);
RWByteAddressBuffer __alias_water_rt_ray_buffer : register(u35, space0);
Texture2D<float4> water_noise_texture : register(t27, space0);
Texture2D<float4> water_mask : register(t28, space0);
Texture2D<float4> half_depth_tex : register(t29, space0);
Texture2D<float4> refraction_tex : register(t30, space0);
Texture2D<float4> taa_history_tex : register(t31, space0);
Texture2D<float4> reprojection_tex : register(t32, space0);

#include "water/water_surface_frag.inc.hlsl"
