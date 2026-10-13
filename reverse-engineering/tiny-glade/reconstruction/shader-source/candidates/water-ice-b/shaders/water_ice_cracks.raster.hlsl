// ==RECONSTRUCTED== shaders/water/ice_cracks.raster.hlsl (reconstructed under the flat name
// water_ice_cracks.raster.hlsl to match the scope layout; bin main_source_path is
// shaders/water/ice_cracks.raster.hlsl).
// Evidence: _water_ice_cracks.raster.hlsl_b903e43ffb3da915.bin (SPIR-V, offsets 23/14862).
//   VS: $Globals b13 (current_view_index, water_mask_dims, model_to_world), frame_constants set1/b0;
//   inputs Vertex_Position/Vertex_Color/Vertex_Normal (TEXCOORD0..2); outputs color / world normal /
//   world position.
//   PS: frame_constants only; packs a fixed ice-crack id/color into a uint2 target: high word carries
//   the sun-direction RGB (view_constants[2]) + id bits 0x4D000000, low word the darkened vertex color
//   (sqrt(c*0.4)) + type bits 0x06000000.
// Status: O (algorithm + bindings from SPIR-V).
// NOTE: the f0ad PS is an empty stub (no bindings); single source cannot reproduce both — b903 is the
// reconstruction target (recorded residual).

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "water/water_mask_sampler.inc.hlsl"
#include "inc/consts.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/math.hlsl"
#include "inc/utils.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint2 water_mask_dims;
    float4x4 model_to_world;
};

struct VSOut
{
    float3 c0 : TEXCOORD0;
    float3 c1 : TEXCOORD1;
    float3 c2 : TEXCOORD2;
    float4 pos : SV_Position;
};

VSOut vs_main(float3 Vertex_Position : TEXCOORD0,
              float3 Vertex_Color : TEXCOORD1,
              float3 Vertex_Normal : TEXCOORD2)
{
    VSOut o;
    float4 p = mul(model_to_world, float4(Vertex_Position, 1.0f));
    o.pos = mul(view_constants[current_view_index].world_to_sample, p);
    o.c0 = Vertex_Color;
    o.c1 = mul(model_to_world, float4(Vertex_Normal, 0.0f)).xyz;
    o.c2 = p.xyz;
    return o;
}

uint2 ps_main(float3 c0 : TEXCOORD0) : SV_Target0
{
    // Sun-direction color (view_constants[2] is the sun view).
    float3 sun_color = (mul(view_constants[2].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz * 0.5f) + 0.5f;

    // Darkened vertex color.
    float3 dark = sqrt(clamp(c0 * 0.4f, 0.0f, 1.0f));
    uint color = pack_rgb8(dark);

    uint2 packed = uint2(pack_rgb8(sun_color) + 1291845632u, color);
    packed.y = color | 100663296u;
    return packed;
}
