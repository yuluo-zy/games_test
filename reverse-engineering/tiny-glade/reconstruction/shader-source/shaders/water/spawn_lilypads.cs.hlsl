// ==RECONSTRUCTED== shaders/water/spawn_lilypads.cs.hlsl | evidence: spirv-cross of _water_spawn_lilypads.cs.hlsl_cs_main_0.bin (animation storage image eased toward garden-clear flag; water band (theme2 -> perlin-warped band); path band; perlin gates; LilypadXform (48B) with random basis * yaw * growth scale; 20% flower -> cmd 15) | status: O (algorithm from SPIR-V); I (thresholds/scale literals, theme constants)

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/draw_indirect.hlsl"
#include "inc/perlin.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/texture_utils.hlsl"
#include "water/water_mask_sampler.inc.hlsl"
#include "paths/path_mask_sampler.inc.hlsl"

struct LilypadXform
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
    uint2 garden_mask_dims;
    uint2 animation_dims;
    int2 grid_dims;
};

RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);
Texture2D<float4> water_mask : register(t15, space0);
Texture2D<float4> path_mask : register(t16, space0);
Texture2D<float4> garden_mask : register(t17, space0);
RWTexture2D<half> animation : register(u18, space0);
RWStructuredBuffer<LilypadXform> __alias_lilypad_buffer : register(u19, space0);
RWStructuredBuffer<LilypadXform> __alias_lilypad_flower_buffer : register(u20, space0);

// frame_constants.hlsl (reconstructed inc/) omits the dead unbounded-array decl
// the original used to emit RuntimeDescriptorArray; reproduce it here.
RWStructuredBuffer<uint> _rda_trigger[] : register(u100, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 id : SV_DispatchThreadID)
{
    if (id.x >= animation_dims.x || id.y >= animation_dims.y)
        return;

    float rx = rand01(hash(id.x));
    float ry = rand01(hash(id.y));
    float2 world_pos = ((float2(id.xy) + 0.5f) / float2(grid_dims) - 0.5f) * 130.0f
        + (lerp(-0.25f, 0.25f, (rx + ry) * 0.5f) * 2.0f).xx;

    float2 uv = world_pos * 0.0076923076994717121124267578125f + 0.5f;
    bool in_bounds = uv.x >= 0.0f && uv.x <= 1.0f && uv.y >= 0.0f && uv.y <= 1.0f;
    float water = in_bounds ? water_mask.SampleLevel(sampler_default, uv, 0.0f).y : 0.0f;

    float garden = smoothstep(0.4f, 0.5f, smoothstep(0.0f, 0.5f, garden_mask.SampleLevel(sampler_default, uv, 0.0f).y));
    animation[id.xy] = lerp(animation[id.xy].x, float(garden > 0.0f), 1.5f * delta_time_seconds);

    bool theme2 = glade_theme_id == 2u;
    float water_band;
    if (theme2)
    {
        water_band = band_falloff(abs((water + perlin(world_pos * 1.83f) * 0.3f) - 0.662f), 2.5445291996002197265625f);
    }
    else
    {
        water_band = band_falloff(abs(water - 0.45f), 3.3333332538604736328125f);
    }

    float path = path_mask.SampleLevel(sampler_llb, uv, 0.0f).x;
    float path_band = band_falloff(abs(path - 0.2f), 5.0f);

    float pad_score = water_band + path_band * smoothstep(0.6f, 0.8f, water);
    float bloom_score = garden * smoothstep(0.4f, 0.5f, water);

    if (pad_score <= 0.0f)
        return;

    bool perlin_gate = theme2
        ? perlin(world_pos) > 0.5f
        : (bloom_score > 0.0f && perlin(world_pos * 2.0f) > 0.5f);

    if (!perlin_gate)
        return;

    float3 pos = float3(world_pos.x, 0.0f, world_pos.y);
    if (glade_theme_id == 4u)
        pos.y = -0.1f;

    float rand = rand01(hash(id.x + id.y));
    float3 scale = (lerp(0.8f, theme2 ? 1.3f : 2.0f, rand)
                     * lerp(0.5f, 1.0f, clamp(pad_score + bloom_score, 0.0f, 1.0f))
                     * lerp(1.0f, 0.75f, bloom_score)).xxx;

    float2 dir2 = float2(rand01(hash(id.x ^ 234u)) * 2.0f - 1.0f, rand01(hash(id.x ^ 8634u)) * 2.0f - 1.0f);
    float3 dir = normalize(float3(dir2.x, 0.0f, dir2.y));
    float3 up = dir.y < 0.0f ? float3(0.0f, -1.0f, 0.0f) : float3(0.0f, 1.0f, 0.0f);
    float4x4 basis = transpose(float4x4(
        float4(dir, 0.0f),
        float4(up, 0.0f),
        float4(cross(dir, up), 0.0f),
        float4(0.0f, 0.0f, 0.0f, 1.0f)));

    float4x4 m = mul(mul(
        affine_translation(pos),
        mul(basis, float4x4(
            float4(scale.x, 0.0f, 0.0f, 0.0f),
            float4(0.0f, scale.y, 0.0f, 0.0f),
            float4(0.0f, 0.0f, scale.z, 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f)))),
        rotate_axis(normalize(float3(0.0f, 1.0f, 0.0f)), 3.14f * rand));

    uint base;
    InterlockedAdd(__alias_draw_commands[uint(draw_indirect_buffer_offset)].instanceCount, 1u, base);
    __alias_lilypad_buffer[base] = LilypadXform(pack_affine(m));

    if (bloom_score > 0.7f && rand01(hash(id.x + id.y + 345u)) > 0.2f)
    {
        float grow = clamp(1.0f - smoothstep(rand01(hash((10u * id.x) + 45u + id.y * 10u)) * 0.2f - 0.2f, 1.0f, animation[id.xy].x), 0.0f, 1.0f);
        float pop = (grow * grow * grow) - (grow * sin(grow * 3.14f) * 1.5f);
        float3 flower_scale = scale * (bloom_score * 0.8f) * (1.0f - pop);
        if (theme2)
            flower_scale *= 2.0f;

        float jx = rand01(hash((id.x + id.y * 2u) + 345u));
        float jy = rand01(hash(id.x * 10u + id.y * 345u));
        float3 fpos = float3(
            pos.x + lerp(-0.3f, 0.3f, jx),
            (pos.y - pop * 0.3f) + jy * 0.1f,
            pos.z + lerp(-0.3f, 0.3f, rand01(hash((id.x + id.y * 3u) - 345u))));

        float4x4 fm = mul(
            affine_translation(fpos),
            mul(basis, float4x4(
                float4(flower_scale.x, 0.0f, 0.0f, 0.0f),
                float4(0.0f, flower_scale.y, 0.0f, 0.0f),
                float4(0.0f, 0.0f, flower_scale.z, 0.0f),
                float4(0.0f, 0.0f, 0.0f, 1.0f))));

        uint fbase;
        InterlockedAdd(__alias_draw_commands[15u].instanceCount, 1u, fbase);
        __alias_lilypad_flower_buffer[fbase] = LilypadXform(pack_affine(fm));
    }
}
