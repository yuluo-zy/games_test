// ==RECONSTRUCTED== shaders/critters/firefly_vis_query.cs.hlsl | target: _critters_firefly_vis_query.cs.hlsl_cs_main_0.bin | entry cs:cs_main LocalSize [32,1,1] | evidence: spirv-cross + SPIR-V access/type inspection (per-firefly depth-window visibility query, 5-sample +2/-2 grid, temporal ease toward the frame ratio) | bindings: sampler_default s9, $Globals b13, depth_tex t14, terrain_heightmap t15, firefly_buffer t17, __alias_firefly_visibility_buffer u18, frame_constants set1/b0 | status: R (reconstruction)

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/frame_constants.hlsl"
Texture2D<float4> terrain_heightmap : register(t15);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/math_const.hlsl"

// INC-issue workaround: original samplers.hlsl declared s0..s12 (13 samplers);
// inc/samplers.hlsl carries only s0..s4,s9..s12, so the missing s5..s8 are
// declared here to reproduce the original $Globals binding (set0/b13).

struct Firefly
{
    float3 position;
    uint pad;
};

StructuredBuffer<Firefly> firefly_buffer : register(t17);
RWStructuredBuffer<float> __alias_firefly_visibility_buffer : register(u18, space0);
Texture2D<float4> depth_tex : register(t14);

uint2 depth_tex_dims;
uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;

// Depth-window probe offsets around the projected pixel.
static const int2 vis_offsets[5] = { int2(0, 0), int2(1, 1), int2(1, -1), int2(-1, 1), int2(-1, -1) };

[numthreads(32, 1, 1)]
void cs_main(uint3 id : SV_DispatchThreadID)
{
    float3 firefly_pos = firefly_buffer[id.x].position;
    float4 proj = mul(view_constants[0].world_to_sample,
        float4(firefly_pos + float3(0.0f,
            lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, firefly_pos.xz * 0.0076923076994717121124267578125f.xx + 0.5f.xx, 0.0f).x),
            0.0f), 1.0f));
    float4 ndc = proj / proj.w.xxxx;
    float2 uv = (ndc.xy * float2(0.5f, -0.5f)) + 0.5f.xx;

    float visible_count = 0.0f;
    for (uint i = 0u; i < 5u; i++)
    {
        visible_count += float(depth_tex.Load(int3(uint2(float2(depth_tex_dims) * uv) + uint2(int2(2, 2) * vis_offsets[i]), 0u)).x < ndc.z);
    }
    float ratio = visible_count * 0.20000000298023223876953125f;
    __alias_firefly_visibility_buffer[id.x] = lerp(
        clamp(clamp(__alias_firefly_visibility_buffer[id.x], 0.0f, 1.0f), ratio - 0.20000000298023223876953125f, ratio + 0.20000000298023223876953125f),
        ratio,
        0.0500000007450580596923828125f);
}
