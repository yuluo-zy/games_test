// ==RECONSTRUCTED== shaders/water_plane_refraction_copy.raster.hlsl
// Evidence: _water_plane_refraction_copy.raster.hlsl_b903e43ffb3da915.bin (SPIR-V, offsets 23/13790).
//   VS: $Globals b13 (current_view_index, bool_render_shadows, model_to_world, terrain_heightmap_dims,
//   visual_terrain_heightmap_dims, water_mask_dims, refraction_source_tex_dims), frame_constants set1/b0.
//   PS: refraction_source_tex t18, SV_Position in, SV_Target0 out (full-res copy of the source image).
//   spirv-cross + disassembly: VS transforms the water plane mesh (xz*130 world scale, y=0) and
//   forwards a fixed color (0.05,0.05,0.2) + green normal + world position; PS is a straight
//   refraction-source copy (Load at the pixel coordinate, alpha=1).
// Status: O (algorithm + bindings from SPIR-V).
// NOTE: the f0ad variant's PS is an empty stub (no bindings); a single source cannot reproduce both
// variants — the full b903 variant is the reconstruction target (recorded residual).

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
    int bool_render_shadows;
    float4x4 model_to_world;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 water_mask_dims;
    uint2 refraction_source_tex_dims;
};

Texture2D<float4> refraction_source_tex : register(t18, space0);

struct VSOut
{
    float3 c0 : TEXCOORD0;
    float3 c1 : TEXCOORD1;
    float3 c2 : TEXCOORD2;
    float4 pos : SV_Position;
};

VSOut vs_main(float3 Vertex_Position : TEXCOORD0)
{
    VSOut o;
    float4 p = mul(model_to_world, float4(Vertex_Position, 1.0f));
    float2 xz = p.xz * 130.0f;
    float4 wp = float4(xz.x, p.y, xz.y, p.w);
    wp.y = 0.0f;
    o.pos = mul(view_constants[current_view_index].world_to_sample, wp);
    o.c0 = float3(0.05f, 0.05f, 0.2f);
    o.c1 = float3(0.0f, 1.0f, 0.0f);
    o.c2 = wp.xyz;
    return o;
}

float4 ps_main(float4 gl_FragCoord : SV_Position) : SV_Target0
{
    return float4(refraction_source_tex.Load(int3(uint2(gl_FragCoord.xy), 0)).xyz, 1.0f);
}
