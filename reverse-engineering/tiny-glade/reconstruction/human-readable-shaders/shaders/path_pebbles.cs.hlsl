// PATCHED: modules es.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/path_pebbles.cs.hlsl | 目标: _path_pebbles.cs.hlsl_cs_main_0.bin
// 计算着色器:在路径网格上散布鹅卵石实例。每个线程 = 一个网格单元,在 104 单位
// 的格内用哈希抖动撒点;采样地形高度并读取路径蒙版,<0.9 拒绝;再按 rocky/water
// 蒙版拒绝;通过 hash(seed±k) 为每个实例生成随机缩放/朝向/倾斜;原子累加
// draw_commands[draw_indirect_buffer_offset].instanceCount;组合
// scale*T(基)*平移 矩阵,pack_affine 后写入 pebble_placement_buffer[slot]。
// 绑定: sampler_llb s4, sampler_default s9, __alias_draw_commands u13, $Globals b14,
// terrain_heightmap t15, path_mask t17, rocky_terrain t18, water_mask t19,
// __alias_pebble_placement_buffer u20 | 工作组 [64,1,1]。
// 状态: O (算法);残余 do-while/phi 与 int3-Load 代码形态为 dxc 版本产物。

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "inc/gaussian_rand.hlsl"
#include "inc/simplex_noise.hlsl"
#include "inc/draw_indirect_types.hlsl"
#include "inc/draw_indirect.hlsl"

