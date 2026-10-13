// ==RECONSTRUCTED== shaders/path_pebble.raster.hlsl | F-R | bins: _path_pebble.raster.hlsl_b903e43ffb3da915.bin, _path_pebble.raster.hlsl_f0adff76827e73ff.bin | entry vs:vs_main (vs_6_4) ps:ps_main (ps_6_4)
// Path pebble instancing pass. One instanced quad per pebble: the vertex shader
// reads the pebble's affine transform from pebble_placement_buffer (one
// PebblePlacement{ Affine3Packed transform } per instance, 12 column-major
// floats) and emits the world position (transformed normal in C1, world pos in
// C2) into the GBuffer; the pixel shader samples the path dirt texture and
// packs an RGB8 albedo + RGB8 normal into a uint2 GBuffer target.
// bindings: $Globals b13 (current_view_index/dirtpath_1_dims/bool_render_shadows),
//   pebble_placement_buffer t14 (StructuredBuffer<PebblePlacement>), frame_constants b0/space1 (vs)
//   | sampler_llr s1, dirtpath_1 t15 (ps). Capabilities: RuntimeDescriptorArray (forced,
//   SPV_EXT_descriptor_indexing on the SSBO) + Shader on both modules.
// evidence: spirv-cross decompilation of _path_pebble.raster.hlsl_b903e43ffb3da915.bin
//   (workspace/f-r-scratch/modules/_path_pebble.raster.hlsl_b903e43ffb3da915_{0,1}.hlsl.cross)
//   transcribed math/order verbatim. The f0ad variant differs only in the PS
//   (empty ps_main, no bindings) — this file targets the b903 variant.
// status: O (algorithm/order/literals from .cross); residual opcode-count +
//   inc-layer OpSource/OpString diffs are expected (recorded, not fixed).

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
// color_library.hlsl is pulled in before the material chain: the reconstructed
// inc layer's material/solid_material.inc.hlsl -> brdf_lut.hlsl ->
// highlight_color.hlsl references `color_library`, which needs its cbuffer
// declared first (inc-layer dependency, flagged).
#include "inc/color_library.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"

// Per-instance placement record: a packed 3x4 column-major affine transform.
struct PebblePlacement
{
    Affine3Packed transform;
};

// Original dxc build emits SPV_EXT_descriptor_indexing + RuntimeDescriptorArray
// for the structured-buffer SSBO; forced here to match the target module.
[[vk::ext_capability(5302)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
StructuredBuffer<PebblePlacement> pebble_placement_buffer : register(t14, space0);

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint2 dirtpath_1_dims;
    int bool_render_shadows;
};

Texture2D<float4> dirtpath_1 : register(t15, space0);

struct vs_main_input
{
    float3 position : TEXCOORD0;
    float3 normal : TEXCOORD1;
    uint instance_id : SV_InstanceID;
};

struct vs_main_output
{
    float3 c1 : TEXCOORD1;
    float3 c2 : TEXCOORD2;
    float4 position : SV_Position;
};

vs_main_output vs_main(vs_main_input input)
{
    PebblePlacement placement = pebble_placement_buffer[input.instance_id];

    // Column-major affine (data[0..2]=col0, 3..5=col1, 6..8=col2, 9..11=translation):
    // transposed to row-major 3x4 for the mul.
    float3x4 world_transform = transpose(float4x3(
        float3(placement.transform.data[0], placement.transform.data[1], placement.transform.data[2]),
        float3(placement.transform.data[3], placement.transform.data[4], placement.transform.data[5]),
        float3(placement.transform.data[6], placement.transform.data[7], placement.transform.data[8]),
        float3(placement.transform.data[9], placement.transform.data[10], placement.transform.data[11])));

    float3 world_pos = mul(world_transform, float4(input.position, 1.0f));

    vs_main_output output;
    output.position = mul(view_constants[current_view_index].world_to_sample, float4(world_pos, 1.0f));
    output.c1 = mul(world_transform, float4(input.normal, 0.0f));
    output.c2 = world_pos;
    return output;
}

struct ps_main_input
{
    float3 c1 : TEXCOORD1;
    float3 c2 : TEXCOORD2;
};

uint2 ps_main(ps_main_input input) : SV_Target0
{
    float4 dirtpath = dirtpath_1.Sample(sampler_llr, input.c2.xz * 0.1500000059604644775390625f);

    float3 albedo = dirtpath.xyz;
    albedo.x = dirtpath.x * 0.699999988079071044921875f;
    albedo.y = dirtpath.y * 0.89999997615814208984375f;

    float3 normal = (normalize(input.c1) * 0.5f) + 0.5f;
    float3 color = sqrt(clamp(albedo, 0.0f, 1.0f));

    uint color_packed = (uint((clamp(color.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
                       + (uint((clamp(color.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u))
                       + (uint((clamp(color.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);

    uint normal_packed = (uint((clamp(normal.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
                        + (uint((clamp(normal.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u))
                        + (uint((clamp(normal.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)
                        + 4278190080u;

    uint2 result = uint2(normal_packed, color_packed);
    result.y = color_packed | 117440512u;
    return result;
}
