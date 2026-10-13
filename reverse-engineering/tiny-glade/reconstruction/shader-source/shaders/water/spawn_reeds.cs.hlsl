// ==RECONSTRUCTED== shaders/water/spawn_reeds.cs.hlsl | evidence: spirv-cross + raw disasm of _water_spawn_reeds.cs.hlsl_cs_main_0.bin (jittered world pos; water-mask height bounds-checked sample; water band falloff around 0.5 + path falloff around 0.2; perlin(world+20)>0.6 && water>0.55 && buildings<0.1 gate; Reed = T*S*R yaw) | status: O (algorithm from SPIR-V); I (function names, falloff literals)

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/draw_indirect.hlsl"
#include "inc/perlin.hlsl"
#include "water/water_mask_sampler.inc.hlsl"
#include "inc/texture_utils.hlsl"
#include "paths/path_mask_sampler.inc.hlsl"

struct Reed
{
    Affine3Packed xform;
};

// Rodrigues rotation around a runtime (normalized) axis — transposed row-major.
float4x4 rotate_axis(float3 axis, float angle)
{
    float c = cos(angle);
    float s = sin(angle);
    float omc = 1.0f - c;
    float x = axis.x;
    float y = axis.y;
    float z = axis.z;
    float cxx = omc * x;
    float cyy = omc * y;
    float czz = omc * z;
    return transpose(float4x4(
        float4(cxx * x + c, cxx * y - z * s, czz * x + y * s, 0.0f),
        float4(cxx * y + z * s, cyy * y + c, cyy * z - x * s, 0.0f),
        float4(czz * x - y * s, cyy * z + x * s, czz * z + c, 0.0f),
        float4(0.0f, 0.0f, 0.0f, 1.0f)));
}

// Banded falloff around `center`: 1 at center, 0 at distance >= half_width.
float band_falloff(float d, float inv_half_width)
{
    float t = d * inv_half_width;
    return 1.0f - (t * t) * (3.0f - d * (2.0f * inv_half_width));
}

cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
    uint2 water_mask_dims;
    uint2 path_mask_dims;
    uint2 buildings_mask_non_blurred_dims;
    int2 grid_dims;
};

RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);
Texture2D<float4> water_mask : register(t15, space0);
Texture2D<float4> path_mask : register(t16, space0);
Texture2D<float4> buildings_mask_non_blurred : register(t17, space0);
RWStructuredBuffer<Reed> __alias_instance_buffer : register(u18, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 id : SV_DispatchThreadID)
{
    uint seed = id.x * id.y + id.x;
    float rnd = rand01(hash(seed));

    float2 world_pos = ((float2(id.xy) + 0.5f) / float2(grid_dims) - 0.5f) * 130.0f + lerp(-0.4f, 0.4f, rnd).xx;

    float2 uv = world_pos * 0.0076923076994717121124267578125f + 0.5f;
    bool in_bounds = uv.x >= 0.0f && uv.x <= 1.0f && uv.y >= 0.0f && uv.y <= 1.0f;

    float water = in_bounds ? water_mask.SampleLevel(sampler_default, uv, 0.0f).y : 0.0f;
    float path = path_mask.SampleLevel(sampler_llb, uv, 0.0f).x;

    float water_band = band_falloff(abs(water - 0.5f), 3.3333332538604736328125f);
    float path_band = band_falloff(abs(path - 0.2f), 5.0f);

    float score = (water_band + path_band * smoothstep(0.6f, 0.8f, water)) - smoothstep(0.2f, 0.25f, path);

    if (score > 0.0f)
    {
        if (!(perlin(world_pos + 20.0f) > 0.6f && water > 0.55f
              && buildings_mask_non_blurred.SampleLevel(sampler_default, uv, 0.0f).x < 0.1f))
            return;

        float scale = lerp(1.0f, 1.5f, rand01(hash(id.x + id.y)));
        float yaw = 3.14f * rnd;

        float4x4 identity = float4x4(
            float4(1.0f, 0.0f, 0.0f, 0.0f),
            float4(0.0f, 1.0f, 0.0f, 0.0f),
            float4(0.0f, 0.0f, 1.0f, 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f));
        float4x4 s = float4x4(
            float4(scale, 0.0f, 0.0f, 0.0f),
            float4(0.0f, scale * lerp(0.5f, 1.0f, score * score), 0.0f, 0.0f),
            float4(0.0f, 0.0f, scale, 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f));

        float4x4 m = mul(mul(
            affine_translation(float3(world_pos.x, -1.0f, world_pos.y)),
            mul(identity, s)),
            rotate_axis(normalize(float3(0.0f, 1.0f, 0.0f)), yaw));

        uint base;
        InterlockedAdd(__alias_draw_commands[uint(draw_indirect_buffer_offset)].instanceCount, 1u, base);
        __alias_instance_buffer[base] = Reed(pack_affine(m));
    }
}
