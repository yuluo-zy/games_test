// ==RECONSTRUCTED== shaders/water_plane.raster.hlsl
// Evidence: _water_plane.raster.hlsl_b903e43ffb3da915.bin (SPIR-V, offsets 23/24054).
//   VS: $Globals b13 (18 members incl. current_view_index, model_to_world, water_color),
//   frame_constants set1/b0; flattens the water plane mesh (xz*130 world scale, y=-0.01), emits a
//   green normal and the world position.
//   PS (from water/water_surface_frag.inc.hlsl): bindings sampler_lnc s0, sampler_llr s1, sampler_nnc
//   s2, sampler_llc s3, sampler_shadow_nearest s11, sampler_shadow_linear s12, $Globals b13,
//   shadow_map t19 (2DArray), ripple_source_buffer t25, water_noise_texture t26, water_mask t27,
//   half_depth_tex t28, refraction_tex t29, taa_history_tex t30, reprojection_tex t31,
//   autofocus_buf t32, __alias_water_rt_count_buffer u33, __alias_water_rt_ray_buffer u34,
//   frame_constants set1/b0.
// Status: O (bindings/VS from SPIR-V); PS shared inc O/I (condensed inner loops).
// NOTE: the f0ad PS is a reduced variant (no shadows/refraction/mask/history); single source cannot
// reproduce both — b903 is the reconstruction target (recorded residual).

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
    float4x4 model_to_world;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint history_valid;
    uint2 terrain_lighting_texture_dims;
    uint2 blue_noise_texture_dims;
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

Texture2DArray<float4> shadow_map : register(t19, space0);
ByteAddressBuffer ripple_source_buffer : register(t25, space0);
ByteAddressBuffer autofocus_buf : register(t32, space0);
RWByteAddressBuffer __alias_water_rt_count_buffer : register(u33, space0);
RWByteAddressBuffer __alias_water_rt_ray_buffer : register(u34, space0);
Texture2D<float4> water_noise_texture : register(t26, space0);
Texture2D<float4> water_mask : register(t27, space0);
Texture2D<float4> half_depth_tex : register(t28, space0);
Texture2D<float4> refraction_tex : register(t29, space0);
Texture2D<float4> taa_history_tex : register(t30, space0);
Texture2D<float4> reprojection_tex : register(t31, space0);

#include "water/water_surface_frag.inc.hlsl"

struct VSOut
{
    float3 c0 : TEXCOORD0;
    float3 c1 : TEXCOORD1;
    float4 pos : SV_Position;
};

VSOut vs_main(float3 Vertex_Position : TEXCOORD0)
{
    VSOut o;
    float4 p = mul(model_to_world, float4(Vertex_Position, 1.0f));
    float2 xz = p.xz * 130.0f;
    float4 wp = float4(xz.x, p.y, xz.y, p.w);
    wp.y = -0.01f;
    o.pos = mul(view_constants[current_view_index].world_to_sample, wp);
    o.c0 = float3(0.0f, 1.0f, 0.0f);
    o.c1 = wp.xyz;
    return o;
}
