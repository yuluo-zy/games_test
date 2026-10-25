// ==RECONSTRUCTED== shaders/paths/path_mask_visual.cs.hlsl
// 目标:_paths_path_mask_visual.cs.hlsl_cs_main_0.bin
// 依据:spirv-cross + SPIR-V 反汇编(由 gid 经 paths_visual_dims*65 推世界 xz;
// terrain_heightmap GatherRed 坡度 -> offset_scale=lerp(1,0,clamp);
// 4 组旋转 2 八度值噪声 fbm 偏移分别采样 path/cobble1/2/3 模糊遮罩 SampleLevel;
// 第二次 GatherRed + 水面遮罩边界保护采样 -> rocky/water smoothstep 因子;
// paths_visual RGBA 读改写 + 取各遮罩最大值的 dirt_mask 写)
// 绑定:sampler_lnc s0, sampler_default s9, $Globals b13, terrain_heightmap t14, water_mask t16,
//       paths_visual u17, dirt_mask u18, path_mask_blurred t19, cobblestone1_mask_blurred t20,
//       cobblestone2_mask_blurred t21, cobblestone3_mask_blurred t22
// local_size [8,8,1], gid int3;存储格式(取自目标 OpTypeImage):
// paths_visual Rgba16f -> RWTexture2D<half4>, dirt_mask R16f -> RWTexture2D<half>
// 状态:O (算法);残留 int3-Load/工具链 + phi/控制流 codegen + inc 闭包差异

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/math.hlsl"
#include "inc/math_const.hlsl"
#include "inc/consts.hlsl"
#include "inc/utils.hlsl"
#include "inc/hash.hlsl"
#include "inc/texture_utils.hlsl"
Texture2D<float4> terrain_heightmap : register(t14, space0);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/uv.hlsl"
#include "inc/perlin.hlsl"
#include "inc/fbm_noise.hlsl"
#include "inc/rocky_terrain.inc.hlsl"
#include "water/water_mask_sampler.inc.hlsl"

// (采样器 sampler_lnc / sampler_default 来自 inc/common.hlsl -> inc/samplers.hlsl)

Texture2D<float4> water_mask : register(t16, space0);
RWTexture2D<half4> paths_visual : register(u17, space0);
RWTexture2D<half> dirt_mask : register(u18, space0);
Texture2D<float4> path_mask_blurred : register(t19, space0);
Texture2D<float4> cobblestone1_mask_blurred : register(t20, space0);
Texture2D<float4> cobblestone2_mask_blurred : register(t21, space0);
Texture2D<float4> cobblestone3_mask_blurred : register(t22, space0);

cbuffer $Globals : register(b13, space0)
{
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 water_mask_dims;
    uint2 paths_visual_dims;
    uint2 dirt_mask_dims;
    uint2 path_mask_blurred_dims;
    uint2 cobblestone1_mask_blurred_dims;
    uint2 cobblestone2_mask_blurred_dims;
    uint2 cobblestone3_mask_blurred_dims;
    uint2 rocky_terrain_dims;
};

// 2 八度旋转值噪声:原 SPIR-V 通过 phi 选择把 LCG/rand 状态
// (1664525, 1013904223, 0x7FFFFF, 0x3F800000)带入循环;
// 携带值为编译期常量,故下面循环就是该状态携带循环的精确算术
// (晶格 = inc/perlin 哈希 + rand01,smoothstep 双线性混合,amp*=0.55,
// 按 36.87 度旋转 / 频率 1.9 缩放)。
float vnoise_fbm(float2 pos)
{
    const float2x2 rot = float2x2(float2(0.800000011920928955078125f, -0.60000002384185791015625f),
                                  float2(0.60000002384185791015625f, 0.800000011920928955078125f));
    float value = 0.0f;
    float amp = 0.5f;
    for (int i = 0; i < 2; i++)
    {
        int2 cell = int2(floor(pos));
        float2 f = frac(pos);
        float2 u = (f * f) * (3.0f.xx - (f * 2.0f));

        float n00 = incb_perlin_hash01(hash(uint2(cell + int2(0, 0))));
        float n10 = incb_perlin_hash01(hash(uint2(cell + int2(1, 0))));
        float n01 = incb_perlin_hash01(hash(uint2(cell + int2(0, 1))));
        float n11 = incb_perlin_hash01(hash(uint2(cell + int2(1, 1))));

        float x0 = 1.0f - u.x;
        float y0 = 1.0f - u.y;
        float n = (y0 * ((n00 * x0) + (n10 * u.x)))
                + ((n01 * x0) * u.y)
                + ((n11 * u.x) * u.y);

        value += amp * ((-1.0f) + (2.0f * n));
        amp *= 0.550000011920928955078125f;
        pos = mul(rot, pos * 1.89999997615814208984375f);
    }
    return value;
}

