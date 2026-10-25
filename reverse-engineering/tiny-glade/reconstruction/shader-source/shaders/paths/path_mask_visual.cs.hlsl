// ==RECONSTRUCTED== shaders/paths/path_mask_visual.cs.hlsl | target: _paths_path_mask_visual.cs.hlsl_cs_main_0.bin | evidence: spirv-cross + SPIR-V disasm (world_xz from gid via paths_visual_dims*65; terrain_heightmap GatherRed slope -> offset_scale=lerp(1,0,clamp); 4x rotated 2-octave value-noise fbm offsets for path/cobble1/2/3 blurred-mask SampleLevel; second GatherRed + water-mask bounds-guarded sample -> rocky/water smoothstep factor; RGBA paths_visual read-modify-write + max-of-masks dirt_mask write) | bindings: sampler_lnc s0, sampler_default s9, $Globals b13, terrain_heightmap t14, water_mask t16, paths_visual u17, dirt_mask u18, path_mask_blurred t19, cobblestone1_mask_blurred t20, cobblestone2_mask_blurred t21, cobblestone3_mask_blurred t22 | local_size [8,8,1], gid int3; storage formats (from target OpTypeImage): paths_visual Rgba16f -> RWTexture2D<half4>, dirt_mask R16f -> RWTexture2D<half> | status: O (algorithm); residual int3-Load/toolchain + phi/control-flow codegen + inc-closure diffs

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

// (samplers sampler_lnc / sampler_default come from inc/common.hlsl -> inc/samplers.hlsl)

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

// 2-octave rotated value noise: the original SPIR-V carries an LCG/rand state
// (1664525, 1013904223, 0x7FFFFF, 0x3F800000) through the loop via phi selects;
// the carried values are compile-time constants, so the loop below is the exact
// arithmetic of that state-carrying loop (lattice = inc/perlin hash + rand01,
// smoothstep bilinear blend, amp*=0.55, rotate by 36.87 deg / scale 1.9).
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

    // Pixel center -> normalized NDC [-65,65] world space via paths_visual dims.
    float2 uv_center = float2(gl_GlobalInvocationID.xy) + 0.5f.xx;
    float2 world = (((uv_center / float2(int2(paths_visual_dims))) * 2.0f) - 1.0f.xx) * 65.0f.xx;
    float2 world_xz = float3(world.x, 0.0f, world.y).xz;

    // Terrain slope from a 4-tap GatherRed (central difference * 26.5 / (130/dims)).
    float2 thm_dims = float2(terrain_heightmap_dims);
    float2 grad_scale = 130.0f.xx / thm_dims;
    float4 g0 = terrain_heightmap.GatherRed(sampler_default, uv_center / thm_dims, int2(0, 0));
    float2 grad0 = (float2((g0.z + g0.y) - (g0.w + g0.x), (g0.x + g0.y) - (g0.w + g0.z)) * 0.5f.xx) * 26.5f;
    float offset_scale = lerp(1.0f, 0.0f, clamp(length(grad0 / grad_scale), 0.0f, 1.0f));

    // 4 independent value-noise offsets (one per blurred coverage mask), each
    // sampled at (world_xz + ((n*0.5)*offset_scale*1.2)) * (1/130) + 0.5.
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

    // Rocky / water factor: slope smoothstep from a second GatherRed, plus a
    // water-mask sample guarded to the [0,1] uv box. The original SPIR-V has
    // 2x FOrdLessThan + 2x FOrdGreaterThan + LogicalNot here; the current dxc
    // folds the range test into FOrd>=/<=, a dxc-version codegen residual.
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

    // paths_visual RGBA (read-modify-write per channel, matching the 4 separate
    // ImageRead/ImageWrite pairs of the original); dirt_mask = max coverage.
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
