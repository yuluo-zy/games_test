// ==RECONSTRUCTED== shaders/garden/marching_squares.cs.hlsl | evidence: spirv-cross + raw disasm of _garden_marching_squares.cs.hlsl_cs_main_0.bin (per 8x8 group loads a 9x9 garden/path/discard sample grid into groupshared; each thread classifies its cell's 4 corners; for the non-degenerate marching-squares cases it bisects the garden-mask isoline (0.4) at 2 contour crossings and drops GardenStone{transform,color} instances (stride 64) at `max(dist*2.5,1)` intervals along the segment) | status: O (algorithm + groupshared layout from SPIR-V); I (case table constants; stone scale/color literals)

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/draw_indirect.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/perlin.hlsl"
Texture2D<float4> terrain_heightmap : register(t20, space0);
#include "inc/terrain_heightmap.inc.hlsl"

struct GardenStone
{
    Affine3Packed transform;
    float4 color;
};

static const uint2 CORNER[4] = {
    uint2(0u, 0u), uint2(1u, 0u), uint2(1u, 1u), uint2(0u, 1u)
};

cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
    uint2 garden_mask_dims;
    uint2 path_mask_dims;
    uint2 water_mask_dims;
    uint2 rocky_terrain_dims;
    uint2 foliage_exclusion_mask_dims;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 contouring_grid_dims;
};

RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);
Texture2D<float4> garden_mask : register(t15, space0);
Texture2D<float4> path_mask : register(t16, space0);
Texture2D<float4> water_mask : register(t17, space0);
Texture2D<float4> rocky_terrain : register(t18, space0);
Texture2D<float4> foliage_exclusion_mask : register(t19, space0);
RWStructuredBuffer<GardenStone> __alias_instance_buffer : register(u22, space0);

// frame_constants.hlsl (reconstructed inc/) omits the dead unbounded-array decl
// the original used to emit RuntimeDescriptorArray; reproduce it here.
RWStructuredBuffer<uint> _rda_trigger[] : register(u100, space0);

groupshared uint valid_garden_samples;
groupshared float2 group_ws_pos[9][9];
groupshared float group_garden_mask[9][9];
groupshared float group_path_mask[9][9];
groupshared bool group_discard_mask[9][9];

// Sample the garden-mask isoline field (value - 0.4) at a world xz position.
float garden_iso(float2 ws)
{
    float2 uv = ws * 0.0076923076994717121124267578125f + 0.5f;
    return garden_mask.SampleLevel(sampler_default, uv, 0.0f).y - 0.4f;
}

// Two-step bisection of the 0.4 isoline crossing between world positions a,b.
float2 bisect_iso(float2 a, float2 b)
{
    float fa = garden_iso(a);
    float fb = garden_iso(b);
    float2 p0 = a;
    float2 p1 = b;
    [unroll]
    for (uint i = 0u; i < 2u; i++)
    {
        float2 m = (p0 + p1) * 0.5f;
        float fm = garden_iso(m);
        bool left = (fm * fa) < 0.0f;
        fa = left ? fa : fm;
        fb = left ? fm : fb;
        p0 = left ? p0 : m;
        p1 = left ? m : p1;
    }
    return lerp(p1, p0, clamp(fb / (fb - fa), 0.0f, 1.0f).xx);
}

// Place stones along the segment from a to b (world xz), one per interval.
void place_stones(float2 a, float2 b, uint grid_id)
{
    float2 p0 = float3(a.x, 0.4f, a.y).xz;
    float2 p1 = float3(b.x, 0.4f, b.y).xz;
    float dist = distance(p0, p1);
    if (dist < 0.4f)
        return;

    uint n = uint(max(dist * 2.5f, 1.0f));
    for (uint i = 0u; i <= n; i++)
    {
        float2 t = ((float(i) / float(n)) + (float(i + 1u) / float(n))) * 0.5f;
        float2 pos = lerp(p0, p1, t.xx);
        int seed = int(grid_id) + int(i) * 100;

        float rx = rand01(hash(uint(seed ^ 2346)));
        float ry = rand01(hash(uint(seed ^ 3459)));
        float rz = rand01(hash(uint(seed ^ 234)));
        float3 dir = normalize(float3(rx * 2.0f - 1.0f, (ry * 2.0f - 1.0f) * 0.2f, rz * 2.0f - 1.0f));
        float3 up = dir.y < 0.0f ? float3(0.0f, -1.0f, 0.0f) : float3(0.0f, 1.0f, 0.0f);

        float sw = lerp(0.8f, 1.5f, rand01(hash(uint(seed ^ 346))));
        float3 scale = float3(sw, lerp(0.8f, 1.3f, rand01(hash(uint(seed ^ 346)))), sw) * 0.75f;

        float4x4 basis = transpose(float4x4(
            float4(dir, 0.0f),
            float4(up, 0.0f),
            float4(cross(dir, up), 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f)));
        float4x4 s = float4x4(
            float4(scale.x, 0.0f, 0.0f, 0.0f),
            float4(0.0f, scale.y, 0.0f, 0.0f),
            float4(0.0f, 0.0f, scale.z, 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f));

        float height = get_terrain_height(terrain_heightmap, sampler_default, pos);
        float4x4 m = mul(
            affine_translation(float3(pos.x, height, pos.y)),
            mul(basis, s));

        uint base;
        InterlockedAdd(__alias_draw_commands[uint(draw_indirect_buffer_offset)].instanceCount, 1u, base);

        float c0 = rand01(hash(uint((pos.x + pos.y) * 10.0f + 640.0f)));
        float c1 = rand01(hash(uint((pos.x + pos.y) * 10.0f + 987.0f)));
        GardenStone stone;
        stone.transform = pack_affine(m);
        stone.color = float4(c0, c1, 0.0f, 0.0f);
        __alias_instance_buffer[base] = stone;
    }
}