[numthreads(8, 8, 1)]
void cs_main(int3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    uint2 p = uint2(gl_GlobalInvocationID.xy);

    // 像素中心 -> 经 paths_visual dims 的归一化 NDC [-65,65] 世界空间。
    float2 uv_center = float2(gl_GlobalInvocationID.xy) + 0.5f.xx;
    float2 world = (((uv_center / float2(int2(paths_visual_dims))) * 2.0f) - 1.0f.xx) * 65.0f.xx;
    float2 world_xz = float3(world.x, 0.0f, world.y).xz;

    // 4 点 GatherRed 求地形坡度(中心差分 * 26.5 / (130/dims))。
    float2 thm_dims = float2(terrain_heightmap_dims);
    float2 grad_scale = 130.0f.xx / thm_dims;
    float4 g0 = terrain_heightmap.GatherRed(sampler_default, uv_center / thm_dims, int2(0, 0));
    float2 grad0 = (float2((g0.z + g0.y) - (g0.w + g0.x), (g0.x + g0.y) - (g0.w + g0.z)) * 0.5f.xx) * 26.5f;
    // 坡度越大,噪声偏移幅度越小(offset_scale 1->0)。
    float offset_scale = lerp(1.0f, 0.0f, clamp(length(grad0 / grad_scale), 0.0f, 1.0f));

    // 4 组独立值噪声偏移(每组对应一张模糊覆盖遮罩),
    // 每个都在 (world_xz + ((n*0.5)*offset_scale*1.2)) * (1/130) + 0.5 处采样。
    float2 fbm_base = (world_xz * 0.75f) + 7.0f.xx;

    float n0 = vnoise_fbm(fbm_base);
    float path_cov = path_mask_blurred.SampleLevel(sampler_lnc,
        ((world_xz + (((n0 * 0.5f) * offset_scale) * 1.2000000476837158203125f).xx) * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x;

    float n1 = vnoise_fbm(fbm_base);
    float c1_cov = cobblestone1_mask_blurred.SampleLevel(sampler_lnc,
        ((world_xz + (((n1 * 0.5f) * offset_scale) * 1.2000000476837158203125f).xx) * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x;

    float n2 = vnoise_fbm(fbm_base);
    float c2_cov = cobblestone2_mask_blurred.SampleLevel(sampler_lnc,
        ((world_xz + (((n2 * 0.5f) * offset_scale) * 1.2000000476837158203125f).xx) * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x;

    float n3 = vnoise_fbm(fbm_base);
    float c3_cov = cobblestone3_mask_blurred.SampleLevel(sampler_lnc,
        ((world_xz + (((n3 * 0.5f) * offset_scale) * 1.2000000476837158203125f).xx) * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x;

    // 岩石 / 水面因子:第二次 GatherRed 的坡度 smoothstep,加一个被 [0,1] uv
    // 边界保护的 water_mask 采样。原 SPIR-V 此处为 2x FOrdLessThan + 2x FOrdGreaterThan
    // + LogicalNot;当前 dxc 把范围测试折叠为 FOrd>=/<=,属于 dxc 版本 codegen 残留。
    float2 uv_ws = (world_xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
    float4 g1 = terrain_heightmap.GatherRed(sampler_default, uv_ws, int2(0, 0));
    float2 grad1 = (float2((g1.z + g1.y) - (g1.w + g1.x), (g1.x + g1.y) - (g1.w + g1.z)) * 0.5f.xx) * 26.5f;
    float water = 0.0f;
    if (uv_ws.x >= 0.0f && uv_ws.x <= 1.0f && uv_ws.y >= 0.0f && uv_ws.y <= 1.0f)
    {
        water = water_mask.SampleLevel(sampler_default, uv_ws, 0.0f).y;
    }
    float rocky_water = smoothstep(0.5f, 1.0f, length(grad1 / grad_scale))
                      + smoothstep(0.0f, 0.449999988079071044921875f, water);

    // paths_visual RGBA(逐通道读改写,对应原版的 4 对独立 ImageRead/ImageWrite);
    // dirt_mask = 各覆盖遮罩最大值。
    float4 outv = paths_visual[p];
    outv.x = path_cov;
    paths_visual[p] = outv;

    outv = paths_visual[p];
    outv.y = clamp(c1_cov - rocky_water, 0.0f, 1.0f);
    paths_visual[p] = outv;

    outv = paths_visual[p];
    outv.z = clamp(c2_cov - rocky_water, 0.0f, 1.0f);
    paths_visual[p] = outv;

    outv = paths_visual[p];
    outv.w = clamp(c3_cov - rocky_water, 0.0f, 1.0f);
    paths_visual[p] = outv;

    dirt_mask[p] = max(max(path_cov, c1_cov), max(c2_cov, c3_cov));
}
