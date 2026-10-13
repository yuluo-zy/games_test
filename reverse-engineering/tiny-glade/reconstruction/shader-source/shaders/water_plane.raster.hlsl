// ==RECONSTRUCTED== shaders/water_plane.raster.hlsl
// SOURCE PATH : shaders/water_plane.raster.hlsl
// KIND        : raster_main
// COMPILED AS MAIN IN (2 bins):
//   _water_plane.raster.hlsl_b903e43ffb3da915.bin  (full water surface render)
//   _water_plane.raster.hlsl_f0adff76827e73ff.bin  (simplified: wave normal + half-res
//     reflection march + software-RT ray emission only)
// entries: vs:vs_main  ps:ps_main
// Status: O (bindings/entry; vs + ps transcribed from spirv-cross)
// Evidence: spirv-cross vs (plane at y=-0.01, xz*130, C1=world) and ps (from the shared
//   water/water_surface_frag.inc.hlsl: waves.inc 4-octave + ripple-source displacement,
//   reflection + refraction half-depth marches, cascade-shadow PCSS/PCF, autofocus DoF coc,
//   software-RT reflection-ray emission, specular BRDF, water attenuation + foam).
// MERGE (water-ice review water_plane.raster.hlsl): A base; PS moved into the shared
//   water/water_surface_frag.inc.hlsl (B's layout — the bin's PS lives in that inc),
//   with the bin's theme-4 fast path + TAA jitter folded in there. WATER_SURFACE_FULL
//   (default 1 = b903) gates the two PS variants (f0ad = 0).
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
// NOTE: original embedded list includes inc/terrain_heightmap.inc.hlsl, but the
// reconstructed inc references a consumer-declared `terrain_heightmap` texture
// this shader never binds — the include is omitted (see decisions.md).
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
// inc/blue_noise.hlsl no longer declares the texture (per-review register fix);
// this family binds blue_noise_texture elsewhere — not needed by the PS.
#include "inc/hammersley.hlsl"
#include "inc/rt.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/shadow.hlsl"
#include "inc/cascades.hlsl"
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

ByteAddressBuffer ripple_source_buffer : register(t25, space0);
ByteAddressBuffer autofocus_buf : register(t32, space0);
RWByteAddressBuffer __alias_water_rt_count_buffer : register(u33, space0);
RWByteAddressBuffer __alias_water_rt_ray_buffer : register(u34, space0);
Texture2DArray<float4> shadow_map : register(t19, space0);
Texture2D<float4> water_noise_texture : register(t26, space0);
Texture2D<float4> water_mask : register(t27, space0);
Texture2D<float4> half_depth_tex : register(t28, space0);
Texture2D<float4> refraction_tex : register(t29, space0);
Texture2D<float4> taa_history_tex : register(t30, space0);
Texture2D<float4> reprojection_tex : register(t31, space0);

#include "water/water_surface_frag.inc.hlsl"

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    float4 world = mul(model_to_world, float4(vin.Vertex_Position, 1.0f));
    float2 scaled = world.xz * 130.0f.xx;
    float4 p = float4(scaled.x, world.y, scaled.y, world.w);
    p.y = -0.00999999977648258209228515625f;
    o.position = mul(view_constants[current_view_index].world_to_sample, p);
    o.C0 = float3(0.0f, 1.0f, 0.0f);
    o.C1 = p.xyz;
    return o;
}
