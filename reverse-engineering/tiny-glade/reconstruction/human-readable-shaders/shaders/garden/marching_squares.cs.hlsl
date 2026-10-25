// PATCHED: modules es.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/garden/marching_squares.cs.hlsl
// 功能:花园石头放置。每个 8x8 工作组把 9x9 的花园/路径/丢弃采样网格载入
// groupshared;每线程分类其格子的 4 角;对非退化 marching-squares 情形在
// 花园遮罩等值线(0.4)的两个轮廓交点做二分,沿线段按 `max(dist*2.5,1)`
// 间隔放置 GardenStone{transform,color} 实例(stride 64)。
// 依据:_garden_marching_squares.cs.hlsl_cs_main_0.bin 的 spirv-cross + 原始反汇编。
// 状态:O (算法 + groupshared 布局取自 SPIR-V);I (case 表常量;石头缩放/颜色字面量)

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
    // 先把 9x9 花园/路径/丢弃采样网格载入 groupshared(valid_garden_samples 记录有效格数)。
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

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// 转译自 _garden_marching_squares.cs.hlsl_cs_main_0.bin(spirv-cross;行为精确,编号变量已统一清理)
// 计算着色器:marching-squares 花园石头放置(逻辑同重建段,算术为 oracle 逐字)。
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct DrawIndexedIndirectCommand
{
    uint count;
    uint instanceCount;
    uint firstIndex;
    uint vertexOffset;
    uint firstInstance;
};

struct Affine3Packed
{
    float data[12];
};

struct GardenStone
{
    Affine3Packed transform;
    float4 color;
};

static uint v159;

static const uint2 v164[4] = { uint2(0u, 0u), uint2(1u, 0u), uint2(1u, 1u), uint2(0u, 1u) };

RWByteAddressBuffer __alias_draw_commands : register(u13, space0);
cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset : packoffset(c0);
    uint2 garden_mask_dims : packoffset(c0.y);
    uint2 path_mask_dims : packoffset(c1);
    uint2 water_mask_dims : packoffset(c1.z);
    uint2 rocky_terrain_dims : packoffset(c2);
    uint2 foliage_exclusion_mask_dims : packoffset(c2.z);
    uint2 terrain_heightmap_dims : packoffset(c3);
    uint2 visual_terrain_heightmap_dims : packoffset(c3.z);
    uint2 contouring_grid_dims : packoffset(c4);
};

