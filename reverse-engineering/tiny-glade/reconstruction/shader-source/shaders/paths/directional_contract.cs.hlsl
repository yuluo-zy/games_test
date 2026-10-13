// ==RECONSTRUCTED== shaders/paths/directional_contract.cs.hlsl | target: _paths_directional_contract.cs.hlsl_cs_main_0.bin | evidence: spirv-cross + SPIR-V disasm (directional contraction of the path mask along terrain_sdg gradient; if sdg==0 fallback to smoothstep(0.9,0.95,path_mask); else gather terrain slope -> gaussian-weighted blur radius 1..10, write R16f img_out) | bindings: sampler_lnc s0, sampler_default s9, $Globals b13, path_mask t14, img_out u15, terrain_sdg t16, terrain_heightmap t17 | local_size [8,8,1], gid int3 | status: O (algorithm); residual int3-Load/toolchain + inc-closure diffs

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/consts.hlsl"
#include "inc/utils.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "inc/texture_utils.hlsl"
Texture2D<float4> terrain_heightmap : register(t17, space0);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/uv.hlsl"

// (samplers sampler_lnc/sampler_default come from inc/common.hlsl -> inc/samplers.hlsl)

Texture2D<float4> path_mask : register(t14, space0);
RWTexture2D<half> img_out : register(u15, space0);
Texture2D<float4> terrain_sdg : register(t16, space0);

cbuffer $Globals : register(b13, space0)
{
    uint2 path_mask_dims;
    uint2 img_out_dims;
    uint2 terrain_sdg_dims;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
};

[numthreads(8, 8, 1)]
void cs_main(int3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    uint2 p = uint2(gl_GlobalInvocationID.xy);
    float2 sdg = terrain_sdg.Load(int3(p, 0)).xy;

    if (length(sdg) == 0.0f)
    {
        img_out[p] = smoothstep(0.9f, 0.95f, path_mask.Load(int3(p, 0)).x);
    }
    else
    {
        int2 pdims = int2(path_mask_dims);
        float2 uv = float2(gl_GlobalInvocationID.xy) + float2(0.5f, 0.5f);
        float2 ndc = (uv / float2(pdims)) * 2.0f - float2(1.0f, 1.0f);
        float2 world = ndc * float2(65.0f, 65.0f);
        float2 world_xz = float3(world.x, 0.0f, world.y).xz;

        float2 thm_dims = float2(terrain_heightmap_dims);
        float2 grad_scale = float2(130.0f, 130.0f) / thm_dims;
        float4 g = terrain_heightmap.GatherRed(sampler_default, uv / thm_dims, int2(0, 0));
        float2 grad = float2((g.z + g.y) - (g.w + g.x), (g.x + g.y) - (g.w + g.z)) * float2(0.5f, 0.5f) * 26.5f;
        float slope = length(grad / grad_scale) * 0.5f;
        float k = lerp(0.2f, 1.0f, clamp(slope, 0.0f, 1.0f));

        int radius = clamp(int(ceil((k / (130.0f / float(pdims.x))) * 0.5f)), 1, 10);
        float step = k / float(radius * 2);

        float acc = 0.0f;
        float wsum = 0.0f;
        for (int i = -radius; i <= radius; i++)
        {
            float ti = step * float(i);
            float nti = -ti;
            float w = exp((nti * ti) * 2.0f);
            float2 sample_uv = (world_xz + sdg * step * float(i)) * float2(0.0076923076994717121124267578125f, 0.0076923076994717121124267578125f) + float2(0.5f, 0.5f);
            acc += path_mask.SampleLevel(sampler_lnc, sample_uv, 0.0f).x * w;
            wsum += w;
        }

        img_out[p] = smoothstep(0.9f, 0.95f, acc / wsum);
    }
}