[numthreads(8, 8, 1)]
void cs_main(uint3 gid : SV_GroupID, uint3 lid : SV_GroupThreadID, uint3 id : SV_DispatchThreadID, uint li : SV_GroupIndex)
{
    float2 grid_dims_f = float2(contouring_grid_dims);

    if (li == 0u)
        valid_garden_samples = 0u;
    GroupMemoryBarrierWithGroupSync();

    for (uint gy = lid.y; gy <= 8u; gy += 8u)
    {
        for (uint gx = lid.x; gx <= 8u; gx += 8u)
        {
            float2 ws = ((float2((gid.xy * uint2(8u, 8u)) + uint2(gx, gy)) / grid_dims_f) - 0.5f) * 130.0f;
            group_ws_pos[gx][gy] = ws;

            float3 w3 = (ws.xxy * 0.0076923076994717121124267578125f) + 0.5f;
            uint2 px = uint2(int2(int(w3.x * float(garden_mask_dims.x)), int(w3.z * float(garden_mask_dims.y))));
            float g = garden_mask.Load(int3(px, 0u)).y;
            group_garden_mask[gx][gy] = g;
            if (g >= 0.4f)
            {
                uint prev;
                InterlockedAdd(valid_garden_samples, 1u, prev);
            }
        }
    }
    GroupMemoryBarrierWithGroupSync();

    if (valid_garden_samples == 0u)
        return;

    for (uint py = lid.y; py <= 8u; py += 8u)
    {
        for (uint px2 = lid.x; px2 <= 8u; px2 += 8u)
        {
            float2 ws = group_ws_pos[px2][py];
            float3 w3 = (ws.xxy * 0.0076923076994717121124267578125f) + 0.5f;
            uint2 ppx = uint2(int2(int(w3.x * float(garden_mask_dims.x)), int(w3.z * float(garden_mask_dims.y))));

            group_path_mask[px2][py] = path_mask.Load(int3(ppx, 0u)).x;

            bool dmask = rocky_terrain.Load(int3(ppx, 0u)).x > 0.0f
                        || water_mask.Load(int3(ppx, 0u)).y > 0.3f
                        || foliage_exclusion_mask.SampleLevel(sampler_llb, ws * 0.0076923076994717121124267578125f + 0.5f, 0.0f).x > 0.0f;
            group_discard_mask[px2][py] = dmask;
        }
    }
    GroupMemoryBarrierWithGroupSync();

    if (float(id.x) > grid_dims_f.x || float(id.y) > grid_dims_f.y)
        return;

    uint config = 0u;
    float2 corner_ws[4];
    bool skip = false;
    [unroll]
    for (int c = 0; c < 4; c++)
    {
        uint2 p = lid.xy + CORNER[c];
        if (group_discard_mask[p.x][p.y])
        {
            skip = true;
            break;
        }
        if (group_garden_mask[p.x][p.y] >= 0.4f && group_path_mask[p.x][p.y] < 0.1f)
            config |= (8u >> (uint(c) & 31u));
        corner_ws[c] = group_ws_pos[p.x][p.y];
    }
    if (skip)
        return;
    if (config == 0u || config == 15u)
        return;

    uint grid_id = (id.x * id.y) + id.x;

    // Marching-squares contour extraction: each configuration yields one or
    // two directed segment crossings of the 0.4 isoline between cell edges.
    if (config == 1u || config == 14u)
    {
        float2 a = bisect_iso(corner_ws[0], corner_ws[3]);
        float2 b = bisect_iso(corner_ws[3], corner_ws[2]);
        place_stones(a, b, grid_id);
    }
    else if (config == 2u || config == 13u)
    {
        float2 a = bisect_iso(corner_ws[1], corner_ws[2]);
        float2 b = bisect_iso(corner_ws[2], corner_ws[3]);
        place_stones(a, b, grid_id);
    }
    else if (config == 3u || config == 12u)
    {
        float2 a = bisect_iso(corner_ws[0], corner_ws[1]);
        float2 b = bisect_iso(corner_ws[1], corner_ws[2]);
        place_stones(a, b, grid_id);
    }
    else if (config == 4u || config == 11u)
    {
        float2 a = bisect_iso(corner_ws[1], corner_ws[2]);
        float2 b = bisect_iso(corner_ws[2], corner_ws[3]);
        place_stones(a, b, grid_id);
    }
    else if (config == 5u || config == 10u)
    {
        float2 a = bisect_iso(corner_ws[0], corner_ws[1]);
        float2 b = bisect_iso(corner_ws[1], corner_ws[2]);
        place_stones(a, b, grid_id);
        a = bisect_iso(corner_ws[2], corner_ws[3]);
        b = bisect_iso(corner_ws[3], corner_ws[0]);
        place_stones(a, b, grid_id);
    }
    else if (config == 6u || config == 9u)
    {
        float2 a = bisect_iso(corner_ws[0], corner_ws[3]);
        float2 b = bisect_iso(corner_ws[3], corner_ws[2]);
        place_stones(a, b, grid_id);
        a = bisect_iso(corner_ws[2], corner_ws[1]);
        b = bisect_iso(corner_ws[1], corner_ws[0]);
        place_stones(a, b, grid_id);
    }
    else // 7u || 8u
    {
        float2 a = bisect_iso(corner_ws[0], corner_ws[3]);
        float2 b = bisect_iso(corner_ws[3], corner_ws[2]);
        place_stones(a, b, grid_id);
        a = bisect_iso(corner_ws[2], corner_ws[1]);
        b = bisect_iso(corner_ws[1], corner_ws[0]);
        place_stones(a, b, grid_id);
    }
}