RWByteAddressBuffer __alias_instance_buffer : register(u22, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> garden_mask : register(t15, space0);
Texture2D<float4> path_mask : register(t16, space0);
Texture2D<float4> water_mask : register(t17, space0);
Texture2D<float4> rocky_terrain : register(t18, space0);
Texture2D<float4> foliage_exclusion_mask : register(t19, space0);
Texture2D<float4> terrain_heightmap : register(t20, space0);

static uint3 gl_WorkGroupID;
static uint3 gl_LocalInvocationID;
static uint3 gl_GlobalInvocationID;
static uint gl_LocalInvocationIndex;
struct SPIRV_Cross_Input
{
    uint3 gl_WorkGroupID : SV_GroupID;
    uint3 gl_LocalInvocationID : SV_GroupThreadID;
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
    uint gl_LocalInvocationIndex : SV_GroupIndex;
};

groupshared uint valid_garden_samples;
groupshared float2 group_ws_pos[9][9];
groupshared float group_garden_mask[9][9];
groupshared float group_path_mask[9][9];
groupshared bool group_discard_mask[9][9];

void comp_main()
{
    do
    {
        float2 v187 = float2(contouring_grid_dims);
        if (0u == gl_LocalInvocationIndex)
        {
            valid_garden_samples = 0u;
        }
        GroupMemoryBarrierWithGroupSync();
        for (uint v193 = gl_LocalInvocationID.y; v193 <= 8u; v193 += 8u)
        {
            for (uint v201 = gl_LocalInvocationID.x; v201 <= 8u; v201 += 8u)
            {
                float2 v215 = ((float2((gl_WorkGroupID.xy * uint2(8u, 8u)) + uint2(v201, v193)) / float2(uint2(v187))) - 0.5f.xx) * 130.0f.xx;
                group_ws_pos[v201][v193] = v215;
                int2 v220 = int2(garden_mask_dims);
                float3 v222 = (v215.xxy * 0.0076923076994717121124267578125f.xxx) + 0.5f.xxx;
                float4 v236 = garden_mask.Load(int3(uint2(int2(int(v222.x * float(v220.x)), int(v222.z * float(v220.y)))), 0u));
                float v237 = v236.y;
                group_garden_mask[v201][v193] = v237;
                if (v237 >= 0.4000000059604644775390625f)
                {
                    uint v242;
                    InterlockedAdd(valid_garden_samples, 1u, v242);
                }
            }
        }
        GroupMemoryBarrierWithGroupSync();
        if (0u == valid_garden_samples)
        {
            break;
        }
        for (uint v248 = gl_LocalInvocationID.y; v248 <= 8u; v248 += 8u)
        {
            for (uint v256 = gl_LocalInvocationID.x; v256 <= 8u; v256 += 8u)
            {
                int2 v267 = int2(garden_mask_dims);
                float3 v269 = (group_ws_pos[v256][v248].xxy * 0.0076923076994717121124267578125f.xxx) + 0.5f.xxx;
                uint2 v288 = uint2(int2(int(v269.x * float(v267.x)), int(v269.z * float(v267.y))));
                group_path_mask[v256][v248] = path_mask.Load(int3(v288, 0u)).x;
                bool v304;
                if (!(rocky_terrain.Load(int3(v288, 0u)).x > 0.0f))
                {
                    v304 = water_mask.Load(int3(v288, 0u)).y > 0.300000011920928955078125f;
                }
                else
                {
                    v304 = true;
                }
                bool v309;
                if (!v304)
                {
                    v309 = foliage_exclusion_mask.SampleLevel(sampler_llb, (group_ws_pos[v256][v248] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x > 0.0f;
                }
                else
                {
                    v309 = true;
                }
                group_discard_mask[v256][v248] = v309;
            }
        }
        GroupMemoryBarrierWithGroupSync();
        bool v322;
        if (!(float(gl_GlobalInvocationID.x) > v187.x))
        {
            v322 = float(gl_GlobalInvocationID.y) > v187.y;
        }
        else
        {
            v322 = true;
        }
        if (v322)
        {
            break;
        }
        uint v326;
        v326 = 0u;
        float2 v176[4];
        uint v327;
        bool v362;
        int v329 = 0;
        [unroll]
        for (;;)
        {
            if (v329 < 4)
            {
                uint2 v336 = gl_LocalInvocationID.xy + v164[v329];
                uint v337 = v336.x;
                uint v338 = v336.y;
                if (group_discard_mask[v337][v338])
                {
                    v362 = true;
                    break;
                }
                if ((int(group_garden_mask[v337][v338] >= 0.4000000059604644775390625f) & int(group_path_mask[v337][v338] < 0.100000001490116119384765625f)) != 0)
                {
                    v327 = v326 | (8u >> (uint(v329) & 31u));
                }
                else
                {
                    v327 = v326;
                }
                v176[v329] = group_ws_pos[v337][v338];
                v326 = v327;
                v329++;
                continue;
            }
            else
            {
                v362 = false;
                break;
            }
        }
        if (v362)
        {
            break;
        }
        int v367 = int((gl_GlobalInvocationID.x * gl_GlobalInvocationID.y) + gl_GlobalInvocationID.x);
        int v368 = int(v326);
        bool v375;
        if (!(v368 == 0))
        {
            v375 = v368 == 15;
        }
        else
        {
            v375 = true;
        }
        if (v375)
        {
        }
        else
        {
            bool v384;
            if (!(v368 == 1))
            {
                v384 = v368 == 14;
            }
            else
            {
                v384 = true;
            }
            if (v384)
            {
                do
                {
                    float2 v401 = (v176[3] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                    float v407;
                    float v410;
                    float2 v412;
                    float2 v414;
                    v407 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v176[0] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                    v410 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, v401, 0.0f).y;
                    v412 = v176[0];
                    v414 = v176[3];
                    for (uint v416 = 0u; v416 < 2u; )
                    {
                        float2 v420 = v412 + v414;
                        float2 v421 = v420 * 0.5f;
                        float v427 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v421 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                        float v428 = v427 * v407;
                        bool v429 = v428 < 0.0f;
                        bool2 v430 = v429.xx;
                        v407 = v429 ? v407 : v427;
                        v410 = v429 ? v427 : v410;
                        v412 = float2(v430.x ? v412.x : v421.x, v430.y ? v412.y : v421.y);
                        v414 = float2(v430.x ? v421.x : v414.x, v430.y ? v421.y : v414.y);
                        v416++;
                        continue;
                    }
                    float2 v435 = lerp(v414, v412, clamp(v410 / (v410 - v407), 0.0f, 1.0f).xx);
                    float2 v439 = float3(v435.x, 0.4000000059604644775390625f, v435.y).xz;
                    float v452;
                    float v455;
                    float2 v457;
                    float2 v459;
                    v452 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, v401, 0.0f).y;
                    v455 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v176[2] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                    v457 = v176[3];
                    v459 = v176[2];
                    for (uint v461 = 0u; v461 < 2u; )
                    {
                        float2 v465 = v457 + v459;
                        float2 v466 = v465 * 0.5f;
                        float v472 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v466 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                        float v473 = v472 * v452;
                        bool v474 = v473 < 0.0f;
                        bool2 v475 = v474.xx;
                        v452 = v474 ? v452 : v472;
                        v455 = v474 ? v472 : v455;
                        v457 = float2(v475.x ? v457.x : v466.x, v475.y ? v457.y : v466.y);
                        v459 = float2(v475.x ? v466.x : v459.x, v475.y ? v466.y : v459.y);
                        v461++;
                        continue;
                    }
                    float2 v480 = lerp(v459, v457, clamp(v455 / (v455 - v452), 0.0f, 1.0f).xx);
                    float2 v484 = float3(v480.x, 0.4000000059604644775390625f, v480.y).xz;
                    float v485 = distance(v439, v484);
                    if (v485 < 0.4000000059604644775390625f)
                    {
                        break;
                    }
                    int v491 = int(max(v485 * 2.5f, 1.0f));
                    uint v493 = 0u;
                    uint v496 = 0u;
                    bool v498 = false;
                    bool v500 = false;
                    int v502 = 0;
                    for (; v502 < (v491 + 1); )
                    {
                        float v508 = float(v491);
                        int v503 = v502 + 1;
                        float2 v515 = lerp(v439, v484, (((float(v502) / v508) + (float(v503) / v508)) * 0.5f).xx);
                        float v516 = v515.x;
                        float v524 = v515.y;
                        int v526 = v367 + (v502 * 100);
                        uint v528 = uint(v526 ^ 2346);
                        uint v530 = v528 + (v528 << 10u);
                        uint v532 = v530 ^ (v530 >> 6u);
                        uint v534 = v532 + (v532 << 3u);
                        uint v536 = v534 ^ (v534 >> 11u);
                        uint v538 = v536 + (v536 << 15u);
                        bool v539 = v500 ? v500 : true;
                        uint v540 = v500 ? v496 : 8388607u;
                        bool v541 = v498 ? v498 : true;
                        uint v542 = v498 ? v493 : 1065353216u;
                        uint v550 = uint(v526 ^ 3459);
                        uint v552 = v550 + (v550 << 10u);
                        uint v554 = v552 ^ (v552 >> 6u);
                        uint v556 = v554 + (v554 << 3u);
                        uint v558 = v556 ^ (v556 >> 11u);
                        bool v561 = v539 ? v539 : true;
                        uint v562 = v539 ? v540 : 8388607u;
                        bool v563 = v541 ? v541 : true;
                        uint v564 = v541 ? v542 : 1065353216u;
                        float v570 = ((asfloat(((v558 + (v558 << 15u)) & v562) | v564) - 1.0f) * 2.0f) - 1.0f;
                        uint v572 = uint(v526 ^ 234);
                        uint v574 = v572 + (v572 << 10u);
                        uint v576 = v574 ^ (v574 >> 6u);
                        uint v578 = v576 + (v576 << 3u);
                        uint v580 = v578 ^ (v578 >> 11u);
                        bool v583 = v561 ? v561 : true;
                        uint v584 = v561 ? v562 : 8388607u;
                        bool v585 = v563 ? v563 : true;
                        uint v586 = v563 ? v564 : 1065353216u;
                        float3 v593 = float3(((asfloat((v538 & v540) | v542) - 1.0f) * 2.0f) - 1.0f, v570, ((asfloat(((v580 + (v580 << 15u)) & v584) | v586) - 1.0f) * 2.0f) - 1.0f);
                        v593.y = v570 * 0.20000000298023223876953125f;
                        float3 v596 = normalize(v593);
                        bool3 v599 = (v596.y < 0.0f).xxx;
                        float3 v600 = float3(v599.x ? float3(0.0f, -1.0f, 0.0f).x : float3(0.0f, 1.0f, 0.0f).x, v599.y ? float3(0.0f, -1.0f, 0.0f).y : float3(0.0f, 1.0f, 0.0f).y, v599.z ? float3(0.0f, -1.0f, 0.0f).z : float3(0.0f, 1.0f, 0.0f).z);
                        uint v603 = uint(v526 ^ 346);
                        uint v605 = v603 + (v603 << 10u);
                        uint v607 = v605 ^ (v605 >> 6u);
                        uint v609 = v607 + (v607 << 3u);
                        uint v611 = v609 ^ (v609 >> 11u);
                        bool v614 = v583 ? v583 : true;
                        uint v615 = v583 ? v584 : 8388607u;
                        bool v616 = v585 ? v585 : true;
                        uint v617 = v585 ? v586 : 1065353216u;
                        bool v624 = v614 ? v614 : true;
                        uint v625 = v614 ? v615 : 8388607u;
                        bool v626 = v616 ? v616 : true;
                        uint v627 = v616 ? v617 : 1065353216u;
                        float2 v633 = lerp(0.800000011920928955078125f, 1.5f, asfloat((v538 & v625) | v627) - 1.0f).xx;
                        float3 v635 = float3(v633.x, lerp(0.800000011920928955078125f, 1.2999999523162841796875f, asfloat(((v611 + (v611 << 15u)) & v615) | v617) - 1.0f).xxx.y, v633.y) * 0.75f;
                        float4x4 v661 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v516), float4(0.0f, 1.0f, 0.0f, lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v515 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x)), float4(0.0f, 0.0f, 1.0f, v524), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(transpose(float4x4(float4(v596, 0.0f), float4(v600, 0.0f), float4(cross(v596, v600), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))), float4x4(float4(v635.x, 0.0f, 0.0f, 0.0f), float4(0.0f, v635.y, 0.0f, 0.0f), float4(0.0f, 0.0f, v635.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                        uint v678;
                        __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, v678);
                        float v681 = 10.0f * (v516 + v524);
                        uint v683 = uint(v681 + 640.0f);
                        uint v685 = v683 + (v683 << 10u);
                        uint v687 = v685 ^ (v685 >> 6u);
                        uint v689 = v687 + (v687 << 3u);
                        uint v691 = v689 ^ (v689 >> 11u);
                        bool v694 = v624 ? v624 : true;
                        uint v695 = v624 ? v625 : 8388607u;
                        bool v696 = v626 ? v626 : true;
                        uint v697 = v626 ? v627 : 1065353216u;
                        uint v703 = uint(v681 + 987.0f);
                        uint v705 = v703 + (v703 << 10u);
                        uint v707 = v705 ^ (v705 >> 6u);
                        uint v709 = v707 + (v707 << 3u);
                        uint v711 = v709 ^ (v709 >> 11u);
                        uint v497 = v694 ? v695 : 8388607u;
                        uint v494 = v696 ? v697 : 1065353216u;
                        uint v719 = uint(int(v678));
                        float v721[12] = { v661[0].x, v661[1].x, v661[2].x, v661[0].y, v661[1].y, v661[2].y, v661[0].z, v661[1].z, v661[2].z, v661[0].w, v661[1].w, v661[2].w };
                        Affine3Packed v722 = { v721 };
                        GardenStone v723 = { v722, float4(asfloat(((v691 + (v691 << 15u)) & v695) | v697) - 1.0f, asfloat(((v711 + (v711 << 15u)) & v497) | v494) - 1.0f, 0.0f, 0.0f) };
                        [unroll]
                        for (int _10ident = 0; _10ident < 12; _10ident++)
                        {
                            __alias_instance_buffer.Store<float>(_10ident * 4 + v719 * 64 + 0, v723.transform.data[_10ident]);
                        }
                        __alias_instance_buffer.Store<float4>(v719 * 64 + 48, v723.color);
                        v493 = v494;
                        v496 = v497;
                        v498 = v696 ? v696 : true;
                        v500 = v694 ? v694 : true;
                        v502 = v503;
                        continue;
                    }
                    break;
                } while(false);
            }
            else
            {
                bool v729;
                if (!(v368 == 2))
                {
                    v729 = v368 == 13;
                }
                else
                {
                    v729 = true;
                }
                if (v729)
                {
                    do
                    {
                        float2 v746 = (v176[2] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                        float v752;
                        float v755;
                        float2 v757;
                        float2 v759;
                        v752 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v176[1] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                        v755 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, v746, 0.0f).y;
                        v757 = v176[1];
                        v759 = v176[2];
                        for (uint v761 = 0u; v761 < 2u; )
                        {
                            float2 v765 = v757 + v759;
                            float2 v766 = v765 * 0.5f;
                            float v772 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v766 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                            float v773 = v772 * v752;
                            bool v774 = v773 < 0.0f;
                            bool2 v775 = v774.xx;
                            v752 = v774 ? v752 : v772;
                            v755 = v774 ? v772 : v755;
                            v757 = float2(v775.x ? v757.x : v766.x, v775.y ? v757.y : v766.y);
                            v759 = float2(v775.x ? v766.x : v759.x, v775.y ? v766.y : v759.y);
                            v761++;
                            continue;
                        }
                        float2 v780 = lerp(v759, v757, clamp(v755 / (v755 - v752), 0.0f, 1.0f).xx);
                        float2 v784 = float3(v780.x, 0.4000000059604644775390625f, v780.y).xz;
                        float v797;
                        float v800;
                        float2 v802;
                        float2 v804;
                        v797 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v176[3] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                        v800 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, v746, 0.0f).y;
                        v802 = v176[3];
                        v804 = v176[2];
                        for (uint v806 = 0u; v806 < 2u; )
                        {
                            float2 v810 = v802 + v804;
                            float2 v811 = v810 * 0.5f;
                            float v817 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v811 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                            float v818 = v817 * v797;
                            bool v819 = v818 < 0.0f;
                            bool2 v820 = v819.xx;
                            v797 = v819 ? v797 : v817;
                            v800 = v819 ? v817 : v800;
                            v802 = float2(v820.x ? v802.x : v811.x, v820.y ? v802.y : v811.y);
                            v804 = float2(v820.x ? v811.x : v804.x, v820.y ? v811.y : v804.y);
                            v806++;
                            continue;
                        }
                        float2 v825 = lerp(v804, v802, clamp(v800 / (v800 - v797), 0.0f, 1.0f).xx);
                        float2 v829 = float3(v825.x, 0.4000000059604644775390625f, v825.y).xz;
                        float v830 = distance(v784, v829);
                        if (v830 < 0.4000000059604644775390625f)
                        {
                            break;
                        }
                        int v836 = int(max(v830 * 2.5f, 1.0f));
                        uint v838 = 0u;
                        uint v841 = 0u;
                        bool v843 = false;
                        bool v845 = false;
                        int v847 = 0;
                        for (; v847 < (v836 + 1); )
                        {
                            float v853 = float(v836);
                            int v848 = v847 + 1;
                            float2 v860 = lerp(v784, v829, (((float(v847) / v853) + (float(v848) / v853)) * 0.5f).xx);
                            float v861 = v860.x;
                            float v869 = v860.y;
                            int v871 = v367 + (v847 * 100);
                            uint v873 = uint(v871 ^ 2346);
                            uint v875 = v873 + (v873 << 10u);
                            uint v877 = v875 ^ (v875 >> 6u);
                            uint v879 = v877 + (v877 << 3u);
                            uint v881 = v879 ^ (v879 >> 11u);
                            uint v883 = v881 + (v881 << 15u);
                            bool v884 = v845 ? v845 : true;
                            uint v885 = v845 ? v841 : 8388607u;
                            bool v886 = v843 ? v843 : true;
                            uint v887 = v843 ? v838 : 1065353216u;
                            uint v895 = uint(v871 ^ 3459);
                            uint v897 = v895 + (v895 << 10u);
                            uint v899 = v897 ^ (v897 >> 6u);
                            uint v901 = v899 + (v899 << 3u);
                            uint v903 = v901 ^ (v901 >> 11u);
                            bool v906 = v884 ? v884 : true;
                            uint v907 = v884 ? v885 : 8388607u;
                            bool v908 = v886 ? v886 : true;
                            uint v909 = v886 ? v887 : 1065353216u;
                            float v915 = ((asfloat(((v903 + (v903 << 15u)) & v907) | v909) - 1.0f) * 2.0f) - 1.0f;
                            uint v917 = uint(v871 ^ 234);
                            uint v919 = v917 + (v917 << 10u);
                            uint v921 = v919 ^ (v919 >> 6u);
                            uint v923 = v921 + (v921 << 3u);
                            uint v925 = v923 ^ (v923 >> 11u);
                            bool v928 = v906 ? v906 : true;
                            uint v929 = v906 ? v907 : 8388607u;
                            bool v930 = v908 ? v908 : true;
                            uint v931 = v908 ? v909 : 1065353216u;
                            float3 v938 = float3(((asfloat((v883 & v885) | v887) - 1.0f) * 2.0f) - 1.0f, v915, ((asfloat(((v925 + (v925 << 15u)) & v929) | v931) - 1.0f) * 2.0f) - 1.0f);
                            v938.y = v915 * 0.20000000298023223876953125f;
                            float3 v941 = normalize(v938);
                            bool3 v944 = (v941.y < 0.0f).xxx;
                            float3 v945 = float3(v944.x ? float3(0.0f, -1.0f, 0.0f).x : float3(0.0f, 1.0f, 0.0f).x, v944.y ? float3(0.0f, -1.0f, 0.0f).y : float3(0.0f, 1.0f, 0.0f).y, v944.z ? float3(0.0f, -1.0f, 0.0f).z : float3(0.0f, 1.0f, 0.0f).z);
                            uint v948 = uint(v871 ^ 346);
                            uint v950 = v948 + (v948 << 10u);
                            uint v952 = v950 ^ (v950 >> 6u);
                            uint v954 = v952 + (v952 << 3u);
                            uint v956 = v954 ^ (v954 >> 11u);
                            bool v959 = v928 ? v928 : true;
                            uint v960 = v928 ? v929 : 8388607u;
                            bool v961 = v930 ? v930 : true;
                            uint v962 = v930 ? v931 : 1065353216u;
                            bool v969 = v959 ? v959 : true;
                            uint v970 = v959 ? v960 : 8388607u;
                            bool v971 = v961 ? v961 : true;
                            uint v972 = v961 ? v962 : 1065353216u;
                            float2 v978 = lerp(0.800000011920928955078125f, 1.5f, asfloat((v883 & v970) | v972) - 1.0f).xx;
                            float3 v980 = float3(v978.x, lerp(0.800000011920928955078125f, 1.2999999523162841796875f, asfloat(((v956 + (v956 << 15u)) & v960) | v962) - 1.0f).xxx.y, v978.y) * 0.75f;
                            float4x4 v1006 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v861), float4(0.0f, 1.0f, 0.0f, lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v860 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x)), float4(0.0f, 0.0f, 1.0f, v869), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(transpose(float4x4(float4(v941, 0.0f), float4(v945, 0.0f), float4(cross(v941, v945), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))), float4x4(float4(v980.x, 0.0f, 0.0f, 0.0f), float4(0.0f, v980.y, 0.0f, 0.0f), float4(0.0f, 0.0f, v980.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                            uint v1023;
                            __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, v1023);
                            float v1026 = 10.0f * (v861 + v869);
                            uint v1028 = uint(v1026 + 640.0f);
                            uint v1030 = v1028 + (v1028 << 10u);
                            uint v1032 = v1030 ^ (v1030 >> 6u);
                            uint v1034 = v1032 + (v1032 << 3u);
                            uint v1036 = v1034 ^ (v1034 >> 11u);
                            bool v1039 = v969 ? v969 : true;
                            uint v1040 = v969 ? v970 : 8388607u;
                            bool v1041 = v971 ? v971 : true;
                            uint v1042 = v971 ? v972 : 1065353216u;
                            uint v1048 = uint(v1026 + 987.0f);
                            uint v1050 = v1048 + (v1048 << 10u);
                            uint v1052 = v1050 ^ (v1050 >> 6u);
                            uint v1054 = v1052 + (v1052 << 3u);
                            uint v1056 = v1054 ^ (v1054 >> 11u);
                            uint v842 = v1039 ? v1040 : 8388607u;
                            uint v839 = v1041 ? v1042 : 1065353216u;
                            uint v1064 = uint(int(v1023));
                            float v1066[12] = { v1006[0].x, v1006[1].x, v1006[2].x, v1006[0].y, v1006[1].y, v1006[2].y, v1006[0].z, v1006[1].z, v1006[2].z, v1006[0].w, v1006[1].w, v1006[2].w };
                            Affine3Packed v1067 = { v1066 };
                            GardenStone v1068 = { v1067, float4(asfloat(((v1036 + (v1036 << 15u)) & v1040) | v1042) - 1.0f, asfloat(((v1056 + (v1056 << 15u)) & v842) | v839) - 1.0f, 0.0f, 0.0f) };
                            [unroll]
                            for (int _11ident = 0; _11ident < 12; _11ident++)
                            {
                                __alias_instance_buffer.Store<float>(_11ident * 4 + v1064 * 64 + 0, v1068.transform.data[_11ident]);
                            }
                            __alias_instance_buffer.Store<float4>(v1064 * 64 + 48, v1068.color);
                            v838 = v839;
                            v841 = v842;
                            v843 = v1041 ? v1041 : true;
                            v845 = v1039 ? v1039 : true;
                            v847 = v848;
                            continue;
                        }
                        break;
                    } while(false);
                }
                else
                {
                    bool v1074;
                    if (!(v368 == 3))
                    {
                        v1074 = v368 == 12;
                    }
                    else
                    {
                        v1074 = true;
                    }
                    if (v1074)
                    {
                        do
                        {
                            float v1097;
                            float v1100;
                            float2 v1102;
                            float2 v1104;
                            v1097 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v176[0] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                            v1100 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v176[3] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                            v1102 = v176[0];
                            v1104 = v176[3];
                            for (uint v1106 = 0u; v1106 < 2u; )
                            {
                                float2 v1110 = v1102 + v1104;
                                float2 v1111 = v1110 * 0.5f;
                                float v1117 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v1111 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                float v1118 = v1117 * v1097;
                                bool v1119 = v1118 < 0.0f;
                                bool2 v1120 = v1119.xx;
                                v1097 = v1119 ? v1097 : v1117;
                                v1100 = v1119 ? v1117 : v1100;
                                v1102 = float2(v1120.x ? v1102.x : v1111.x, v1120.y ? v1102.y : v1111.y);
                                v1104 = float2(v1120.x ? v1111.x : v1104.x, v1120.y ? v1111.y : v1104.y);
                                v1106++;
                                continue;
                            }
                            float2 v1125 = lerp(v1104, v1102, clamp(v1100 / (v1100 - v1097), 0.0f, 1.0f).xx);
                            float2 v1129 = float3(v1125.x, 0.4000000059604644775390625f, v1125.y).xz;
                            float v1145;
                            float v1148;
                            float2 v1150;
                            float2 v1152;
                            v1145 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v176[1] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                            v1148 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v176[2] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                            v1150 = v176[1];
                            v1152 = v176[2];
                            for (uint v1154 = 0u; v1154 < 2u; )
                            {
                                float2 v1158 = v1150 + v1152;
                                float2 v1159 = v1158 * 0.5f;
                                float v1165 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v1159 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                float v1166 = v1165 * v1145;
                                bool v1167 = v1166 < 0.0f;
                                bool2 v1168 = v1167.xx;
                                v1145 = v1167 ? v1145 : v1165;
                                v1148 = v1167 ? v1165 : v1148;
                                v1150 = float2(v1168.x ? v1150.x : v1159.x, v1168.y ? v1150.y : v1159.y);
                                v1152 = float2(v1168.x ? v1159.x : v1152.x, v1168.y ? v1159.y : v1152.y);
                                v1154++;
                                continue;
                            }
                            float2 v1173 = lerp(v1152, v1150, clamp(v1148 / (v1148 - v1145), 0.0f, 1.0f).xx);
                            float2 v1177 = float3(v1173.x, 0.4000000059604644775390625f, v1173.y).xz;
                            float v1178 = distance(v1129, v1177);
                            if (v1178 < 0.4000000059604644775390625f)
                            {
                                break;
                            }
                            int v1184 = int(max(v1178 * 2.5f, 1.0f));
                            uint v1186 = 0u;
                            uint v1189 = 0u;
                            bool v1191 = false;
                            bool v1193 = false;
                            int v1195 = 0;
                            for (; v1195 < (v1184 + 1); )
                            {
                                float v1201 = float(v1184);
                                int v1196 = v1195 + 1;
                                float2 v1208 = lerp(v1129, v1177, (((float(v1195) / v1201) + (float(v1196) / v1201)) * 0.5f).xx);
                                float v1209 = v1208.x;
                                float v1217 = v1208.y;
                                int v1219 = v367 + (v1195 * 100);
                                uint v1221 = uint(v1219 ^ 2346);
                                uint v1223 = v1221 + (v1221 << 10u);
                                uint v1225 = v1223 ^ (v1223 >> 6u);
                                uint v1227 = v1225 + (v1225 << 3u);
                                uint v1229 = v1227 ^ (v1227 >> 11u);
                                uint v1231 = v1229 + (v1229 << 15u);
                                bool v1232 = v1193 ? v1193 : true;
                                uint v1233 = v1193 ? v1189 : 8388607u;
                                bool v1234 = v1191 ? v1191 : true;
                                uint v1235 = v1191 ? v1186 : 1065353216u;
                                uint v1243 = uint(v1219 ^ 3459);
                                uint v1245 = v1243 + (v1243 << 10u);
                                uint v1247 = v1245 ^ (v1245 >> 6u);
                                uint v1249 = v1247 + (v1247 << 3u);
                                uint v1251 = v1249 ^ (v1249 >> 11u);
                                bool v1254 = v1232 ? v1232 : true;
                                uint v1255 = v1232 ? v1233 : 8388607u;
                                bool v1256 = v1234 ? v1234 : true;
                                uint v1257 = v1234 ? v1235 : 1065353216u;
                                float v1263 = ((asfloat(((v1251 + (v1251 << 15u)) & v1255) | v1257) - 1.0f) * 2.0f) - 1.0f;
                                uint v1265 = uint(v1219 ^ 234);
                                uint v1267 = v1265 + (v1265 << 10u);
                                uint v1269 = v1267 ^ (v1267 >> 6u);
                                uint v1271 = v1269 + (v1269 << 3u);
                                uint v1273 = v1271 ^ (v1271 >> 11u);
                                bool v1276 = v1254 ? v1254 : true;
                                uint v1277 = v1254 ? v1255 : 8388607u;
                                bool v1278 = v1256 ? v1256 : true;
                                uint v1279 = v1256 ? v1257 : 1065353216u;
                                float3 v1286 = float3(((asfloat((v1231 & v1233) | v1235) - 1.0f) * 2.0f) - 1.0f, v1263, ((asfloat(((v1273 + (v1273 << 15u)) & v1277) | v1279) - 1.0f) * 2.0f) - 1.0f);
                                v1286.y = v1263 * 0.20000000298023223876953125f;
                                float3 v1289 = normalize(v1286);
                                bool3 v1292 = (v1289.y < 0.0f).xxx;
                                float3 v1293 = float3(v1292.x ? float3(0.0f, -1.0f, 0.0f).x : float3(0.0f, 1.0f, 0.0f).x, v1292.y ? float3(0.0f, -1.0f, 0.0f).y : float3(0.0f, 1.0f, 0.0f).y, v1292.z ? float3(0.0f, -1.0f, 0.0f).z : float3(0.0f, 1.0f, 0.0f).z);
                                uint v1296 = uint(v1219 ^ 346);
                                uint v1298 = v1296 + (v1296 << 10u);
                                uint v1300 = v1298 ^ (v1298 >> 6u);
                                uint v1302 = v1300 + (v1300 << 3u);
                                uint v1304 = v1302 ^ (v1302 >> 11u);
                                bool v1307 = v1276 ? v1276 : true;
                                uint v1308 = v1276 ? v1277 : 8388607u;
                                bool v1309 = v1278 ? v1278 : true;
                                uint v1310 = v1278 ? v1279 : 1065353216u;
                                bool v1317 = v1307 ? v1307 : true;
                                uint v1318 = v1307 ? v1308 : 8388607u;
                                bool v1319 = v1309 ? v1309 : true;
                                uint v1320 = v1309 ? v1310 : 1065353216u;
                                float2 v1326 = lerp(0.800000011920928955078125f, 1.5f, asfloat((v1231 & v1318) | v1320) - 1.0f).xx;
                                float3 v1328 = float3(v1326.x, lerp(0.800000011920928955078125f, 1.2999999523162841796875f, asfloat(((v1304 + (v1304 << 15u)) & v1308) | v1310) - 1.0f).xxx.y, v1326.y) * 0.75f;
                                float4x4 v1354 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v1209), float4(0.0f, 1.0f, 0.0f, lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v1208 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x)), float4(0.0f, 0.0f, 1.0f, v1217), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(transpose(float4x4(float4(v1289, 0.0f), float4(v1293, 0.0f), float4(cross(v1289, v1293), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))), float4x4(float4(v1328.x, 0.0f, 0.0f, 0.0f), float4(0.0f, v1328.y, 0.0f, 0.0f), float4(0.0f, 0.0f, v1328.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                uint v1371;
                                __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, v1371);
                                float v1374 = 10.0f * (v1209 + v1217);
                                uint v1376 = uint(v1374 + 640.0f);
                                uint v1378 = v1376 + (v1376 << 10u);
                                uint v1380 = v1378 ^ (v1378 >> 6u);
                                uint v1382 = v1380 + (v1380 << 3u);
                                uint v1384 = v1382 ^ (v1382 >> 11u);
                                bool v1387 = v1317 ? v1317 : true;
                                uint v1388 = v1317 ? v1318 : 8388607u;
                                bool v1389 = v1319 ? v1319 : true;
                                uint v1390 = v1319 ? v1320 : 1065353216u;
                                uint v1396 = uint(v1374 + 987.0f);
                                uint v1398 = v1396 + (v1396 << 10u);
                                uint v1400 = v1398 ^ (v1398 >> 6u);
                                uint v1402 = v1400 + (v1400 << 3u);
                                uint v1404 = v1402 ^ (v1402 >> 11u);
                                uint v1190 = v1387 ? v1388 : 8388607u;
                                uint v1187 = v1389 ? v1390 : 1065353216u;
                                uint v1412 = uint(int(v1371));
                                float v1414[12] = { v1354[0].x, v1354[1].x, v1354[2].x, v1354[0].y, v1354[1].y, v1354[2].y, v1354[0].z, v1354[1].z, v1354[2].z, v1354[0].w, v1354[1].w, v1354[2].w };
                                Affine3Packed v1415 = { v1414 };
                                GardenStone v1416 = { v1415, float4(asfloat(((v1384 + (v1384 << 15u)) & v1388) | v1390) - 1.0f, asfloat(((v1404 + (v1404 << 15u)) & v1190) | v1187) - 1.0f, 0.0f, 0.0f) };
                                [unroll]
                                for (int _12ident = 0; _12ident < 12; _12ident++)
                                {
                                    __alias_instance_buffer.Store<float>(_12ident * 4 + v1412 * 64 + 0, v1416.transform.data[_12ident]);
                                }
                                __alias_instance_buffer.Store<float4>(v1412 * 64 + 48, v1416.color);
                                v1186 = v1187;
                                v1189 = v1190;
                                v1191 = v1389 ? v1389 : true;
                                v1193 = v1387 ? v1387 : true;
                                v1195 = v1196;
                                continue;
                            }
                            break;
                        } while(false);
                    }
                    else
                    {
                        bool v1422;
                        if (!(v368 == 4))
                        {
                            v1422 = v368 == 11;
                        }
                        else
                        {
                            v1422 = true;
                        }
                        if (v1422)
                        {
                            do
                            {
                                float2 v1439 = (v176[1] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                float v1445;
                                float v1448;
                                float2 v1450;
                                float2 v1452;
                                v1445 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v176[0] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                v1448 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, v1439, 0.0f).y;
                                v1450 = v176[0];
                                v1452 = v176[1];
                                for (uint v1454 = 0u; v1454 < 2u; )
                                {
                                    float2 v1458 = v1450 + v1452;
                                    float2 v1459 = v1458 * 0.5f;
                                    float v1465 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v1459 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                    float v1466 = v1465 * v1445;
                                    bool v1467 = v1466 < 0.0f;
                                    bool2 v1468 = v1467.xx;
                                    v1445 = v1467 ? v1445 : v1465;
                                    v1448 = v1467 ? v1465 : v1448;
                                    v1450 = float2(v1468.x ? v1450.x : v1459.x, v1468.y ? v1450.y : v1459.y);
                                    v1452 = float2(v1468.x ? v1459.x : v1452.x, v1468.y ? v1459.y : v1452.y);
                                    v1454++;
                                    continue;
                                }
                                float2 v1473 = lerp(v1452, v1450, clamp(v1448 / (v1448 - v1445), 0.0f, 1.0f).xx);
                                float2 v1477 = float3(v1473.x, 0.4000000059604644775390625f, v1473.y).xz;
                                float v1490;
                                float v1493;
                                float2 v1495;
                                float2 v1497;
                                v1490 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, v1439, 0.0f).y;
                                v1493 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v176[2] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                v1495 = v176[1];
                                v1497 = v176[2];
                                for (uint v1499 = 0u; v1499 < 2u; )
                                {
                                    float2 v1503 = v1495 + v1497;
                                    float2 v1504 = v1503 * 0.5f;
                                    float v1510 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v1504 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                    float v1511 = v1510 * v1490;
                                    bool v1512 = v1511 < 0.0f;
                                    bool2 v1513 = v1512.xx;
                                    v1490 = v1512 ? v1490 : v1510;
                                    v1493 = v1512 ? v1510 : v1493;
                                    v1495 = float2(v1513.x ? v1495.x : v1504.x, v1513.y ? v1495.y : v1504.y);
                                    v1497 = float2(v1513.x ? v1504.x : v1497.x, v1513.y ? v1504.y : v1497.y);
                                    v1499++;
                                    continue;
                                }
                                float2 v1518 = lerp(v1497, v1495, clamp(v1493 / (v1493 - v1490), 0.0f, 1.0f).xx);
                                float2 v1522 = float3(v1518.x, 0.4000000059604644775390625f, v1518.y).xz;
                                float v1523 = distance(v1477, v1522);
                                if (v1523 < 0.4000000059604644775390625f)
                                {
                                    break;
                                }
                                int v1529 = int(max(v1523 * 2.5f, 1.0f));
                                uint v1531 = 0u;
                                uint v1534 = 0u;
                                bool v1536 = false;
                                bool v1538 = false;
                                int v1540 = 0;
                                for (; v1540 < (v1529 + 1); )
                                {
                                    float v1546 = float(v1529);
                                    int v1541 = v1540 + 1;
                                    float2 v1553 = lerp(v1477, v1522, (((float(v1540) / v1546) + (float(v1541) / v1546)) * 0.5f).xx);
                                    float v1554 = v1553.x;
                                    float v1562 = v1553.y;
                                    int v1564 = v367 + (v1540 * 100);
                                    uint v1566 = uint(v1564 ^ 2346);
                                    uint v1568 = v1566 + (v1566 << 10u);
                                    uint v1570 = v1568 ^ (v1568 >> 6u);
                                    uint v1572 = v1570 + (v1570 << 3u);
                                    uint v1574 = v1572 ^ (v1572 >> 11u);
                                    uint v1576 = v1574 + (v1574 << 15u);
                                    bool v1577 = v1538 ? v1538 : true;
                                    uint v1578 = v1538 ? v1534 : 8388607u;
                                    bool v1579 = v1536 ? v1536 : true;
                                    uint v1580 = v1536 ? v1531 : 1065353216u;
                                    uint v1588 = uint(v1564 ^ 3459);
                                    uint v1590 = v1588 + (v1588 << 10u);
                                    uint v1592 = v1590 ^ (v1590 >> 6u);
                                    uint v1594 = v1592 + (v1592 << 3u);
                                    uint v1596 = v1594 ^ (v1594 >> 11u);
                                    bool v1599 = v1577 ? v1577 : true;
                                    uint v1600 = v1577 ? v1578 : 8388607u;
                                    bool v1601 = v1579 ? v1579 : true;
                                    uint v1602 = v1579 ? v1580 : 1065353216u;
                                    float v1608 = ((asfloat(((v1596 + (v1596 << 15u)) & v1600) | v1602) - 1.0f) * 2.0f) - 1.0f;
                                    uint v1610 = uint(v1564 ^ 234);
                                    uint v1612 = v1610 + (v1610 << 10u);
                                    uint v1614 = v1612 ^ (v1612 >> 6u);
                                    uint v1616 = v1614 + (v1614 << 3u);
                                    uint v1618 = v1616 ^ (v1616 >> 11u);
                                    bool v1621 = v1599 ? v1599 : true;
                                    uint v1622 = v1599 ? v1600 : 8388607u;
                                    bool v1623 = v1601 ? v1601 : true;
                                    uint v1624 = v1601 ? v1602 : 1065353216u;
                                    float3 v1631 = float3(((asfloat((v1576 & v1578) | v1580) - 1.0f) * 2.0f) - 1.0f, v1608, ((asfloat(((v1618 + (v1618 << 15u)) & v1622) | v1624) - 1.0f) * 2.0f) - 1.0f);
                                    v1631.y = v1608 * 0.20000000298023223876953125f;
                                    float3 v1634 = normalize(v1631);
                                    bool3 v1637 = (v1634.y < 0.0f).xxx;
                                    float3 v1638 = float3(v1637.x ? float3(0.0f, -1.0f, 0.0f).x : float3(0.0f, 1.0f, 0.0f).x, v1637.y ? float3(0.0f, -1.0f, 0.0f).y : float3(0.0f, 1.0f, 0.0f).y, v1637.z ? float3(0.0f, -1.0f, 0.0f).z : float3(0.0f, 1.0f, 0.0f).z);
                                    uint v1641 = uint(v1564 ^ 346);
                                    uint v1643 = v1641 + (v1641 << 10u);
                                    uint v1645 = v1643 ^ (v1643 >> 6u);
                                    uint v1647 = v1645 + (v1645 << 3u);
                                    uint v1649 = v1647 ^ (v1647 >> 11u);
                                    bool v1652 = v1621 ? v1621 : true;
                                    uint v1653 = v1621 ? v1622 : 8388607u;
                                    bool v1654 = v1623 ? v1623 : true;
                                    uint v1655 = v1623 ? v1624 : 1065353216u;
                                    bool v1662 = v1652 ? v1652 : true;
                                    uint v1663 = v1652 ? v1653 : 8388607u;
                                    bool v1664 = v1654 ? v1654 : true;
                                    uint v1665 = v1654 ? v1655 : 1065353216u;
                                    float2 v1671 = lerp(0.800000011920928955078125f, 1.5f, asfloat((v1576 & v1663) | v1665) - 1.0f).xx;
                                    float3 v1673 = float3(v1671.x, lerp(0.800000011920928955078125f, 1.2999999523162841796875f, asfloat(((v1649 + (v1649 << 15u)) & v1653) | v1655) - 1.0f).xxx.y, v1671.y) * 0.75f;
                                    float4x4 v1699 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v1554), float4(0.0f, 1.0f, 0.0f, lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v1553 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x)), float4(0.0f, 0.0f, 1.0f, v1562), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(transpose(float4x4(float4(v1634, 0.0f), float4(v1638, 0.0f), float4(cross(v1634, v1638), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))), float4x4(float4(v1673.x, 0.0f, 0.0f, 0.0f), float4(0.0f, v1673.y, 0.0f, 0.0f), float4(0.0f, 0.0f, v1673.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                    uint v1716;
                                    __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, v1716);
                                    float v1719 = 10.0f * (v1554 + v1562);
                                    uint v1721 = uint(v1719 + 640.0f);
                                    uint v1723 = v1721 + (v1721 << 10u);
                                    uint v1725 = v1723 ^ (v1723 >> 6u);
                                    uint v1727 = v1725 + (v1725 << 3u);
                                    uint v1729 = v1727 ^ (v1727 >> 11u);
                                    bool v1732 = v1662 ? v1662 : true;
                                    uint v1733 = v1662 ? v1663 : 8388607u;
                                    bool v1734 = v1664 ? v1664 : true;
                                    uint v1735 = v1664 ? v1665 : 1065353216u;
                                    uint v1741 = uint(v1719 + 987.0f);
                                    uint v1743 = v1741 + (v1741 << 10u);
                                    uint v1745 = v1743 ^ (v1743 >> 6u);
                                    uint v1747 = v1745 + (v1745 << 3u);
                                    uint v1749 = v1747 ^ (v1747 >> 11u);
                                    uint v1535 = v1732 ? v1733 : 8388607u;
                                    uint v1532 = v1734 ? v1735 : 1065353216u;
                                    uint v1757 = uint(int(v1716));
                                    float v1759[12] = { v1699[0].x, v1699[1].x, v1699[2].x, v1699[0].y, v1699[1].y, v1699[2].y, v1699[0].z, v1699[1].z, v1699[2].z, v1699[0].w, v1699[1].w, v1699[2].w };
                                    Affine3Packed v1760 = { v1759 };
                                    GardenStone v1761 = { v1760, float4(asfloat(((v1729 + (v1729 << 15u)) & v1733) | v1735) - 1.0f, asfloat(((v1749 + (v1749 << 15u)) & v1535) | v1532) - 1.0f, 0.0f, 0.0f) };
                                    [unroll]
                                    for (int _13ident = 0; _13ident < 12; _13ident++)
                                    {
                                        __alias_instance_buffer.Store<float>(_13ident * 4 + v1757 * 64 + 0, v1761.transform.data[_13ident]);
                                    }
                                    __alias_instance_buffer.Store<float4>(v1757 * 64 + 48, v1761.color);
                                    v1531 = v1532;
                                    v1534 = v1535;
                                    v1536 = v1734 ? v1734 : true;
                                    v1538 = v1732 ? v1732 : true;
                                    v1540 = v1541;
                                    continue;
                                }
                                break;
                            } while(false);
                        }
                        else
                        {
                            if (v368 == 5)
                            {
                                float2 v1769;
                                float2 v1779;
                                float2 v1818;
                                float2 v1824;
                                uint v2102;
                                uint v2103;
                                bool v2104;
                                bool v2105;
                                do
                                {
                                    v1769 = v176[1];
                                    float2 v1771 = (v176[0] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                    v1779 = (v1769 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                    float v1785;
                                    float v1788;
                                    float2 v1790;
                                    float2 v1792;
                                    v1785 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, v1771, 0.0f).y;
                                    v1788 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, v1779, 0.0f).y;
                                    v1790 = v176[0];
                                    v1792 = v1769;
                                    for (uint v1794 = 0u; v1794 < 2u; )
                                    {
                                        float2 v1798 = v1790 + v1792;
                                        float2 v1799 = v1798 * 0.5f;
                                        float v1805 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v1799 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                        float v1806 = v1805 * v1785;
                                        bool v1807 = v1806 < 0.0f;
                                        bool2 v1808 = v1807.xx;
                                        v1785 = v1807 ? v1785 : v1805;
                                        v1788 = v1807 ? v1805 : v1788;
                                        v1790 = float2(v1808.x ? v1790.x : v1799.x, v1808.y ? v1790.y : v1799.y);
                                        v1792 = float2(v1808.x ? v1799.x : v1792.x, v1808.y ? v1799.y : v1792.y);
                                        v1794++;
                                        continue;
                                    }
                                    float2 v1813 = lerp(v1792, v1790, clamp(v1788 / (v1788 - v1785), 0.0f, 1.0f).xx);
                                    float2 v1817 = float3(v1813.x, 0.4000000059604644775390625f, v1813.y).xz;
                                    v1818 = v176[3];
                                    v1824 = (v1818 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                    float v1830;
                                    float v1833;
                                    float2 v1835;
                                    float2 v1837;
                                    v1830 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, v1771, 0.0f).y;
                                    v1833 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, v1824, 0.0f).y;
                                    v1835 = v176[0];
                                    v1837 = v1818;
                                    for (uint v1839 = 0u; v1839 < 2u; )
                                    {
                                        float2 v1843 = v1835 + v1837;
                                        float2 v1844 = v1843 * 0.5f;
                                        float v1850 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v1844 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                        float v1851 = v1850 * v1830;
                                        bool v1852 = v1851 < 0.0f;
                                        bool2 v1853 = v1852.xx;
                                        v1830 = v1852 ? v1830 : v1850;
                                        v1833 = v1852 ? v1850 : v1833;
                                        v1835 = float2(v1853.x ? v1835.x : v1844.x, v1853.y ? v1835.y : v1844.y);
                                        v1837 = float2(v1853.x ? v1844.x : v1837.x, v1853.y ? v1844.y : v1837.y);
                                        v1839++;
                                        continue;
                                    }
                                    float2 v1858 = lerp(v1837, v1835, clamp(v1833 / (v1833 - v1830), 0.0f, 1.0f).xx);
                                    float2 v1862 = float3(v1858.x, 0.4000000059604644775390625f, v1858.y).xz;
                                    float v1863 = distance(v1817, v1862);
                                    if (v1863 < 0.4000000059604644775390625f)
                                    {
                                        v2102 = v159;
                                        v2103 = v159;
                                        v2104 = false;
                                        v2105 = false;
                                        break;
                                    }
                                    int v1869 = int(max(v1863 * 2.5f, 1.0f));
                                    uint v1871;
                                    uint v1874;
                                    bool v1876;
                                    bool v1878;
                                    v1871 = v159;
                                    v1874 = v159;
                                    v1876 = false;
                                    v1878 = false;
                                    for (int v1880 = 0; v1880 < (v1869 + 1); )
                                    {
                                        float v1886 = float(v1869);
                                        int v1881 = v1880 + 1;
                                        float2 v1893 = lerp(v1817, v1862, (((float(v1880) / v1886) + (float(v1881) / v1886)) * 0.5f).xx);
                                        float v1894 = v1893.x;
                                        float v1902 = v1893.y;
                                        int v1904 = v367 + (v1880 * 100);
                                        uint v1906 = uint(v1904 ^ 2346);
                                        uint v1908 = v1906 + (v1906 << 10u);
                                        uint v1910 = v1908 ^ (v1908 >> 6u);
                                        uint v1912 = v1910 + (v1910 << 3u);
                                        uint v1914 = v1912 ^ (v1912 >> 11u);
                                        uint v1916 = v1914 + (v1914 << 15u);
                                        bool v1917 = v1878 ? v1878 : true;
                                        uint v1918 = v1878 ? v1874 : 8388607u;
                                        bool v1919 = v1876 ? v1876 : true;
                                        uint v1920 = v1876 ? v1871 : 1065353216u;
                                        uint v1928 = uint(v1904 ^ 3459);
                                        uint v1930 = v1928 + (v1928 << 10u);
                                        uint v1932 = v1930 ^ (v1930 >> 6u);
                                        uint v1934 = v1932 + (v1932 << 3u);
                                        uint v1936 = v1934 ^ (v1934 >> 11u);
                                        bool v1939 = v1917 ? v1917 : true;
                                        uint v1940 = v1917 ? v1918 : 8388607u;
                                        bool v1941 = v1919 ? v1919 : true;
                                        uint v1942 = v1919 ? v1920 : 1065353216u;
                                        float v1948 = ((asfloat(((v1936 + (v1936 << 15u)) & v1940) | v1942) - 1.0f) * 2.0f) - 1.0f;
                                        uint v1950 = uint(v1904 ^ 234);
                                        uint v1952 = v1950 + (v1950 << 10u);
                                        uint v1954 = v1952 ^ (v1952 >> 6u);
                                        uint v1956 = v1954 + (v1954 << 3u);
                                        uint v1958 = v1956 ^ (v1956 >> 11u);
                                        bool v1961 = v1939 ? v1939 : true;
                                        uint v1962 = v1939 ? v1940 : 8388607u;
                                        bool v1963 = v1941 ? v1941 : true;
                                        uint v1964 = v1941 ? v1942 : 1065353216u;
                                        float3 v1971 = float3(((asfloat((v1916 & v1918) | v1920) - 1.0f) * 2.0f) - 1.0f, v1948, ((asfloat(((v1958 + (v1958 << 15u)) & v1962) | v1964) - 1.0f) * 2.0f) - 1.0f);
                                        v1971.y = v1948 * 0.20000000298023223876953125f;
                                        float3 v1974 = normalize(v1971);
                                        bool3 v1977 = (v1974.y < 0.0f).xxx;
                                        float3 v1978 = float3(v1977.x ? float3(0.0f, -1.0f, 0.0f).x : float3(0.0f, 1.0f, 0.0f).x, v1977.y ? float3(0.0f, -1.0f, 0.0f).y : float3(0.0f, 1.0f, 0.0f).y, v1977.z ? float3(0.0f, -1.0f, 0.0f).z : float3(0.0f, 1.0f, 0.0f).z);
                                        uint v1981 = uint(v1904 ^ 346);
                                        uint v1983 = v1981 + (v1981 << 10u);
                                        uint v1985 = v1983 ^ (v1983 >> 6u);
                                        uint v1987 = v1985 + (v1985 << 3u);
                                        uint v1989 = v1987 ^ (v1987 >> 11u);
                                        bool v1992 = v1961 ? v1961 : true;
                                        uint v1993 = v1961 ? v1962 : 8388607u;
                                        bool v1994 = v1963 ? v1963 : true;
                                        uint v1995 = v1963 ? v1964 : 1065353216u;
                                        bool v2002 = v1992 ? v1992 : true;
                                        uint v2003 = v1992 ? v1993 : 8388607u;
                                        bool v2004 = v1994 ? v1994 : true;
                                        uint v2005 = v1994 ? v1995 : 1065353216u;
                                        float2 v2011 = lerp(0.800000011920928955078125f, 1.5f, asfloat((v1916 & v2003) | v2005) - 1.0f).xx;
                                        float3 v2013 = float3(v2011.x, lerp(0.800000011920928955078125f, 1.2999999523162841796875f, asfloat(((v1989 + (v1989 << 15u)) & v1993) | v1995) - 1.0f).xxx.y, v2011.y) * 0.75f;
                                        float4x4 v2039 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v1894), float4(0.0f, 1.0f, 0.0f, lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v1893 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x)), float4(0.0f, 0.0f, 1.0f, v1902), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(transpose(float4x4(float4(v1974, 0.0f), float4(v1978, 0.0f), float4(cross(v1974, v1978), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))), float4x4(float4(v2013.x, 0.0f, 0.0f, 0.0f), float4(0.0f, v2013.y, 0.0f, 0.0f), float4(0.0f, 0.0f, v2013.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                        uint v2056;
                                        __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, v2056);
                                        float v2059 = 10.0f * (v1894 + v1902);
                                        uint v2061 = uint(v2059 + 640.0f);
                                        uint v2063 = v2061 + (v2061 << 10u);
                                        uint v2065 = v2063 ^ (v2063 >> 6u);
                                        uint v2067 = v2065 + (v2065 << 3u);
                                        uint v2069 = v2067 ^ (v2067 >> 11u);
                                        bool v2072 = v2002 ? v2002 : true;
                                        uint v2073 = v2002 ? v2003 : 8388607u;
                                        bool v2074 = v2004 ? v2004 : true;
                                        uint v2075 = v2004 ? v2005 : 1065353216u;
                                        uint v2081 = uint(v2059 + 987.0f);
                                        uint v2083 = v2081 + (v2081 << 10u);
                                        uint v2085 = v2083 ^ (v2083 >> 6u);
                                        uint v2087 = v2085 + (v2085 << 3u);
                                        uint v2089 = v2087 ^ (v2087 >> 11u);
                                        uint v1875 = v2072 ? v2073 : 8388607u;
                                        uint v1872 = v2074 ? v2075 : 1065353216u;
                                        uint v2097 = uint(int(v2056));
                                        float v2099[12] = { v2039[0].x, v2039[1].x, v2039[2].x, v2039[0].y, v2039[1].y, v2039[2].y, v2039[0].z, v2039[1].z, v2039[2].z, v2039[0].w, v2039[1].w, v2039[2].w };
                                        Affine3Packed v2100 = { v2099 };
                                        GardenStone v2101 = { v2100, float4(asfloat(((v2069 + (v2069 << 15u)) & v2073) | v2075) - 1.0f, asfloat(((v2089 + (v2089 << 15u)) & v1875) | v1872) - 1.0f, 0.0f, 0.0f) };
                                        [unroll]
                                        for (int _14ident = 0; _14ident < 12; _14ident++)
                                        {
                                            __alias_instance_buffer.Store<float>(_14ident * 4 + v2097 * 64 + 0, v2101.transform.data[_14ident]);
                                        }
                                        __alias_instance_buffer.Store<float4>(v2097 * 64 + 48, v2101.color);
                                        v1871 = v1872;
                                        v1874 = v1875;
                                        v1876 = v2074 ? v2074 : true;
                                        v1878 = v2072 ? v2072 : true;
                                        v1880 = v1881;
                                        continue;
                                    }
                                    v2102 = v1871;
                                    v2103 = v1874;
                                    v2104 = v1876;
                                    v2105 = v1878;
                                    break;
                                } while(false);
                                do
                                {
                                    float2 v2114 = (v176[2] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                    float v2120;
                                    float v2123;
                                    float2 v2125;
                                    float2 v2127;
                                    v2120 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, v1779, 0.0f).y;
                                    v2123 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, v2114, 0.0f).y;
                                    v2125 = v1769;
                                    v2127 = v176[2];
                                    for (uint v2129 = 0u; v2129 < 2u; )
                                    {
                                        float2 v2133 = v2125 + v2127;
                                        float2 v2134 = v2133 * 0.5f;
                                        float v2140 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v2134 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                        float v2141 = v2140 * v2120;
                                        bool v2142 = v2141 < 0.0f;
                                        bool2 v2143 = v2142.xx;
                                        v2120 = v2142 ? v2120 : v2140;
                                        v2123 = v2142 ? v2140 : v2123;
                                        v2125 = float2(v2143.x ? v2125.x : v2134.x, v2143.y ? v2125.y : v2134.y);
                                        v2127 = float2(v2143.x ? v2134.x : v2127.x, v2143.y ? v2134.y : v2127.y);
                                        v2129++;
                                        continue;
                                    }
                                    float2 v2148 = lerp(v2127, v2125, clamp(v2123 / (v2123 - v2120), 0.0f, 1.0f).xx);
                                    float2 v2152 = float3(v2148.x, 0.4000000059604644775390625f, v2148.y).xz;
                                    float v2162;
                                    float v2165;
                                    float2 v2167;
                                    float2 v2169;
                                    v2162 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, v1824, 0.0f).y;
                                    v2165 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, v2114, 0.0f).y;
                                    v2167 = v1818;
                                    v2169 = v176[2];
                                    for (uint v2171 = 0u; v2171 < 2u; )
                                    {
                                        float2 v2175 = v2167 + v2169;
                                        float2 v2176 = v2175 * 0.5f;
                                        float v2182 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v2176 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                        float v2183 = v2182 * v2162;
                                        bool v2184 = v2183 < 0.0f;
                                        bool2 v2185 = v2184.xx;
                                        v2162 = v2184 ? v2162 : v2182;
                                        v2165 = v2184 ? v2182 : v2165;
                                        v2167 = float2(v2185.x ? v2167.x : v2176.x, v2185.y ? v2167.y : v2176.y);
                                        v2169 = float2(v2185.x ? v2176.x : v2169.x, v2185.y ? v2176.y : v2169.y);
                                        v2171++;
                                        continue;
                                    }
                                    float2 v2190 = lerp(v2169, v2167, clamp(v2165 / (v2165 - v2162), 0.0f, 1.0f).xx);
                                    float2 v2194 = float3(v2190.x, 0.4000000059604644775390625f, v2190.y).xz;
                                    float v2195 = distance(v2152, v2194);
                                    if (v2195 < 0.4000000059604644775390625f)
                                    {
                                        break;
                                    }
                                    int v2201 = int(max(v2195 * 2.5f, 1.0f));
                                    uint v2203 = v2102;
                                    uint v2206 = v2103;
                                    bool v2208 = v2104;
                                    bool v2210 = v2105;
                                    int v2212 = 0;
                                    for (; v2212 < (v2201 + 1); )
                                    {
                                        float v2218 = float(v2201);
                                        int v2213 = v2212 + 1;
                                        float2 v2225 = lerp(v2152, v2194, (((float(v2212) / v2218) + (float(v2213) / v2218)) * 0.5f).xx);
                                        float v2226 = v2225.x;
                                        float v2234 = v2225.y;
                                        int v2236 = v367 + (v2212 * 100);
                                        uint v2238 = uint(v2236 ^ 2346);
                                        uint v2240 = v2238 + (v2238 << 10u);
                                        uint v2242 = v2240 ^ (v2240 >> 6u);
                                        uint v2244 = v2242 + (v2242 << 3u);
                                        uint v2246 = v2244 ^ (v2244 >> 11u);
                                        uint v2248 = v2246 + (v2246 << 15u);
                                        bool v2249 = v2210 ? v2210 : true;
                                        uint v2250 = v2210 ? v2206 : 8388607u;
                                        bool v2251 = v2208 ? v2208 : true;
                                        uint v2252 = v2208 ? v2203 : 1065353216u;
                                        uint v2260 = uint(v2236 ^ 3459);
                                        uint v2262 = v2260 + (v2260 << 10u);
                                        uint v2264 = v2262 ^ (v2262 >> 6u);
                                        uint v2266 = v2264 + (v2264 << 3u);
                                        uint v2268 = v2266 ^ (v2266 >> 11u);
                                        bool v2271 = v2249 ? v2249 : true;
                                        uint v2272 = v2249 ? v2250 : 8388607u;
                                        bool v2273 = v2251 ? v2251 : true;
                                        uint v2274 = v2251 ? v2252 : 1065353216u;
                                        float v2280 = ((asfloat(((v2268 + (v2268 << 15u)) & v2272) | v2274) - 1.0f) * 2.0f) - 1.0f;
                                        uint v2282 = uint(v2236 ^ 234);
                                        uint v2284 = v2282 + (v2282 << 10u);
                                        uint v2286 = v2284 ^ (v2284 >> 6u);
                                        uint v2288 = v2286 + (v2286 << 3u);
                                        uint v2290 = v2288 ^ (v2288 >> 11u);
                                        bool v2293 = v2271 ? v2271 : true;
                                        uint v2294 = v2271 ? v2272 : 8388607u;
                                        bool v2295 = v2273 ? v2273 : true;
                                        uint v2296 = v2273 ? v2274 : 1065353216u;
                                        float3 v2303 = float3(((asfloat((v2248 & v2250) | v2252) - 1.0f) * 2.0f) - 1.0f, v2280, ((asfloat(((v2290 + (v2290 << 15u)) & v2294) | v2296) - 1.0f) * 2.0f) - 1.0f);
                                        v2303.y = v2280 * 0.20000000298023223876953125f;
                                        float3 v2306 = normalize(v2303);
                                        bool3 v2309 = (v2306.y < 0.0f).xxx;
                                        float3 v2310 = float3(v2309.x ? float3(0.0f, -1.0f, 0.0f).x : float3(0.0f, 1.0f, 0.0f).x, v2309.y ? float3(0.0f, -1.0f, 0.0f).y : float3(0.0f, 1.0f, 0.0f).y, v2309.z ? float3(0.0f, -1.0f, 0.0f).z : float3(0.0f, 1.0f, 0.0f).z);
                                        uint v2313 = uint(v2236 ^ 346);
                                        uint v2315 = v2313 + (v2313 << 10u);
                                        uint v2317 = v2315 ^ (v2315 >> 6u);
                                        uint v2319 = v2317 + (v2317 << 3u);
                                        uint v2321 = v2319 ^ (v2319 >> 11u);
                                        bool v2324 = v2293 ? v2293 : true;
                                        uint v2325 = v2293 ? v2294 : 8388607u;
                                        bool v2326 = v2295 ? v2295 : true;
                                        uint v2327 = v2295 ? v2296 : 1065353216u;
                                        bool v2334 = v2324 ? v2324 : true;
                                        uint v2335 = v2324 ? v2325 : 8388607u;
                                        bool v2336 = v2326 ? v2326 : true;
                                        uint v2337 = v2326 ? v2327 : 1065353216u;
                                        float2 v2343 = lerp(0.800000011920928955078125f, 1.5f, asfloat((v2248 & v2335) | v2337) - 1.0f).xx;
                                        float3 v2345 = float3(v2343.x, lerp(0.800000011920928955078125f, 1.2999999523162841796875f, asfloat(((v2321 + (v2321 << 15u)) & v2325) | v2327) - 1.0f).xxx.y, v2343.y) * 0.75f;
                                        float4x4 v2371 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v2226), float4(0.0f, 1.0f, 0.0f, lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v2225 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x)), float4(0.0f, 0.0f, 1.0f, v2234), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(transpose(float4x4(float4(v2306, 0.0f), float4(v2310, 0.0f), float4(cross(v2306, v2310), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))), float4x4(float4(v2345.x, 0.0f, 0.0f, 0.0f), float4(0.0f, v2345.y, 0.0f, 0.0f), float4(0.0f, 0.0f, v2345.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                        uint v2388;
                                        __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, v2388);
                                        float v2391 = 10.0f * (v2226 + v2234);
                                        uint v2393 = uint(v2391 + 640.0f);
                                        uint v2395 = v2393 + (v2393 << 10u);
                                        uint v2397 = v2395 ^ (v2395 >> 6u);
                                        uint v2399 = v2397 + (v2397 << 3u);
                                        uint v2401 = v2399 ^ (v2399 >> 11u);
                                        bool v2404 = v2334 ? v2334 : true;
                                        uint v2405 = v2334 ? v2335 : 8388607u;
                                        bool v2406 = v2336 ? v2336 : true;
                                        uint v2407 = v2336 ? v2337 : 1065353216u;
                                        uint v2413 = uint(v2391 + 987.0f);
                                        uint v2415 = v2413 + (v2413 << 10u);
                                        uint v2417 = v2415 ^ (v2415 >> 6u);
                                        uint v2419 = v2417 + (v2417 << 3u);
                                        uint v2421 = v2419 ^ (v2419 >> 11u);
                                        uint v2207 = v2404 ? v2405 : 8388607u;
                                        uint v2204 = v2406 ? v2407 : 1065353216u;
                                        uint v2429 = uint(int(v2388));
                                        float v2431[12] = { v2371[0].x, v2371[1].x, v2371[2].x, v2371[0].y, v2371[1].y, v2371[2].y, v2371[0].z, v2371[1].z, v2371[2].z, v2371[0].w, v2371[1].w, v2371[2].w };
                                        Affine3Packed v2432 = { v2431 };
                                        GardenStone v2433 = { v2432, float4(asfloat(((v2401 + (v2401 << 15u)) & v2405) | v2407) - 1.0f, asfloat(((v2421 + (v2421 << 15u)) & v2207) | v2204) - 1.0f, 0.0f, 0.0f) };
                                        [unroll]
                                        for (int _15ident = 0; _15ident < 12; _15ident++)
                                        {
                                            __alias_instance_buffer.Store<float>(_15ident * 4 + v2429 * 64 + 0, v2433.transform.data[_15ident]);
                                        }
                                        __alias_instance_buffer.Store<float4>(v2429 * 64 + 48, v2433.color);
                                        v2203 = v2204;
                                        v2206 = v2207;
                                        v2208 = v2406 ? v2406 : true;
                                        v2210 = v2404 ? v2404 : true;
                                        v2212 = v2213;
                                        continue;
                                    }
                                    break;
                                } while(false);
                            }
                            else
                            {
                                bool v2439;
                                if (!(v368 == 6))
                                {
                                    v2439 = v368 == 9;
                                }
                                else
                                {
                                    v2439 = true;
                                }
                                if (v2439)
                                {
                                    do
                                    {
                                        float v2462;
                                        float v2465;
                                        float2 v2467;
                                        float2 v2469;
                                        v2462 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v176[0] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                        v2465 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v176[1] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                        v2467 = v176[0];
                                        v2469 = v176[1];
                                        for (uint v2471 = 0u; v2471 < 2u; )
                                        {
                                            float2 v2475 = v2467 + v2469;
                                            float2 v2476 = v2475 * 0.5f;
                                            float v2482 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v2476 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                            float v2483 = v2482 * v2462;
                                            bool v2484 = v2483 < 0.0f;
                                            bool2 v2485 = v2484.xx;
                                            v2462 = v2484 ? v2462 : v2482;
                                            v2465 = v2484 ? v2482 : v2465;
                                            v2467 = float2(v2485.x ? v2467.x : v2476.x, v2485.y ? v2467.y : v2476.y);
                                            v2469 = float2(v2485.x ? v2476.x : v2469.x, v2485.y ? v2476.y : v2469.y);
                                            v2471++;
                                            continue;
                                        }
                                        float2 v2490 = lerp(v2469, v2467, clamp(v2465 / (v2465 - v2462), 0.0f, 1.0f).xx);
                                        float2 v2494 = float3(v2490.x, 0.4000000059604644775390625f, v2490.y).xz;
                                        float v2510;
                                        float v2513;
                                        float2 v2515;
                                        float2 v2517;
                                        v2510 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v176[3] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                        v2513 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v176[2] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                        v2515 = v176[3];
                                        v2517 = v176[2];
                                        for (uint v2519 = 0u; v2519 < 2u; )
                                        {
                                            float2 v2523 = v2515 + v2517;
                                            float2 v2524 = v2523 * 0.5f;
                                            float v2530 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v2524 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                            float v2531 = v2530 * v2510;
                                            bool v2532 = v2531 < 0.0f;
                                            bool2 v2533 = v2532.xx;
                                            v2510 = v2532 ? v2510 : v2530;
                                            v2513 = v2532 ? v2530 : v2513;
                                            v2515 = float2(v2533.x ? v2515.x : v2524.x, v2533.y ? v2515.y : v2524.y);
                                            v2517 = float2(v2533.x ? v2524.x : v2517.x, v2533.y ? v2524.y : v2517.y);
                                            v2519++;
                                            continue;
                                        }
                                        float2 v2538 = lerp(v2517, v2515, clamp(v2513 / (v2513 - v2510), 0.0f, 1.0f).xx);
                                        float2 v2542 = float3(v2538.x, 0.4000000059604644775390625f, v2538.y).xz;
                                        float v2543 = distance(v2494, v2542);
                                        if (v2543 < 0.4000000059604644775390625f)
                                        {
                                            break;
                                        }
                                        int v2549 = int(max(v2543 * 2.5f, 1.0f));
                                        uint v2551 = 0u;
                                        uint v2554 = 0u;
                                        bool v2556 = false;
                                        bool v2558 = false;
                                        int v2560 = 0;
                                        for (; v2560 < (v2549 + 1); )
                                        {
                                            float v2566 = float(v2549);
                                            int v2561 = v2560 + 1;
                                            float2 v2573 = lerp(v2494, v2542, (((float(v2560) / v2566) + (float(v2561) / v2566)) * 0.5f).xx);
                                            float v2574 = v2573.x;
                                            float v2582 = v2573.y;
                                            int v2584 = v367 + (v2560 * 100);
                                            uint v2586 = uint(v2584 ^ 2346);
                                            uint v2588 = v2586 + (v2586 << 10u);
                                            uint v2590 = v2588 ^ (v2588 >> 6u);
                                            uint v2592 = v2590 + (v2590 << 3u);
                                            uint v2594 = v2592 ^ (v2592 >> 11u);
                                            uint v2596 = v2594 + (v2594 << 15u);
                                            bool v2597 = v2558 ? v2558 : true;
                                            uint v2598 = v2558 ? v2554 : 8388607u;
                                            bool v2599 = v2556 ? v2556 : true;
                                            uint v2600 = v2556 ? v2551 : 1065353216u;
                                            uint v2608 = uint(v2584 ^ 3459);
                                            uint v2610 = v2608 + (v2608 << 10u);
                                            uint v2612 = v2610 ^ (v2610 >> 6u);
                                            uint v2614 = v2612 + (v2612 << 3u);
                                            uint v2616 = v2614 ^ (v2614 >> 11u);
                                            bool v2619 = v2597 ? v2597 : true;
                                            uint v2620 = v2597 ? v2598 : 8388607u;
                                            bool v2621 = v2599 ? v2599 : true;
                                            uint v2622 = v2599 ? v2600 : 1065353216u;
                                            float v2628 = ((asfloat(((v2616 + (v2616 << 15u)) & v2620) | v2622) - 1.0f) * 2.0f) - 1.0f;
                                            uint v2630 = uint(v2584 ^ 234);
                                            uint v2632 = v2630 + (v2630 << 10u);
                                            uint v2634 = v2632 ^ (v2632 >> 6u);
                                            uint v2636 = v2634 + (v2634 << 3u);
                                            uint v2638 = v2636 ^ (v2636 >> 11u);
                                            bool v2641 = v2619 ? v2619 : true;
                                            uint v2642 = v2619 ? v2620 : 8388607u;
                                            bool v2643 = v2621 ? v2621 : true;
                                            uint v2644 = v2621 ? v2622 : 1065353216u;
                                            float3 v2651 = float3(((asfloat((v2596 & v2598) | v2600) - 1.0f) * 2.0f) - 1.0f, v2628, ((asfloat(((v2638 + (v2638 << 15u)) & v2642) | v2644) - 1.0f) * 2.0f) - 1.0f);
                                            v2651.y = v2628 * 0.20000000298023223876953125f;
                                            float3 v2654 = normalize(v2651);
                                            bool3 v2657 = (v2654.y < 0.0f).xxx;
                                            float3 v2658 = float3(v2657.x ? float3(0.0f, -1.0f, 0.0f).x : float3(0.0f, 1.0f, 0.0f).x, v2657.y ? float3(0.0f, -1.0f, 0.0f).y : float3(0.0f, 1.0f, 0.0f).y, v2657.z ? float3(0.0f, -1.0f, 0.0f).z : float3(0.0f, 1.0f, 0.0f).z);
                                            uint v2661 = uint(v2584 ^ 346);
                                            uint v2663 = v2661 + (v2661 << 10u);
                                            uint v2665 = v2663 ^ (v2663 >> 6u);
                                            uint v2667 = v2665 + (v2665 << 3u);
                                            uint v2669 = v2667 ^ (v2667 >> 11u);
                                            bool v2672 = v2641 ? v2641 : true;
                                            uint v2673 = v2641 ? v2642 : 8388607u;
                                            bool v2674 = v2643 ? v2643 : true;
                                            uint v2675 = v2643 ? v2644 : 1065353216u;
                                            bool v2682 = v2672 ? v2672 : true;
                                            uint v2683 = v2672 ? v2673 : 8388607u;
                                            bool v2684 = v2674 ? v2674 : true;
                                            uint v2685 = v2674 ? v2675 : 1065353216u;
                                            float2 v2691 = lerp(0.800000011920928955078125f, 1.5f, asfloat((v2596 & v2683) | v2685) - 1.0f).xx;
                                            float3 v2693 = float3(v2691.x, lerp(0.800000011920928955078125f, 1.2999999523162841796875f, asfloat(((v2669 + (v2669 << 15u)) & v2673) | v2675) - 1.0f).xxx.y, v2691.y) * 0.75f;
                                            float4x4 v2719 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v2574), float4(0.0f, 1.0f, 0.0f, lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v2573 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x)), float4(0.0f, 0.0f, 1.0f, v2582), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(transpose(float4x4(float4(v2654, 0.0f), float4(v2658, 0.0f), float4(cross(v2654, v2658), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))), float4x4(float4(v2693.x, 0.0f, 0.0f, 0.0f), float4(0.0f, v2693.y, 0.0f, 0.0f), float4(0.0f, 0.0f, v2693.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                            uint v2736;
                                            __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, v2736);
                                            float v2739 = 10.0f * (v2574 + v2582);
                                            uint v2741 = uint(v2739 + 640.0f);
                                            uint v2743 = v2741 + (v2741 << 10u);
                                            uint v2745 = v2743 ^ (v2743 >> 6u);
                                            uint v2747 = v2745 + (v2745 << 3u);
                                            uint v2749 = v2747 ^ (v2747 >> 11u);
                                            bool v2752 = v2682 ? v2682 : true;
                                            uint v2753 = v2682 ? v2683 : 8388607u;
                                            bool v2754 = v2684 ? v2684 : true;
                                            uint v2755 = v2684 ? v2685 : 1065353216u;
                                            uint v2761 = uint(v2739 + 987.0f);
                                            uint v2763 = v2761 + (v2761 << 10u);
                                            uint v2765 = v2763 ^ (v2763 >> 6u);
                                            uint v2767 = v2765 + (v2765 << 3u);
                                            uint v2769 = v2767 ^ (v2767 >> 11u);
                                            uint v2555 = v2752 ? v2753 : 8388607u;
                                            uint v2552 = v2754 ? v2755 : 1065353216u;
                                            uint v2777 = uint(int(v2736));
                                            float v2779[12] = { v2719[0].x, v2719[1].x, v2719[2].x, v2719[0].y, v2719[1].y, v2719[2].y, v2719[0].z, v2719[1].z, v2719[2].z, v2719[0].w, v2719[1].w, v2719[2].w };
                                            Affine3Packed v2780 = { v2779 };
                                            GardenStone v2781 = { v2780, float4(asfloat(((v2749 + (v2749 << 15u)) & v2753) | v2755) - 1.0f, asfloat(((v2769 + (v2769 << 15u)) & v2555) | v2552) - 1.0f, 0.0f, 0.0f) };
                                            [unroll]
                                            for (int _16ident = 0; _16ident < 12; _16ident++)
                                            {
                                                __alias_instance_buffer.Store<float>(_16ident * 4 + v2777 * 64 + 0, v2781.transform.data[_16ident]);
                                            }
                                            __alias_instance_buffer.Store<float4>(v2777 * 64 + 48, v2781.color);
                                            v2551 = v2552;
                                            v2554 = v2555;
                                            v2556 = v2754 ? v2754 : true;
                                            v2558 = v2752 ? v2752 : true;
                                            v2560 = v2561;
                                            continue;
                                        }
                                        break;
                                    } while(false);
                                }
                                else
                                {
                                    bool v2787;
                                    if (!(v368 == 7))
                                    {
                                        v2787 = v368 == 8;
                                    }
                                    else
                                    {
                                        v2787 = true;
                                    }
                                    if (v2787)
                                    {
                                        do
                                        {
                                            float2 v2796 = (v176[0] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                            float v2810;
                                            float v2813;
                                            float2 v2815;
                                            float2 v2817;
                                            v2810 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, v2796, 0.0f).y;
                                            v2813 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v176[1] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                            v2815 = v176[0];
                                            v2817 = v176[1];
                                            for (uint v2819 = 0u; v2819 < 2u; )
                                            {
                                                float2 v2823 = v2815 + v2817;
                                                float2 v2824 = v2823 * 0.5f;
                                                float v2830 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v2824 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                                float v2831 = v2830 * v2810;
                                                bool v2832 = v2831 < 0.0f;
                                                bool2 v2833 = v2832.xx;
                                                v2810 = v2832 ? v2810 : v2830;
                                                v2813 = v2832 ? v2830 : v2813;
                                                v2815 = float2(v2833.x ? v2815.x : v2824.x, v2833.y ? v2815.y : v2824.y);
                                                v2817 = float2(v2833.x ? v2824.x : v2817.x, v2833.y ? v2824.y : v2817.y);
                                                v2819++;
                                                continue;
                                            }
                                            float2 v2838 = lerp(v2817, v2815, clamp(v2813 / (v2813 - v2810), 0.0f, 1.0f).xx);
                                            float2 v2842 = float3(v2838.x, 0.4000000059604644775390625f, v2838.y).xz;
                                            float v2855;
                                            float v2858;
                                            float2 v2860;
                                            float2 v2862;
                                            v2855 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, v2796, 0.0f).y;
                                            v2858 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v176[3] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                            v2860 = v176[0];
                                            v2862 = v176[3];
                                            for (uint v2864 = 0u; v2864 < 2u; )
                                            {
                                                float2 v2868 = v2860 + v2862;
                                                float2 v2869 = v2868 * 0.5f;
                                                float v2875 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v2869 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                                float v2876 = v2875 * v2855;
                                                bool v2877 = v2876 < 0.0f;
                                                bool2 v2878 = v2877.xx;
                                                v2855 = v2877 ? v2855 : v2875;
                                                v2858 = v2877 ? v2875 : v2858;
                                                v2860 = float2(v2878.x ? v2860.x : v2869.x, v2878.y ? v2860.y : v2869.y);
                                                v2862 = float2(v2878.x ? v2869.x : v2862.x, v2878.y ? v2869.y : v2862.y);
                                                v2864++;
                                                continue;
                                            }
                                            float2 v2883 = lerp(v2862, v2860, clamp(v2858 / (v2858 - v2855), 0.0f, 1.0f).xx);
                                            float2 v2887 = float3(v2883.x, 0.4000000059604644775390625f, v2883.y).xz;
                                            float v2888 = distance(v2842, v2887);
                                            if (v2888 < 0.4000000059604644775390625f)
                                            {
                                                break;
                                            }
                                            int v2894 = int(max(v2888 * 2.5f, 1.0f));
                                            uint v2896 = 0u;
                                            uint v2899 = 0u;
                                            bool v2901 = false;
                                            bool v2903 = false;
                                            int v2905 = 0;
                                            for (; v2905 < (v2894 + 1); )
                                            {
                                                float v2911 = float(v2894);
                                                int v2906 = v2905 + 1;
                                                float2 v2918 = lerp(v2842, v2887, (((float(v2905) / v2911) + (float(v2906) / v2911)) * 0.5f).xx);
                                                float v2919 = v2918.x;
                                                float v2927 = v2918.y;
                                                int v2929 = v367 + (v2905 * 100);
                                                uint v2931 = uint(v2929 ^ 2346);
                                                uint v2933 = v2931 + (v2931 << 10u);
                                                uint v2935 = v2933 ^ (v2933 >> 6u);
                                                uint v2937 = v2935 + (v2935 << 3u);
                                                uint v2939 = v2937 ^ (v2937 >> 11u);
                                                uint v2941 = v2939 + (v2939 << 15u);
                                                bool v2942 = v2903 ? v2903 : true;
                                                uint v2943 = v2903 ? v2899 : 8388607u;
                                                bool v2944 = v2901 ? v2901 : true;
                                                uint v2945 = v2901 ? v2896 : 1065353216u;
                                                uint v2953 = uint(v2929 ^ 3459);
                                                uint v2955 = v2953 + (v2953 << 10u);
                                                uint v2957 = v2955 ^ (v2955 >> 6u);
                                                uint v2959 = v2957 + (v2957 << 3u);
                                                uint v2961 = v2959 ^ (v2959 >> 11u);
                                                bool v2964 = v2942 ? v2942 : true;
                                                uint v2965 = v2942 ? v2943 : 8388607u;
                                                bool v2966 = v2944 ? v2944 : true;
                                                uint v2967 = v2944 ? v2945 : 1065353216u;
                                                float v2973 = ((asfloat(((v2961 + (v2961 << 15u)) & v2965) | v2967) - 1.0f) * 2.0f) - 1.0f;
                                                uint v2975 = uint(v2929 ^ 234);
                                                uint v2977 = v2975 + (v2975 << 10u);
                                                uint v2979 = v2977 ^ (v2977 >> 6u);
                                                uint v2981 = v2979 + (v2979 << 3u);
                                                uint v2983 = v2981 ^ (v2981 >> 11u);
                                                bool v2986 = v2964 ? v2964 : true;
                                                uint v2987 = v2964 ? v2965 : 8388607u;
                                                bool v2988 = v2966 ? v2966 : true;
                                                uint v2989 = v2966 ? v2967 : 1065353216u;
                                                float3 v2996 = float3(((asfloat((v2941 & v2943) | v2945) - 1.0f) * 2.0f) - 1.0f, v2973, ((asfloat(((v2983 + (v2983 << 15u)) & v2987) | v2989) - 1.0f) * 2.0f) - 1.0f);
                                                v2996.y = v2973 * 0.20000000298023223876953125f;
                                                float3 v2999 = normalize(v2996);
                                                bool3 v3002 = (v2999.y < 0.0f).xxx;
                                                float3 v3003 = float3(v3002.x ? float3(0.0f, -1.0f, 0.0f).x : float3(0.0f, 1.0f, 0.0f).x, v3002.y ? float3(0.0f, -1.0f, 0.0f).y : float3(0.0f, 1.0f, 0.0f).y, v3002.z ? float3(0.0f, -1.0f, 0.0f).z : float3(0.0f, 1.0f, 0.0f).z);
                                                uint v3006 = uint(v2929 ^ 346);
                                                uint v3008 = v3006 + (v3006 << 10u);
                                                uint v3010 = v3008 ^ (v3008 >> 6u);
                                                uint v3012 = v3010 + (v3010 << 3u);
                                                uint v3014 = v3012 ^ (v3012 >> 11u);
                                                bool v3017 = v2986 ? v2986 : true;
                                                uint v3018 = v2986 ? v2987 : 8388607u;
                                                bool v3019 = v2988 ? v2988 : true;
                                                uint v3020 = v2988 ? v2989 : 1065353216u;
                                                bool v3027 = v3017 ? v3017 : true;
                                                uint v3028 = v3017 ? v3018 : 8388607u;
                                                bool v3029 = v3019 ? v3019 : true;
                                                uint v3030 = v3019 ? v3020 : 1065353216u;
                                                float2 v3036 = lerp(0.800000011920928955078125f, 1.5f, asfloat((v2941 & v3028) | v3030) - 1.0f).xx;
                                                float3 v3038 = float3(v3036.x, lerp(0.800000011920928955078125f, 1.2999999523162841796875f, asfloat(((v3014 + (v3014 << 15u)) & v3018) | v3020) - 1.0f).xxx.y, v3036.y) * 0.75f;
                                                float4x4 v3064 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v2919), float4(0.0f, 1.0f, 0.0f, lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v2918 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x)), float4(0.0f, 0.0f, 1.0f, v2927), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(transpose(float4x4(float4(v2999, 0.0f), float4(v3003, 0.0f), float4(cross(v2999, v3003), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))), float4x4(float4(v3038.x, 0.0f, 0.0f, 0.0f), float4(0.0f, v3038.y, 0.0f, 0.0f), float4(0.0f, 0.0f, v3038.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                                uint v3081;
                                                __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, v3081);
                                                float v3084 = 10.0f * (v2919 + v2927);
                                                uint v3086 = uint(v3084 + 640.0f);
                                                uint v3088 = v3086 + (v3086 << 10u);
                                                uint v3090 = v3088 ^ (v3088 >> 6u);
                                                uint v3092 = v3090 + (v3090 << 3u);
                                                uint v3094 = v3092 ^ (v3092 >> 11u);
                                                bool v3097 = v3027 ? v3027 : true;
                                                uint v3098 = v3027 ? v3028 : 8388607u;
                                                bool v3099 = v3029 ? v3029 : true;
                                                uint v3100 = v3029 ? v3030 : 1065353216u;
                                                uint v3106 = uint(v3084 + 987.0f);
                                                uint v3108 = v3106 + (v3106 << 10u);
                                                uint v3110 = v3108 ^ (v3108 >> 6u);
                                                uint v3112 = v3110 + (v3110 << 3u);
                                                uint v3114 = v3112 ^ (v3112 >> 11u);
                                                uint v2900 = v3097 ? v3098 : 8388607u;
                                                uint v2897 = v3099 ? v3100 : 1065353216u;
                                                uint v3122 = uint(int(v3081));
                                                float v3124[12] = { v3064[0].x, v3064[1].x, v3064[2].x, v3064[0].y, v3064[1].y, v3064[2].y, v3064[0].z, v3064[1].z, v3064[2].z, v3064[0].w, v3064[1].w, v3064[2].w };
                                                Affine3Packed v3125 = { v3124 };
                                                GardenStone v3126 = { v3125, float4(asfloat(((v3094 + (v3094 << 15u)) & v3098) | v3100) - 1.0f, asfloat(((v3114 + (v3114 << 15u)) & v2900) | v2897) - 1.0f, 0.0f, 0.0f) };
                                                [unroll]
                                                for (int _17ident = 0; _17ident < 12; _17ident++)
                                                {
                                                    __alias_instance_buffer.Store<float>(_17ident * 4 + v3122 * 64 + 0, v3126.transform.data[_17ident]);
                                                }
                                                __alias_instance_buffer.Store<float4>(v3122 * 64 + 48, v3126.color);
                                                v2896 = v2897;
                                                v2899 = v2900;
                                                v2901 = v3099 ? v3099 : true;
                                                v2903 = v3097 ? v3097 : true;
                                                v2905 = v2906;
                                                continue;
                                            }
                                            break;
                                        } while(false);
                                    }
                                    else
                                    {
                                        if (v368 == 10)
                                        {
                                            float2 v3132;
                                            float2 v3135;
                                            float2 v3182;
                                            float2 v3188;
                                            uint v3466;
                                            uint v3467;
                                            bool v3468;
                                            bool v3469;
                                            do
                                            {
                                                v3132 = v176[0];
                                                v3135 = (v3132 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                                float2 v3143 = (v176[1] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                                float v3149;
                                                float v3152;
                                                float2 v3154;
                                                float2 v3156;
                                                v3149 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, v3135, 0.0f).y;
                                                v3152 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, v3143, 0.0f).y;
                                                v3154 = v3132;
                                                v3156 = v176[1];
                                                for (uint v3158 = 0u; v3158 < 2u; )
                                                {
                                                    float2 v3162 = v3154 + v3156;
                                                    float2 v3163 = v3162 * 0.5f;
                                                    float v3169 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v3163 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                                    float v3170 = v3169 * v3149;
                                                    bool v3171 = v3170 < 0.0f;
                                                    bool2 v3172 = v3171.xx;
                                                    v3149 = v3171 ? v3149 : v3169;
                                                    v3152 = v3171 ? v3169 : v3152;
                                                    v3154 = float2(v3172.x ? v3154.x : v3163.x, v3172.y ? v3154.y : v3163.y);
                                                    v3156 = float2(v3172.x ? v3163.x : v3156.x, v3172.y ? v3163.y : v3156.y);
                                                    v3158++;
                                                    continue;
                                                }
                                                float2 v3177 = lerp(v3156, v3154, clamp(v3152 / (v3152 - v3149), 0.0f, 1.0f).xx);
                                                float2 v3181 = float3(v3177.x, 0.4000000059604644775390625f, v3177.y).xz;
                                                v3182 = v176[2];
                                                v3188 = (v3182 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                                float v3194;
                                                float v3197;
                                                float2 v3199;
                                                float2 v3201;
                                                v3194 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, v3143, 0.0f).y;
                                                v3197 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, v3188, 0.0f).y;
                                                v3199 = v176[1];
                                                v3201 = v3182;
                                                for (uint v3203 = 0u; v3203 < 2u; )
                                                {
                                                    float2 v3207 = v3199 + v3201;
                                                    float2 v3208 = v3207 * 0.5f;
                                                    float v3214 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v3208 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                                    float v3215 = v3214 * v3194;
                                                    bool v3216 = v3215 < 0.0f;
                                                    bool2 v3217 = v3216.xx;
                                                    v3194 = v3216 ? v3194 : v3214;
                                                    v3197 = v3216 ? v3214 : v3197;
                                                    v3199 = float2(v3217.x ? v3199.x : v3208.x, v3217.y ? v3199.y : v3208.y);
                                                    v3201 = float2(v3217.x ? v3208.x : v3201.x, v3217.y ? v3208.y : v3201.y);
                                                    v3203++;
                                                    continue;
                                                }
                                                float2 v3222 = lerp(v3201, v3199, clamp(v3197 / (v3197 - v3194), 0.0f, 1.0f).xx);
                                                float2 v3226 = float3(v3222.x, 0.4000000059604644775390625f, v3222.y).xz;
                                                float v3227 = distance(v3181, v3226);
                                                if (v3227 < 0.4000000059604644775390625f)
                                                {
                                                    v3466 = v159;
                                                    v3467 = v159;
                                                    v3468 = false;
                                                    v3469 = false;
                                                    break;
                                                }
                                                int v3233 = int(max(v3227 * 2.5f, 1.0f));
                                                uint v3235;
                                                uint v3238;
                                                bool v3240;
                                                bool v3242;
                                                v3235 = v159;
                                                v3238 = v159;
                                                v3240 = false;
                                                v3242 = false;
                                                for (int v3244 = 0; v3244 < (v3233 + 1); )
                                                {
                                                    float v3250 = float(v3233);
                                                    int v3245 = v3244 + 1;
                                                    float2 v3257 = lerp(v3181, v3226, (((float(v3244) / v3250) + (float(v3245) / v3250)) * 0.5f).xx);
                                                    float v3258 = v3257.x;
                                                    float v3266 = v3257.y;
                                                    int v3268 = v367 + (v3244 * 100);
                                                    uint v3270 = uint(v3268 ^ 2346);
                                                    uint v3272 = v3270 + (v3270 << 10u);
                                                    uint v3274 = v3272 ^ (v3272 >> 6u);
                                                    uint v3276 = v3274 + (v3274 << 3u);
                                                    uint v3278 = v3276 ^ (v3276 >> 11u);
                                                    uint v3280 = v3278 + (v3278 << 15u);
                                                    bool v3281 = v3242 ? v3242 : true;
                                                    uint v3282 = v3242 ? v3238 : 8388607u;
                                                    bool v3283 = v3240 ? v3240 : true;
                                                    uint v3284 = v3240 ? v3235 : 1065353216u;
                                                    uint v3292 = uint(v3268 ^ 3459);
                                                    uint v3294 = v3292 + (v3292 << 10u);
                                                    uint v3296 = v3294 ^ (v3294 >> 6u);
                                                    uint v3298 = v3296 + (v3296 << 3u);
                                                    uint v3300 = v3298 ^ (v3298 >> 11u);
                                                    bool v3303 = v3281 ? v3281 : true;
                                                    uint v3304 = v3281 ? v3282 : 8388607u;
                                                    bool v3305 = v3283 ? v3283 : true;
                                                    uint v3306 = v3283 ? v3284 : 1065353216u;
                                                    float v3312 = ((asfloat(((v3300 + (v3300 << 15u)) & v3304) | v3306) - 1.0f) * 2.0f) - 1.0f;
                                                    uint v3314 = uint(v3268 ^ 234);
                                                    uint v3316 = v3314 + (v3314 << 10u);
                                                    uint v3318 = v3316 ^ (v3316 >> 6u);
                                                    uint v3320 = v3318 + (v3318 << 3u);
                                                    uint v3322 = v3320 ^ (v3320 >> 11u);
                                                    bool v3325 = v3303 ? v3303 : true;
                                                    uint v3326 = v3303 ? v3304 : 8388607u;
                                                    bool v3327 = v3305 ? v3305 : true;
                                                    uint v3328 = v3305 ? v3306 : 1065353216u;
                                                    float3 v3335 = float3(((asfloat((v3280 & v3282) | v3284) - 1.0f) * 2.0f) - 1.0f, v3312, ((asfloat(((v3322 + (v3322 << 15u)) & v3326) | v3328) - 1.0f) * 2.0f) - 1.0f);
                                                    v3335.y = v3312 * 0.20000000298023223876953125f;
                                                    float3 v3338 = normalize(v3335);
                                                    bool3 v3341 = (v3338.y < 0.0f).xxx;
                                                    float3 v3342 = float3(v3341.x ? float3(0.0f, -1.0f, 0.0f).x : float3(0.0f, 1.0f, 0.0f).x, v3341.y ? float3(0.0f, -1.0f, 0.0f).y : float3(0.0f, 1.0f, 0.0f).y, v3341.z ? float3(0.0f, -1.0f, 0.0f).z : float3(0.0f, 1.0f, 0.0f).z);
                                                    uint v3345 = uint(v3268 ^ 346);
                                                    uint v3347 = v3345 + (v3345 << 10u);
                                                    uint v3349 = v3347 ^ (v3347 >> 6u);
                                                    uint v3351 = v3349 + (v3349 << 3u);
                                                    uint v3353 = v3351 ^ (v3351 >> 11u);
                                                    bool v3356 = v3325 ? v3325 : true;
                                                    uint v3357 = v3325 ? v3326 : 8388607u;
                                                    bool v3358 = v3327 ? v3327 : true;
                                                    uint v3359 = v3327 ? v3328 : 1065353216u;
                                                    bool v3366 = v3356 ? v3356 : true;
                                                    uint v3367 = v3356 ? v3357 : 8388607u;
                                                    bool v3368 = v3358 ? v3358 : true;
                                                    uint v3369 = v3358 ? v3359 : 1065353216u;
                                                    float2 v3375 = lerp(0.800000011920928955078125f, 1.5f, asfloat((v3280 & v3367) | v3369) - 1.0f).xx;
                                                    float3 v3377 = float3(v3375.x, lerp(0.800000011920928955078125f, 1.2999999523162841796875f, asfloat(((v3353 + (v3353 << 15u)) & v3357) | v3359) - 1.0f).xxx.y, v3375.y) * 0.75f;
                                                    float4x4 v3403 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v3258), float4(0.0f, 1.0f, 0.0f, lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v3257 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x)), float4(0.0f, 0.0f, 1.0f, v3266), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(transpose(float4x4(float4(v3338, 0.0f), float4(v3342, 0.0f), float4(cross(v3338, v3342), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))), float4x4(float4(v3377.x, 0.0f, 0.0f, 0.0f), float4(0.0f, v3377.y, 0.0f, 0.0f), float4(0.0f, 0.0f, v3377.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                                    uint v3420;
                                                    __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, v3420);
                                                    float v3423 = 10.0f * (v3258 + v3266);
                                                    uint v3425 = uint(v3423 + 640.0f);
                                                    uint v3427 = v3425 + (v3425 << 10u);
                                                    uint v3429 = v3427 ^ (v3427 >> 6u);
                                                    uint v3431 = v3429 + (v3429 << 3u);
                                                    uint v3433 = v3431 ^ (v3431 >> 11u);
                                                    bool v3436 = v3366 ? v3366 : true;
                                                    uint v3437 = v3366 ? v3367 : 8388607u;
                                                    bool v3438 = v3368 ? v3368 : true;
                                                    uint v3439 = v3368 ? v3369 : 1065353216u;
                                                    uint v3445 = uint(v3423 + 987.0f);
                                                    uint v3447 = v3445 + (v3445 << 10u);
                                                    uint v3449 = v3447 ^ (v3447 >> 6u);
                                                    uint v3451 = v3449 + (v3449 << 3u);
                                                    uint v3453 = v3451 ^ (v3451 >> 11u);
                                                    uint v3239 = v3436 ? v3437 : 8388607u;
                                                    uint v3236 = v3438 ? v3439 : 1065353216u;
                                                    uint v3461 = uint(int(v3420));
                                                    float v3463[12] = { v3403[0].x, v3403[1].x, v3403[2].x, v3403[0].y, v3403[1].y, v3403[2].y, v3403[0].z, v3403[1].z, v3403[2].z, v3403[0].w, v3403[1].w, v3403[2].w };
                                                    Affine3Packed v3464 = { v3463 };
                                                    GardenStone v3465 = { v3464, float4(asfloat(((v3433 + (v3433 << 15u)) & v3437) | v3439) - 1.0f, asfloat(((v3453 + (v3453 << 15u)) & v3239) | v3236) - 1.0f, 0.0f, 0.0f) };
                                                    [unroll]
                                                    for (int _18ident = 0; _18ident < 12; _18ident++)
                                                    {
                                                        __alias_instance_buffer.Store<float>(_18ident * 4 + v3461 * 64 + 0, v3465.transform.data[_18ident]);
                                                    }
                                                    __alias_instance_buffer.Store<float4>(v3461 * 64 + 48, v3465.color);
                                                    v3235 = v3236;
                                                    v3238 = v3239;
                                                    v3240 = v3438 ? v3438 : true;
                                                    v3242 = v3436 ? v3436 : true;
                                                    v3244 = v3245;
                                                    continue;
                                                }
                                                v3466 = v3235;
                                                v3467 = v3238;
                                                v3468 = v3240;
                                                v3469 = v3242;
                                                break;
                                            } while(false);
                                            do
                                            {
                                                float2 v3478 = (v176[3] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                                float v3484;
                                                float v3487;
                                                float2 v3489;
                                                float2 v3491;
                                                v3484 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, v3135, 0.0f).y;
                                                v3487 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, v3478, 0.0f).y;
                                                v3489 = v3132;
                                                v3491 = v176[3];
                                                for (uint v3493 = 0u; v3493 < 2u; )
                                                {
                                                    float2 v3497 = v3489 + v3491;
                                                    float2 v3498 = v3497 * 0.5f;
                                                    float v3504 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v3498 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                                    float v3505 = v3504 * v3484;
                                                    bool v3506 = v3505 < 0.0f;
                                                    bool2 v3507 = v3506.xx;
                                                    v3484 = v3506 ? v3484 : v3504;
                                                    v3487 = v3506 ? v3504 : v3487;
                                                    v3489 = float2(v3507.x ? v3489.x : v3498.x, v3507.y ? v3489.y : v3498.y);
                                                    v3491 = float2(v3507.x ? v3498.x : v3491.x, v3507.y ? v3498.y : v3491.y);
                                                    v3493++;
                                                    continue;
                                                }
                                                float2 v3512 = lerp(v3491, v3489, clamp(v3487 / (v3487 - v3484), 0.0f, 1.0f).xx);
                                                float2 v3516 = float3(v3512.x, 0.4000000059604644775390625f, v3512.y).xz;
                                                float v3526;
                                                float v3529;
                                                float2 v3531;
                                                float2 v3533;
                                                v3526 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, v3478, 0.0f).y;
                                                v3529 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, v3188, 0.0f).y;
                                                v3531 = v176[3];
                                                v3533 = v3182;
                                                for (uint v3535 = 0u; v3535 < 2u; )
                                                {
                                                    float2 v3539 = v3531 + v3533;
                                                    float2 v3540 = v3539 * 0.5f;
                                                    float v3546 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (v3540 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                                    float v3547 = v3546 * v3526;
                                                    bool v3548 = v3547 < 0.0f;
                                                    bool2 v3549 = v3548.xx;
                                                    v3526 = v3548 ? v3526 : v3546;
                                                    v3529 = v3548 ? v3546 : v3529;
                                                    v3531 = float2(v3549.x ? v3531.x : v3540.x, v3549.y ? v3531.y : v3540.y);
                                                    v3533 = float2(v3549.x ? v3540.x : v3533.x, v3549.y ? v3540.y : v3533.y);
                                                    v3535++;
                                                    continue;
                                                }
                                                float2 v3554 = lerp(v3533, v3531, clamp(v3529 / (v3529 - v3526), 0.0f, 1.0f).xx);
                                                float2 v3558 = float3(v3554.x, 0.4000000059604644775390625f, v3554.y).xz;
                                                float v3559 = distance(v3516, v3558);
                                                if (v3559 < 0.4000000059604644775390625f)
                                                {
                                                    break;
                                                }
                                                int v3565 = int(max(v3559 * 2.5f, 1.0f));
                                                uint v3567 = v3466;
                                                uint v3570 = v3467;
                                                bool v3572 = v3468;
                                                bool v3574 = v3469;
                                                int v3576 = 0;
                                                for (; v3576 < (v3565 + 1); )
                                                {
                                                    float v3582 = float(v3565);
                                                    int v3577 = v3576 + 1;
                                                    float2 v3589 = lerp(v3516, v3558, (((float(v3576) / v3582) + (float(v3577) / v3582)) * 0.5f).xx);
                                                    float v3590 = v3589.x;
                                                    float v3598 = v3589.y;
                                                    int v3600 = v367 + (v3576 * 100);
                                                    uint v3602 = uint(v3600 ^ 2346);
                                                    uint v3604 = v3602 + (v3602 << 10u);
                                                    uint v3606 = v3604 ^ (v3604 >> 6u);
                                                    uint v3608 = v3606 + (v3606 << 3u);
                                                    uint v3610 = v3608 ^ (v3608 >> 11u);
                                                    uint v3612 = v3610 + (v3610 << 15u);
                                                    bool v3613 = v3574 ? v3574 : true;
                                                    uint v3614 = v3574 ? v3570 : 8388607u;
                                                    bool v3615 = v3572 ? v3572 : true;
                                                    uint v3616 = v3572 ? v3567 : 1065353216u;
                                                    uint v3624 = uint(v3600 ^ 3459);
                                                    uint v3626 = v3624 + (v3624 << 10u);
                                                    uint v3628 = v3626 ^ (v3626 >> 6u);
                                                    uint v3630 = v3628 + (v3628 << 3u);
                                                    uint v3632 = v3630 ^ (v3630 >> 11u);
                                                    bool v3635 = v3613 ? v3613 : true;
                                                    uint v3636 = v3613 ? v3614 : 8388607u;
                                                    bool v3637 = v3615 ? v3615 : true;
                                                    uint v3638 = v3615 ? v3616 : 1065353216u;
                                                    float v3644 = ((asfloat(((v3632 + (v3632 << 15u)) & v3636) | v3638) - 1.0f) * 2.0f) - 1.0f;
                                                    uint v3646 = uint(v3600 ^ 234);
                                                    uint v3648 = v3646 + (v3646 << 10u);
                                                    uint v3650 = v3648 ^ (v3648 >> 6u);
                                                    uint v3652 = v3650 + (v3650 << 3u);
                                                    uint v3654 = v3652 ^ (v3652 >> 11u);
                                                    bool v3657 = v3635 ? v3635 : true;
                                                    uint v3658 = v3635 ? v3636 : 8388607u;
                                                    bool v3659 = v3637 ? v3637 : true;
                                                    uint v3660 = v3637 ? v3638 : 1065353216u;
                                                    float3 v3667 = float3(((asfloat((v3612 & v3614) | v3616) - 1.0f) * 2.0f) - 1.0f, v3644, ((asfloat(((v3654 + (v3654 << 15u)) & v3658) | v3660) - 1.0f) * 2.0f) - 1.0f);
                                                    v3667.y = v3644 * 0.20000000298023223876953125f;
                                                    float3 v3670 = normalize(v3667);
                                                    bool3 v3673 = (v3670.y < 0.0f).xxx;
                                                    float3 v3674 = float3(v3673.x ? float3(0.0f, -1.0f, 0.0f).x : float3(0.0f, 1.0f, 0.0f).x, v3673.y ? float3(0.0f, -1.0f, 0.0f).y : float3(0.0f, 1.0f, 0.0f).y, v3673.z ? float3(0.0f, -1.0f, 0.0f).z : float3(0.0f, 1.0f, 0.0f).z);
                                                    uint v3677 = uint(v3600 ^ 346);
                                                    uint v3679 = v3677 + (v3677 << 10u);
                                                    uint v3681 = v3679 ^ (v3679 >> 6u);
                                                    uint v3683 = v3681 + (v3681 << 3u);
                                                    uint v3685 = v3683 ^ (v3683 >> 11u);
                                                    bool v3688 = v3657 ? v3657 : true;
                                                    uint v3689 = v3657 ? v3658 : 8388607u;
                                                    bool v3690 = v3659 ? v3659 : true;
                                                    uint v3691 = v3659 ? v3660 : 1065353216u;
                                                    bool v3698 = v3688 ? v3688 : true;
                                                    uint v3699 = v3688 ? v3689 : 8388607u;
                                                    bool v3700 = v3690 ? v3690 : true;
                                                    uint v3701 = v3690 ? v3691 : 1065353216u;
                                                    float2 v3707 = lerp(0.800000011920928955078125f, 1.5f, asfloat((v3612 & v3699) | v3701) - 1.0f).xx;
                                                    float3 v3709 = float3(v3707.x, lerp(0.800000011920928955078125f, 1.2999999523162841796875f, asfloat(((v3685 + (v3685 << 15u)) & v3689) | v3691) - 1.0f).xxx.y, v3707.y) * 0.75f;
                                                    float4x4 v3735 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v3590), float4(0.0f, 1.0f, 0.0f, lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v3589 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x)), float4(0.0f, 0.0f, 1.0f, v3598), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(transpose(float4x4(float4(v3670, 0.0f), float4(v3674, 0.0f), float4(cross(v3670, v3674), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))), float4x4(float4(v3709.x, 0.0f, 0.0f, 0.0f), float4(0.0f, v3709.y, 0.0f, 0.0f), float4(0.0f, 0.0f, v3709.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                                    uint v3752;
                                                    __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, v3752);
                                                    float v3755 = 10.0f * (v3590 + v3598);
                                                    uint v3757 = uint(v3755 + 640.0f);
                                                    uint v3759 = v3757 + (v3757 << 10u);
                                                    uint v3761 = v3759 ^ (v3759 >> 6u);
                                                    uint v3763 = v3761 + (v3761 << 3u);
                                                    uint v3765 = v3763 ^ (v3763 >> 11u);
                                                    bool v3768 = v3698 ? v3698 : true;
                                                    uint v3769 = v3698 ? v3699 : 8388607u;
                                                    bool v3770 = v3700 ? v3700 : true;
                                                    uint v3771 = v3700 ? v3701 : 1065353216u;
                                                    uint v3777 = uint(v3755 + 987.0f);
                                                    uint v3779 = v3777 + (v3777 << 10u);
                                                    uint v3781 = v3779 ^ (v3779 >> 6u);
                                                    uint v3783 = v3781 + (v3781 << 3u);
                                                    uint v3785 = v3783 ^ (v3783 >> 11u);
                                                    uint v3571 = v3768 ? v3769 : 8388607u;
                                                    uint v3568 = v3770 ? v3771 : 1065353216u;
                                                    uint v3793 = uint(int(v3752));
                                                    float v3795[12] = { v3735[0].x, v3735[1].x, v3735[2].x, v3735[0].y, v3735[1].y, v3735[2].y, v3735[0].z, v3735[1].z, v3735[2].z, v3735[0].w, v3735[1].w, v3735[2].w };
                                                    Affine3Packed v3796 = { v3795 };
                                                    GardenStone v3797 = { v3796, float4(asfloat(((v3765 + (v3765 << 15u)) & v3769) | v3771) - 1.0f, asfloat(((v3785 + (v3785 << 15u)) & v3571) | v3568) - 1.0f, 0.0f, 0.0f) };
                                                    [unroll]
                                                    for (int _19ident = 0; _19ident < 12; _19ident++)
                                                    {
                                                        __alias_instance_buffer.Store<float>(_19ident * 4 + v3793 * 64 + 0, v3797.transform.data[_19ident]);
                                                    }
                                                    __alias_instance_buffer.Store<float4>(v3793 * 64 + 48, v3797.color);
                                                    v3567 = v3568;
                                                    v3570 = v3571;
                                                    v3572 = v3770 ? v3770 : true;
                                                    v3574 = v3768 ? v3768 : true;
                                                    v3576 = v3577;
                                                    continue;
                                                }
                                                break;
                                            } while(false);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        break;
    } while(false);
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_WorkGroupID = stage_input.gl_WorkGroupID;
    gl_LocalInvocationID = stage_input.gl_LocalInvocationID;
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    gl_LocalInvocationIndex = stage_input.gl_LocalInvocationIndex;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