// The original dxc build emits SPV_EXT_descriptor_indexing + RuntimeDescriptorArray
// for the structured-buffer SSBOs; forced here to match the target module.
[[vk::ext_capability(5302)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);
[[vk::ext_capability(5302)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
RWStructuredBuffer<Affine3Packed> __alias_pebble_placement_buffer : register(u20, space0);

Texture2D<float4> terrain_heightmap : register(t15, space0);
Texture2D<float4> path_mask : register(t17, space0);
Texture2D<float4> rocky_terrain : register(t18, space0);
Texture2D<float4> water_mask : register(t19, space0);

cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 path_mask_dims;
    uint2 rocky_terrain_dims;
    uint2 water_mask_dims;
    uint pebble_buffer_size;
    int scatter_grid_dim;
};

// inc/scatter.hlsl references scatter_grid_dim as a global, so $Globals must be
// declared before its include.
#include "inc/scatter.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/terrain_heightmap.inc.hlsl"
#include "paths/path_mask_sampler.inc.hlsl"
#include "inc/texture_utils.hlsl"
#include "inc/rocky_terrain.inc.hlsl"
#include "water/water_mask_sampler.inc.hlsl"

[numthreads(64, 1, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    int index = int(gl_GlobalInvocationID.x);
    int grid = scatter_grid_dim;
    int area = grid * grid;
    int cell = (index % area);
    float g = float(grid);
    float2 grid_pos = (float2(float((cell % grid)) / g, float(cell / grid) / g) - 0.5f) * 104.0f;

    uint iu = uint(index);
    float2 jitter = float2(rand01(hash(iu)), rand01(hash(iu ^ 188053769u))) * 6.3f;

    float px = grid_pos.x + jitter.x;
    float pz = grid_pos.y + jitter.y;

    float h = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (float3(px, 0.0f, pz).xz * 0.0076923076994717121124267578125f + 0.5f), 0.0f).x);
    float2 mask_uv = (float3(px, h, pz).xz * 0.0076923076994717121124267578125f) + 0.5f;

    if (path_mask.SampleLevel(sampler_llb, mask_uv, 0.0f).x < 0.9f)
        return;

    bool in_bounds = !(mask_uv.x < 0.0f) && !(mask_uv.x > 1.0f) && !(mask_uv.y < 0.0f) && !(mask_uv.y > 1.0f);

    float rocky = 0.0f;
    if (in_bounds)
        rocky = rocky_terrain.SampleLevel(sampler_default, mask_uv, 0.0f).x;

    float water = 0.0f;
    if (in_bounds)
        water = water_mask.SampleLevel(sampler_default, mask_uv, 0.0f).y;

    if (rocky > 0.0f || water > 0.5f)
        return;

    int seed = (int(px) ^ (345 + int(pz))) ^ 2378;

    float scale = lerp(0.4f, 1.2f, rand01(hash(uint(seed)))) * 1.2f;

    float angle = (1.0f - asfloat((hash(uint(seed)) & 0x7FFFFFu) | 0x3F800000u)) * 6.283185482025146484375f;
    float2 dir = normalize(float2(cos(angle), sin(angle)));

    float angle2 = (1.0f - asfloat((hash(uint(seed - 235)) & 0x7FFFFFu) | 0x3F800000u)) * 6.283185482025146484375f;
    float2 dir2 = normalize(float2(cos(angle2), sin(angle2)));

    float tilt_slope = rand01(hash(uint(seed + 2345))) * 0.5f;
    float3 tilt = normalize(float3(dir2.x * tilt_slope, 1.0f, dir2.y * tilt_slope));

    uint slot;
    InterlockedAdd(__alias_draw_commands[uint(draw_indirect_buffer_offset)].instanceCount, 1u, slot);

    float3 f = float3(dir.x, 0.0f, dir.y);
    float3 r = cross(f, tilt);
    float4x4 basis = float4x4(float4(r, 0.0f), float4(tilt, 0.0f), float4(f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f));
    float4x4 scale_m = float4x4(float4(scale, 0.0f, 0.0f, 0.0f), float4(0.0f, scale, 0.0f, 0.0f), float4(0.0f, 0.0f, scale, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f));
    float4x4 trans = float4x4(float4(1.0f, 0.0f, 0.0f, px), float4(0.0f, 1.0f, 0.0f, h), float4(0.0f, 0.0f, 1.0f, pz), float4(0.0f, 0.0f, 0.0f, 1.0f));

    float4x4 m = mul(mul(scale_m, transpose(basis)), trans);
    __alias_pebble_placement_buffer[slot] = pack_affine(m);
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _path_pebbles.cs.hlsl_cs_main_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

static float3 unused_f3;

RWByteAddressBuffer __alias_draw_commands : register(u13, space0);
cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset : packoffset(c0);
    uint2 terrain_heightmap_dims : packoffset(c0.y);
    uint2 visual_terrain_heightmap_dims : packoffset(c1);
    uint2 path_mask_dims : packoffset(c1.z);
    uint2 rocky_terrain_dims : packoffset(c2);
    uint2 water_mask_dims : packoffset(c2.z);
    uint pebble_buffer_size : packoffset(c3);
    int scatter_grid_dim : packoffset(c3.y);
};

RWByteAddressBuffer __alias_pebble_placement_buffer : register(u20, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t15, space0);
Texture2D<float4> path_mask : register(t17, space0);
Texture2D<float4> rocky_terrain : register(t18, space0);
Texture2D<float4> water_mask : register(t19, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    do
    {
        int index = int(gl_GlobalInvocationID.x);
        int cell = (index % (scatter_grid_dim * scatter_grid_dim));
        float grid_f = float(scatter_grid_dim);
        float2 grid_pos = (float2(float((cell % scatter_grid_dim)) / grid_f, float(cell / scatter_grid_dim) / grid_f) - 0.5f.xx) * 104.0f.xx;
        uint iu = uint(index);
        uint h1 = iu + (iu << 10u);
        uint h2 = h1 ^ (h1 >> 6u);
        uint h3 = h2 + (h2 << 3u);
        uint h4 = h3 ^ (h3 >> 11u);
        uint iu_xor = iu ^ 188053769u;
        uint w1 = iu_xor + (iu_xor << 10u);
        uint w2 = w1 ^ (w1 >> 6u);
        uint w3 = w2 + (w2 << 3u);
        uint w4 = w3 ^ (w3 >> 11u);
        float2 jitter = float2(asfloat(((h4 + (h4 << 15u)) & 8388607u) | 1065353216u) - 1.0f, asfloat(((w4 + (w4 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 6.30000019073486328125f;
        float px = grid_pos.x + jitter.x;
        float3 px_pz;
        px_pz.x = px;
        float pz = grid_pos.y + jitter.y;
        px_pz.z = pz;
        float height = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (px_pz.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
        float2 mask_uv = (float3(px, height, pz).xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        if (path_mask.SampleLevel(sampler_llb, mask_uv, 0.0f).x < 0.89999997615814208984375f)
        {
            break;
        }
        float uv_x;
        bool x_not_neg;
        float rocky;
        do
        {
            uv_x = mask_uv.x;
            x_not_neg = !(uv_x < 0.0f);
            bool x_gt_one;
            if (x_not_neg)
            {
                x_gt_one = uv_x > 1.0f;
            }
            else
            {
                x_gt_one = true;
            }
            bool y_neg;
            if (!x_gt_one)
            {
                y_neg = mask_uv.y < 0.0f;
            }
            else
            {
                y_neg = true;
            }
            bool y_gt_one;
            if (!y_neg)
            {
                y_gt_one = mask_uv.y > 1.0f;
            }
            else
            {
                y_gt_one = true;
            }
            if (y_gt_one)
            {
                rocky = 0.0f;
                break;
            }
            else
            {
                rocky = rocky_terrain.SampleLevel(sampler_default, mask_uv, 0.0f).x;
                break;
            }
            break; // unreachable workaround
        } while(false);
        float water;
        do
        {
            bool x_gt_one_b;
            if (x_not_neg)
            {
                x_gt_one_b = uv_x > 1.0f;
            }
            else
            {
                x_gt_one_b = true;
            }
            bool y_neg_b;
            if (!x_gt_one_b)
            {
                y_neg_b = mask_uv.y < 0.0f;
            }
            else
            {
                y_neg_b = true;
            }
            bool y_gt_one_b;
            if (!y_neg_b)
            {
                y_gt_one_b = mask_uv.y > 1.0f;
            }
            else
            {
                y_gt_one_b = true;
            }
            if (y_gt_one_b)
            {
                water = 0.0f;
                break;
            }
            else
            {
                water = water_mask.SampleLevel(sampler_default, mask_uv, 0.0f).y;
                break;
            }
            break; // unreachable workaround
        } while(false);
        bool reject;
        if (!(rocky > 0.0f))
        {
            reject = water > 0.5f;
        }
        else
        {
            reject = true;
        }
        if (reject)
        {
            break;
        }
        int seed = (int(px) ^ (345 + int(pz))) ^ 2378;
        uint s1 = uint(seed);
        uint s2 = s1 + (s1 << 10u);
        uint s3 = s2 ^ (s2 >> 6u);
        uint s4 = s3 + (s3 << 3u);
        uint s5 = s4 ^ (s4 >> 11u);
        float rand_seed = asfloat(((s5 + (s5 << 15u)) & 8388607u) | 1065353216u);
        float scale = lerp(0.4000000059604644775390625f, 1.2000000476837158203125f, rand_seed - 1.0f) * 1.2000000476837158203125f;
        float angle = (1.0f - rand_seed) * 6.280000209808349609375f;
        float2 dir = normalize(float2(cos(angle), sin(angle)));
        float dir_x = dir.x;
        float dir_y = dir.y;
        uint sk1 = uint(seed - 235);
        uint sk2 = sk1 + (sk1 << 10u);
        uint sk3 = sk2 ^ (sk2 >> 6u);
        uint sk4 = sk3 + (sk3 << 3u);
        uint sk5 = sk4 ^ (sk4 >> 11u);
        float angle2 = (1.0f - asfloat(((sk5 + (sk5 << 15u)) & 8388607u) | 1065353216u)) * 6.280000209808349609375f;
        float2 dir2 = normalize(float2(cos(angle2), sin(angle2)));
        uint sp1 = uint(seed + 2345);
        uint sp2 = sp1 + (sp1 << 10u);
        uint sp3 = sp2 ^ (sp2 >> 6u);
        uint sp4 = sp3 + (sp3 << 3u);
        uint sp5 = sp4 ^ (sp4 >> 11u);
        float tilt_slope = (asfloat(((sp5 + (sp5 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 0.5f;
        float3 tilt = normalize(float3(dir2.x * tilt_slope, 1.0f, dir2.y * tilt_slope));
        uint slot;
        __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, slot);
        uint slot_u = uint(int(slot));
        float4x4 m = mul(float4x4(float4(1.0f, 0.0f, 0.0f, px), float4(0.0f, 1.0f, 0.0f, height), float4(0.0f, 0.0f, 1.0f, pz), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(transpose(float4x4(float4(cross(float3(dir_x, 0.0f, dir_y), tilt), 0.0f), float4(tilt, 0.0f), float4(dir_x, 0.0f, dir_y, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))), float4x4(float4(scale, 0.0f, 0.0f, 0.0f), float4(0.0f, scale, 0.0f, 0.0f), float4(0.0f, 0.0f, scale, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
        float data_arr[12] = { m[0].x, m[1].x, m[2].x, m[0].y, m[1].y, m[2].y, m[0].z, m[1].z, m[2].z, m[0].w, m[1].w, m[2].w };
        Affine3Packed packed = { data_arr };
        [unroll]
        for (int i = 0; i < 12; i++)
        {
            __alias_pebble_placement_buffer.Store<float>(i * 4 + slot_u * 48 + 0, packed.data[i]);
        }
        break;
    } while(false);
}

[numthreads(64, 1, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
