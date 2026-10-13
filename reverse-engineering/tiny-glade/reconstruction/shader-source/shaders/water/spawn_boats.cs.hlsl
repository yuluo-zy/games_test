// ==RECONSTRUCTED== shaders/water/spawn_boats.cs.hlsl | evidence: spirv-cross + raw disasm of _water_spawn_boats.cs.hlsl_cs_main_0.bin (groupshared best_choice: per-thread score from water-depth min across a boat-sized patch + bow-sample gate + path/building clearance, packed as uint(score*0xFFFFFF00)|localIndex, InterlockedMax; winner writes BoatXform = T(world)*S(0.85)*R(axis normalize(1,0,1), bob)*R(axis normalize(0,1,0), yaw)) | status: O (algorithm from SPIR-V); I (boat size constants 2.2/1.1/0.2 world units, bob amplitude, -0.68 draft)

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/draw_indirect.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/perlin.hlsl"

struct BoatXform
{
    Affine3Packed transform;
};

// Rodrigues rotation around a runtime (normalized) axis — transposed row-major
// 4x4. Axis is passed normalized so the full 9-term matrix is emitted.
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

cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
    uint2 path_mask_dims;
    uint2 water_mask_dims;
    uint2 buildings_mask_dims;
    uint2 grid_dims;
};

RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);
Texture2D<float4> path_mask : register(t15, space0);
Texture2D<float4> water_mask : register(t16, space0);
Texture2D<float4> buildings_mask : register(t17, space0);
RWStructuredBuffer<BoatXform> __alias_instance_buffer : register(u18, space0);

// frame_constants.hlsl (reconstructed inc/) omits the dead unbounded-array decl
// the original used to emit RuntimeDescriptorArray; reproduce it here.
RWStructuredBuffer<uint> _rda_trigger[] : register(u100, space0);

groupshared uint best_choice;

[numthreads(8, 8, 1)]
void cs_main(uint3 id : SV_DispatchThreadID, uint local_index : SV_GroupIndex)
{
    if (local_index == 0u)
        best_choice = 0u;
    GroupMemoryBarrierWithGroupSync();

    float2 uv = (float2(id.xy) + 0.5f) / float2(grid_dims);
    float2 world_pos = (uv - 0.5f) * 130.0f;

    float angle = 40.665630340576171875f * float(id.x + id.y * grid_dims.x);
    float2 dir = float2(cos(angle), sin(angle));

    float2 fwd = dir * 0.016923077404499053955078125f;
    float2 side = float2(dir.y, -dir.x) * 0.003076923079788684844970703125f;
    float2 bow = uv + dir * 0.03384615480899810791015625f;

    float min_water = min(min(min(min(min(
        water_mask.SampleLevel(sampler_lnc, uv, 0.0f).x,
        water_mask.SampleLevel(sampler_lnc, uv - fwd + side, 0.0f).x),
        water_mask.SampleLevel(sampler_lnc, uv + fwd + side, 0.0f).x),
        water_mask.SampleLevel(sampler_lnc, uv - fwd - side, 0.0f).x),
        water_mask.SampleLevel(sampler_lnc, uv + fwd - side, 0.0f).x),
        water_mask.SampleLevel(sampler_lnc, uv - fwd * 1.5f, 0.0f).x);

    bool deep = water_mask.SampleLevel(sampler_lnc, bow, 0.0f).x < 0.5f ? (min_water > 0.5f) : false;

    float deep_score = deep ? min_water : 0.0f;
    float score = (abs(water_mask.SampleLevel(sampler_lnc, uv + fwd, 0.0f).w - water_mask.SampleLevel(sampler_lnc, uv - fwd, 0.0f).w) > 0.1f)
        ? 0.0f
        : ((path_mask.Load(int3(uint2(int2(uv * float2(path_mask_dims))), 0u)).x > 0.0f)
            ? 0.0f
            : deep_score * clamp(buildings_mask.SampleLevel(sampler_lnc, uv + dir * 0.06769230841702222824096679688f, 0.0f).y - 0.3f, 0.0f, 1.0f));

    uint packed = uint(clamp(score, 0.0f, 1.0f) * 4294967040.0f) | local_index;
    uint prev;
    InterlockedMax(best_choice, packed, prev);
    GroupMemoryBarrierWithGroupSync();

    bool is_winner = best_choice == packed && score > 0.0f;

    if (is_winner)
    {
        uint seed = id.x * id.y + id.x;
        float rnd = rand01(hash(seed));
        float bob = 0.05f * sin((fmod(float(game_time_sec), TWO_PI) + game_time_frac) + TWO_PI * rnd);
        float yaw = cos(angle);

        float4x4 identity = float4x4(
            float4(1.0f, 0.0f, 0.0f, 0.0f),
            float4(0.0f, 1.0f, 0.0f, 0.0f),
            float4(0.0f, 0.0f, 1.0f, 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f));
        float4x4 scale = float4x4(
            float4(0.85f, 0.0f, 0.0f, 0.0f),
            float4(0.0f, 0.85f, 0.0f, 0.0f),
            float4(0.0f, 0.0f, 0.85f, 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f));
        float4x4 m = mul(mul(mul(
            affine_translation(float3(world_pos.x, -0.68f, world_pos.y)),
            mul(identity, scale)),
            rotate_axis(normalize(normalize(float3(1.0f, 0.0f, 1.0f))), bob)),
            rotate_axis(normalize(float3(0.0f, 1.0f, 0.0f)), yaw));

        uint base;
        InterlockedAdd(__alias_draw_commands[uint(draw_indirect_buffer_offset)].instanceCount, 1u, base);
        __alias_instance_buffer[base] = BoatXform(pack_affine(m));
    }
}
